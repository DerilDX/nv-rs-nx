//! `(unplaced)/run_00d2c7e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constructor `00d2c7e0` (`this` plus one stack word, `RET 4`).
const CONSTRUCTOR_00D2C7E0: u32 = 0x00d2_c7e0;
/// `__security_check_cookie` (`00ec408c`, fastcall: the cookie in `ECX`).
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;
/// The security cookie (`.data`).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;
/// The constant the function returns (an address in `.data`; what it points at is unconfirmed).
const RETURNED_ADDRESS_010CE740: u32 = 0x010c_e740;

// Translated from 00d2c8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2c8a0 - the compiler's exception frame (FS:[0] chain) is not
// translated; the stack cookie check is (through `__security_check_cookie`).
/// Unnamed in the engine map: builds a 0x38-byte object in a local, on the stack (preceded
/// by a pointer to it), with `00d2c7e0(object, 0)`, checks the stack cookie and returns the
/// constant `010ce740`. The object is not destroyed here. The stack-aligned locals are the
/// engine's `with_stack` block.
pub fn fn_00d2c8a0(e: &mut Engine) -> u32 {
    e.with_stack(0x3c, |e, frame| {
        let object = frame.addr() + 4;
        e.mem.set_u32(frame.addr(), object);
        e.call(CONSTRUCTOR_00D2C7E0, &args![object, 0u32]);
    });
    // The cookie stored in the frame is the global XOR the frame address, and the check
    // XORs the frame address back: the value checked is the global itself.
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
    RETURNED_ADDRESS_010CE740
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_c8a0, fn_00d2c8a0() -> u32)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn constructs_the_object_checks_the_cookie_and_returns_the_address() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.set_global(SECURITY_COOKIE_011C16BC, 0xdead_beefu32);
        e.register(CONSTRUCTOR_00D2C7E0, |e, a| {
            e.mem.set_u32(a[0], 0x42);
            Ret::default()
        });
        e.register(SECURITY_CHECK_COOKIE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d2_c8a0, &args![]).u32(), 0x010c_e740);
        let built = calls_to(&mut e, CONSTRUCTOR_00D2C7E0);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], 0);
        assert_ne!(built[0][0], 0);
        assert_eq!(
            calls_to(&mut e, SECURITY_CHECK_COOKIE),
            vec![vec![0xdead_beef]]
        );
    }
}
