//! `NiXenonRenderer/run_00e7af60` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The display-format record's depth/stencil chooser (`00e7ade0`, unnamed; `thiscall` on the
/// record: back buffer format, depth bits, stencil bits).
const CHOOSE_DEPTH_00E7ADE0: u32 = 0x00e7_ade0;

// Translated from 00e7b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7af60`): the device description's depth/stencil
/// format, `thiscall` with four words: the display format, the back buffer format, the
/// requested depth bits and the stencil flag. The record for the display format is looked up
/// in the list at +0x134 (its element's first word; none: 0). Then `00e7ade0` on it with the
/// back buffer format and: without stencil the requested depth and 0 stencil bits; with
/// stencil 1, 24 depth bits above 16 requested (else 15) and 1 stencil bit; with any other
/// stencil value 24 and 8 bits. Answers its result.
pub fn fn_00e7b0f0(
    e: &mut Engine,
    this: Ptr,
    display_format: u32,
    back_buffer: u32,
    depth_bits: u32,
    stencil: u32,
) -> u32 {
    let mut item = e.mem.u32(this.addr().wrapping_add(0x134));
    let record = loop {
        if item == 0 {
            return 0;
        }
        let record = e.mem.u32(item.wrapping_add(8));
        item = e.mem.u32(item);
        if e.mem.u32(record) == display_format {
            break record;
        }
    };
    let (depth, stencil_bits) = if stencil == 0 {
        (depth_bits, 0)
    } else if depth_bits > 0x10 {
        if stencil == 1 {
            (0x18, 1)
        } else {
            (0x18, 8)
        }
    } else if stencil == 1 {
        (0xf, 1)
    } else {
        (0x18, 8)
    };
    e.call(
        CHOOSE_DEPTH_00E7ADE0,
        &args![record, back_buffer, depth, stencil_bits],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_b0f0, fn_00e7b0f0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00e7_b040, fn_00e7b040(Ptr)),
    ]
}

/// What `00e7b040` uses: the Ni free `00aa1460` (`cdecl`: block, size), the map's two tables
/// (`010eec7c`, then `010eec54`), the map clean-up `00b601a0` (unnamed; `thiscall`, no stack
/// words) and the free `00aa10f0` (`cdecl`, one word).
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
const MAP_VTABLE_010EEC7C: u32 = 0x010e_ec7c;
const MAP_BASE_VTABLE_010EEC54: u32 = 0x010e_ec54;
const MAP_CLEAN_UP_00B601A0: u32 = 0x00b6_01a0;
const FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00e7b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7af60`), `thiscall`, no stack words: empties and undoes
/// the pointer map at +8 (`NiTMapBase`: bucket count +4, buckets +8; nodes: next +0, key +4,
/// value +8). Each node's value (0x48 bytes) is freed (`00aa1460`, no null check) after the
/// next node is found (its next, or the first bucket after the key's, the map's slot 4 giving
/// the key's bucket). Then the map's table `010eec7c`, `00b601a0`, its base table `010eec54`,
/// `00b601a0`, and its buckets freed (`00aa10f0`).
pub fn fn_00e7b040(e: &mut Engine, this: Ptr) {
    let map = this.addr().wrapping_add(8);
    let count = e.mem.u32(map.wrapping_add(4));
    let buckets = e.mem.u32(map.wrapping_add(8));
    let mut node = 0u32;
    for index in 0..count {
        let first = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if first != 0 {
            node = first;
            break;
        }
    }
    while node != 0 {
        let next = e.mem.u32(node);
        let value = e.mem.u32(node.wrapping_add(8));
        node = if next != 0 {
            next
        } else {
            let key = e.mem.u32(node.wrapping_add(4));
            let mut index = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
            let count = e.mem.u32(map.wrapping_add(4));
            let mut found = 0;
            while index < count {
                let buckets = e.mem.u32(map.wrapping_add(8));
                let bucket = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
                if bucket != 0 {
                    found = bucket;
                    break;
                }
                index += 1;
            }
            found
        };
        e.call(NI_FREE_00AA1460, &args![value, 0x48u32]);
    }
    e.mem.set_u32(map, MAP_VTABLE_010EEC7C);
    e.call(MAP_CLEAN_UP_00B601A0, &args![map]);
    e.mem.set_u32(map, MAP_BASE_VTABLE_010EEC54);
    e.call(MAP_CLEAN_UP_00B601A0, &args![map]);
    let buckets = e.mem.u32(map.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    fn rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(CHOOSE_DEPTH_00E7ADE0, |_, a| {
            (a[1] << 16 | a[2] << 8 | a[3]).into_ret()
        });
        let desc = e.mem.alloc(0x144);
        let records: Vec<u32> = [0x15u32, 0x16]
            .iter()
            .map(|format| {
                let record = e.mem.alloc(8);
                e.mem.set_u32(record, *format);
                record
            })
            .collect();
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, records[1]);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, records[0]);
        e.mem.set_u32(desc + 0x134, first);
        (e, desc, records[1])
    }

    #[test]
    fn fn_00e7b0f0_picks_depth_and_stencil_bits() {
        let (mut e, desc, _) = rig();
        let choose = |e: &mut Engine, depth: u32, stencil: u32| {
            e.call(0x00e7_b0f0, &args![desc, 0x16u32, 0x1u32, depth, stencil])
                .u32()
        };
        assert_eq!(choose(&mut e, 0x20, 0), 0x1_2000);
        assert_eq!(choose(&mut e, 0x20, 1), 0x1_1801);
        assert_eq!(choose(&mut e, 0x20, 8), 0x1_1808);
        assert_eq!(choose(&mut e, 0x10, 1), 0x1_0f01);
        assert_eq!(choose(&mut e, 0x10, 8), 0x1_1808);
        // No record for the display format: 0 without asking.
        assert_eq!(
            e.call(0x00e7_b0f0, &args![desc, 0x17u32, 1u32, 0x20u32, 0u32])
                .u32(),
            0
        );
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    #[test]
    fn fn_00e7b040_frees_the_values_and_undoes_the_map() {
        let mut e = Engine::new();
        for no_op in [NI_FREE_00AA1460, MAP_CLEAN_UP_00B601A0, FREE_00AA10F0] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(0x0ec0_0004, |_, a| a[1].into_ret());
        e.put_vtable(0x0ec1_0000, &[0, 0x0ec0_0004]);
        let this = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x0ec1_0000);
        e.mem.set_u32(this + 0xc, 4);
        e.mem.set_u32(this + 0x10, buckets);
        // Bucket 0: two chained nodes; bucket 2: one node (key 2).
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xc)).collect();
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(nodes[1] + 4, 0);
        e.mem.set_u32(nodes[2] + 4, 2);
        for (index, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, 0x100 + index as u32);
        }
        e.mem.set_u32(buckets, nodes[0]);
        e.mem.set_u32(buckets + 8, nodes[2]);
        e.call_log = Some(vec![]);
        e.call(0x00e7_b040, &args![this]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == NI_FREE_00AA1460)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            freed,
            vec![vec![0x100, 0x48], vec![0x101, 0x48], vec![0x102, 0x48]]
        );
        let cleaned: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == MAP_CLEAN_UP_00B601A0)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(cleaned, vec![this + 8, this + 8]);
        assert!(log.contains(&(FREE_00AA10F0, vec![buckets])));
        assert_eq!(e.mem.u32(this + 8), MAP_BASE_VTABLE_010EEC54);
    }
}
