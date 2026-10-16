//! `Havok SDK/hkmemoryrouter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread-local-storage index (a word in `.data`).
const TLS_INDEX_01268108: u32 = 0x0126_8108;
/// The import slot of `TlsGetValue` (`kernel32`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;

// Translated from 00c85750 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `hkmemoryrouter.cpp`): `TlsGetValue`
/// of the index stored at `01268108`; the other Havok code uses the result
/// as the calling thread's memory router. Returns that value.
pub fn fn_00c85750(e: &mut Engine) -> u32 {
    let index = e.global::<u32>(TLS_INDEX_01268108);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

// Translated from 00c85770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::hkMemoryRouter` (Xbox PDB), `thiscall`, no stack arguments:
/// clears the first eight words (0x20 bytes) of the router. Returns `this`.
pub fn hk_memory_router_hk_memory_router(e: &mut Engine, this: Ptr) -> Ptr {
    for word in 0..8 {
        e.mem.set_u32(this.addr() + word * 4, 0);
    }
    this
}

// Translated from 00c856c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::easyFree` (Xbox PDB): does nothing for a null `block`.
/// Otherwise the allocation starts 0x10 bytes before `block`: zeroes its
/// first word and frees it through slot +8 of `router`'s vtable, with the
/// start and the size (the word at `block - 0xC`, plus 0x10).
pub fn hk_memory_router_easy_free(e: &mut Engine, router: u32, block: u32) {
    if block == 0 {
        return;
    }
    let start = block.wrapping_sub(0x10);
    let size = e.mem.u32(start + 4).wrapping_add(0x10);
    e.mem.set_u32(start, 0);
    e.vcall(router, 8, &args![start, size]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c85770, hk_memory_router_hk_memory_router(Ptr) -> Ptr),
        entry!(0x00c856c0, hk_memory_router_easy_free(u32, u32)),
        entry!(0x00c85750, fn_00c85750() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c85750_returns_the_tls_value_of_the_stored_index() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(TLS_INDEX_01268108, 7u32);
        e.register(TLS_GET_VALUE, |_, a| Ret {
            eax: 0x5000 + a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x00c8_5750, &args![]).u32(), 0x5007);
    }

    #[test]
    fn easy_free_returns_a_block_through_the_routers_slot() {
        let mut e = Engine::new();
        let router = e.mem.alloc(4);
        let block = e.mem.alloc(0x30);
        e.put_vtable(0x4400_0000, &[0, 0, 0x4500_0000]);
        e.mem.set_u32(router, 0x4400_0000);
        e.mem.set_u32(block, 0x1234);
        e.mem.set_u32(block + 4, 0x20);
        e.register(0x4500_0000, |e, a| {
            // The first word of the allocation is already cleared.
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_56c0, &args![router, block + 0x10]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x4500_0000, vec![router, block, 0x30])));
    }

    #[test]
    fn easy_free_ignores_a_null_block() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00c8_56c0, &args![0x1000u32, 0u32]);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
    }

    #[test]
    fn the_constructor_clears_eight_words_and_nothing_more() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x30);
        e.mem.write(router, &[0xcd; 0x30]);
        assert_eq!(e.call(0x00c8_5770, &args![router]).u32(), router);
        assert_eq!(e.mem.bytes(router, 0x20), vec![0; 0x20]);
        assert_eq!(e.mem.u32(router + 0x20), 0xcdcd_cdcd);
    }
}
