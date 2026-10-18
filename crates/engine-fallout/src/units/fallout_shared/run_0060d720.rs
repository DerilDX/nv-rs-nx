//! `fallout shared/run_0060d720` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `strlen`-style length (`0044a670`, `cdecl`), `strcpy_s` (`00406d30`,
/// `cdecl` (dest, size, src)), `BSStringT<char>::StrCmp` (`00408a80`) and
/// the `BSStringT<char>` text assignment (`004037f0`).
const STRING_LENGTH: u32 = 0x0044_a670;
const STRCPY_S: u32 = 0x0040_6d30;
const STRING_COMPARE: u32 = 0x0040_8a80;
const STRING_ASSIGN: u32 = 0x0040_37f0;
/// The quest's editor id (`cFormEditorID`, `BSStringT<char>`, +0x64).
const EDITOR_ID: u32 = 0x64;

// Translated from 0060dab0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060dab0 - the /GS stack cookie check (00ec408c) is not translated.
/// The quest's `SetEditorID` (vtable slot; `thiscall`, `RET 4`): copies
/// `name` to a stack buffer of its length + 1 (the compiler's `alloca`,
/// `00ec5ec0`, here the engine's stack), and answers false when the editor
/// id (+0x64) already equals it (`StrCmp` with case ignored); else stores
/// it there and answers true.
pub fn fn_0060dab0(e: &mut Engine, this: Ptr, name: Ptr) -> bool {
    let size = e.call(STRING_LENGTH, &args![name]).u32().wrapping_add(1);
    e.with_stack(size, |e, buffer| {
        e.call(STRCPY_S, &args![buffer, size, name]);
        let editor_id = this.addr() + EDITOR_ID;
        if e.call(STRING_COMPARE, &args![editor_id, buffer, 1u32])
            .u32()
            == 0
        {
            return false;
        }
        e.call(STRING_ASSIGN, &args![editor_id, buffer, 0u32]);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0060dab0, fn_0060dab0(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_editor_id_unless_it_is_already_that() {
        let mut e = Engine::new();
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.write(a[0], &s);
            e.mem.set_u8(a[0] + s.len() as u32, 0);
            Ret::default()
        });
        e.register_double(STRING_COMPARE, |_, _| 1u32.into_ret());
        e.register(STRING_ASSIGN, |_, _| Ret::default());
        let quest = e.mem.alloc(0x6c);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"VMS49\0");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060dab0, &[quest, name]).u32() & 0xff, 1);
        let log = e.call_log.take().unwrap();
        let assign = log.iter().find(|(a, _)| *a == STRING_ASSIGN).unwrap();
        assert_eq!(assign.1[0], quest + 0x64);
        assert_eq!(e.mem.cstr(assign.1[1]), b"VMS49");
        assert_eq!(assign.1[2], 0);
        // Equal already: nothing stored.
        e.register_double(STRING_COMPARE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060dab0, &[quest, name]).u32() & 0xff, 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == STRING_ASSIGN));
    }
}
