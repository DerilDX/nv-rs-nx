//! `NiXenonRenderer/nixenonerrorshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::NiTArray;

/// The base constructor `00a9be80` (no unit in the engine map; `thiscall`, one stack word: the
/// program type), `NiD3DShaderConstantMap`'s renderer set-up `00e81d00` (unit
/// `nid3dshaderconstantmap.cpp`, unnamed; `thiscall`, one word) and the entry array's
/// initializer `00b799c0` (unnamed; `thiscall` on the array, no stack words).
const BASE_CONSTRUCT_00A9BE80: u32 = 0x00a9_be80;
const SET_RENDERER_00E81D00: u32 = 0x00e8_1d00;
const ARRAY_INITIALIZE_00B799C0: u32 = 0x00b7_99c0;
/// The map's vtable (`010ef544`, RTTI `.?AVNiD3DShaderConstantMap@@`) and its two entry
/// arrays' (`010f003c`, RTTI `.?AV?$NiTObjectArray@V?$NiPointer@VNiShaderConstantMapEntry@@@@@@`).
const CONSTANT_MAP_VTABLE_010EF544: u32 = 0x010e_f544;
const ENTRY_ARRAY_VTABLE_010F003C: u32 = 0x010f_003c;

/// An empty `NiTObjectArray` at `array`: grow-by 1, max size, size and element count 0, no
/// elements, the array's vtable (the order the code stores them).
fn construct_entry_array(e: &mut Engine, array: u32) {
    let array: Ptr<NiTArray> = Ptr::new(array);
    e.set(array, NiTArray::m_usGrowBy, 1u16);
    e.set(array, NiTArray::m_usMaxSize, 0u16);
    e.set(array, NiTArray::m_usSize, 0u16);
    e.set(array, NiTArray::m_usESize, 0u16);
    e.set(array, NiTArray::m_pBase, 0u32);
    e.mem.set_u32(array.addr(), ENTRY_ARRAY_VTABLE_010F003C);
}

// Translated from 00b7e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (engine map), `thiscall` with two stack
/// words: the renderer and the program type. Runs the base constructor `00a9be80` with the
/// program type, stores the vtable `010ef544`, makes the two empty entry arrays
/// `m_aspEntries` (+0xc) and `m_aspDefinedEntries` (+0x1c) (Xbox PDB), clears
/// `m_pkLastShaderProgram`, `m_eLastError`, `m_pkD3DDevice`, `m_pkD3DRenderer` and
/// `m_pkD3DRenderState` (+0x30 to +0x40), sets `m_bModified` (+0x2c), passes the renderer to
/// `00e81d00` and runs `00b799c0` on the entry array at +0xc. Answers `this`.
// Translation incomplete: 00b7e330 - the C++ exception frame (FS:[0] chain, unwind state 2 for the base and the arrays) is not translated.
pub fn ni_d3d_shader_constant_map_ni_d3d_shader_constant_map(
    e: &mut Engine,
    this: Ptr,
    renderer: Ptr,
    program_type: u32,
) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00A9BE80, &args![base, program_type]);
    e.mem.set_u32(base, CONSTANT_MAP_VTABLE_010EF544);
    construct_entry_array(e, base + 0xc);
    construct_entry_array(e, base + 0x1c);
    for offset in [0x30u32, 0x34, 0x38, 0x3c, 0x40] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x2c, 1);
    e.call(SET_RENDERER_00E81D00, &args![base, renderer]);
    e.call(ARRAY_INITIALIZE_00B799C0, &args![base + 0xc]);
    this
}

/// The map's destructor `00e87690` (unit `nid3dshaderconstantmap.cpp`, unnamed; `thiscall`) and
/// the sized free `00aa1460` (`cdecl`: block, size).
const CONSTANT_MAP_DESTRUCT_00E87690: u32 = 0x00e8_7690;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00b7e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::_scalar_deleting_destructor_` (engine map), `thiscall` with one
/// stack word (the flags): runs the destructor `00e87690`; with bit 0 set, frees the 0x44 bytes
/// (`00aa1460`). Answers `this`.
pub fn ni_d3d_shader_constant_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(CONSTANT_MAP_DESTRUCT_00E87690, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x44u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b7_e330,
            ni_d3d_shader_constant_map_ni_d3d_shader_constant_map(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00b7_e410,
            ni_d3d_shader_constant_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_map_constructor_builds_both_arrays_in_order() {
        let mut e = Engine::new();
        for target in [
            BASE_CONSTRUCT_00A9BE80,
            SET_RENDERER_00E81D00,
            ARRAY_INITIALIZE_00B799C0,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x44);
        e.mem.write(this, &[0xcc; 0x44]);
        let answer = e
            .call(0x00b7_e330, &args![this, 0x0e50_0000u32, 3u32])
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), CONSTANT_MAP_VTABLE_010EF544);
        for array in [this + 0xc, this + 0x1c] {
            assert_eq!(e.mem.u32(array), ENTRY_ARRAY_VTABLE_010F003C);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u16(array + 8), 0);
            assert_eq!(e.mem.u16(array + 0xa), 0);
            assert_eq!(e.mem.u16(array + 0xc), 0);
            assert_eq!(e.mem.u16(array + 0xe), 1);
        }
        assert_eq!(e.mem.u8(this + 0x2c), 1);
        assert_eq!(e.mem.bytes(this + 0x2d, 3), vec![0xcc; 3]);
        assert_eq!(e.mem.bytes(this + 0x30, 0x14), vec![0; 0x14]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT_00A9BE80, vec![this, 3]),
                (SET_RENDERER_00E81D00, vec![this, 0x0e50_0000]),
                (ARRAY_INITIALIZE_00B799C0, vec![this + 0xc]),
            ]
        );
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;

    #[test]
    fn constant_map_scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        e.register(CONSTANT_MAP_DESTRUCT_00E87690, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x44);
        assert_eq!(e.call(0x00b7_e410, &args![this, 3u32]).u32(), this);
        e.call(0x00b7_e410, &args![this, 2u32]);
        let log: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(addr, _)| *addr != 0x00b7_e410)
            .collect();
        assert_eq!(
            log,
            vec![
                (CONSTANT_MAP_DESTRUCT_00E87690, vec![this]),
                (NI_FREE_SIZED_00AA1460, vec![this, 0x44]),
                (CONSTANT_MAP_DESTRUCT_00E87690, vec![this]),
            ]
        );
    }
}
