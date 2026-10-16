//! `Havok SDK/run_00dfd9b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfd9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x80-byte frame stores the vtable word 0x010e_33dc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00dfd9b0(e: &mut Engine) -> u32 {
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010e_33dc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        e.mem.u32(base)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dfd9b0, fn_00dfd9b0() -> u32)]
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
    fn fn_00dfd9b0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_d9b0, &args![]).u32();
        assert_eq!(result, 0x010e_33dc);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_33dc);
        }
    }
}
