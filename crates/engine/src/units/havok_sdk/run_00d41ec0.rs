//! `Havok SDK/run_00d41ec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`); the allocator is the word at router + 0x10, and its
/// virtual function at offset 4 allocates a block of the given size.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpGskfAgent::hkpGskfAgent` (Xbox PDB, `00dacbd0`, `this` and three words).
const GSKF_AGENT_CONSTRUCT: u32 = 0x00da_cbd0;
/// `hkpGskBaseAgent::hkpGskBaseAgent` (Xbox PDB, `00dabf10`, `this` and three words).
const GSK_BASE_AGENT_CONSTRUCT: u32 = 0x00da_bf10;
/// The vtable the factory stores into the larger agent after construction (`010ceb94`;
/// unconfirmed class).
const LARGE_AGENT_VTABLE: u32 = 0x010c_eb94;

// Translated from 00d41f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent factory (cdecl, four words `a, b, _, flag`; the third word is not read). The class is
/// unconfirmed. With a non-zero `flag` it allocates 0x80 bytes, stores the size at +4, runs
/// the `hkpGskfAgent` constructor (`00dacbd0`) with `(a, b, flag)` and then stores vtable
/// `010ceb94` over the one the constructor wrote. With a zero `flag` it allocates 0x30
/// bytes, stores the size at +4 and runs the `hkpGskBaseAgent` constructor (`00dabf10`) with
/// `(a, b, 0)`. Returns the new object (the constructor's return in the second case).
pub fn fn_00d41f50(e: &mut Engine, a: u32, b: u32, _unused_2: u32, flag: u32) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if flag != 0 {
        let block = e.vcall(allocator, 4, &args![0x80u32]).u32();
        e.mem.set_u16(block + 4, 0x80);
        e.call(GSKF_AGENT_CONSTRUCT, &args![block, a, b, flag]);
        e.mem.set_u32(block, LARGE_AGENT_VTABLE);
        return Ptr::new(block);
    }
    let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(block + 4, 0x30);
    e.call(GSK_BASE_AGENT_CONSTRUCT, &args![block, a, b, 0u32])
        .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_1f50, fn_00d41f50(u32, u32, u32, u32) -> Ptr)]
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

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOC_SLOT_DOUBLE: u32 = 0x0900_0004;

    fn rig(e: &mut Engine, block: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOC_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            ALLOC_SLOT_DOUBLE,
            GSKF_AGENT_CONSTRUCT,
            GSK_BASE_AGENT_CONSTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: block,
                    ..Ret::default()
                }
            });
        }
        log
    }

    #[test]
    fn a_non_zero_flag_builds_the_large_agent() {
        let mut e = test_engine();
        let block = e.mem.alloc(0x100);
        let log = rig(&mut e, block);
        let got = e.call(0x00d4_1f50, &args![5u32, 6u32, 7u32, 9u32]);
        assert_eq!(got.u32(), block);
        assert_eq!(e.mem.u16(block + 4), 0x80);
        assert_eq!(e.mem.u32(block), LARGE_AGENT_VTABLE);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1], 0x80);
        assert_eq!(calls[1], (GSKF_AGENT_CONSTRUCT, vec![block, 5, 6, 9]));
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn a_zero_flag_builds_the_base_agent() {
        let mut e = test_engine();
        let block = e.mem.alloc(0x100);
        let log = rig(&mut e, block);
        let got = e.call(0x00d4_1f50, &args![5u32, 6u32, 7u32, 0u32]);
        assert_eq!(got.u32(), block);
        assert_eq!(e.mem.u16(block + 4), 0x30);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1], 0x30);
        assert_eq!(calls[1], (GSK_BASE_AGENT_CONSTRUCT, vec![block, 5, 6, 0]));
    }
}
