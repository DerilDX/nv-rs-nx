//! `fallout shared/bgsdefaultobjectmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058d820` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058d7f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_d820, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `BGSDefaultObjectManager::Instance` (Xbox PDB, `0058d680`): the singleton, built on first use.
const DEFAULT_OBJECT_MANAGER_INSTANCE: u32 = 0x0058_d680;
/// `004013e0` (unnamed in the engine map): `thiscall`; whether bit 3 (mask 8) of the dword at
/// `this + 8` is set, in `AL`.
const MANAGER_IS_LOADED: u32 = 0x0040_13e0;

// Translated from 0058db10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDefaultObjectManager::GetDefaultObject` (Xbox PDB): `cdecl`, one stack word (the index).
/// Zero when the flag test (`004013e0`) on the singleton is false; otherwise the dword at
/// `instance + 0x18 + index * 4` (the singleton is fetched a second time for it, as in the exe).
pub fn bgs_default_object_manager_get_default_object(e: &mut Engine, index: u32) -> u32 {
    let manager = e.call(DEFAULT_OBJECT_MANAGER_INSTANCE, &args![]).u32();
    if !e.call(MANAGER_IS_LOADED, &args![manager]).bool() {
        return 0;
    }
    let manager = e.call(DEFAULT_OBJECT_MANAGER_INSTANCE, &args![]).u32();
    e.mem.u32(
        manager
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(0x18),
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_db10, bgs_default_object_manager_get_default_object(u32) -> u32),
        entry!(0x0058_d7f0, fn_0058d7f0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0058d7f0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_d820, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_d7f0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_d7f0)
                .collect();
            let mut expected = vec![(0x0058_d820, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0058db10_answers_the_indexed_default_object_when_the_manager_is_ready() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 0x18, 0x1000);
        e.mem.set_u32(manager + 0x1c, 0x2000);
        e.mem.set_u32(manager + 0x18 + 4 * 5, 0x6000);
        e.register_double(DEFAULT_OBJECT_MANAGER_INSTANCE, move |_, _| {
            manager.into_ret()
        });
        e.register(MANAGER_IS_LOADED, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0058_db10, &args![0u32]).u32(), 0x1000);
        assert_eq!(e.call(0x0058_db10, &args![1u32]).u32(), 0x2000);
        assert_eq!(e.call(0x0058_db10, &args![5u32]).u32(), 0x6000);
        let log = e.call_log.take().unwrap();
        // The singleton is fetched twice per lookup, the flag test gets it as `this`.
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == DEFAULT_OBJECT_MANAGER_INSTANCE)
                .count(),
            6
        );
        assert!(log
            .iter()
            .filter(|c| c.0 == MANAGER_IS_LOADED)
            .all(|c| c.1 == vec![manager]));
    }

    #[test]
    fn at_0058db10_answers_zero_when_the_manager_is_not_ready() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 0x18, 0x1000);
        e.register_double(DEFAULT_OBJECT_MANAGER_INSTANCE, move |_, _| {
            manager.into_ret()
        });
        e.register(MANAGER_IS_LOADED, |_, _| Ret::default());
        assert_eq!(e.call(0x0058_db10, &args![0u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == DEFAULT_OBJECT_MANAGER_INSTANCE)
                .count(),
            1
        );
    }
}
