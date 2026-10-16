//! `fallout shared/distant terrain system/bgsdistanttreeblock.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006f99e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map-item allocator: allocates twelve bytes without initializing them.
pub fn allocate_item(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(0x00401000, &args![12u32]).ptr()
}

/// The function `fn_006f9080` calls on the member (`0045cec0`).
const CALLEE_0045CEC0: u32 = 0x0045_cec0;

// Translated from 006f9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0045cec0` with `ECX` = `this + 0x1c` and passes on nothing (the callee returns no value).
pub fn fn_006f9080(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0045CEC0, &args![this.byte_add(0x1c)]);
}

// Translated from 006f94a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106ded0` (RTTI `.?AVBGSDistantTreeBlockLoadTask@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3cea0`. One word, plain `RET`, returns nothing.
pub fn fn_006f94a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_ded0);
    e.call(0x00c3_cea0, &args![this]);
}

/// The constructor body `006fa180` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_006FA180: u32 = 0x006f_a180;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<BGSDistantTreeBlock::TreeType,256>`).
const VTABLE_006FA100: u32 = 0x0106_dfe0;

// Translated from 006fa100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bgsdistanttreeblock.cpp`): stores the vtable `0106dfe0`
/// (RTTI class `BSSimpleArray<BGSDistantTreeBlock::TreeType,...>`), then calls
/// `006fa180(this, 0, 0)`. Returns `this`.
pub fn fn_006fa100(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_006FA100);
    e.call(CONSTRUCTOR_BODY_006FA180, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_94a0, fn_006f94a0(Ptr)),
        entry!(0x006f99e0, allocate_item(Ptr) -> Ptr),
        entry!(0x006f_9080, fn_006f9080(Ptr)),
        entry!(0x006f_a100, fn_006fa100(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006f9080_passes_this_plus_1c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0045CEC0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_9080, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0045CEC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x1c]);
    }

    #[test]
    fn fn_006f94a0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_cea0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_94a0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_cea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_ded0);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_ded0,
            "the vtable was stored before the call"
        );
    }
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
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_006FA180, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_a100, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_006FA180),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_006FA100);
    }
}
