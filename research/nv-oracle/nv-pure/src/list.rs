//! Which translations the oracle check can run: translated, complete, pure in
//! the exe (transitively), with a signature whose arguments `nv-call` can
//! pass and whose calling convention agrees with the exe's own stack use.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::purity::{self, Facts};
use crate::source::{Source, Ty};

/// One checkable function.
#[derive(Debug, Clone)]
pub struct Pure {
    pub addr: u32,
    pub cc: &'static str,
    pub params: Vec<Ty>,
    pub ret: Ty,
    pub unit: String,
    /// Every function it reaches (all must be translated for the Rust side).
    pub callees: Vec<u32>,
    /// Constants of the function and its callees (input tokens, see
    /// `purity::Facts::consts`), used as boundary inputs.
    pub consts: Vec<String>,
}

/// Why a translated function is not on the list (counted in the summary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Skip {
    Incomplete,
    NoSignature,
    NotInExeTable,
    Impure,
    VoidResult,
    PointerArgument,
    ReadsThroughPointer,
    /// The exe's stack cleanup or register use does not fit the Rust
    /// signature under any convention (a candidate translation issue, listed).
    ConventionMismatch,
    /// A leading `f64` that the exe takes in x87 ST0 (`nv-call` cannot
    /// preset ST0).
    St0Input,
}

pub struct Listing {
    pub pure: Vec<Pure>,
    pub skipped: BTreeMap<Skip, usize>,
    pub convention_mismatches: Vec<(u32, String)>,
}

/// The convention `nv-call` should use, or why none fits.
fn convention(params: &[Ty], f: &Facts) -> Result<&'static str, String> {
    let words: usize = params.iter().map(|t| t.words()).sum();
    let reg: Vec<&str> = f.reg_in.iter().map(String::as_str).collect();
    let (cc, stack_words) = match reg.as_slice() {
        [] => {
            if f.ret_pop == 0 {
                ("cdecl", words)
            } else if f.ret_pop == 4 * words as i64 {
                ("stdcall", words)
            } else {
                return Err(format!(
                    "pops {} bytes, Rust has {} argument words",
                    f.ret_pop, words
                ));
            }
        }
        ["ECX"] => {
            if params.first().map(|t| t.words()) != Some(1) {
                return Err("reads ECX on entry but the first parameter is not one word".into());
            }
            ("thiscall", words - 1)
        }
        ["ECX", "EDX"] => {
            let in_regs = params
                .iter()
                .take(2)
                .filter(|t| t.words() == 1 && **t != Ty::F32)
                .count();
            if in_regs < 2 {
                return Err(
                    "reads ECX and EDX on entry but the first two parameters are not integer words"
                        .into(),
                );
            }
            ("fastcall", words - 2)
        }
        other => return Err(format!("reads {} on entry", other.join(","))),
    };
    if cc != "cdecl" && f.ret_pop != 4 * stack_words as i64 {
        return Err(format!(
            "{cc}: pops {} bytes, Rust leaves {} stack words",
            f.ret_pop, stack_words
        ));
    }
    if f.ret_pop < 0 {
        return Err("no RET in the body".into());
    }
    if f.stack_args > 4 * stack_words as i64 {
        return Err(format!(
            "reads {} stack argument bytes, Rust passes {}",
            f.stack_args,
            4 * stack_words
        ));
    }
    Ok(cc)
}

pub fn build(src: &Source, facts: &BTreeMap<u32, Facts>) -> Listing {
    let pure_set = purity::transitively_pure(facts);
    let mut out = Listing {
        pure: Vec::new(),
        skipped: BTreeMap::new(),
        convention_mismatches: Vec::new(),
    };
    let mut skipped: BTreeMap<Skip, usize> = BTreeMap::new();
    let mut skip = |s: Skip| *skipped.entry(s).or_insert(0) += 1;
    for (&addr, unit) in &src.translated {
        if src.incomplete.contains(&addr) {
            skip(Skip::Incomplete);
            continue;
        }
        let Some(Some(sig)) = src.signatures.get(&addr) else {
            skip(Skip::NoSignature);
            continue;
        };
        let Some(f) = facts.get(&addr) else {
            skip(Skip::NotInExeTable);
            continue;
        };
        if !pure_set.contains(&addr) {
            skip(Skip::Impure);
            continue;
        }
        if sig.ret == Ty::Void {
            skip(Skip::VoidResult);
            continue;
        }
        if !sig.params.iter().all(|t| t.is_scalar()) || sig.ret == Ty::Ptr {
            skip(Skip::PointerArgument);
            continue;
        }
        let callees: BTreeSet<u32> = purity::callees(facts, addr);
        let reads_pointer = f.r_pointer > 0
            || callees
                .iter()
                .any(|c| facts.get(c).is_some_and(|g| g.r_pointer > 0));
        if reads_pointer {
            skip(Skip::ReadsThroughPointer);
            continue;
        }
        if sig.params.first() == Some(&Ty::F64)
            && f.reg_in.is_empty()
            && f.ret_pop == 0
            && f.stack_args < 8
        {
            skip(Skip::St0Input);
            continue;
        }
        let cc = match convention(&sig.params, f) {
            Ok(cc) => cc,
            Err(why) => {
                skip(Skip::ConventionMismatch);
                out.convention_mismatches.push((addr, why));
                continue;
            }
        };
        out.pure.push(Pure {
            addr,
            cc,
            params: sig.params.clone(),
            ret: sig.ret,
            unit: unit.to_string_lossy().replace('\\', "/"),
            consts: {
                let mut k: Vec<String> = Vec::new();
                for a in std::iter::once(&addr).chain(callees.iter()) {
                    for t in facts.get(a).map(|g| g.consts.as_slice()).unwrap_or(&[]) {
                        if !k.contains(t) && k.len() < 48 {
                            k.push(t.clone());
                        }
                    }
                }
                k
            },
            callees: callees.into_iter().collect(),
        });
    }
    out.skipped = skipped;
    out
}

