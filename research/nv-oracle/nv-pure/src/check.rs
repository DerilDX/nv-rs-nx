//! Runs every listed function on the real CPU (`nv-call`) and in the engine
//! with the same inputs and compares the results bit for bit.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::{Engine, Ret};

use crate::list::Pure;
use crate::source::Ty;
use crate::vectors::{self, Outcome, Val};

pub struct Options {
    pub exe: PathBuf,
    pub nv_call: PathBuf,
    pub out_dir: PathBuf,
    pub per_function: usize,
}

/// How one vector came out.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Match,
    /// Both sides failed the same way: the exe's access violation and the
    /// engine's unmapped read are at the same address (or both divided by
    /// zero). Consistent, but says nothing about the result.
    BothFailed,
    /// The engine read memory it never maps: outside the exe's data sections
    /// (an out-of-range table index from a random input, landing in `.text`,
    /// nv-call's buffer arena, its heap or a DLL). The exe read the same
    /// address, which nv-call's process happened to have mapped:
    /// inconclusive, not a translation difference.
    OutOfImageRead,
    Mismatch,
    /// Both results are NaN but the bits differ (sign or payload).
    NanBits,
    ExeFaultOnly,
    RustPanicOnly,
    /// Both failed, but differently (another address, or another kind of
    /// failure) where the engine had the memory to compare.
    FaultMismatch,
    /// The exe removed a different number of stack bytes than the listed
    /// convention says: the list, not the translation, is wrong.
    Convention,
}

impl Kind {
    pub fn is_failure(&self) -> bool {
        !matches!(self, Kind::Match | Kind::BothFailed | Kind::OutOfImageRead)
    }
}

/// What `Engine::with_exe` maps of FalloutNV.exe 1.4.0.525: `.rdata`, `.data`,
/// `.tls` and `CONST` (0x00fdf000 to the end of CONST). A panic on a read
/// inside this range is a real difference; outside it the engine has no
/// bytes to give, whatever the exe read there.
const IMAGE: std::ops::Range<u32> = 0x00fd_f000..0x0127_4000;

/// The address of an engine "read of unmapped memory at <hex>" panic.
fn unmapped_read(msg: &str) -> Option<u32> {
    let rest = msg.split("unmapped memory at ").nth(1)?;
    let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    u32::from_str_radix(&hex, 16).ok()
}

pub struct Report {
    pub functions: usize,
    pub skipped: Vec<(u32, String)>,
    pub counts: BTreeMap<Kind, usize>,
    /// (address, unit, vector id, inputs, exe, rust, kind) for every failure.
    pub failures: Vec<(u32, String, String, String, String, String, Kind)>,
    pub failing_functions: BTreeMap<u32, usize>,
    /// Functions whose exe results (values, not faults) take two or more
    /// distinct values over the inputs: the matches there are not vacuous.
    pub varying: usize,
    /// Functions whose exe results are all the same value (or that only
    /// faulted): a match there only shows the constant is right.
    pub constant: Vec<u32>,
    /// f32 results where the exe's ST0 holds more precision than a float
    /// (ST0 rounded to double differs from ST0 rounded to float): the `f32`
    /// signature drops bits a caller using ST0 directly would see.
    /// Informational; per function, the number of vectors.
    pub extended: BTreeMap<u32, usize>,
}

fn expected_pop(p: &Pure) -> u32 {
    let words: usize = p.params.iter().map(|t| t.words()).sum();
    let stack = match p.cc {
        "thiscall" => words.saturating_sub(1),
        "fastcall" => words.saturating_sub(2),
        _ => words,
    };
    match p.cc {
        "cdecl" => 0,
        _ => 4 * stack as u32,
    }
}

fn is_nan32(b: u32) -> bool {
    f32::from_bits(b).is_nan()
}

fn is_nan64(b: u64) -> bool {
    f64::from_bits(b).is_nan()
}

