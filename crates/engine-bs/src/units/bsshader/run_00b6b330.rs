//! `BSShader/run_00b6b330` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    construct_pointer_array, ni_pointer_assign, ni_pointer_clear, DESTRUCT_00B66F20,
};

/// `NiObject`'s constructor (`00a5d3a0`, unit `niobject.cpp`; `thiscall`, no stack words) and
/// the object's vtable (`010ae500`; no RTTI locator before it).
const NI_OBJECT_CONSTRUCT_00A5D3A0: u32 = 0x00a5_d3a0;
const VTABLE_010AE500: u32 = 0x010a_e500;
/// The destructor the four-pointer array at +0x30 passes (`0045cec0`).
const DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// The value maker `00b6b180` (unnamed; `cdecl`: the texture and the third argument) and the
/// render target maker `00ee8760` (unit `nirendertargetgroup.cpp`, unnamed; `cdecl`, three
/// words: a buffer, the word at `011f4748`, the value).
const VALUE_00B6B180: u32 = 0x00b6_b180;
const RENDER_TARGET_00EE8760: u32 = 0x00ee_8760;
const GLOBAL_011F4748: u32 = 0x011f_4748;
/// `NiRenderedCubeMap::ms_RTTI` (`012708e4`: its static initializer `00fc7950` gives it the
/// name `"NiRenderedCubeMap"`).
const RENDERED_CUBE_MAP_RTTI_012708E4: u32 = 0x0127_08e4;

