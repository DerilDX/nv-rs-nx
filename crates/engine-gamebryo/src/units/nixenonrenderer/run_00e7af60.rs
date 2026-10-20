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
    vec![entry!(0x00e7_b0f0, fn_00e7b0f0(Ptr, u32, u32, u32, u32) -> u32)]
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
