//! `NiXenonRenderer/nid3dshaderdeclaration.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni array allocation `00aa1420` (no unit in the engine map; `cdecl`, the byte count),
/// the compiler's `vector constructor iterator` (`00401050`, `stdcall`: array, element size,
/// count, constructor) with the stream constructor at `00e78bd0` (code Ghidra made no function
/// of), and the entry constructor `00e76320` (unit `nid3dshaderdeclaration.cpp`, unnamed;
/// `thiscall`, no stack words).
const NI_ARRAY_ALLOCATE_00AA1420: u32 = 0x00aa_1420;
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const STREAM_CONSTRUCT_00E78BD0: u32 = 0x00e7_8bd0;
const ENTRY_CONSTRUCT_00E76320: u32 = 0x00e7_6320;

/// The byte count of `count` elements of `size` bytes as the code computes it (`MUL`, `SETO`,
/// `NEG`, `OR`): all ones when the product overflows.
fn array_bytes(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    }
}

// Translated from 00e78c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the declaration's
/// initializer that `NiXenonShaderDeclaration::Create` (`00e76700`) runs, `thiscall` with
/// three stack words: the renderer, the entry count and the stream count. Keeps the renderer
/// (`m_pkRenderer`, +8), the renderer's word at +0x8bc (`m_pkVBManager`, +0xc) and its device
/// at +0x288 (`m_pkD3DDevice`, +0x10; the device's vtable slot +4 is called on it, `stdcall`),
/// the entry count (`m_uiMaxStreamEntryCount`, +0x1c) and the stream count
/// (`m_uiStreamCount`, +0x20) (Xbox PDB names). Allocates the streams (16 bytes each,
/// `00aa1420`; built by the `vector constructor iterator` with `00e78bd0` when the allocation
/// succeeds) into `m_pkStreamEntries` (+0x24); then for each stream (the count read again from
/// +0x20 each time) allocates the entries (0x1c bytes each, the count read from +0x1c), runs
/// `00e76320` on each of them when the allocation succeeds (none when that count is 0 or above
/// `i32::MAX`: the loop counts down a signed register) and stores the array (null when the
/// allocation failed) at the stream's +8. Answers 1.
pub fn fn_00e78c60(
    e: &mut Engine,
    this: Ptr,
    renderer: Ptr,
    entry_count: u32,
    stream_count: u32,
) -> bool {
    let base = this.addr();
    let renderer = renderer.addr();
    e.mem.set_u32(base + 8, renderer);
    let manager = e.mem.u32(renderer.wrapping_add(0x8bc));
    e.mem.set_u32(base + 0xc, manager);
    let device = e.mem.u32(renderer.wrapping_add(0x288));
    e.mem.set_u32(base + 0x10, device);
    e.vcall(device, 4, &args![]);
    e.mem.set_u32(base + 0x1c, entry_count);
    e.mem.set_u32(base + 0x20, stream_count);
    let block = e
        .call(
            NI_ARRAY_ALLOCATE_00AA1420,
            &args![array_bytes(stream_count, 0x10)],
        )
        .u32();
    let streams = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT_00401050,
            &args![block, 0x10u32, stream_count, STREAM_CONSTRUCT_00E78BD0],
        );
        block
    } else {
        0
    };
    e.mem.set_u32(base + 0x24, streams);
    let mut index = 0u32;
    if e.mem.u32(base + 0x20) != 0 {
        loop {
            let count = e.mem.u32(base + 0x1c);
            let entries = e
                .call(NI_ARRAY_ALLOCATE_00AA1420, &args![array_bytes(count, 0x1c)])
                .u32();
            if entries != 0 {
                // `LEA EBX,[EBP-1]`, `JL`: a signed count down to 0.
                let mut left = count.wrapping_sub(1) as i32;
                let mut entry = entries;
                while left >= 0 {
                    e.call(ENTRY_CONSTRUCT_00E76320, &args![entry]);
                    entry = entry.wrapping_add(0x1c);
                    left = left.wrapping_sub(1);
                }
            }
            let streams = e.mem.u32(base + 0x24);
            let stream = streams.wrapping_add(index.wrapping_mul(0x10));
            e.mem.set_u32(stream.wrapping_add(8), entries);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base + 0x20) {
                break;
            }
        }
    }
    true
}