// Translated from 00b6b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`): the constructor of a 0x40-byte object over a
/// texture, `thiscall` with three stack words: the texture, a byte and a word. Runs
/// `NiObject`'s constructor, stores the vtable `010ae500`, constructs the six `NiPointer`s at
/// +8 (destructor `00b66f20`), clears +0x20 to +0x28, constructs the four `NiPointer`s at
/// +0x30 (destructor `0045cec0`) and assigns the texture to the first of them. The value is
/// `00b6b180(texture, word)` when the byte is set, else 0. When the texture's class
/// (`GetRTTI`, vtable slot +8, then each base through the RTTI's +4) is `NiRenderedCubeMap`:
/// each of the six pointers at +8 gets `00ee8760` of the texture's word at +0x4c + 4 i, the
/// word at `011f4748` and the value. Otherwise (also for a null texture, whose vtable the code
/// still reads) the first gets `00ee8760` of what the texture's vtable slot +0xa4 answers (the
/// word at `011f4748` read first) and the other five are cleared. Then the pointers at +0x20,
/// +0x24 and +0x28 are cleared, +0x2c is set to -1 and `this` is answered.
// Translation incomplete: 00b6b380 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the base and the arrays) is not translated.
pub fn fn_00b6b380(e: &mut Engine, this: Ptr, texture: Ptr, make_value: u8, word: u32) -> Ptr {
    let base = this.addr();
    let texture = texture.addr();
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![base]);
    e.mem.set_u32(base, VTABLE_010AE500);
    construct_pointer_array(e, base + 8, 6, DESTRUCT_00B66F20);
    for offset in [0x20u32, 0x24, 0x28] {
        e.mem.set_u32(base + offset, 0);
    }
    construct_pointer_array(e, base + 0x30, 4, DESTRUCT_0045CEC0);
    ni_pointer_assign(e, base + 0x30, texture);
    let value = if make_value != 0 {
        e.call(VALUE_00B6B180, &args![texture, word]).u32()
    } else {
        0
    };
    let mut cube_map = false;
    if texture != 0 {
        let mut rtti = e.vcall(texture, 8, &args![]).u32();
        while rtti != 0 {
            if rtti == RENDERED_CUBE_MAP_RTTI_012708E4 {
                cube_map = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
    }
    if cube_map {
        for index in 0..6u32 {
            let buffer = e.mem.u32(texture + 0x4c + 4 * index);
            let renderer = e.global::<u32>(GLOBAL_011F4748);
            let target = e
                .call(RENDER_TARGET_00EE8760, &args![buffer, renderer, value])
                .u32();
            ni_pointer_assign(e, base + 8 + 4 * index, target);
        }
    } else {
        let renderer = e.global::<u32>(GLOBAL_011F4748);
        let buffer = e.vcall(texture, 0xa4, &args![]).u32();
        let target = e
            .call(RENDER_TARGET_00EE8760, &args![buffer, renderer, value])
            .u32();
        ni_pointer_assign(e, base + 8, target);
        for index in 1..6u32 {
            ni_pointer_clear(e, base + 8 + 4 * index);
        }
    }
    for offset in [0x20u32, 0x24, 0x28] {
        ni_pointer_clear(e, base + offset);
    }
    e.mem.set_u32(base + 0x2c, 0xffff_ffff);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b6_b380, fn_00b6b380(Ptr, Ptr, u8, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::VECTOR_CONSTRUCT;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const TEXTURE_VTABLE: u32 = 0x0e80_0000;
    const GET_RTTI: u32 = 0x0e81_0000;
    const BUFFER_OF: u32 = 0x0e81_0010;

    /// The rig with a texture whose `GetRTTI` answers an RTTI chain ending in `base_rtti`
    /// (one more RTTI in front of it), render targets made as new objects (count 0).
    fn engine(base_rtti: u32) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_4000, 0x1000);
        e.set_global(GLOBAL_011F4748, 0x0e90_0000u32);
        let mut slots = vec![0u32; 0x2a];
        slots[1] = REFERENCE_DELETE;
        slots[2] = GET_RTTI;
        slots[0x29] = BUFFER_OF;
        e.put_vtable(TEXTURE_VTABLE, &slots);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, base_rtti);
        e.register_double(GET_RTTI, move |_, _| rtti.into_ret());
        e.register(BUFFER_OF, |_, _| 0x0e91_0000u32.into_ret());
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |_, a| a[0].into_ret());
        e.register(VALUE_00B6B180, |_, _| 0x55u32.into_ret());
        e.register(RENDER_TARGET_00EE8760, |e, _| object(e, 0).into_ret());
        let texture = e.mem.alloc(0x70);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        for index in 0..6 {
            e.mem
                .set_u32(texture + 0x4c + 4 * index, 0x0e92_0000 + index);
        }
        (e, texture)
    }

    #[test]
    fn fn_00b6b380_takes_six_targets_from_a_cube_map() {
        let (mut e, texture) = engine(RENDERED_CUBE_MAP_RTTI_012708E4);
        let this = e.mem.alloc(0x40);
        let answer = e.call(0x00b6_b380, &args![this, texture, 1u32, 9u32]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), VTABLE_010AE500);
        assert_eq!(e.mem.u32(this + 0x30), texture);
        assert_eq!(calls(&e, VALUE_00B6B180), vec![vec![texture, 9]]);
        let targets = calls(&e, RENDER_TARGET_00EE8760);
        let expected: Vec<Vec<u32>> = (0..6)
            .map(|i| vec![0x0e92_0000 + i, 0x0e90_0000, 0x55])
            .collect();
        assert_eq!(targets, expected);
        for index in 0..6 {
            let target = e.mem.u32(this + 8 + 4 * index);
            assert_ne!(target, 0);
            assert_eq!(e.mem.u32(target + 4), 1);
        }
        assert_eq!(e.mem.bytes(this + 0x20, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(this + 0x2c), 0xffff_ffff);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 8, 6, DESTRUCT_00B66F20),
                pointer_array(this + 0x30, 4, DESTRUCT_0045CEC0)
            ]
        );
        assert!(calls(&e, BUFFER_OF).is_empty());
    }

    #[test]
    fn fn_00b6b380_takes_one_target_from_another_texture() {
        let (mut e, texture) = engine(0);
        let this = e.mem.alloc(0x40);
        e.call(0x00b6_b380, &args![this, texture, 0u32, 9u32]);
        assert!(calls(&e, VALUE_00B6B180).is_empty());
        assert_eq!(calls(&e, BUFFER_OF), vec![vec![texture]]);
        assert_eq!(
            calls(&e, RENDER_TARGET_00EE8760),
            vec![vec![0x0e91_0000, 0x0e90_0000, 0]]
        );
        assert_ne!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.bytes(this + 0xc, 0x14), vec![0; 0x14]);
        assert_eq!(calls(&e, GET_RTTI), vec![vec![texture]]);
    }
}
