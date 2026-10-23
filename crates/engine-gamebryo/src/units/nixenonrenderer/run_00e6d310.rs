//! `NiXenonRenderer/run_00e6d310` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e6d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSBatchRenderer::RenderPassImmediately_Skinned` calls it
/// for each skin partition (`thiscall`, `RET 0xc`: the partition, a second object, an
/// unread word). Draws the partition's strips through the device at `this + 0x288`
/// (its slot +0x148, six words): for each of the partition's `+0x48` strips, the primitive
/// type (5 when the second object's `u16` +0x22 is non-zero, else 4), the partition's
/// word +0x38, 0, the second object's `u16` +0x1c, the strip's first index and its length.
/// A strip's length is its 16-bit entry in the partition's array +0x4c minus 2, or the
/// partition's word +0x40 when there is no array; the first index advances by the length
/// plus 2.
pub fn fn_00e6d310(e: &mut Engine, this: Ptr, partition: u32, object: u32, _unused_3: u32) {
    let mut start = 0u32;
    let mut strip = 0u32;
    if e.mem.u32(partition + 0x48) == 0 {
        return;
    }
    loop {
        let lengths = e.mem.u32(partition + 0x4c);
        let length = if lengths != 0 {
            u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
        } else {
            e.mem.u32(partition + 0x40)
        };
        let primitive = if e.mem.u16(object + 0x22) != 0 {
            5u32
        } else {
            4
        };
        let vertex_count = u32::from(e.mem.u16(object + 0x1c));
        let device = e.mem.u32(this.addr() + 0x288);
        let word_38 = e.mem.u32(partition + 0x38);
        e.vcall(
            device,
            0x148,
            &args![primitive, word_38, 0u32, vertex_count, start, length],
        );
        strip += 1;
        start = start.wrapping_add(length).wrapping_add(2);
        if strip >= e.mem.u32(partition + 0x48) {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e6_d310, fn_00e6d310(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRAW: u32 = 0x0000_5148;

    fn setup(lengths: Option<&[u16]>, strips: u32, type_word: u16) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(DRAW, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x14c);
        e.mem.set_u32(vtable + 0x148, DRAW);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let this = e.mem.alloc(0x28c);
        e.mem.set_u32(this + 0x288, device);
        let partition = e.mem.alloc(0x50);
        e.mem.set_u32(partition + 0x48, strips);
        e.mem.set_u32(partition + 0x38, 0x38);
        e.mem.set_u32(partition + 0x40, 9);
        if let Some(lengths) = lengths {
            let array = e.mem.alloc(2 * lengths.len() as u32);
            for (i, l) in lengths.iter().enumerate() {
                e.mem.set_u16(array + 2 * i as u32, *l);
            }
            e.mem.set_u32(partition + 0x4c, array);
        }
        let object = e.mem.alloc(0x24);
        e.mem.set_u16(object + 0x1c, 0x30);
        e.mem.set_u16(object + 0x22, type_word);
        e.call_log = Some(vec![]);
        (e, this, partition, object)
    }

    fn draws(e: &mut Engine) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == DRAW)
            .map(|(_, w)| w[1..].to_vec())
            .collect()
    }

    #[test]
    fn draws_each_strip_from_the_length_array() {
        let (mut e, this, partition, object) = setup(Some(&[5, 7]), 2, 1);
        e.call(0x00e6_d310, &[this, partition, object, 0]);
        assert_eq!(
            draws(&mut e),
            vec![vec![5, 0x38, 0, 0x30, 0, 3], vec![5, 0x38, 0, 0x30, 5, 5]]
        );
    }

    #[test]
    fn draws_with_the_partition_length_and_type_4() {
        let (mut e, this, partition, object) = setup(None, 2, 0);
        e.call(0x00e6_d310, &[this, partition, object, 0]);
        assert_eq!(
            draws(&mut e),
            vec![vec![4, 0x38, 0, 0x30, 0, 9], vec![4, 0x38, 0, 0x30, 11, 9]]
        );
        let (mut e, this, partition, object) = setup(None, 0, 0);
        e.call(0x00e6_d310, &[this, partition, object, 0]);
        assert!(draws(&mut e).is_empty());
    }
}
