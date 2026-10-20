//! `BSShader/run_00ba3e30` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010b80a8`; no RTTI locator before it).
const VTABLE_010B80A8: u32 = 0x010b_80a8;
/// The shared element count (`012003ec`), set on first use from the shader model setting
/// (`011f91c0`): 0x1a below model 2, else 0x4e.
const ELEMENT_COUNT_012003EC: u32 = 0x0120_03ec;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00ba53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the constructor the creator `00ba58b0` runs,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010b80a8`, clears +0x74 to +0x80, stores 0.0 at +0xec to +0x128, sets
/// +0x68 to 0xe, the byte +0x24 to 1 and +0x6c to 0x241; clears the pass pointers at +0x74 and
/// +0x78 (their counts at +0x64 dropped, `00e80190` at 0), assigns the declaration to the
/// `NiPointer` at +0x28, clears the `NiPointer`s at +0x7c and +0x80, stores 0.0 at +0xd4 to
/// +0x128; the element count at `012003ec` becomes 0x1a (shader model below 2, signed) or
/// 0x4e when it is 0; that many 0x30-byte elements (`operator new[]` size check) from the
/// memory manager go to +0x70 and are cleared, and 0x50 bytes at +0x84 are cleared
/// (`memset`). Answers `this`.
// Translation incomplete: 00ba53f0 - the C++ exception frame (FS:[0] chain, unwind state 4 for the base and the pointers) is not translated.
pub fn fn_00ba53f0(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010B80A8);
    for offset in [0x74u32, 0x78, 0x7c, 0x80] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in (0xecu32..=0x128).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0xe);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x241);
    pass_pointer_clear(e, base + 0x74);
    pass_pointer_clear(e, base + 0x78);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0x7c);
    ni_pointer_clear(e, base + 0x80);
    for offset in (0xd4u32..=0x128).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    let mut count = e.global::<u32>(ELEMENT_COUNT_012003EC);
    if count == 0 {
        count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x1a
        } else {
            0x4e
        };
        e.set_global::<u32>(ELEMENT_COUNT_012003EC, count);
    }
    let buffer = allocate_array(e, count, 0x30);
    e.mem.set_u32(base + 0x70, buffer);
    e.call(MEMSET, &args![buffer, 0u32, count.wrapping_mul(0x30)]);
    e.call(MEMSET, &args![base + 0x84, 0u32, 0x50u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_53f0, fn_00ba53f0(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(count: u32, model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.map(0x0120_0000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global::<u32>(ELEMENT_COUNT_012003EC, count);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x130);
        e.mem.write(this, &[0xcc; 0x130]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00ba_53f0, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00ba53f0_builds_the_shader() {
        let (e, this, declaration) = construct(0, 2);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010B80A8);
        for offset in [0x74u32, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in (0xd4u32..=0x128).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xd0), 0xcccc_cccc);
        assert_eq!(e.mem.u32(this + 0x68), 0xe);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x241);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x4e);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xea0]]
        );
        let buffer = e.mem.u32(this + 0x70);
        assert_eq!(
            calls(&e, MEMSET),
            vec![vec![buffer, 0, 0xea0], vec![this + 0x84, 0, 0x50]]
        );
    }

    #[test]
    fn fn_00ba53f0_count_follows_the_model_once() {
        let (e, _, _) = construct(0, 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x1a);
        let (e, _, _) = construct(3, 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 3);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x90]]);
    }
}
