//! `Havok SDK/hkpshapetype.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The addresses of the shape-type names (`"HK_SHAPE_ALL"`, `"HK_SHAPE_INVALID"`,
/// `"HK_SHAPE_SPHERE"`, ...), indexed by the shape type plus one: the targets of the jump table
/// of `00d464f0`, each `MOV EAX, <string>`.
const SHAPE_TYPE_NAMES: [u32; 34] = [
    0x010c_f70c,
    0x010c_f71c,
    0x010c_f6a8,
    0x010c_f65c,
    0x010c_f694,
    0x010c_f684,
    0x010c_f670,
    0x010c_f640,
    0x010c_f6cc,
    0x010c_f6b8,
    0x010c_f600,
    0x010c_f53c,
    0x010c_f50c,
    0x010c_f4f0,
    0x010c_f57c,
    0x010c_f4d8,
    0x010c_f528,
    0x010c_f6e0,
    0x010c_f4ac,
    0x010c_f6fc,
    0x010c_f4c0,
    0x010c_f628,
    0x010c_f610,
    0x010c_f5e8,
    0x010c_f5c8,
    0x010c_f5b4,
    0x010c_f59c,
    0x010c_f568,
    0x010c_f55c,
    0x010c_f54c,
    0x010c_f490,
    0x010c_f480,
    0x010c_f470,
    0x010c_f460,
];

// Translated from 00d464f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGetShapeTypeName` (Xbox PDB): the address of the name string of a shape type (cdecl, one
/// argument). The type plus one, as an unsigned number, indexes the jump table; `-1` is
/// `"HK_SHAPE_ALL"`, `0` to `32` are the named types, and anything above answers zero.
pub fn hk_get_shape_type_name(_e: &mut Engine, shape_type: u32) -> u32 {
    let index = shape_type.wrapping_add(1);
    if index > 0x21 {
        return 0;
    }
    SHAPE_TYPE_NAMES[index as usize]
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_64f0, hk_get_shape_type_name(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_looked_up_by_type_plus_one() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d4_64f0, &args![0xffff_ffffu32]).u32(),
            0x010c_f70c
        );
        assert_eq!(e.call(0x00d4_64f0, &args![0u32]).u32(), 0x010c_f71c);
        assert_eq!(e.call(0x00d4_64f0, &args![1u32]).u32(), 0x010c_f6a8);
        assert_eq!(e.call(0x00d4_64f0, &args![18u32]).u32(), 0x010c_f6fc);
        assert_eq!(e.call(0x00d4_64f0, &args![32u32]).u32(), 0x010c_f460);
    }

    #[test]
    fn unknown_types_have_no_name() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_64f0, &args![33u32]).u32(), 0);
        assert_eq!(e.call(0x00d4_64f0, &args![0xffff_fffeu32]).u32(), 0);
        assert_eq!(e.call(0x00d4_64f0, &args![0x8000_0000u32]).u32(), 0);
    }
}
