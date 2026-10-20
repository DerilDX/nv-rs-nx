//! `fallout shared/tesscriptcompiler.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The console command table: 0x28-byte command definitions for opcodes
/// 0x100 to 0x1cc.
const CONSOLE_COMMANDS: u32 = 0x0118_e8e0;
/// The script command table: definitions for opcodes 0x1000 to 0x127f.
const SCRIPT_COMMANDS: u32 = 0x0119_0910;

// Translated from 005b1120 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::GetFunctionDef` (Xbox PDB), `cdecl`: the address of the
/// definition of a command opcode in the console table (0x100..=0x1cc) or
/// the script table (0x1000..=0x127f), signed comparisons; null otherwise.
pub fn script_compiler_get_function_def(_e: &mut Engine, opcode: i32) -> Ptr {
    if (0x100..0x1cd).contains(&opcode) {
        Ptr::new(CONSOLE_COMMANDS + (opcode - 0x100) as u32 * 0x28)
    } else if (0x1000..0x1280).contains(&opcode) {
        Ptr::new(SCRIPT_COMMANDS + (opcode - 0x1000) as u32 * 0x28)
    } else {
        Ptr::NULL
    }
}

/// The opcode word (+8) of the entry just past each table (0x1CD console
/// entries, 0x280 script entries): the tables' end markers.
const CONSOLE_END_OPCODE: u32 = 0x0119_08f0;
const SCRIPT_END_OPCODE: u32 = 0x0119_6d18;
/// `Error` (`0040fbe0`, engine map; `cdecl`, a format and its words) and the
/// two messages: "The script function list doesn't have all the names
/// filled in." (`010374a0`) and "The script console-command list doesn't
/// have all the names filled in." (`01037458`).
const ERROR: u32 = 0x0040_fbe0;
const SCRIPT_LIST_INCOMPLETE: u32 = 0x0103_74a0;
const CONSOLE_LIST_INCOMPLETE: u32 = 0x0103_7458;

// Translated from 005aea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptCompiler::ScriptCompiler` (Xbox PDB; `thiscall`, no stack words):
/// checks that the script command table ends with opcode `0x1280` and the
/// console table with `0x1CD` (the entries past their ends), reporting each
/// mismatch through `Error`. Returns `this`.
pub fn script_compiler_script_compiler(e: &mut Engine, this: Ptr) -> Ptr {
    if e.mem.u32(SCRIPT_END_OPCODE) != 0x1280 {
        e.call(ERROR, &args![SCRIPT_LIST_INCOMPLETE]);
    }
    if e.mem.u32(CONSOLE_END_OPCODE) != 0x1cd {
        e.call(ERROR, &args![CONSOLE_LIST_INCOMPLETE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005b1120,
            script_compiler_get_function_def(i32) -> Ptr
        ),
        entry!(0x005aea50, script_compiler_script_compiler(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn reports_tables_without_their_end_markers() {
        let mut e = Engine::new();
        e.register(ERROR, |_, _| Ret::default());
        e.map(0x0119_0000, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_ea50, &args![0x40u32]).u32(), 0x40);
        let errors: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == ERROR)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            errors,
            vec![vec![SCRIPT_LIST_INCOMPLETE], vec![CONSOLE_LIST_INCOMPLETE]]
        );
        e.mem.set_u32(SCRIPT_END_OPCODE, 0x1280);
        e.mem.set_u32(CONSOLE_END_OPCODE, 0x1cd);
        e.call_log = Some(vec![]);
        e.call(0x005a_ea50, &args![0x40u32]);
        assert!(!e.call_log.take().unwrap().iter().any(|c| c.0 == ERROR));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_both_opcode_ranges_and_rejects_the_rest() {
        let mut e = Engine::new();
        let cases: &[(i32, u32)] = &[
            (0xff, 0),
            (0x100, 0x0118_e8e0),
            (0x101, 0x0118_e908),
            (0x1cc, 0x0118_e8e0 + 0xcc * 0x28),
            (0x1cd, 0),
            (0xfff, 0),
            (0x1000, 0x0119_0910),
            (0x1059, 0x0119_0910 + 0x59 * 0x28),
            (0x127f, 0x0119_0910 + 0x27f * 0x28),
            (0x1280, 0),
            (-1, 0),
            (i32::MIN, 0),
            (0x10100, 0),
        ];
        for &(opcode, expected) in cases {
            assert_eq!(
                e.call(0x005b1120, &args![opcode]).u32(),
                expected,
                "{opcode:x}"
            );
        }
    }
}
