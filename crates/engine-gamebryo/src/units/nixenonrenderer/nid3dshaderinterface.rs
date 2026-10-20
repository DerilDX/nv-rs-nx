//! `NiXenonRenderer/nid3dshaderinterface.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The five set-up calls `InitializeShaderSystem` makes with the renderer (`cdecl`, unnamed in
/// the engine map), the global it keeps the renderer's shader library in (`0126f99c`), the
/// getter `00e91f10` (unnamed, no arguments), the program factory global (`0126fa10`) and its
/// constructor (`00e93ad0`, unnamed; `thiscall`, no stack words), the allocation `00aa13e0`,
/// the report `00a29680` and its message (`"Failed to create shader program factory!"`).
const SET_UP_00E7F920: u32 = 0x00e7_f920;
const SET_UP_00E7E8D0: u32 = 0x00e7_e8d0;
const SET_UP_00E92F70: u32 = 0x00e9_2f70;
const SET_UP_00E93F50: u32 = 0x00e9_3f50;
const SET_UP_00E7EEA0: u32 = 0x00e7_eea0;
const SHADER_LIBRARY_0126F99C: u32 = 0x0126_f99c;
const GETTER_00E91F10: u32 = 0x00e9_1f10;
const PROGRAM_FACTORY_0126FA10: u32 = 0x0126_fa10;
const PROGRAM_FACTORY_CONSTRUCT_00E93AD0: u32 = 0x00e9_3ad0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const REPORT_00A29680: u32 = 0x00a2_9680;
const FACTORY_FAILED_010EF488: u32 = 0x010e_f488;

// Translated from 00e81940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderInterface::InitializeShaderSystem` (engine map), `cdecl`, the renderer; answers
/// 0. Runs `00e7f920`, `00e7e8d0` (then keeps the renderer's shader library, +0x8b8, at
/// `0126f99c`), `00e92f70`, `00e93f50` and `00e7eea0` with the renderer, the library's virtual
/// slot 0x100, and, when `00e91f10` answers an object, its slot 0x70 with the renderer. Without
/// a program factory (`0126fa10`) one is made (0x1c bytes, `00e93ad0`); a null result reports
/// "Failed to create shader program factory!" (a failed allocation also clears the global).
pub fn ni_d3d_shader_interface_initialize_shader_system(e: &mut Engine, renderer: u32) -> u32 {
    let library = e.mem.u32(renderer.wrapping_add(0x8b8));
    e.call(SET_UP_00E7F920, &args![renderer]);
    e.call(SET_UP_00E7E8D0, &args![renderer]);
    e.mem.set_u32(SHADER_LIBRARY_0126F99C, library);
    e.call(SET_UP_00E92F70, &args![renderer]);
    e.call(SET_UP_00E93F50, &args![renderer]);
    e.call(SET_UP_00E7EEA0, &args![renderer]);
    e.vcall(library, 0x100, &args![]);
    let object = e.call(GETTER_00E91F10, &args![]).u32();
    if object != 0 {
        e.vcall(object, 0x70, &args![renderer]);
    }
    if e.mem.u32(PROGRAM_FACTORY_0126FA10) != 0 {
        return 0;
    }
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    if block == 0 {
        e.mem.set_u32(PROGRAM_FACTORY_0126FA10, 0);
        e.call(REPORT_00A29680, &args![FACTORY_FAILED_010EF488]);
        return 0;
    }
    let factory = e
        .call(PROGRAM_FACTORY_CONSTRUCT_00E93AD0, &args![block])
        .u32();
    e.mem.set_u32(PROGRAM_FACTORY_0126FA10, factory);
    if factory == 0 {
        e.call(REPORT_00A29680, &args![FACTORY_FAILED_010EF488]);
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e8_1940,
        ni_d3d_shader_interface_initialize_shader_system(u32) -> u32
    )]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(factory: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let mut slots = vec![0u32; 0x41];
        slots[0x40] = 0x6200;
        slots[0x1c] = 0x6300;
        e.put_vtable(0x6000, &slots);
        for no_op in [
            SET_UP_00E7F920,
            SET_UP_00E7E8D0,
            SET_UP_00E92F70,
            SET_UP_00E93F50,
            SET_UP_00E7EEA0,
            REPORT_00A29680,
            0x6200,
            0x6300,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let library = e.mem.alloc(4);
        e.mem.set_u32(library, 0x6000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x8b8, library);
        e.register_double(GETTER_00E91F10, move |_, _| library.into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(PROGRAM_FACTORY_CONSTRUCT_00E93AD0, move |_, _| {
            factory.into_ret()
        });
        e.call_log = Some(vec![]);
        (e, renderer, library)
    }

    #[test]
    fn initialize_shader_system_sets_up_and_makes_the_factory() {
        let (mut e, renderer, library) = rig(0x7777);
        assert_eq!(e.call(0x00e8_1940, &args![renderer]).u32(), 0);
        assert_eq!(e.mem.u32(SHADER_LIBRARY_0126F99C), library);
        assert_eq!(calls(&e, 0x6200), vec![vec![library]]);
        assert_eq!(calls(&e, 0x6300), vec![vec![library, renderer]]);
        assert_eq!(e.mem.u32(PROGRAM_FACTORY_0126FA10), 0x7777);
        assert!(calls(&e, REPORT_00A29680).is_empty());
        // A factory already there: nothing made.
        e.call_log = Some(vec![]);
        e.call(0x00e8_1940, &args![renderer]);
        assert!(calls(&e, NEW_OBJECT).is_empty());
    }

    #[test]
    fn initialize_shader_system_reports_a_failed_factory() {
        let (mut e, renderer, _) = rig(0);
        e.call(0x00e8_1940, &args![renderer]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![FACTORY_FAILED_010EF488]]
        );
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e8_1940, &args![renderer]);
        assert!(calls(&e, PROGRAM_FACTORY_CONSTRUCT_00E93AD0).is_empty());
        assert_eq!(calls(&e, REPORT_00A29680).len(), 1);
    }
}
