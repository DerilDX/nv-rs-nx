//! `fallout shared/bgsimpactdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `.?AVTESForm@@`, `.?AVBGSTextureSet@@` and `.?AVTESSound@@`: the type descriptors of the
/// casts.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TEXTURE_SET_TYPE: u32 = 0x0118_7788;
const SOUND_TYPE: u32 = 0x0118_32fc;
/// "MASTERFILE: Unable to find texture set (%08X) on impact data %s (%08X)."
const MISSING_TEXTURE_SET: u32 = 0x0103_30f0;
/// "MASTERFILE: Unable to find sound (%08X) on impact data %s (%08X)."
const MISSING_SOUND: u32 = 0x0103_30a8;

/// Resolves the form ID at `offset`: when not 0, fixed up in a stack slot through
/// `00485d50` (cdecl, the slot and the form's file from `00484e60` with -1), looked up
/// with `004839c0`, cast to `target_type` and stored back at `offset`; a null result
/// prints `warning` with the ID, the form's name (vtable slot +0x130) and the word
/// `0084e3a0` gives for it.
fn resolve_member(e: &mut Engine, this: Ptr, offset: u32, target_type: u32, warning: u32) {
    let base = this.addr();
    let loaded = e.mem.u32(base + offset);
    if loaded == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), loaded);
        let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, target_type, 0u32],
        )
        .u32();
    e.mem.set_u32(base + offset, cast);
    if e.mem.u32(base + offset) != 0 {
        return;
    }
    let form_id = e.call(0x0084_e3a0, &args![this]).u32();
    let name = e.vcall(base, 0x130, &args![]).u32();
    e.call(0x005b_5e40, &args![warning, id, name, form_id]);
}

// Translated from 0058e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): when `0058de80` answers true, fills
/// the 0x24-byte record at +0x54 through `005932b0` from the record `0056a880` gives for
/// the object `006733e0` returns, or, when that is null, from the one `0056a8c0` gives
/// (called without arguments).
pub fn fn_0058e730(e: &mut Engine, this: Ptr) {
    if !e.call(0x0058_de80, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    if e.call(0x0067_33e0, &args![this]).u32() != 0 {
        let owner = e.call(0x0067_33e0, &args![this]).u32();
        let record = e.call(0x0056_a880, &args![owner]).u32();
        e.call(0x0059_32b0, &args![base + 0x54, record]);
    } else {
        let record = e.call(0x0056_a8c0, &args![]).u32();
        e.call(0x0059_32b0, &args![base + 0x54, record]);
    }
}

// Translated from 0058e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSImpactData::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, resolves the texture set at +0x48 and the sounds at +0x4c and +0x50 (see
/// `resolve_member`), runs `0058e730`, then `00484ab0` with 1.
pub fn bgs_impact_data_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    resolve_member(e, this, 0x48, TEXTURE_SET_TYPE, MISSING_TEXTURE_SET);
    resolve_member(e, this, 0x4c, SOUND_TYPE, MISSING_SOUND);
    resolve_member(e, this, 0x50, SOUND_TYPE, MISSING_SOUND);
    fn_0058e730(e, this);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_e730, fn_0058e730(Ptr)),
        entry!(0x0058_e560, bgs_impact_data_init_item(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// Doubles for `0058e730`'s callees: the check answers `active`, the owner is `owner`.
    fn record_case(e: &mut Engine, active: bool, owner: u32) {
        e.register_double(0x0058_de80, move |_, _| Ret {
            eax: u32::from(active),
            ..Ret::default()
        });
        e.register_double(0x0067_33e0, move |_, _| Ret {
            eax: owner,
            ..Ret::default()
        });
        e.register(0x0056_a880, |_, a| Ret {
            eax: a[0] + 0x24,
            ..Ret::default()
        });
        e.register(0x0056_a8c0, |_, _| Ret {
            eax: 0x0abc_0000,
            ..Ret::default()
        });
        e.register(0x0059_32b0, |_, _| Ret::default());
    }

    #[test]
    fn fn_0058e730_copies_the_owner_or_default_record() {
        for (active, owner, expected) in [
            (false, 0x7000u32, None),
            (true, 0x7000, Some(0x7024u32)),
            (true, 0, Some(0x0abc_0000)),
        ] {
            let mut e = Engine::new();
            record_case(&mut e, active, owner);
            let data = e.mem.alloc(0x80);
            e.call_log = Some(vec![]);
            e.call(0x0058_e730, &args![data]);
            let log = e.call_log.take().unwrap();
            match expected {
                None => assert_eq!(log.len(), 2),
                Some(record) => {
                    assert_eq!(calls_to(&log, 0x0059_32b0), vec![vec![data + 0x54, record]]);
                }
            }
        }
    }

    #[test]
    fn impact_data_init_item_resolves_the_members() {
        // IDs below 0x2000 are found (cast gives the form + 1), others are not.
        let mut e = Engine::new();
        record_case(&mut e, false, 0);
        let data = e.mem.alloc(0x80);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(data, 0x0300_0000);
        e.mem.set_u32(data + 0x48, 0x10);
        e.mem.set_u32(data + 0x4c, 0);
        e.mem.set_u32(data + 0x50, 0x3000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 1);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x2000 { a[0] + 0x100 } else { 0 },
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.register(0x0048_4ab0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_e560, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(data + 0x48), 0x111);
        assert_eq!(e.mem.u32(data + 0x4c), 0);
        assert_eq!(e.mem.u32(data + 0x50), 0);
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![
                vec![0x11, 0, TES_FORM_TYPE, TEXTURE_SET_TYPE, 0],
                vec![0x3001, 0, TES_FORM_TYPE, SOUND_TYPE, 0],
            ]
        );
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![MISSING_SOUND, 0x3001, 0x7777, 0xbeef]]
        );
        assert_eq!(calls_to(&log, 0x0058_de80), vec![vec![data]]);
        assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![data, 1]));
    }

    #[test]
    fn impact_data_init_item_stops_when_checked() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x80);
        e.register(0x0040_13e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_e560, &args![data]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
