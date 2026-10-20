//! `(unplaced)/run_004887a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Copies the save buffer's change flags (the word at +0x17 of the
/// buffer) into a local (`00428110`) and tests a mask on them
/// (`004280f0`).
const COPY_CHANGE_FLAGS: u32 = 0x0042_8110;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `BGSSaveGameBuffer`: the byte save (`00865e50`), `SaveFormID_ov2`
/// (`00865df0`, two words), `StartVariableSizedValue` (`00865f20`) and
/// `SaveVariableSizedValue_ov2` (`00865ff0`).
const SAVE_BYTES: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const FINISH_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// List helpers: the item pointer's address of a node (`006815c0`) and the
/// next node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `BSFaceGenKeyframe::GetTime` (`00488d50`, engine map; thiscall, the
/// time in ST0).
const KEYFRAME_GET_TIME: u32 = 0x0048_8d50;
/// The `float` saved when an entry has no keyframe (`01012054`, -1.0).
const NO_TIME: u32 = 0x0101_2054;
/// The list whose first node sits at +0x14 of `this`.
const LIST_14: u32 = 0x14;
/// An entry: its form (+0), two 16-bit fields (+4, +6) and its keyframe
/// (+8).
const ENTRY_FIELD_4: u32 = 4;
const ENTRY_FIELD_6: u32 = 6;
const ENTRY_KEYFRAME: u32 = 8;

// Translated from 00488c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the list at +0x14 into the save buffer when its change flags have
/// bit 31 (thiscall, `RET 4`; unnamed in the engine map): a counted list
/// (variable-sized count patched in at the end) of the non-null entries,
/// each its form id (`SaveFormID_ov2(form, 0)`), the 16-bit fields at +6
/// and +4, and a `float` time: -1.0 (`01012054`) or, with a keyframe at +8,
/// its time (`00488d50`, stored as a `float`).
pub fn fn_00488c50(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let wanted = e.with_stack(4, |e, local| {
        let flags = e.call(COPY_CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, 0x8000_0000u32]).u32() & 0xff != 0
    });
    if !wanted {
        return;
    }
    let mut count = 0u32;
    let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = this.addr() + LIST_14;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            let form = e.mem.u32(entry);
            e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
            e.call(
                SAVE_BYTES,
                &args![buffer, entry + ENTRY_FIELD_6, 2u32, 0u32],
            );
            e.call(
                SAVE_BYTES,
                &args![buffer, entry + ENTRY_FIELD_4, 2u32, 0u32],
            );
            let mut time = x87_quiet_bits(e.global::<u32>(NO_TIME));
            let keyframe = e.mem.u32(entry + ENTRY_KEYFRAME);
            if keyframe != 0 {
                let value = e.call(KEYFRAME_GET_TIME, &args![keyframe]).f64();
                time = x87_store32(value);
            }
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), time);
                e.call(SAVE_BYTES, &args![buffer, local, 4u32, 0u32]);
            });
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(FINISH_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_8c50, fn_00488c50(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORD: u32 = 0x0342_0000;

    fn setup(e: &mut Engine, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 0x17, flags);
        e.register(COPY_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(FLAGS_TEST, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| 0x40u32.into_ret());
        e.register(FINISH_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(KEYFRAME_GET_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.map(NO_TIME, 4);
        e.mem.set_u32(NO_TIME, (-1.0f32).to_bits());
        e.map(RECORD, 0x100);
        e.register(SAVE_BYTES, |e, a| {
            let used = e.mem.u32(RECORD);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(RECORD + 4 + used, &bytes);
            e.mem.set_u32(RECORD, used + a[2]);
            Ret::default()
        });
        buffer
    }

    #[test]
    fn saves_each_entry_with_its_time() {
        let mut e = Engine::new();
        let buffer = setup(&mut e, 0x8000_0000);
        let with_keyframe = e.mem.alloc(12);
        e.mem.set_u32(with_keyframe, 0x0700_0001);
        e.mem.set_u16(with_keyframe + 4, 0x0102);
        e.mem.set_u16(with_keyframe + 6, 0x0304);
        e.mem.set_u32(with_keyframe + 8, 0x0800_0000);
        let without = e.mem.alloc(12);
        e.mem.set_u32(without, 0x0700_0002);
        e.mem.set_u16(without + 4, 0x0506);
        e.mem.set_u16(without + 6, 0x0708);
        let null_node = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_u32(last, without);
        e.mem.set_u32(null_node + 4, last);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + LIST_14, with_keyframe);
        e.mem.set_u32(this + LIST_14 + 4, null_node);
        e.call_log = Some(vec![]);
        e.call(0x0048_8c50, &args![this, buffer]);
        let mut expected = vec![0x04, 0x03, 0x02, 0x01];
        expected.extend(2.5f32.to_le_bytes());
        expected.extend([0x08, 0x07, 0x06, 0x05]);
        expected.extend((-1.0f32).to_le_bytes());
        assert_eq!(e.mem.bytes(RECORD + 4, e.mem.u32(RECORD)), expected);
        let log = e.call_log.take().unwrap();
        let forms: Vec<_> = log.iter().filter(|(a, _)| *a == SAVE_FORM_ID).collect();
        assert_eq!(forms[0].1, vec![buffer, 0x0700_0001, 0]);
        assert_eq!(forms[1].1, vec![buffer, 0x0700_0002, 0]);
        let finish: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == FINISH_VARIABLE_SIZED_VALUE)
            .collect();
        assert_eq!(finish[0].1, vec![buffer, 2, 0x40]);
    }

    #[test]
    fn saves_nothing_without_the_flag() {
        let mut e = Engine::new();
        let buffer = setup(&mut e, 0x7fff_ffff);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x0048_8c50, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == START_VARIABLE_SIZED_VALUE));
        assert_eq!(e.mem.u32(RECORD), 0);
    }
}
