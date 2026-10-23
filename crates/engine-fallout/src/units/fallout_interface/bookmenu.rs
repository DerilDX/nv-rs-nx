//! `fallout/interface/menus/bookmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0073ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01070ecc` (RTTI `.?AVBookMenu@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00a1c520`. One word, plain `RET`, returns nothing.
pub fn fn_0073ad30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_0ecc);
    e.call(0x00a1_c520, &args![this]);
}

/// The interface height (`00715da0`, no arguments; the value in `ST0`).
const INTERFACE_HEIGHT: u32 = 0x0071_5da0;
/// The `double` `01011590` (2.0 in the exe) the second height is divided by.
const DIVISOR_01011590: u32 = 0x0101_1590;
/// `_ftol2_sse` (`00ec62c0`): the value in `ST0` (a leading `f64`) to an integer in
/// `EDX:EAX`.
const FTOL: u32 = 0x00ec_62c0;

// Translated from 0073b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bookmenu.cpp`; `thiscall`, no stack words): the
/// interface height minus (the interface height / 2.0 + the `float` at +0x34 of `this`),
/// truncated to an integer (`_ftol2_sse`), whose `EDX:EAX` it leaves. The x87 steps run in the
/// exe's order (`FDIV`, `FADD` of the float, `FSUBR` from the first height kept as a
/// `double`).
pub fn fn_0073b020(e: &mut Engine, this: Ptr) -> u64 {
    let first = e.call(INTERFACE_HEIGHT, &args![]).f64();
    let second = e.call(INTERFACE_HEIGHT, &args![]).f64();
    let divisor: f64 = e.global(DIVISOR_01011590);
    let half = x87_div(second, divisor);
    let offset = x87_m32(e.mem.u32(this.addr() + 0x34));
    let sum = x87_add(half, offset);
    let value = x87_sub(first, sum);
    e.call(FTOL, &args![value]).u64()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0073_ad30, fn_0073ad30(Ptr)),
        entry!(0x0073_b020, fn_0073b020(Ptr) -> u64),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0073ad30_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00a1_c520, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_ad30, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00a1_c520)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_0ecc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_0ecc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod height_tests {
    use super::*;

    #[test]
    fn fn_0073b020_takes_half_the_height_and_the_offset_from_the_height() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.set_global(DIVISOR_01011590, 2.0f64);
        e.register(INTERFACE_HEIGHT, |_, _| 960.0f32.into_ret());
        // The truncation double sees the value and answers it plus a marker in EDX.
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                eax: value as i32 as u32,
                edx: 0x77,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x34, 100.5);
        e.call_log = Some(vec![]);
        let result = e.call(0x0073_b020, &args![this]).u64();
        assert_eq!(result, (0x77 << 32) | 379);
        let heights = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == INTERFACE_HEIGHT)
            .count();
        assert_eq!(heights, 2);
    }
}
