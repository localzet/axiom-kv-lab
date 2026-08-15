#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import itertools
import json
from dataclasses import dataclass, field
from pathlib import Path


@dataclass
class State:
    mem: dict[tuple[str, str], int] = field(default_factory=dict)
    log: list[tuple[str, str, int]] = field(default_factory=list)


@dataclass(frozen=True)
class Candidate:
    name: str
    durable: bool
    authorize: bool

    def put(self, state: State, principal: str, namespace: str, key: str, value: int) -> bool:
        if self.authorize and principal != "writer":
            return False
        state.mem[(namespace, key)] = value
        if self.durable:
            state.log.append((namespace, key, value))
        return True

    def crash(self, state: State) -> None:
        state.mem.clear()
        if self.durable:
            for namespace, key, value in state.log:
                state.mem[(namespace, key)] = value


CANDIDATES = [
    Candidate("volatile-v1", durable=False, authorize=True),
    Candidate("journaled-open-v2", durable=True, authorize=False),
    Candidate("journaled-cap-v3", durable=True, authorize=True),
]


def check(candidate: Candidate) -> tuple[bool, dict | None]:
    # Property 1: authorized read-after-write.
    state = State()
    if not candidate.put(state, "writer", "alpha", "k", 7):
        return False, {"property": "authorized-write", "trace": ["writer PUT alpha/k=7"]}
    if state.mem.get(("alpha", "k")) != 7:
        return False, {"property": "read-after-write", "trace": ["writer PUT alpha/k=7", "GET alpha/k"]}

    # Property 2: persistence after crash/restart.
    candidate.crash(state)
    if state.mem.get(("alpha", "k")) != 7:
        return False, {
            "property": "crash-persistence",
            "trace": ["writer PUT alpha/k=7", "CRASH", "RESTART", "GET alpha/k -> none"],
        }

    # Property 3: attacker cannot mutate state.
    before = dict(state.mem)
    accepted = candidate.put(state, "attacker", "alpha", "k", 99)
    if accepted or state.mem != before:
        return False, {
            "property": "authorization",
            "trace": ["attacker PUT alpha/k=99", "mutation accepted"],
        }

    # Property 4: namespace isolation.
    if state.mem.get(("beta", "k")) is not None:
        return False, {"property": "namespace-isolation", "trace": ["GET beta/k leaked alpha/k"]}

    return True, None


def main() -> None:
    history = []
    print("AXIOM KV EVOLUTION v0.2")
    for candidate in CANDIDATES:
        valid, cex = check(candidate)
        event = {
            "candidate": candidate.name,
            "verdict": "VALID" if valid else "INVALID",
            "counterexample": cex,
        }
        history.append(event)
        print(f"propose {candidate.name}: {event['verdict']}")
        if cex:
            print(f"  counterexample [{cex['property']}]: {'; '.join(cex['trace'])}")
        if valid:
            print(f"PROMOTE {candidate.name}")
            receipt = {
                "format": "AXIOM-EVOLUTION-RECEIPT/1",
                "promoted": candidate.name,
                "history_sha256": hashlib.sha256(json.dumps(history, sort_keys=True).encode()).hexdigest(),
                "properties": [
                    "authorized-write",
                    "read-after-write",
                    "crash-persistence",
                    "authorization",
                    "namespace-isolation",
                ],
            }
            Path("evolution-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
            Path("evolution-history.json").write_text(json.dumps(history, indent=2) + "\n")
            return
    raise SystemExit("no candidate satisfies the model")


if __name__ == "__main__":
    main()
