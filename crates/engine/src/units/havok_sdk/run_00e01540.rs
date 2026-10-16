//! `Havok SDK/run_00e01540` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e01540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9300.
pub fn return_constant_00e01540(_e: &mut Engine) -> u32 {
    0x010e_9300
}

// Translated from 00e01580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9358.
pub fn return_constant_00e01580(_e: &mut Engine) -> u32 {
    0x010e_9358
}

// Translated from 00e015c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_93ac.
pub fn return_constant_00e015c0(_e: &mut Engine) -> u32 {
    0x010e_93ac
}

// Translated from 00e01670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_943c.
pub fn return_constant_00e01670(_e: &mut Engine) -> u32 {
    0x010e_943c
}

// Translated from 00e016b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9494.
pub fn return_constant_00e016b0(_e: &mut Engine) -> u32 {
    0x010e_9494
}

// Translated from 00e01710 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x2c-byte frame stores the vtable word 0x010e_9544 at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x24, 0x28 in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00e01710(e: &mut Engine) -> u32 {
    e.with_stack(0x2c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_9544);
        e.call(0x00cb_0a70, &args![base + 0x24, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x28, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e01540, return_constant_00e01540() -> u32),
        entry!(0x00e01580, return_constant_00e01580() -> u32),
        entry!(0x00e015c0, return_constant_00e015c0() -> u32),
        entry!(0x00e01670, return_constant_00e01670() -> u32),
        entry!(0x00e016b0, return_constant_00e016b0() -> u32),
        entry!(0x00e01710, fn_00e01710() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn return_constant_00e01540_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1540, &args![]).u32(), 0x010e_9300);
    }

    #[test]
    fn return_constant_00e01580_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1580, &args![]).u32(), 0x010e_9358);
    }

    #[test]
    fn return_constant_00e015c0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_15c0, &args![]).u32(), 0x010e_93ac);
    }

    #[test]
    fn return_constant_00e01670_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1670, &args![]).u32(), 0x010e_943c);
    }

    #[test]
    fn return_constant_00e016b0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_16b0, &args![]).u32(), 0x010e_9494);
    }

    #[test]
    fn fn_00e01710_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x18, 0x1c], false);
        let result = e.call(0x00e0_1710, &args![]).u32();
        assert_eq!(result, 0x010e_9544);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_9544);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x4);
    }
}
