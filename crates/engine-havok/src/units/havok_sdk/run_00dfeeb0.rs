//! `Havok SDK/run_00dfeeb0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfeeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_62a4.
pub fn return_constant_00dfeeb0(_e: &mut Engine) -> u32 {
    0x010e_62a4
}

// Translated from 00dfeef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_6394.
pub fn return_constant_00dfeef0(_e: &mut Engine) -> u32 {
    0x010e_6394
}

// Translated from 00dfef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_ebf4.
pub fn return_constant_00dfef40(_e: &mut Engine) -> u32 {
    0x010d_ebf4
}

// Translated from 00dfef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x60-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_08f4 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dfef90(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_08f4
    })
}

// Translated from 00dff000 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_66fc.
pub fn return_constant_00dff000(_e: &mut Engine) -> u32 {
    0x010e_66fc
}

// Translated from 00dfef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010d08f4`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dfef50(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_08f4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfeeb0, return_constant_00dfeeb0() -> u32),
        entry!(0x00dfeef0, return_constant_00dfeef0() -> u32),
        entry!(0x00dfef40, return_constant_00dfef40() -> u32),
        entry!(0x00dfef90, fn_00dfef90() -> u32),
        entry!(0x00dff000, return_constant_00dff000() -> u32),
        entry!(0x00df_ef50, fn_00dfef50(Ptr, u32)),
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
    fn return_constant_00dfeeb0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eeb0, &args![]).u32(), 0x010e_62a4);
    }

    #[test]
    fn return_constant_00dfeef0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eef0, &args![]).u32(), 0x010e_6394);
    }

    #[test]
    fn return_constant_00dfef40_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ef40, &args![]).u32(), 0x010d_ebf4);
    }

    #[test]
    fn fn_00dfef90_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_ef90, &args![]).u32();
        assert_eq!(result, 0x010d_08f4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn return_constant_00dff000_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f000, &args![]).u32(), 0x010e_66fc);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A null object does nothing; otherwise the vtable word ends at the second value and the
    /// string function runs in between on `object + 0x14`, seeing the first value.
    #[test]
    fn fn_00dfef50_runs_the_string_function_between_the_vtable_writes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x14, 7]);
            assert_eq!(e.mem.u32(object), 0x010c_c2fc);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00df_ef50, &args![0u32, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00df_ef50, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_08f4);
    }
}