// Translated from 00e76320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the constructor of a 0x1c-byte
/// declaration entry, `thiscall`, no stack words: clears +0 and +4, sets +8 to -1 and +0xc to
/// 0x11, clears +0x10 to +0x18 and answers `this`.
pub fn fn_00e76320(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0xffff_ffff);
    e.mem.set_u32(base + 0xc, 0x11);
    for offset in [0x10u32, 0x14, 0x18] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_8c60, fn_00e78c60(Ptr, Ptr, u32, u32) -> bool),
        entry!(0x00e7_6320, fn_00e76320(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADD_REFERENCE: u32 = 0x0e60_0000;
    const DEVICE_VTABLE: u32 = 0x0e61_0000;

    /// An engine whose allocation double answers memory for sizes up to 0x1000 and 0 above.
    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REFERENCE]);
        e.register(NI_ARRAY_ALLOCATE_00AA1420, |e, a| {
            if a[0] <= 0x1000 {
                e.mem.alloc(a[0].max(1)).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(VECTOR_CONSTRUCT_00401050, |_, a| a[0].into_ret());
        e.register(ENTRY_CONSTRUCT_00E76320, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let renderer = e.mem.alloc(0x900);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8bc, 0x0e62_0000);
        (e, renderer, device)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e78c60_allocates_the_streams_and_their_entries() {
        let (mut e, renderer, device) = engine();
        let this = e.mem.alloc(0x2c);
        let answer = e
            .call(0x00e7_8c60, &args![this, renderer, 3u32, 2u32])
            .u32();
        assert_eq!(answer & 0xff, 1);
        assert_eq!(e.mem.u32(this + 8), renderer);
        assert_eq!(e.mem.u32(this + 0xc), 0x0e62_0000);
        assert_eq!(e.mem.u32(this + 0x10), device);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![device]]);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        assert_eq!(e.mem.u32(this + 0x20), 2);
        let streams = e.mem.u32(this + 0x24);
        assert_eq!(
            calls(&e, NI_ARRAY_ALLOCATE_00AA1420),
            vec![vec![0x20], vec![0x54], vec![0x54]]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![streams, 0x10, 2, STREAM_CONSTRUCT_00E78BD0]]
        );
        let first = e.mem.u32(streams + 8);
        let second = e.mem.u32(streams + 0x18);
        assert_eq!(
            calls(&e, ENTRY_CONSTRUCT_00E76320),
            vec![
                vec![first],
                vec![first + 0x1c],
                vec![first + 0x38],
                vec![second],
                vec![second + 0x1c],
                vec![second + 0x38],
            ]
        );
    }

    #[test]
    fn fn_00e78c60_keeps_null_arrays_and_skips_an_empty_count() {
        let (mut e, renderer, _) = engine();
        let this = e.mem.alloc(0x2c);
        e.call(0x00e7_8c60, &args![this, renderer, 0u32, 1u32]);
        assert!(calls(&e, ENTRY_CONSTRUCT_00E76320).is_empty());
        // An entry count whose size overflows asks for all ones; the null array is kept.
        let (mut e, renderer, _) = engine();
        let this = e.mem.alloc(0x2c);
        e.call(0x00e7_8c60, &args![this, renderer, 0x1000_0000u32, 1u32]);
        assert_eq!(
            calls(&e, NI_ARRAY_ALLOCATE_00AA1420),
            vec![vec![0x10], vec![u32::MAX]]
        );
        let streams = e.mem.u32(this + 0x24);
        assert_eq!(e.mem.u32(streams + 8), 0);
        assert!(calls(&e, ENTRY_CONSTRUCT_00E76320).is_empty());
        // No streams: nothing but the empty stream array.
        let (mut e, renderer, _) = engine();
        let this = e.mem.alloc(0x2c);
        e.call(0x00e7_8c60, &args![this, renderer, 3u32, 0u32]);
        assert_eq!(calls(&e, NI_ARRAY_ALLOCATE_00AA1420), vec![vec![0]]);
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn fn_00e76320_sets_up_the_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcc; 0x20]);
        assert_eq!(e.call(0x00e7_6320, &args![this]).u32(), this);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0xffff_ffff, 0x11, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0x1c), 0xcccc_cccc);
    }
}
