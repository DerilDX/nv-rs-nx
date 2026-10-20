//! `NiXenonRenderer/nixenonshaderdeclaration.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor `00e78840` (engine map unit `nixenonshaderdeclaration.cpp`, unnamed;
/// `thiscall`, no stack words) and the initializer `00e78c60` (unit
/// `nid3dshaderdeclaration.cpp`, unnamed; `thiscall`: renderer, entry count, stream count;
/// answers a byte).
const BASE_CONSTRUCT_00E78840: u32 = 0x00e7_8840;
const INITIALIZE_00E78C60: u32 = 0x00e7_8c60;
/// The declaration's vtable (`010ee70c`; its RTTI type descriptor reads
/// `.?AVNiDX9ShaderDeclaration@@`).
const DECLARATION_VTABLE: u32 = 0x010e_e70c;

// Translated from 00e76700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonShaderDeclaration::Create` (Xbox PDB), `cdecl` (renderer, entry count, stream
/// count). Answers 0 when the stream count is above the renderer's word at +0x6e8 (unsigned).
/// Otherwise allocates 0x38 bytes (0 when that fails), runs the base constructor `00e78840`,
/// stores the vtable `010ee70c`, clears the words +0x2c and +0x30 and the byte +0x34, and runs
/// the initializer `00e78c60` with the three arguments; when it answers a clear byte the
/// declaration is deleted through its vtable slot 0 with 1 and 0 is answered, else the
/// declaration.
pub fn ni_xenon_shader_declaration_create(
    e: &mut Engine,
    renderer: Ptr,
    entry_count: u32,
    stream_count: u32,
) -> Ptr {
    if stream_count > e.mem.u32(renderer.addr() + 0x6e8) {
        return Ptr::new(0);
    }
    let declaration = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
    if declaration == 0 {
        return Ptr::new(0);
    }
    e.call(BASE_CONSTRUCT_00E78840, &args![declaration]);
    e.mem.set_u32(declaration, DECLARATION_VTABLE);
    e.mem.set_u32(declaration + 0x2c, 0);
    e.mem.set_u32(declaration + 0x30, 0);
    e.mem.set_u8(declaration + 0x34, 0);
    let ready = e
        .call(
            INITIALIZE_00E78C60,
            &args![declaration, renderer, entry_count, stream_count],
        )
        .bool();
    if !ready {
        e.vcall(declaration, 0, &args![1u32]);
        return Ptr::new(0);
    }
    Ptr::new(declaration)
}

/// The base constructor `00ee88b0` (no unit in the engine map; `thiscall`, no stack words)
/// and the `NiD3DShaderDeclaration` vtable (`010ee7cc`; RTTI
/// `.?AVNiD3DShaderDeclaration@@`).
const SHADER_DECLARATION_BASE_00EE88B0: u32 = 0x00ee_88b0;
const D3D_DECLARATION_VTABLE_010EE7CC: u32 = 0x010e_e7cc;

// Translated from 00e78840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nixenonshaderdeclaration.cpp`): the
/// `NiD3DShaderDeclaration` constructor (the vtable it stores has that class's RTTI),
/// `thiscall`, no stack words. Runs the base constructor `00ee88b0`, clears the words +0x8 to
/// +0x24 (`m_pkRenderer` to `m_pkStreamEntries`, Xbox PDB) and the byte +0x28 (`m_bModified`),
/// stores the vtable `010ee7cc` and answers `this`.
pub fn fn_00e78840(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_DECLARATION_BASE_00EE88B0, &args![base]);
    for offset in (0x8u32..=0x24).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x28, 0);
    e.mem.set_u32(base, D3D_DECLARATION_VTABLE_010EE7CC);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_6700, ni_xenon_shader_declaration_create(Ptr, u32, u32) -> Ptr),
        entry!(0x00e7_8840, fn_00e78840(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0e30_0000;

    fn engine(ready: u32, allocation: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register_double(NI_ALLOCATE, move |e, a| {
            if allocation {
                e.mem.alloc(a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(BASE_CONSTRUCT_00E78840, |_, a| a[0].into_ret());
        e.register_double(INITIALIZE_00E78C60, move |_, _| ready.into_ret());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(DECLARATION_VTABLE, &[DELETE]);
        e.call_log = Some(vec![]);
        let renderer = e.mem.alloc(0x700);
        e.mem.set_u32(renderer + 0x6e8, 2);
        (e, renderer)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_builds_and_initializes_the_declaration() {
        let (mut e, renderer) = engine(1, true);
        let made = e.call(0x00e7_6700, &args![renderer, 5u32, 2u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), DECLARATION_VTABLE);
        assert_eq!(e.mem.u32(made + 0x2c), 0);
        assert_eq!(e.mem.u8(made + 0x34), 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x38]]);
        assert_eq!(calls(&e, BASE_CONSTRUCT_00E78840), vec![vec![made]]);
        assert_eq!(
            calls(&e, INITIALIZE_00E78C60),
            vec![vec![made, renderer, 5, 2]]
        );
        assert!(calls(&e, DELETE).is_empty());
    }

    #[test]
    fn create_deletes_a_declaration_that_fails_to_initialize() {
        let (mut e, renderer) = engine(0x100, true);
        assert_eq!(e.call(0x00e7_6700, &args![renderer, 5u32, 1u32]).u32(), 0);
        let made = calls(&e, BASE_CONSTRUCT_00E78840)[0][0];
        assert_eq!(calls(&e, DELETE), vec![vec![made, 1]]);
    }

    #[test]
    fn create_refuses_too_many_streams_and_a_failed_allocation() {
        let (mut e, renderer) = engine(1, true);
        assert_eq!(e.call(0x00e7_6700, &args![renderer, 5u32, 3u32]).u32(), 0);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
        let (mut e, renderer) = engine(1, false);
        assert_eq!(e.call(0x00e7_6700, &args![renderer, 5u32, 2u32]).u32(), 0);
        assert!(calls(&e, BASE_CONSTRUCT_00E78840).is_empty());
    }
}

#[cfg(test)]
mod b000l_tests {
    use super::*;

    #[test]
    fn fn_00e78840_clears_the_declaration() {
        let mut e = Engine::new();
        e.register(SHADER_DECLARATION_BASE_00EE88B0, |e, a| {
            e.mem.set_u32(a[0], 0x0e40_0000);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcc; 0x2c]);
        assert_eq!(e.call(0x00e7_8840, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), D3D_DECLARATION_VTABLE_010EE7CC);
        assert_eq!(e.mem.u32(this + 4), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 8, 0x21), vec![0; 0x21]);
        assert_eq!(e.mem.bytes(this + 0x29, 3), vec![0xcc; 3]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SHADER_DECLARATION_BASE_00EE88B0, vec![this]));
    }
}
