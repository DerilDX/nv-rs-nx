//! `NiMain/nishader.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiShader::ms_RTTI` (the class's `NiRTTI` record).
const NI_SHADER_RTTI: u32 = 0x011f_5d08;

// Translated from 00a8f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_shader_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SHADER_RTTI)
}

// Translated from 00a8f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::Initialize` (Xbox PDB name in the engine map; `thiscall`, no
/// stack words): sets the byte at `+0x10` to 1 and answers 1 (`AL`).
pub fn ni_shader_initialize(e: &mut Engine, this: Ptr) -> bool {
    e.mem.set_u8(this.addr() + 0x10, 1);
    true
}

// Translated from 00a8f410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method shared by many vtables
/// (`thiscall`, `RET 8`: two stack words, neither read) that answers 0.
pub fn fn_00a8f410(_e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) -> u32 {
    0
}

/// `NiShader`'s vtable (`0109f584`, RTTI `.?AVNiShader@@`).
const NI_SHADER_VTABLE: u32 = 0x0109_f584;
/// `NiRefObject`'s vtable (`0101dce4`, RTTI `.?AVNiRefObject@@`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// `NiRefObject::ms_uiObjects` (`011f4400`), the live object count.
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The empty `NiFixedString` handle (`0109b220`), never released.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// The `InterlockedDecrement` import slot (`kernel32`, stdcall, one word).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a8f490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiShader`'s destructor body (`thiscall`, no
/// stack words): stores `NiShader`'s vtable, releases the name handle at `+0x8`
/// (unless it equals the handle in `0109b220`: `InterlockedDecrement` of the
/// count 8 bytes before the text), then stores `NiRefObject`'s vtable and
/// decrements `NiRefObject`'s object count `011f4400`.
pub fn fn_00a8f490(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_SHADER_VTABLE);
    let name = e.mem.u32(this.addr() + 0x8);
    if name != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8f480, ni_shader_get_rtti() -> Ptr),
        entry!(0x00a8_f3d0, ni_shader_initialize(Ptr) -> bool),
        entry!(0x00a8_f410, fn_00a8f410(Ptr, u32, u32) -> u32),
        entry!(0x00a8_f490, fn_00a8f490(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_shader_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8f480, &args![]).u32(), NI_SHADER_RTTI);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_shader_initialize_sets_the_flag() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00a8_f3d0, &args![this]).u32() & 0xff, 1);
        assert_eq!(e.mem.u8(this + 0x10), 1);
    }

    #[test]
    fn fn_00a8f410_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8_f410, &args![0x1000u32, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn fn_00a8f490_releases_the_name_unless_empty() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global::<u32>(FIXED_STRING_EMPTY, 0x4444);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let run = |e: &mut Engine, name: u32| -> Vec<Vec<u32>> {
            e.mem.set_u32(this + 8, name);
            e.call_log = Some(vec![]);
            e.call(0x00a8_f490, &args![this]);
            e.call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == INTERLOCKED_DECREMENT)
                .map(|c| c.1)
                .collect()
        };
        assert_eq!(
            run(&mut e, 0x8008),
            vec![vec![0x8000], vec![NI_REF_OBJECT_COUNT]]
        );
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(run(&mut e, 0x4444), vec![vec![NI_REF_OBJECT_COUNT]]);
    }
}
