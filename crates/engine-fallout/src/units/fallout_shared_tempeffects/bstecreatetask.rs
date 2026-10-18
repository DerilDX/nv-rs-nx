//! `fallout shared/tempeffects/bstecreatetask.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00682490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTask` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006824c0` and frees 0xc bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn ni_task_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0068_24c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00682480 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011f5afc without reading the receiver. The map names this body `NiTask::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00682480(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_5afc
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_2480, fn_00682480(Ptr) -> u32),
        entry!(0x0068_2490, ni_task_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn ni_task_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_2490, 0x0068_24c0, 0xc);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00682480_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_2480, &args![0x1000u32]).u32(), 0x011f_5afc);
        assert_eq!(e.call(0x0068_2480, &args![0u32]).u32(), 0x011f_5afc);
    }
}
