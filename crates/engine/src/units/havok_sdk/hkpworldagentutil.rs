//! `Havok SDK/hkpworldagentutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The agent storage the functions walk (offsets, from the code): a block at +0x5c holds an
//! array of pointers to 0x200-byte chunks of agent entries, +0x60 is the number of chunks
//! and +0x58 the used size of the last chunk; each entry's byte at +3 is its size.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgentNnMachine_WarpTimeInAgent` (Xbox PDB, `00cfdc90`, cdecl `(entry, float, float,
/// word)`).
const WARP_TIME_IN_AGENT: u32 = 0x00cf_dc90;
/// `hkpWorldAgentUtil::removeAgent` (Xbox PDB, `00cc10b0`, cdecl `(agent)`).
const REMOVE_AGENT: u32 = 0x00cc_10b0;
/// `hkAgentNnMachine_InvalidateTimInAgent` (Xbox PDB, `00cfdbe0`, cdecl `(agent, tim)`).
const INVALIDATE_TIM_IN_AGENT: u32 = 0x00cf_dbe0;

// Translated from 00cc0e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::getIslandFromAgentEntry` (Xbox PDB, cdecl): of the two blocks `first`
/// and `second`, searches the one with the smaller (or equal, for `first`) chunk count for a
/// chunk `[start, start + 0x200)` that contains `entry`, and returns `first` when the answer
/// "found" equals "the search was done in `first`", else `second`.
pub fn hkp_world_agent_util_get_island_from_agent_entry(
    e: &mut Engine,
    entry: u32,
    first: Ptr,
    second: Ptr,
) -> Ptr {
    let searched_first = e.mem.i32(first.addr() + 0x60) <= e.mem.i32(second.addr() + 0x60);
    let searched = if searched_first { first } else { second };
    let count = e.mem.i32(searched.addr() + 0x60);
    let mut found = false;
    if count > 0 {
        let chunks = e.mem.u32(searched.addr() + 0x5c);
        for index in 0..count as u32 {
            let start = e.mem.u32(chunks + 4 * index);
            if start <= entry && entry < start.wrapping_add(0x200) {
                found = true;
                break;
            }
        }
    }
    if found == searched_first {
        first
    } else {
        second
    }
}

// Translated from 00cc0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::warpTime` (Xbox PDB, cdecl): walks every agent entry of every chunk
/// (a full 0x200 bytes, except the last chunk, which holds the size at +0x58) and calls
/// `hkAgentNnMachine_WarpTimeInAgent(entry, first, second, third)` on each, moving on by the
/// entry's size byte at +3. The two floats go through the x87 (a signalling NaN comes out
/// quiet).
pub fn hkp_world_agent_util_warp_time(
    e: &mut Engine,
    island: Ptr,
    first: f32,
    second: f32,
    third: u32,
) {
    let mut chunk = 0u32;
    if e.mem.i32(island.addr() + 0x60) > 0 {
        loop {
            let chunks = e.mem.u32(island.addr() + 0x5c);
            let mut entry = e.mem.u32(chunks + 4 * chunk);
            chunk += 1;
            let end = if chunk as i32 == e.mem.i32(island.addr() + 0x60) {
                e.mem.u32(island.addr() + 0x58).wrapping_add(entry)
            } else {
                entry.wrapping_add(0x200)
            };
            while entry < end {
                e.call(
                    WARP_TIME_IN_AGENT,
                    &args![entry, x87_quiet(first), x87_quiet(second), third],
                );
                entry = entry.wrapping_add(u32::from(e.mem.u8(entry + 3)));
            }
            if chunk as i32 >= e.mem.i32(island.addr() + 0x60) {
                break;
            }
        }
    }
}

