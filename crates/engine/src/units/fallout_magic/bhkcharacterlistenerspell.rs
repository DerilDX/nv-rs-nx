//! `fallout/magic/bhkcharacterlistenerspell.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00620b80` (`this`): the character proxy of a reference (a wrapper over `0044ddc0`); the
/// map files it under `cameracaster.cpp`.
const GET_HAVOK_PROXY: u32 = 0x0062_0b80;
/// `00458b20` (no arguments): answers the address `01267e30` (a static zero vector).
const ZERO_VECTOR: u32 = 0x0045_8b20;
/// A virtual thunk (`00c89720`, `this`): jumps through vtable slot 0x94; the engine map names
/// it `bhkCharacterProxy::UnMarkRead`.
const THUNK_SLOT_94: u32 = 0x00c8_9720;
/// `hkpCharacterProxy::getLinearVelocity` (Xbox PDB, `00cac950`, `this` = the Havok proxy).
const HKP_GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;

// Translated from 00807080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::GethkLinearVelocity` (Xbox PDB): the Havok character proxy of `this`
/// (`00620b80`); with none, the static zero vector (`00458b20`). With one, the call through
/// the thunk at `00c89720` (slot 0x94 of `this`) brackets the read of the proxy's linear
/// velocity (`hkpCharacterProxy::getLinearVelocity`), whose result is returned.
pub fn bhk_character_proxy_get_hk_linear_velocity(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.call(GET_HAVOK_PROXY, &args![this]).u32();
    let mut velocity = e.call(ZERO_VECTOR, &args![]).u32();
    if proxy != 0 {
        e.call(THUNK_SLOT_94, &args![this]);
        velocity = e.call(HKP_GET_LINEAR_VELOCITY, &args![proxy]).u32();
        e.call(THUNK_SLOT_94, &args![this]);
    }
    velocity
}

// Translated from 00806f00 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01268344 without reading the receiver. The map names this body `bhkCharacterProxy::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00806f00(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_8344
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_6f00, fn_00806f00(Ptr) -> u32),
        entry!(0x0080_7080, bhk_character_proxy_get_hk_linear_velocity(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn rig(proxy: u32) -> (Engine, Rc<RefCell<Vec<(u32, u32)>>>) {
        let mut e = test_engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        e.register_double(GET_HAVOK_PROXY, move |_, _| Ret {
            eax: proxy,
            ..Ret::default()
        });
        e.register_double(ZERO_VECTOR, |_, _| Ret {
            eax: 0x0126_7e30,
            ..Ret::default()
        });
        let thunk = Rc::clone(&log);
        e.register_double(THUNK_SLOT_94, move |_, a| {
            thunk.borrow_mut().push((THUNK_SLOT_94, a[0]));
            Ret::default()
        });
        let velocity = Rc::clone(&log);
        e.register_double(HKP_GET_LINEAR_VELOCITY, move |_, a| {
            velocity.borrow_mut().push((HKP_GET_LINEAR_VELOCITY, a[0]));
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        (e, log)
    }

    #[test]
    fn with_a_proxy_the_velocity_is_read_between_two_thunk_calls() {
        let (mut e, log) = rig(0x4444);
        assert_eq!(e.call(0x0080_7080, &args![0x1000u32]).u32(), 0x7777);
        assert_eq!(
            *log.borrow(),
            vec![
                (THUNK_SLOT_94, 0x1000),
                (HKP_GET_LINEAR_VELOCITY, 0x4444),
                (THUNK_SLOT_94, 0x1000)
            ]
        );
    }

    #[test]
    fn without_a_proxy_the_zero_vector_is_returned() {
        let (mut e, log) = rig(0);
        assert_eq!(e.call(0x0080_7080, &args![0x1000u32]).u32(), 0x0126_7e30);
        assert!(log.borrow().is_empty());
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00806f00_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0080_6f00, &args![0x1000u32]).u32(), 0x0126_8344);
        assert_eq!(e.call(0x0080_6f00, &args![0u32]).u32(), 0x0126_8344);
    }
}
