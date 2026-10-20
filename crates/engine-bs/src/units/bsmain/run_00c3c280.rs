//! `BSMain/run_00c3c280` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `MessageHandler` constructor (`00c3bcf0`, thiscall, `RET 4`: the flag byte).
const MESSAGE_HANDLER_CONSTRUCTOR: u32 = 0x00c3_bcf0;
/// The `MessageHandler` destructor (`00c3bda0`, thiscall).
const MESSAGE_HANDLER_DESTRUCTOR: u32 = 0x00c3_bda0;
/// The global holding the message handler.
const MESSAGE_HANDLER: u32 = 0x0120_2d74;

// Translated from 00c3c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MessageHandler`'s scalar deleting destructor (its vtable's class by RTTI): runs the
/// destructor (`00c3bda0`) and frees `this` through the memory manager when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00c3c430(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MESSAGE_HANDLER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c3c460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3c460 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f109cb, state variable) is not translated
/// Makes the message handler: allocates 8 bytes from the memory manager, constructs a
/// `MessageHandler` in them (`00c3bcf0` with 0) unless the allocation failed, and stores the
/// result (or null) in `01202d74`.
pub fn fn_00c3c460(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    let handler = if block == 0 {
        0
    } else {
        e.call(MESSAGE_HANDLER_CONSTRUCTOR, &args![block, 0u32])
            .u32()
    };
    e.set_global::<u32>(MESSAGE_HANDLER, handler);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_c430, fn_00c3c430(Ptr, u32) -> Ptr),
        entry!(0x00c3_c460, fn_00c3c460()),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn deleting_destructor_frees_on_bit_0() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            let mut e = Engine::new();
            e.register(MESSAGE_HANDLER_DESTRUCTOR, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c3_c430, &args![0x4000u32, flags]).u32(), 0x4000);
            let mut want = vec![(MESSAGE_HANDLER_DESTRUCTOR, vec![0x4000])];
            if frees {
                want.push((DEALLOCATE, vec![MEMORY_MANAGER, 0x4000]));
            }
            assert_eq!(log_of(&mut e, 0x00c3_c430), want);
        }
    }

    #[test]
    fn makes_the_handler_or_stores_null() {
        for block in [0u32, 0x5000] {
            let mut e = Engine::new();
            e.map(MESSAGE_HANDLER, 4);
            e.mem.set_u32(MESSAGE_HANDLER, 0xffff);
            e.register_double(ALLOCATE, move |_, _| block.into_ret());
            e.register(MESSAGE_HANDLER_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x00c3_c460, &args![]);
            let mut want = vec![(ALLOCATE, vec![MEMORY_MANAGER, 8])];
            if block != 0 {
                want.push((MESSAGE_HANDLER_CONSTRUCTOR, vec![block, 0]));
            }
            assert_eq!(log_of(&mut e, 0x00c3_c460), want);
            let stored = if block == 0 { 0 } else { block + 1 };
            assert_eq!(e.mem.u32(MESSAGE_HANDLER), stored);
        }
    }
}
