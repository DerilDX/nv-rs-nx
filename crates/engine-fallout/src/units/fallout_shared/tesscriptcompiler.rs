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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x005b1120,
        script_compiler_get_function_def(i32) -> Ptr
    )]
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
