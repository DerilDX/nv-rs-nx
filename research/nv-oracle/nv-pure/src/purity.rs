//! The exe-side purity facts (`research/ghidra/NvPurity.java`) and the
//! transitive closure over the call graph.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct Facts {
    pub ret_pop: i64,
    pub calls: Vec<u32>,
    pub indirect: u32,
    pub imports: u32,
    pub w_global: u32,
    pub w_pointer: u32,
    pub w_implicit: u32,
    pub r_pointer: u32,
    pub fs: u32,
    /// `ECX`/`EDX` read before written in the entry block.
    pub reg_in: Vec<String>,
    /// Stack argument bytes the body reads (lower bound); -1 unknown.
    pub stack_args: i64,
    /// Constants the body compares and computes with, as input tokens:
    /// `i<decimal>`, `f<8 hex>` (float bits), `d<16 hex>` (double bits).
    /// Empty when the table predates the `consts_*` columns.
    pub consts: Vec<String>,
}

impl Facts {
    /// Nothing but its own frame: no stores outside it, no unknown or
    /// imported calls, no TLS/SEH.
    pub fn locally_pure(&self) -> bool {
        self.indirect == 0
            && self.imports == 0
            && self.w_global == 0
            && self.w_pointer == 0
            && self.w_implicit == 0
            && self.fs == 0
    }
}

fn hex(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim(), 16).ok()
}

/// Parses NvPurity's TSV (header row first).
pub fn parse(text: &str) -> Result<BTreeMap<u32, Facts>, String> {
    let mut lines = text.lines();
    let header: Vec<&str> = lines
        .next()
        .ok_or("empty purity table")?
        .split('\t')
        .collect();
    let col = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .ok_or_else(|| format!("purity table has no column {name} (rerun NvPurity.java)"))
    };
    let (c_entry, c_ret, c_calls, c_tails) = (
        col("entry")?,
        col("ret_pop")?,
        col("calls")?,
        col("tail_calls")?,
    );
    let (c_ind, c_imp, c_wg, c_wp, c_wi) = (
        col("indirect")?,
        col("imports")?,
        col("w_global")?,
        col("w_pointer")?,
        col("w_implicit")?,
    );
    let (c_rp, c_fs, c_reg, c_sa) = (
        col("r_pointer")?,
        col("fs")?,
        col("reg_in")?,
        col("stack_args")?,
    );
    let c_ci = header.iter().position(|h| *h == "consts_int");
    let c_cf = header.iter().position(|h| *h == "consts_float");
    let mut out = BTreeMap::new();
    for (n, l) in lines.enumerate() {
        let c: Vec<&str> = l.split('\t').collect();
        let bad = || format!("purity table line {}: malformed", n + 2);
        let num = |i: usize| {
            c.get(i)
                .and_then(|v| v.trim().parse::<i64>().ok())
                .ok_or_else(bad)
        };
        let list = |i: usize| -> Vec<u32> {
            c.get(i)
                .map(|v| {
                    v.split(',')
                        .filter(|s| !s.is_empty())
                        .filter_map(hex)
                        .collect()
                })
                .unwrap_or_default()
        };
        let entry = c.get(c_entry).and_then(|v| hex(v)).ok_or_else(bad)?;
        let mut calls = list(c_calls);
        calls.extend(list(c_tails));
        out.insert(
            entry,
            Facts {
                ret_pop: num(c_ret)?,
                calls,
                indirect: num(c_ind)? as u32,
                imports: num(c_imp)? as u32,
                w_global: num(c_wg)? as u32,
                w_pointer: num(c_wp)? as u32,
                w_implicit: num(c_wi)? as u32,
                r_pointer: num(c_rp)? as u32,
                fs: num(c_fs)? as u32,
                reg_in: c
                    .get(c_reg)
                    .map(|v| {
                        v.split(',')
                            .filter(|s| !s.is_empty())
                            .map(str::to_string)
                            .collect()
                    })
                    .unwrap_or_default(),
                stack_args: num(c_sa)?,
                consts: {
                    let field = |i: Option<usize>| -> Vec<&str> {
                        i.and_then(|i| c.get(i))
                            .map(|v| v.split(',').filter(|s| !s.is_empty()).collect())
                            .unwrap_or_default()
                    };
                    let mut k: Vec<String> = field(c_ci).iter().map(|v| format!("i{v}")).collect();
                    k.extend(field(c_cf).iter().map(|v| v.to_string()));
                    k
                },
            },
        );
    }
    Ok(out)
}

/// Functions that are locally pure and whose callees are all (transitively)
/// pure; a callee missing from the table counts as impure. Greatest fixed
/// point, so mutually recursive pure functions stay pure.
pub fn transitively_pure(facts: &BTreeMap<u32, Facts>) -> BTreeSet<u32> {
    let mut pure: BTreeSet<u32> = facts
        .iter()
        .filter(|(_, f)| f.locally_pure())
        .map(|(a, _)| *a)
        .collect();
    loop {
        let drop: Vec<u32> = pure
            .iter()
            .copied()
            .filter(|a| facts[a].calls.iter().any(|c| !pure.contains(c)))
            .collect();
        if drop.is_empty() {
            return pure;
        }
        for a in drop {
            pure.remove(&a);
        }
    }
}

/// Every function reachable from `a` through calls (not including `a`).
pub fn callees(facts: &BTreeMap<u32, Facts>, a: u32) -> BTreeSet<u32> {
    let mut seen = BTreeSet::new();
    let mut stack: Vec<u32> = facts.get(&a).map(|f| f.calls.clone()).unwrap_or_default();
    while let Some(c) = stack.pop() {
        if c != a && seen.insert(c) {
            if let Some(f) = facts.get(&c) {
                stack.extend(f.calls.iter().copied());
            }
        }
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: &str = "entry\tsize\tret_pop\tebp_frame\tcalls\ttail_calls\tindirect\timports\tw_stack\tw_global\tw_pointer\tw_implicit\tr_pointer\tfs\treg_in\tstack_args\n\
00401000\t10\t0\t1\t00401020\t\t0\t0\t1\t0\t0\t0\t0\t0\t\t4\n\
00401020\t10\t4\t1\t\t\t0\t0\t1\t0\t0\t0\t0\t0\tECX\t4\n\
00401040\t10\t0\t1\t00401060\t\t0\t0\t1\t0\t0\t0\t0\t0\t\t0\n\
00401060\t10\t0\t1\t\t\t0\t0\t1\t0\t1\t0\t0\t0\t\t0\n";

    #[test]
    fn closure() {
        let f = parse(T).unwrap();
        assert_eq!(f[&0x0040_1020].reg_in, vec!["ECX".to_string()]);
        let p = transitively_pure(&f);
        assert!(p.contains(&0x0040_1000) && p.contains(&0x0040_1020));
        assert!(!p.contains(&0x0040_1060), "stores through a pointer");
        assert!(!p.contains(&0x0040_1040), "calls an impure function");
        assert_eq!(
            callees(&f, 0x0040_1000).into_iter().collect::<Vec<_>>(),
            vec![0x0040_1020]
        );
    }
}
