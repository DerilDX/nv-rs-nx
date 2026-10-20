//! `(unplaced)/run_00e74c00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer's Direct3D device (+0x288), its `TestCooperativeLevel`
/// (COM slot +0xC) and the slot +0x200 the check calls with 0 when the flag
/// at `0126f0d0` is set.
const DEVICE: u32 = 0x288;
const TEST_COOPERATIVE_LEVEL_SLOT: u32 = 0xc;
const DEVICE_SLOT_200: u32 = 0x200;
const DEVICE_SLOT_200_FLAG: u32 = 0x0126_f0d0;
/// `D3DERR_DEVICENOTRESET` and `D3DERR_DEVICELOST`.
const DEVICE_NOT_RESET: u32 = 0x8876_0869;
const DEVICE_LOST: u32 = 0x8876_0868;
/// The reset the renderer runs when the device can be reset (`00e736b0`,
/// `thiscall`, a byte; reached by a tail jump).
const RESET_DEVICE: u32 = 0x00e7_36b0;
/// The device-lost state (+0x6FC), the lost-device callbacks (+0xAF8, a
/// `cdecl (data)` function per entry answering a byte), their data (+0xB08)
/// and their count (+0xAFE, 16-bit).
const DEVICE_LOST_STATE: u32 = 0x6fc;
const LOST_CALLBACKS: u32 = 0xaf8;
const LOST_CALLBACK_DATA: u32 = 0xb08;
const LOST_CALLBACK_COUNT: u32 = 0xafe;
/// The message `00a29680` (`cdecl`, one word) is given when a callback
/// fails (the string at `010ee468`).
const REPORT: u32 = 0x00a2_9680;
const CALLBACK_FAILED_MESSAGE: u32 = 0x010e_e468;

// Translated from 00e74c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the renderer's device check): asks the device
/// `TestCooperativeLevel` (and calls its slot +0x200 with 0 when the flag at
/// `0126f0d0` is set). A device that can be reset is reset (`00e736b0`,
/// whose answer is the answer). A lost device, the first time (+0x6FC),
/// runs the lost-device callbacks in order (a failing one stops them with
/// the message `00a29680`); false either way. Any other result: true.
pub fn fn_00e74c00(e: &mut Engine, this: Ptr) -> bool {
    let device = e.mem.u32(this.addr().wrapping_add(DEVICE));
    let level = e.vcall(device, TEST_COOPERATIVE_LEVEL_SLOT, &args![]).u32();
    if e.mem.u8(DEVICE_SLOT_200_FLAG) != 0 {
        let device = e.mem.u32(this.addr().wrapping_add(DEVICE));
        e.vcall(device, DEVICE_SLOT_200, &args![0u32]);
    }
    if level == DEVICE_NOT_RESET {
        return e.call(RESET_DEVICE, &args![this]).bool();
    }
    if level != DEVICE_LOST {
        return true;
    }
    if e.mem.u8(this.addr().wrapping_add(DEVICE_LOST_STATE)) != 0 {
        return false;
    }
    let count = u32::from(e.mem.u16(this.addr().wrapping_add(LOST_CALLBACK_COUNT)));
    e.mem.set_u8(this.addr().wrapping_add(DEVICE_LOST_STATE), 1);
    for i in 0..count {
        let callbacks = e.mem.u32(this.addr().wrapping_add(LOST_CALLBACKS));
        let callback = e.mem.u32(callbacks.wrapping_add(4 * i));
        let data_array = e.mem.u32(this.addr().wrapping_add(LOST_CALLBACK_DATA));
        let data = e.mem.u32(data_array.wrapping_add(4 * i));
        if callback != 0 && !e.call(callback, &args![data]).bool() {
            e.call(REPORT, &args![CALLBACK_FAILED_MESSAGE]);
            return false;
        }
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e74c00, fn_00e74c00(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const COOPERATIVE: u32 = 0x7200_000c;
    const SLOT_200: u32 = 0x7200_0200;
    const CALLBACK: u32 = 0x7100_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn setup(level: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x204 / 4];
        slots[3] = COOPERATIVE;
        slots[0x200 / 4] = SLOT_200;
        e.put_vtable(0x7000_0000, &slots);
        e.map(0x0126_f000, 0x1000);
        e.map(0x7300_0000, 4);
        e.mem.set_u32(0x7300_0000, level);
        e.register(COOPERATIVE, |e, _| e.mem.u32(0x7300_0000).into_ret());
        e.register(SLOT_200, |_, _| Ret::default());
        e.register(RESET_DEVICE, |_, _| true.into_ret());
        e.register(REPORT, |_, _| Ret::default());
        // A callback answering its data word.
        e.register(CALLBACK, |_, a| (a[0] != 0).into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x7000_0000);
        let this = e.mem.alloc(0xb10);
        e.mem.set_u32(this + 0x288, device);
        (e, this)
    }

    #[test]
    fn device_states() {
        let (mut e, this) = setup(0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74c00, &args![this]).bool());
        assert!(calls(&e, SLOT_200).is_empty());
        e.mem.set_u8(DEVICE_SLOT_200_FLAG, 1);
        e.mem.set_u32(0x7300_0000, DEVICE_NOT_RESET);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74c00, &args![this]).bool());
        assert_eq!(calls(&e, SLOT_200).len(), 1);
        assert_eq!(calls(&e, RESET_DEVICE), vec![vec![this]]);
    }

    #[test]
    fn lost_device_runs_the_callbacks_once() {
        let (mut e, this) = setup(DEVICE_LOST);
        let callbacks = e.mem.alloc(12);
        let data = e.mem.alloc(12);
        e.mem.set_u32(callbacks, CALLBACK);
        e.mem.set_u32(callbacks + 8, CALLBACK);
        e.mem.set_u32(data, 1);
        e.mem.set_u32(data + 8, 1);
        e.mem.set_u32(this + 0xaf8, callbacks);
        e.mem.set_u32(this + 0xb08, data);
        e.mem.set_u16(this + 0xafe, 3);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e74c00, &args![this]).bool());
        assert_eq!(calls(&e, CALLBACK).len(), 2);
        assert!(calls(&e, REPORT).is_empty());
        assert_eq!(e.mem.u8(this + 0x6fc), 1);
        // Already lost: nothing runs.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e74c00, &args![this]).bool());
        assert!(calls(&e, CALLBACK).is_empty());
        // A failing callback reports and stops.
        e.mem.set_u8(this + 0x6fc, 0);
        e.mem.set_u32(data, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e74c00, &args![this]).bool());
        assert_eq!(calls(&e, CALLBACK).len(), 1);
        assert_eq!(calls(&e, REPORT), vec![vec![CALLBACK_FAILED_MESSAGE]]);
    }
}