// Translated from 00cc1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::removeAgentAndItsToiEvents` (Xbox PDB, cdecl): finds the object
/// `[[[agent + 0x10] + (signed byte at +0x10) + 0xCC] + 0x18] + 8`, calls its virtual slot
/// `0x28` with the agent, then `removeAgent` (`00cc10b0`).
pub fn hkp_world_agent_util_remove_agent_and_its_toi_events(e: &mut Engine, agent: Ptr) {
    let entity = e.mem.u32(agent.addr() + 0x10);
    let offset = i32::from(e.mem.i8(entity + 0x10)) as u32;
    let holder = e.mem.u32(entity.wrapping_add(offset).wrapping_add(0xcc));
    let inner = e.mem.u32(holder + 0x18);
    let object = e.mem.u32(inner + 8);
    e.vcall(object, 0x28, &args![agent]);
    e.call(REMOVE_AGENT, &args![agent]);
}

// Translated from 00cc0ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::invalidateTim` (Xbox PDB, cdecl): for every 8-byte element of the array
/// at `container + 0x60` (count at `+0x64`, read again after each call) calls
/// `hkAgentNnMachine_InvalidateTimInAgent(first word of the element, tim)`.
pub fn hkp_world_agent_util_invalidate_tim(e: &mut Engine, container: Ptr, tim: u32) {
    let c = container.addr();
    let mut index = 0u32;
    if e.mem.i32(c + 0x64) > 0 {
        loop {
            let elements = e.mem.u32(c + 0x60);
            let agent = e.mem.u32(elements.wrapping_add(index.wrapping_mul(8)));
            e.call(INVALIDATE_TIM_IN_AGENT, &args![agent, tim]);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(c + 0x64) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cc_0e20,
            hkp_world_agent_util_get_island_from_agent_entry(u32, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00cc_0ea0, hkp_world_agent_util_invalidate_tim(Ptr, u32)),
        entry!(
            0x00cc_0ed0,
            hkp_world_agent_util_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(
            0x00cc_1160,
            hkp_world_agent_util_remove_agent_and_its_toi_events(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A block with the given chunk start addresses.
    fn block(e: &mut Engine, chunks: &[u32]) -> u32 {
        let b = e.mem.alloc(0x80);
        let array = e.mem.alloc(4 * chunks.len().max(1) as u32);
        for (i, c) in chunks.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        e.mem.set_u32(b + 0x5c, array);
        e.mem.set_i32(b + 0x60, chunks.len() as i32);
        b
    }

    #[test]
    fn island_search_uses_the_block_with_fewer_chunks() {
        let mut e = Engine::new();
        let small = block(&mut e, &[0x1000]);
        let big = block(&mut e, &[0x5000, 0x6000, 0x7000]);
        let call = |e: &mut Engine, entry: u32, a: u32, b: u32| {
            e.call(0x00cc_0e20, &args![entry, a, b]).u32()
        };
        // first has fewer chunks and contains the entry: found == searched-in-first -> first
        assert_eq!(call(&mut e, 0x1100, small, big), small);
        // first has fewer chunks and does not contain it -> second
        assert_eq!(call(&mut e, 0x6100, small, big), big);
        // second has fewer chunks (searched in second): found there -> not first -> second
        assert_eq!(call(&mut e, 0x1100, big, small), small);
        // second has fewer chunks and does not contain it -> first
        assert_eq!(call(&mut e, 0x6100, big, small), big);
        // the chunk end is exclusive, the start inclusive
        assert_eq!(call(&mut e, 0x1200, small, big), big);
        assert_eq!(call(&mut e, 0x1000, small, big), small);
        // equal counts search the first block
        let other = block(&mut e, &[0x9000]);
        assert_eq!(call(&mut e, 0x9010, small, other), other);
        assert_eq!(call(&mut e, 0x1010, small, other), small);
    }

    #[test]
    fn island_search_with_no_chunks_finds_nothing() {
        let mut e = Engine::new();
        let empty = block(&mut e, &[]);
        let other = block(&mut e, &[0x1000]);
        // first (0 chunks) is searched and nothing found -> second
        let got = e.call(0x00cc_0e20, &args![0x1000u32, empty, other]).u32();
        assert_eq!(got, other);
    }

    #[test]
    fn warp_time_visits_every_entry_of_every_chunk() {
        let mut e = Engine::new();
        let first_chunk = e.mem.alloc(0x200);
        let last_chunk = e.mem.alloc(0x200);
        // first chunk: 0x80-byte entries (4 of them fill 0x200); last chunk: used 0x30 bytes
        for i in 0..4 {
            e.mem.set_u8(first_chunk + 0x80 * i + 3, 0x80);
        }
        e.mem.set_u8(last_chunk + 3, 0x10);
        e.mem.set_u8(last_chunk + 0x10 + 3, 0x20);
        let island = block(&mut e, &[first_chunk, last_chunk]);
        e.mem.set_u32(island + 0x58, 0x30);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(WARP_TIME_IN_AGENT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(
            0x00cc_0ed0,
            &args![island, f32::from_bits(0x7f80_0001), 2.5f32, 0x77u32],
        );
        let seen = seen.borrow();
        let entries: Vec<u32> = seen.iter().map(|c| c[0]).collect();
        let want: Vec<u32> = (0..4)
            .map(|i| first_chunk + 0x80 * i)
            .chain([last_chunk, last_chunk + 0x10])
            .collect();
        assert_eq!(entries, want);
        assert_eq!(seen[0][1], 0x7fc0_0001);
        assert_eq!(seen[0][2], 2.5f32.to_bits());
        assert_eq!(seen[0][3], 0x77);
    }

    #[test]
    fn warp_time_without_chunks_calls_nothing() {
        let mut e = Engine::new();
        let island = block(&mut e, &[]);
        e.register(WARP_TIME_IN_AGENT, |_, _| panic!("no entries"));
        e.call(0x00cc_0ed0, &args![island, 1.0f32, 2.0f32, 3u32]);
    }

    #[test]
    fn remove_agent_calls_the_toi_slot_then_remove_agent() {
        let mut e = Engine::new();
        const VTABLE: u32 = 0x0a00_0000;
        const SLOT: u32 = 0x0a10_0001;
        let mut slots = vec![0u32; 0x30 / 4];
        slots[0x28 / 4] = SLOT;
        e.put_vtable(VTABLE, &slots);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, VTABLE);
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner + 8, object);
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder + 0x18, inner);
        let entity = e.mem.alloc(0x200);
        e.mem.set_i8(entity + 0x10, 4);
        e.mem.set_u32(entity + 4 + 0xcc, holder);
        let agent = e.mem.alloc(0x20);
        e.mem.set_u32(agent + 0x10, entity);
        e.register(SLOT, |_, _| Ret::default());
        e.register(REMOVE_AGENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cc_1160, &args![agent]);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| c.0 == SLOT || c.0 == REMOVE_AGENT)
            .collect();
        assert_eq!(
            order,
            vec![(SLOT, vec![object, agent]), (REMOVE_AGENT, vec![agent])]
        );
    }

    #[test]
    fn invalidate_tim_visits_every_element_and_rereads_the_count() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x80);
        let elements = e.mem.alloc(0x20);
        for (i, agent) in [0x11u32, 0x22, 0x33].iter().enumerate() {
            e.mem.set_u32(elements + 8 * i as u32, *agent);
            e.mem.set_u32(elements + 8 * i as u32 + 4, 0xdead);
        }
        e.mem.set_u32(container + 0x60, elements);
        e.mem.set_u32(container + 0x64, 3);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(INVALIDATE_TIM_IN_AGENT, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            if a[0] == 0x22 {
                e.mem.set_u32(container + 0x64, 2);
            }
            Ret::default()
        });
        e.call(0x00cc_0ea0, &args![container, 0x99u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x11, 0x99], vec![0x22, 0x99]]);
    }

    #[test]
    fn invalidate_tim_with_no_elements_calls_nothing() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x64, 0);
        e.register(INVALIDATE_TIM_IN_AGENT, |_, _| panic!("no elements"));
        e.call(0x00cc_0ea0, &args![container, 1u32]);
        e.mem.set_u32(container + 0x64, 0xffff_ffff);
        e.call(0x00cc_0ea0, &args![container, 1u32]);
    }
}
