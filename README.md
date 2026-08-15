# axiom-kv-lab

A concrete self-evolution experiment rather than a CRUD demo. Two storage implementations compete against a
specification. The first is intentionally volatile and fails crash persistence; the evolution loop captures a
counterexample and promotes the journaled implementation only after every scenario passes.

> **Maturity:** research prototype v0.1. The default verifier proves properties by exhaustive evaluation over an
> explicitly finite input domain. A VALID receipt is therefore a theorem about that bounded model, not a claim of
> unbounded program correctness.

```bash
cargo run -- evolve
```

Expected transcript includes a rejected `volatile-v1`, a crash-recovery counterexample, and promotion of `journaled-v2`.