pub const HEADER: &str = "\
# Pure translations for the automatic oracle check (research/nv-oracle/nv-pure, #182).
# Generated by `nv-pure list`: translated and complete in crates/engine, pure in
# FalloutNV.exe 1.4.0.525 (NvPurity.java, transitively), scalar arguments and result.
# Addresses, signatures and the constants each function compares with (used as
# boundary inputs). Regenerate after translations land.
address\tcc\tparams\tret\tunit\tcallees\tconsts
";

pub fn to_tsv(list: &[Pure]) -> String {
    let mut s = String::from(HEADER);
    for p in list {
        let params: Vec<&str> = p.params.iter().map(|t| t.name()).collect();
        let callees: Vec<String> = p.callees.iter().map(|c| format!("{c:08x}")).collect();
        let _ = writeln!(
            s,
            "{:08x}\t{}\t{}\t{}\t{}\t{}\t{}",
            p.addr,
            p.cc,
            params.join(","),
            p.ret.name(),
            p.unit,
            callees.join(","),
            p.consts.join(",")
        );
    }
    s
}

pub fn from_tsv(text: &str) -> Result<Vec<Pure>, String> {
    let mut out = Vec::new();
    for (n, l) in text.lines().enumerate() {
        if l.starts_with('#') || l.starts_with("address\t") || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        let bad = || format!("pure list line {}: malformed", n + 1);
        if c.len() < 6 {
            return Err(bad());
        }
        let cc = match c[1] {
            "cdecl" => "cdecl",
            "stdcall" => "stdcall",
            "thiscall" => "thiscall",
            "fastcall" => "fastcall",
            _ => return Err(bad()),
        };
        let params = c[2]
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| Ty::from_name(s).ok_or_else(bad))
            .collect::<Result<Vec<_>, _>>()?;
        out.push(Pure {
            addr: u32::from_str_radix(c[0], 16).map_err(|_| bad())?,
            cc,
            params,
            ret: Ty::from_name(c[3]).ok_or_else(bad)?,
            unit: c[4].to_string(),
            callees: c[5]
                .split(',')
                .filter(|s| !s.is_empty())
                .filter_map(|s| u32::from_str_radix(s, 16).ok())
                .collect(),
            consts: c
                .get(6)
                .map(|v| {
                    v.split(',')
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(ret_pop: i64, reg: &[&str], stack_args: i64) -> Facts {
        Facts {
            ret_pop,
            calls: vec![],
            indirect: 0,
            imports: 0,
            w_global: 0,
            w_pointer: 0,
            w_implicit: 0,
            r_pointer: 0,
            fs: 0,
            reg_in: reg.iter().map(|s| s.to_string()).collect(),
            stack_args,
            consts: vec![],
        }
    }

    #[test]
    fn conventions() {
        assert_eq!(
            convention(&[Ty::F32, Ty::F32], &facts(0, &[], 8)),
            Ok("cdecl")
        );
        assert_eq!(
            convention(&[Ty::F32, Ty::F32], &facts(8, &[], 8)),
            Ok("stdcall")
        );
        assert_eq!(
            convention(&[Ty::U32, Ty::I32], &facts(4, &["ECX"], 4)),
            Ok("thiscall")
        );
        assert_eq!(
            convention(&[Ty::U32, Ty::U32, Ty::F64], &facts(8, &["ECX", "EDX"], 8)),
            Ok("fastcall")
        );
        assert!(
            convention(&[Ty::F32], &facts(8, &[], 8)).is_err(),
            "pops more than it takes"
        );
        assert!(
            convention(&[Ty::F32], &facts(0, &[], 12)).is_err(),
            "reads more than it is given"
        );
        assert!(
            convention(&[Ty::U32, Ty::U32], &facts(0, &["ECX"], 4)).is_err(),
            "thiscall leaves a stack word"
        );
    }

    #[test]
    fn tsv_round_trip() {
        let p = Pure {
            addr: 0x00a0_1a20,
            cc: "stdcall",
            params: vec![Ty::F32, Ty::U16],
            ret: Ty::Bool,
            unit: "crates/engine/src/units/x.rs".into(),
            callees: vec![0x0040_1000],
            consts: vec!["i26".into(), "d3ff0000000000000".into()],
        };
        let back = from_tsv(&to_tsv(std::slice::from_ref(&p))).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].addr, p.addr);
        assert_eq!(back[0].params, p.params);
        assert_eq!(back[0].ret, p.ret);
        assert_eq!(back[0].callees, p.callees);
        assert_eq!(back[0].consts, p.consts);
    }
}
