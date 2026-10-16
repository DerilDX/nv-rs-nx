//! `Havok SDK/run_00dfc400` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfc480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_04ac.
pub fn return_constant_00dfc480(_e: &mut Engine) -> u32 {
    0x010e_04ac
}

// Translated from 00dfc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x140-byte frame stores the vtable word 0x010d_77d8 at
/// its start, runs the constructor at `00d21de0` on the member at frame+0x20 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00dfc400(e: &mut Engine) -> u32 {
    e.with_stack(0x140, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010d_77d8);
        e.call(0x00d2_1de0, &args![base + 0x20, 0u32]);
        e.mem.u32(base)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfc480, return_constant_00dfc480() -> u32),
        entry!(0x00dfc400, fn_00dfc400() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfc480_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c480, &args![]).u32(), 0x010e_04ac);
    }
}

#[cfg(test)]
mod record_tests {
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
    fn fn_00dfc400_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00d2_1de0, &[0x20], false);
        let result = e.call(0x00df_c400, &args![]).u32();
        assert_eq!(result, 0x010d_77d8);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010d_77d8);
        }
    }
}
