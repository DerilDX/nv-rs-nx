//! `Havok SDK/hkmonitorstream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The calling thread's memory router (`00c85750`).
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The thread-local-storage index of the monitor stream (a word in `.data`).
const MONITOR_STREAM_TLS_INDEX: u32 = 0x0126_835c;
/// The import slot of `TlsSetValue` (`kernel32`).
const TLS_SET_VALUE: u32 = 0x00fd_f04c;

// Translated from 00cb0db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStream::init` (Xbox PDB): takes the calling thread's router
/// (`00c85750`), allocates 0x14 bytes through virtual slot +4 of the object
/// at router `+0x14`, stores the block as the thread's value for the TLS
/// index at `0126835c` (`TlsSetValue`), and clears its words at +0, +4, +8,
/// +0xC and its byte at +0x10.
pub fn hk_monitor_stream_init(e: &mut Engine) {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    // hkMemoryRouter +0x14: the object whose slot +4 allocates.
    let allocator = e.mem.u32(router + 0x14);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    let index = e.global::<u32>(MONITOR_STREAM_TLS_INDEX);
    e.call(TLS_SET_VALUE, &args![index, block]);
    e.mem.set_u8(block + 0x10, 0);
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 0xc, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cb0db0, hk_monitor_stream_init())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_allocates_clears_and_stores_the_block_for_the_thread() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(MONITOR_STREAM_TLS_INDEX, 9u32);
        let router = e.mem.alloc(0x24);
        let allocator = e.mem.alloc(8);
        let block = e.mem.alloc(0x14);
        for offset in 0..5 {
            e.mem.set_u32(block + 4 * offset, 0xdead_beef);
        }
        e.put_vtable(0x4400_0000, &[0, 0x4500_0000]);
        e.mem.set_u32(allocator, 0x4400_0000);
        e.mem.set_u32(router + 0x14, allocator);
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x4500_0000, move |_, a| {
            assert_eq!(a, [allocator, 0x14]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(TLS_SET_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cb_0db0, &args![]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(TLS_SET_VALUE, vec![9, block])));
        for offset in [0, 4, 8, 0xc] {
            assert_eq!(e.mem.u32(block + offset), 0);
        }
        assert_eq!(e.mem.u8(block + 0x10), 0);
        // Only the byte at +0x10 is cleared, not the rest of that word.
        assert_eq!(e.mem.u8(block + 0x11), 0xbe);
    }
}