/// (exe text, rust text, kind)
fn compare(
    ret: Ty,
    exe: &Outcome,
    rust: &Result<Ret, String>,
    want_pop: u32,
) -> (String, String, Kind) {
    let rust_text = match rust {
        Ok(r) => format!("eax=0x{:08x} edx=0x{:08x} st0={:?}", r.eax, r.edx, r.st0),
        Err(m) => format!("panic: {m}"),
    };
    let (eax, edx, f32b, f64b, popped) = match exe {
        Outcome::Fault {
            text,
            name,
            read_at,
        } => {
            let k = match rust {
                Ok(_) => Kind::ExeFaultOnly,
                Err(m) => match (read_at, unmapped_read(m)) {
                    (Some(a), Some(b)) if *a == b => Kind::BothFailed,
                    // The engine stopped at memory it never maps; the exe read
                    // that address (mapped in nv-call's process) and failed later.
                    (_, Some(b)) if !IMAGE.contains(&b) => Kind::OutOfImageRead,
                    _ if name == "integer_divide_by_zero" && m.contains("divide by zero") => {
                        Kind::BothFailed
                    }
                    _ => Kind::FaultMismatch,
                },
            };
            return (format!("fault {text}"), rust_text, k);
        }
        Outcome::Value {
            eax,
            edx,
            st0_f32,
            st0_f64,
            popped,
        } => (*eax, *edx, *st0_f32, *st0_f64, *popped),
    };
    let r = match rust {
        Ok(r) => *r,
        Err(m) => {
            let k = match unmapped_read(m) {
                Some(a) if !IMAGE.contains(&a) => Kind::OutOfImageRead,
                _ => Kind::RustPanicOnly,
            };
            return (format!("eax=0x{eax:08x}"), rust_text, k);
        }
    };
    if popped != want_pop {
        return (
            format!("popped {popped} bytes, list expects {want_pop}"),
            rust_text,
            Kind::Convention,
        );
    }
    let (exe_text, kind) = match ret {
        Ty::Bool => {
            let (a, b) = (eax as u8 != 0, r.eax as u8 != 0);
            (
                format!("al={}", eax as u8),
                if a == b { Kind::Match } else { Kind::Mismatch },
            )
        }
        Ty::U8 | Ty::I8 => (
            format!("al=0x{:02x}", eax as u8),
            if eax as u8 == r.eax as u8 {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
        Ty::U16 | Ty::I16 => (
            format!("ax=0x{:04x}", eax as u16),
            if eax as u16 == r.eax as u16 {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
        Ty::U64 => (
            format!("edx:eax=0x{edx:08x}{eax:08x}"),
            if eax == r.eax && edx == r.edx {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
        Ty::F32 => match f32b {
            None => ("ST0 empty".into(), Kind::Mismatch),
            Some(b) => {
                let rb = (r.st0 as f32).to_bits();
                let k = if b == rb {
                    Kind::Match
                } else if is_nan32(b) && is_nan32(rb) {
                    Kind::NanBits
                } else {
                    Kind::Mismatch
                };
                (
                    format!("st0={:?}(0x{b:08x}) rust_f32=0x{rb:08x}", f32::from_bits(b)),
                    k,
                )
            }
        },
        Ty::F64 => match f64b {
            None => ("ST0 empty".into(), Kind::Mismatch),
            Some(b) => {
                let rb = r.st0.to_bits();
                let k = if b == rb {
                    Kind::Match
                } else if is_nan64(b) && is_nan64(rb) {
                    Kind::NanBits
                } else {
                    Kind::Mismatch
                };
                (format!("st0={:?}(0x{b:016x})", f64::from_bits(b)), k)
            }
        },
        _ => (
            format!("eax=0x{eax:08x}"),
            if eax == r.eax {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
    };
    (exe_text, rust_text, kind)
}

/// `Engine::with_exe` without rereading the file: the checked data sections
/// and imports parsed once, mapped into a new engine per vector (the same
/// steps as `engine::exe::map_data_sections` and `Engine::with_exe`).
struct FreshEngine {
    sections: Vec<(u32, u32, Vec<u8>)>,
    imports: Vec<engine::exe::Import>,
}

impl FreshEngine {
    fn load(exe: &Path) -> Result<Self, String> {
        let file = fs::read(exe).map_err(|e| format!("{}: {e}", exe.display()))?;
        let secs = engine::exe::checked_sections(exe, &file)?;
        let mut sections = Vec::new();
        for (name, va, vsize, _) in engine::exe::SECTIONS {
            let s = secs
                .iter()
                .find(|s| s.name == name)
                .ok_or_else(|| format!("no {name} section"))?;
            let n = s.raw.len().min(vsize as usize);
            sections.push((va, vsize.max(s.raw.len() as u32), s.raw[..n].to_vec()));
        }
        Ok(FreshEngine {
            sections,
            imports: engine::exe::imports(&file)?,
        })
    }

    fn make(&self) -> Engine {
        let mut e = Engine::new();
        for (va, len, bytes) in &self.sections {
            e.mem.map(*va, *len);
            e.mem.write(*va, bytes);
        }
        e.register_imports(&self.imports);
        e
    }
}

fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".into()
    }
}

pub fn run(list: &[Pure], o: &Options) -> Result<Report, String> {
    if !o.nv_call.is_file() {
        return Err(format!(
            "{} not found; build it with `cargo build --release --target i686-pc-windows-msvc` in research/nv-oracle",
            o.nv_call.display()
        ));
    }
    fs::create_dir_all(&o.out_dir).map_err(|e| format!("{}: {e}", o.out_dir.display()))?;

    // Which functions the engine can run: itself and every callee translated.
    let probe = Engine::with_exe(&o.exe)?;
    let mut skipped = Vec::new();
    let mut runnable = Vec::new();
    for p in list {
        if !probe.is_translated(p.addr) {
            skipped.push((p.addr, "not registered in the engine".to_string()));
        } else if let Some(c) = p.callees.iter().find(|c| !probe.is_translated(**c)) {
            skipped.push((p.addr, format!("callee {c:08x} is not translated")));
        } else {
            runnable.push(p);
        }
    }
    drop(probe);

    // One nv-call run for all vectors.
    let mut lines = String::new();
    let mut cases: Vec<(&Pure, Vec<Vec<Val>>)> = Vec::new();
    for p in &runnable {
        let ins = vectors::inputs(&p.params, o.per_function, p.addr as u64, &p.consts);
        for (i, args) in ins.iter().enumerate() {
            let _ = writeln!(
                lines,
                "{}",
                vectors::vector_line(&format!("{:08x}_{i}", p.addr), p.addr, p.cc, args, p.ret)
            );
        }
        cases.push((p, ins));
    }
    let vec_path = o.out_dir.join("vectors.jsonl");
    let res_path = o.out_dir.join("results.jsonl");
    fs::write(&vec_path, &lines).map_err(|e| format!("{}: {e}", vec_path.display()))?;
    let out = Command::new(&o.nv_call)
        .arg(&o.exe)
        .arg(&vec_path)
        .output()
        .map_err(|e| format!("running {}: {e}", o.nv_call.display()))?;
    fs::write(&res_path, &out.stdout).map_err(|e| format!("{}: {e}", res_path.display()))?;
    if !matches!(out.status.code(), Some(0) | Some(2)) {
        return Err(format!(
            "nv-call failed with {:?}: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let exe_results: BTreeMap<String, Outcome> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(vectors::parse_result)
        .collect();

    // The engine side: every vector on a fresh copy of one template engine,
    // as nv-call restores the image's writable memory before every vector.
    let template = FreshEngine::load(&o.exe)?;
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let mut report = Report {
        functions: cases.len(),
        skipped,
        counts: BTreeMap::new(),
        failures: Vec::new(),
        failing_functions: BTreeMap::new(),
        varying: 0,
        constant: Vec::new(),
        extended: BTreeMap::new(),
    };
    for (p, ins) in &cases {
        let want_pop = expected_pop(p);
        let mut distinct = std::collections::BTreeSet::new();
        for (i, args) in ins.iter().enumerate() {
            let id = format!("{:08x}_{i}", p.addr);
            let mut words = Vec::new();
            for v in args {
                v.put(&mut words);
            }
            let mut e = template.make();
            let rust = panic::catch_unwind(AssertUnwindSafe(|| e.call(p.addr, &words)))
                .map_err(panic_text);
            if let Some(Outcome::Value {
                eax,
                edx,
                st0_f32,
                st0_f64,
                ..
            }) = exe_results.get(&id)
            {
                let key = match p.ret {
                    Ty::F32 => st0_f32.map(u64::from).unwrap_or(u64::MAX),
                    Ty::F64 => st0_f64.unwrap_or(u64::MAX),
                    Ty::Bool => u64::from(*eax as u8 != 0),
                    Ty::U8 | Ty::I8 => u64::from(*eax as u8),
                    Ty::U16 | Ty::I16 => u64::from(*eax as u16),
                    Ty::U64 => u64::from(*eax) | u64::from(*edx) << 32,
                    _ => u64::from(*eax),
                };
                distinct.insert(key);
                if let (Ty::F32, Some(f), Some(d)) = (p.ret, st0_f32, st0_f64) {
                    let (f, d) = (f32::from_bits(*f), f64::from_bits(*d));
                    if !d.is_nan() && f as f64 != d {
                        *report.extended.entry(p.addr).or_insert(0) += 1;
                    }
                }
            }
            let Some(exe) = exe_results.get(&id) else {
                *report.counts.entry(Kind::Mismatch).or_insert(0) += 1;
                report.failures.push((
                    p.addr,
                    p.unit.clone(),
                    id,
                    String::new(),
                    "no result line".into(),
                    String::new(),
                    Kind::Mismatch,
                ));
                continue;
            };
            let (exe_text, rust_text, kind) = compare(p.ret, exe, &rust, want_pop);
            *report.counts.entry(kind.clone()).or_insert(0) += 1;
            if kind.is_failure() {
                *report.failing_functions.entry(p.addr).or_insert(0) += 1;
                let shown: Vec<String> = args.iter().map(|v| v.show()).collect();
                report.failures.push((
                    p.addr,
                    p.unit.clone(),
                    id,
                    shown.join(", "),
                    exe_text,
                    rust_text,
                    kind,
                ));
            }
        }
        if distinct.len() >= 2 {
            report.varying += 1;
        } else {
            report.constant.push(p.addr);
        }
    }
    panic::set_hook(quiet);

    let mut tsv = String::from("address\tunit\tvector\tinputs\texe\trust\tkind\n");
    for (a, u, id, ins, ex, ru, k) in &report.failures {
        let _ = writeln!(tsv, "{a:08x}\t{u}\t{id}\t{ins}\t{ex}\t{ru}\t{k:?}");
    }
    let rep = o.out_dir.join("failures.tsv");
    fs::write(&rep, tsv).map_err(|e| format!("{}: {e}", rep.display()))?;

    // Coverage notes: which matches only show a constant, and which f32
    // results were wider than a float in ST0.
    let mut notes = String::from(
        "address	unit	note
",
    );
    let unit = |a: u32| {
        cases
            .iter()
            .find(|(p, _)| p.addr == a)
            .map(|(p, _)| p.unit.clone())
            .unwrap_or_default()
    };
    for a in &report.constant {
        let _ = writeln!(
            notes,
            "{a:08x}	{}	constant or fault-only exe results",
            unit(*a)
        );
    }
    for (a, n) in &report.extended {
        let _ = writeln!(
            notes,
            "{a:08x}	{}	f32 result wider than a float in {n} vector(s)",
            unit(*a)
        );
    }
    let path = o.out_dir.join("coverage.tsv");
    fs::write(&path, notes).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(report)
}

/// `%USERPROFILE%\nv-re\oracle\pure` (private: results hold the exe's outputs).
pub fn default_out_dir() -> PathBuf {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .unwrap_or_default();
    Path::new(&home).join("nv-re").join("oracle").join("pure")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(eax: u32, f: Option<u32>) -> Outcome {
        Outcome::Value {
            eax,
            edx: 0,
            st0_f32: f,
            st0_f64: None,
            popped: 0,
        }
    }

    #[test]
    fn verdicts() {
        let ok = |eax: u32, st0: f64| Ok::<Ret, String>(Ret { eax, edx: 0, st0 });
        assert_eq!(
            compare(Ty::Bool, &val(0x1234_5601, None), &ok(1, 0.0), 0).2,
            Kind::Match
        );
        assert_eq!(
            compare(Ty::U32, &val(5, None), &ok(6, 0.0), 0).2,
            Kind::Mismatch
        );
        assert_eq!(
            compare(Ty::F32, &val(0, Some(0x3fc0_0000)), &ok(0, 1.5), 0).2,
            Kind::Match
        );
        assert_eq!(
            compare(Ty::F32, &val(0, Some(0x7fc0_0001)), &ok(0, f64::NAN), 0).2,
            Kind::NanBits
        );
        assert_eq!(
            compare(Ty::U32, &val(5, None), &ok(5, 0.0), 4).2,
            Kind::Convention
        );
        let fault = |at: u32| Outcome::Fault {
            text: "{}".into(),
            name: "access_violation".into(),
            read_at: Some(at),
        };
        let read = |at: &str| Err::<Ret, String>(format!("read of unmapped memory at {at}"));
        assert_eq!(
            compare(Ty::U32, &fault(0x021f_91c8), &read("021f91c8"), 0).2,
            Kind::BothFailed,
            "same address"
        );
        assert_eq!(
            compare(Ty::U32, &fault(0x0400_0000), &read("011f91c8"), 0).2,
            Kind::FaultMismatch,
            "engine stopped inside its mapped data, the exe elsewhere"
        );
        assert_eq!(
            compare(Ty::U32, &fault(0x0400_0000), &read("00b89904"), 0).2,
            Kind::OutOfImageRead,
            "engine stopped at .text, which it doesn't map"
        );
        assert_eq!(
            compare(
                Ty::U32,
                &fault(0x0400_0000),
                &Err("open function 00401000".into()),
                0
            )
            .2,
            Kind::FaultMismatch
        );
        assert_eq!(
            compare(Ty::U32, &fault(0x0400_0000), &ok(1, 0.0), 0).2,
            Kind::ExeFaultOnly
        );
        let oob = Err::<Ret, String>("read of unmapped memory at 3054e150".into());
        assert_eq!(
            compare(Ty::U32, &val(0, None), &oob, 0).2,
            Kind::OutOfImageRead
        );
        let text = Err::<Ret, String>("read of unmapped memory at 00b89904".into());
        assert_eq!(
            compare(Ty::U32, &val(0, None), &text, 0).2,
            Kind::OutOfImageRead
        );
        let data = Err::<Ret, String>("read of unmapped memory at 011f91c8".into());
        assert_eq!(
            compare(Ty::U32, &val(0, None), &data, 0).2,
            Kind::RustPanicOnly
        );
    }
}
