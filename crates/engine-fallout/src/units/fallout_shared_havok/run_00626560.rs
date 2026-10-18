//! `fallout shared/havok/run_00626560` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__thiscall` initializer the copy constructor runs first (`00626630`, no arguments).
const FUNCTION_INIT: u32 = 0x0062_6630;
/// `__thiscall` copy from the source (`00626600`, one argument).
const FUNCTION_COPY_FROM: u32 = 0x0062_6600;
/// `__thiscall` (`004a4840`, one argument) called on a one-byte local.
const CHECK_HOLDER: u32 = 0x004a_4840;
/// `__thiscall` (`00cb1870`, `hkmap.cpp`, three arguments).
const MAP_STORE: u32 = 0x00cb_1870;

// Translated from 00626560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00626560 - the compiler's exception frame (FS:[0] handler 00efc1f8, state -1 then 0 around the copy) is not translated.
/// A `std::function<...>` copy constructor (the library's multiple-match
/// name): initializes `this` (`00626630`), then copies `source` into it
/// (`00626600`); returns `this`.
pub fn function_copy_constructor(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.call(FUNCTION_INIT, &args![this]);
    e.call(FUNCTION_COPY_FROM, &args![this, source]);
    this
}

// Translated from 006265c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall`: runs `004a4840` on a zeroed one-byte local with
/// `this`, then `00cb1870` on `this` with that result and the two arguments.
pub fn fn_006265c0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let checked = e.with_stack(1, |e, local| {
        e.call(CHECK_HOLDER, &args![local, this]).u32()
    });
    e.call(MAP_STORE, &args![this, checked, first, second]);
}

// Translated from 00626660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall`: the address of element `index` of the 8-byte
/// elements at the pointer stored in `this`.
pub fn fn_00626660(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(this.addr()).wrapping_add(index.wrapping_mul(8))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00626560, function_copy_constructor(Ptr, u32) -> Ptr),
        entry!(0x006265c0, fn_006265c0(Ptr, u32, u32)),
        entry!(0x00626660, fn_00626660(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_constructor_initializes_then_copies_and_returns_this() {
        let mut e = Engine::new();
        e.register(FUNCTION_INIT, |_, _| Ret::default());
        e.register(FUNCTION_COPY_FROM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_6560, &args![Ptr::<()>::new(0x4000), 0x5000u32]);
        assert_eq!(result.u32(), 0x4000);
        let log = e.call_log.clone().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 != 0x0062_6560).collect();
        assert_eq!(calls[0], &(FUNCTION_INIT, vec![0x4000]));
        assert_eq!(calls[1], &(FUNCTION_COPY_FROM, vec![0x4000, 0x5000]));
    }

    #[test]
    fn fn_006265c0_passes_the_checked_result_and_arguments_on() {
        let mut e = Engine::new();
        e.register(CHECK_HOLDER, |e, a| {
            // The local byte is zeroed memory.
            assert_eq!(e.mem.u8(a[0]), 0);
            Ret {
                eax: a[1] + 1,
                ..Ret::default()
            }
        });
        e.register(MAP_STORE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_65c0, &args![Ptr::<()>::new(0x4000), 7u32, 9u32]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(MAP_STORE, vec![0x4000, 0x4001, 7, 9])));
    }

    #[test]
    fn fn_00626660_indexes_eight_byte_elements() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1000);
        assert_eq!(
            e.call(0x0062_6660, &args![Ptr::<()>::new(this), 3u32])
                .u32(),
            0x1018
        );
    }
}
