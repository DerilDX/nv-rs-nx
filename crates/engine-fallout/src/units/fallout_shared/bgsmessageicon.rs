//! `fallout shared/bgsmessageicon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0047a300` calls on the member (`00408da0`).
const CALLEE_00408DA0: u32 = 0x0040_8da0;

// Translated from 0047a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00408da0` with `ECX` = `this + 0x4` and passes on its result.
pub fn fn_0047a300(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00408DA0, &args![this.byte_add(0x4)]).u32()
}

// Translated from 0047a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::InitializeDataComponent` (Xbox PDB): runs the
/// `InitializeDataComponent` (vtable slot 0) of the icon's texture
/// component at `this + 4` and passes on its result.
pub fn bgs_message_icon_initialize_data_component(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 4, 0, &[]).u32()
}

// Translated from 0047a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::ClearDataComponent` (Xbox PDB): runs the
/// `ClearDataComponent` (vtable slot 4) of the texture component at
/// `this + 4` and passes on its result.
pub fn bgs_message_icon_clear_data_component(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 4, 4, &[]).u32()
}

// Translated from 0047a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (icon, file): loads the icon's texture chunk:
/// `TESTexture::LoadTextureChunk` (`0048e3d0`) on the texture `00717e50`
/// returns for `icon` (the engine map's name for `00717e50` is
/// `BaseProcess::GetActorPackageThatIsRunning`, a folded body that returns
/// `this + 4`), with `file`.
pub fn fn_0047a290(e: &mut Engine, icon: Ptr, file: Ptr) {
    let texture = e.call(0x0071_7e50, &args![icon]).u32();
    e.call(0x0048_e3d0, &args![texture, file]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_a300, fn_0047a300(Ptr) -> u32),
        entry!(0x0047_a200, bgs_message_icon_initialize_data_component(Ptr) -> u32),
        entry!(0x0047_a220, bgs_message_icon_clear_data_component(Ptr) -> u32),
        entry!(0x0047_a290, fn_0047a290(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0047a300_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00408DA0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a300, &args![0x4100u32]).u32(), 0x1451);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00408DA0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn icon(e: &mut Engine) -> Ptr {
        e.put_vtable(0x0150_0000, &[0x0000_1000, 0x0000_1004]);
        e.register(0x0000_1000, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(0x0000_1004, |_, a| Ret {
            eax: a[0] + 2,
            ..Ret::default()
        });
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p + 4, 0x0150_0000);
        Ptr::new(p)
    }

    #[test]
    fn b020_0047a200_initializes_the_texture_component() {
        let mut e = Engine::new();
        let p = icon(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a200, &args![p]).u32(), p.addr() + 5);
        assert_eq!(
            calls(&mut e, 0x0047_a200),
            vec![(0x0000_1000, vec![p.addr() + 4])]
        );
    }

    #[test]
    fn b020_0047a220_clears_the_texture_component() {
        let mut e = Engine::new();
        let p = icon(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a220, &args![p]).u32(), p.addr() + 6);
        assert_eq!(
            calls(&mut e, 0x0047_a220),
            vec![(0x0000_1004, vec![p.addr() + 4])]
        );
    }

    #[test]
    fn b020_0047a290_loads_the_texture_chunk() {
        let mut e = Engine::new();
        e.register(0x0071_7e50, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(0x0048_e3d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_a290, &args![0x5000u32, 0x9000u32]);
        assert_eq!(
            calls(&mut e, 0x0047_a290),
            vec![
                (0x0071_7e50, vec![0x5000]),
                (0x0048_e3d0, vec![0x5004, 0x9000])
            ]
        );
    }
}
