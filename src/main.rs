use anyhow::{bail, Result};
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
struct Request {
    principal: &'static str,
    namespace: &'static str,
    key: &'static str,
    value: Option<i64>,
}
trait KvCandidate {
    fn name(&self) -> &'static str;
    fn put(&mut self, r: &Request) -> bool;
    fn get(&self, ns: &str, key: &str) -> Option<i64>;
    fn crash_restart(&mut self);
}
#[derive(Default)]
struct Volatile {
    mem: BTreeMap<(String, String), i64>,
}
impl KvCandidate for Volatile {
    fn name(&self) -> &'static str {
        "volatile-v1"
    }
    fn put(&mut self, r: &Request) -> bool {
        if r.principal != "writer" {
            return false;
        }
        self.mem
            .insert((r.namespace.into(), r.key.into()), r.value.unwrap());
        true
    }
    fn get(&self, ns: &str, key: &str) -> Option<i64> {
        self.mem.get(&(ns.into(), key.into())).copied()
    }
    fn crash_restart(&mut self) {
        self.mem.clear();
    }
}
#[derive(Default)]
struct Journaled {
    mem: BTreeMap<(String, String), i64>,
    log: Vec<((String, String), i64)>,
}
impl KvCandidate for Journaled {
    fn name(&self) -> &'static str {
        "journaled-v2"
    }
    fn put(&mut self, r: &Request) -> bool {
        if r.principal != "writer" {
            return false;
        }
        let item = ((r.namespace.into(), r.key.into()), r.value.unwrap());
        self.log.push(item.clone());
        self.mem.insert(item.0, item.1);
        true
    }
    fn get(&self, ns: &str, key: &str) -> Option<i64> {
        self.mem.get(&(ns.into(), key.into())).copied()
    }
    fn crash_restart(&mut self) {
        self.mem.clear();
        for (k, v) in self.log.clone() {
            self.mem.insert(k, v);
        }
    }
}
#[derive(Debug)]
struct Counterexample {
    property: &'static str,
    trace: String,
}
fn verify(c: &mut dyn KvCandidate) -> std::result::Result<(), Counterexample> {
    let w = Request {
        principal: "writer",
        namespace: "alpha",
        key: "k",
        value: Some(7),
    };
    if !c.put(&w) {
        return Err(Counterexample {
            property: "authorized write",
            trace: "writer PUT alpha/k=7 was denied".into(),
        });
    }
    if c.get("alpha", "k") != Some(7) {
        return Err(Counterexample {
            property: "read-after-write",
            trace: "PUT 7; GET != 7".into(),
        });
    }
    let attacker = Request {
        principal: "attacker",
        namespace: "alpha",
        key: "k",
        value: Some(99),
    };
    if c.put(&attacker) {
        return Err(Counterexample {
            property: "authorization",
            trace: "attacker mutated alpha/k".into(),
        });
    }
    if c.get("beta", "k").is_some() {
        return Err(Counterexample {
            property: "namespace isolation",
            trace: "alpha/k leaked into beta/k".into(),
        });
    }
    c.crash_restart();
    if c.get("alpha", "k") != Some(7) {
        return Err(Counterexample {
            property: "crash persistence",
            trace: "PUT alpha/k=7; CRASH; RESTART; GET alpha/k -> none".into(),
        });
    }
    Ok(())
}
fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() != Some("evolve") {
        bail!("usage: axiom-kv-lab evolve");
    }
    println!("AXIOM EVOLUTION RUN");
    println!("spec: authorized writes + namespace isolation + crash persistence\n");
    let mut candidates: Vec<Box<dyn KvCandidate>> = vec![
        Box::new(Volatile::default()),
        Box::new(Journaled::default()),
    ];
    for mut c in candidates.drain(..) {
        println!("propose {}", c.name());
        match verify(c.as_mut()) {
            Ok(()) => {
                println!("  VERIFIED -> PROMOTE {}", c.name());
                println!("evolution complete");
                return Ok(());
            }
            Err(e) => {
                println!("  REJECTED property={}", e.property);
                println!("  counterexample: {}", e.trace);
                println!("  synthesizer receives counterexample and searches next architecture\n")
            }
        }
    }
    bail!("no candidate satisfies specification")
}
