//! `NiXenonRenderer/run_00e89ed0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The bucket allocation `00aa1070` (`cdecl`, size), `memset` (`00ec61c0`), the set-up that
/// ends the constructor (`00e8b170`, unnamed; `thiscall`, no stack words) and the map's
/// virtual tables (RTTI `NiTMapBase<..., NiLight *, NiDX9LightManager::LightEntry *>`, then
/// `NiTPointerMap<NiLight *, NiDX9LightManager::LightEntry *>`).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const FINISH_00E8B170: u32 = 0x00e8_b170;
const LIGHT_MAP_BASE_VTABLE: u32 = 0x010e_fdd8;
const LIGHT_MAP_VTABLE: u32 = 0x010e_fdf8;

// Translated from 00e8bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the light manager constructor (its map at +0
/// holds `NiDX9LightManager::LightEntry` records by the RTTI), `thiscall` with two words: the
/// shader library and the device. The map (0x25 buckets of `00aa1070`, cleared with `memset`;
/// base then final virtual table), +0x10, +0x14, +0x18 cleared, the library at +0x20, -1 at
/// +0x24, the device at +0x1c (counted: its virtual slot 4), +0x38 cleared and the 0x200 bytes
/// from +0x3c cleared (`memset`), then `00e8b170`. Answers `this`.
pub fn fn_00e8bae0(e: &mut Engine, this: Ptr, library: u32, device: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, LIGHT_MAP_BASE_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0x25);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
    e.mem.set_u32(base.wrapping_add(8), buckets);
    let size = e.mem.u32(base.wrapping_add(4)).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u32(base, LIGHT_MAP_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x18), 0);
    e.mem.set_u32(base.wrapping_add(0x10), 0);
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    e.mem.set_u32(base.wrapping_add(0x20), library);
    e.mem.set_u32(base.wrapping_add(0x24), 0xffff_ffff);
    e.mem.set_u32(base.wrapping_add(0x1c), device);
    e.vcall(device, 4, &args![]);
    e.mem.set_u32(base.wrapping_add(0x38), 0);
    e.call(MEMSET, &args![base.wrapping_add(0x3c), 0u32, 0x200u32]);
    e.call(FINISH_00E8B170, &args![base]);
    this
}

// Translated from 00e8b170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e89ed0`): the light manager's reset, `thiscall`, no
/// stack words. Calls the render state's (+0x20) virtual slot 0x68 with (0x8b, 0, 0, 0) and
/// (0x89, 0, 0, 0), clears +0x30, the byte +0x2d and +0x34, the same slot with 0x94, 0x93 and
/// 0x91, then clears the byte +0x2c and copies +0x24 to +0x28.
pub fn fn_00e8b170(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let set_state = |e: &mut Engine, state: u32| {
        let render_state = e.mem.u32(base + 0x20);
        e.vcall(render_state, 0x68, &args![state, 0u32, 0u32, 0u32]);
    };
    set_state(e, 0x8b);
    set_state(e, 0x89);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u8(base + 0x2d, 0);
    e.mem.set_u32(base + 0x34, 0);
    set_state(e, 0x94);
    set_state(e, 0x93);
    set_state(e, 0x91);
    e.mem.set_u8(base + 0x2c, 0);
    let value = e.mem.u32(base + 0x24);
    e.mem.set_u32(base + 0x28, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_bae0, fn_00e8bae0(Ptr, u32, u32) -> Ptr),
        entry!(0x00e8_b170, fn_00e8b170(Ptr)),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e8bae0_builds_the_light_manager() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(FINISH_00E8B170, |_, _| Ret::default());
        e.put_vtable(0x6000, &[0, 0x6100]);
        e.register(0x6100, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x23c);
        assert_eq!(
            e.call(0x00e8_bae0, &args![this, 0x99u32, device]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), LIGHT_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 0x20), 0x99);
        assert_eq!(e.mem.u32(this + 0x1c), device);
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ffff);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6100, vec![device])));
        assert!(log.contains(&(MEMSET, vec![this + 0x3c, 0, 0x200])));
        assert_eq!(log.last().unwrap(), &(FINISH_00E8B170, vec![this]));
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e8b170_resets_the_light_states() {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x20).map(|i| 0x6100 + 4 * i).collect();
        e.put_vtable(0x6000, &slots);
        e.register(0x6168, |_, _| Ret::default());
        let state = e.mem.alloc(4);
        e.mem.set_u32(state, 0x6000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, state);
        e.mem.set_u32(this + 0x24, 0x99);
        e.mem.set_u32(this + 0x2c, 0xffff_ffff);
        e.mem.set_u32(this + 0x30, 5);
        e.call_log = Some(vec![]);
        e.call(0x00e8_b170, &args![this]);
        let states: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x6168)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(states, vec![0x8b, 0x89, 0x94, 0x93, 0x91]);
        assert_eq!(e.mem.u32(this + 0x2c), 0xffff_0000);
        assert_eq!(e.mem.u32(this + 0x28), 0x99);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
}
