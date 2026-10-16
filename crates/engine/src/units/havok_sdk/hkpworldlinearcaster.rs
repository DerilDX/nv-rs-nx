//! `Havok SDK/hkpworldlinearcaster.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0.5f` (`.rdata`).
const HALF_01016248: u32 = 0x0101_6248;
/// Four `1.0f` (`.rdata`).
const ONES_010EC7F0: u32 = 0x010e_c7f0;

fn read_lanes_00cfaed0(e: &Engine, addr: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(addr + offset))
}

fn write_lanes_00cfaed0(e: &mut Engine, addr: u32, lanes: [f32; 4]) {
    for (i, lane) in lanes.iter().enumerate() {
        e.mem.set_f32(addr + 4 * i as u32, *lane);
    }
}

// Translated from 00cfaed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldLinearCaster::linearCast` (map name). Thiscall, nine words
/// (`RET 0x24`), SSE in `f32` per lane. Stores its arguments in `this`
/// (`+4 input`, `+8 word`, `+0xc`, `+0x10`, `+0x14 collidable`,
/// `+0x18 = [[collidable] + 0xc]`), copies six words of `config` to
/// `this + 0x20` and its 16-byte blocks at `+0x20`, `+0x30` and `+0x40` to
/// `this + 0x40`, `+0x50` and `+0x60`, stores `extra` at `+0x88`, the
/// vector `input[0..4] - [[collidable] + 8][0x30..0x40]` at `+0x70`, its
/// length at `+0x84` (`SQRTSS` of `s2 + (s1 + s0)`), `input[5]` at `+0x28`
/// and `input[4]` at `+0x80`.
///
/// It then asks `[collidable]`'s virtual slot `+0x1c` for two vectors `va`
/// and `vb` (arguments: `[collidable] + 8`, `input[5]`, the output block),
/// builds on the stack the 0x50 bytes `(1 - 0.5) * va + vb * 0.5`, that plus
/// the difference vector, `0.5 * (vb - va)` and the word `flag` (at `+0x80`
/// of the frame), and calls `object`'s virtual slot `+0x6c` with the
/// address of that block and `this`.
#[allow(clippy::too_many_arguments)]
pub fn hkp_world_linear_caster_linear_cast(
    e: &mut Engine,
    this: Ptr,
    object: Ptr,
    collidable: Ptr,
    input: Ptr,
    word: u32,
    config: Ptr,
    extra: u32,
    flag: u32,
    stored_c: u32,
    stored_10: u32,
) {
    let (this_a, collidable_a, input_a, config_a) =
        (this.addr(), collidable.addr(), input.addr(), config.addr());
    e.mem.set_u32(this_a + 0xc, stored_c);
    e.mem.set_u32(this_a + 8, word);
    e.mem.set_u32(this_a + 0x10, stored_10);
    e.mem.set_u32(this_a + 0x14, collidable_a);
    e.mem.set_u32(this_a + 4, input_a);
    let shape_holder = e.mem.u32(collidable_a);
    let value = e.mem.u32(shape_holder + 0xc);
    e.mem.set_u32(this_a + 0x18, value);
    for i in 0..6u32 {
        let value = e.mem.u32(config_a + 4 * i);
        e.mem.set_u32(this_a + 0x20 + 4 * i, value);
    }
    for (from, to) in [(0x20u32, 0x40u32), (0x30, 0x50)] {
        let lanes = read_lanes_00cfaed0(e, config_a + from);
        write_lanes_00cfaed0(e, this_a + to, lanes);
    }
    let last_block = read_lanes_00cfaed0(e, config_a + 0x40);
    write_lanes_00cfaed0(e, this_a + 0x60, last_block);
    e.mem.set_u32(this_a + 0x88, extra);

    let time = x87_quiet(e.mem.f32(input_a + 0x14));
    let body = e.mem.u32(collidable_a + 8);
    let position = read_lanes_00cfaed0(e, input_a);
    let origin = read_lanes_00cfaed0(e, body + 0x30);
    let delta = [0, 1, 2, 3].map(|i| position[i] - origin[i]);
    write_lanes_00cfaed0(e, this_a + 0x70, delta);
    let squares = delta.map(|lane| lane * lane);
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    e.mem.set_f32(this_a + 0x84, x87_quiet(length));
    e.mem.set_f32(this_a + 0x28, time);

    // The stack frame: the two vectors the virtual call fills, then the
    // block handed to `object`.
    e.with_stack(0x90, |e, frame| {
        let vectors = frame.addr() + 0x30;
        let holder = e.mem.u32(collidable_a);
        let body = e.mem.u32(collidable_a + 8);
        let time = x87_quiet(e.mem.f32(input_a + 0x14));
        e.vcall(holder, 0x1c, &args![body, time, vectors]);
        let half = e.mem.f32(HALF_01016248);
        let ones = read_lanes_00cfaed0(e, ONES_010EC7F0);
        let first = read_lanes_00cfaed0(e, vectors);
        let second = read_lanes_00cfaed0(e, vectors + 0x10);
        let speed = x87_quiet(e.mem.f32(input_a + 0x10));
        e.mem.set_f32(this_a + 0x80, speed);
        let midpoint = [0, 1, 2, 3].map(|i| (ones[i] - half) * first[i] + second[i] * half);
        write_lanes_00cfaed0(e, frame.addr() + 0x50, midpoint);
        let moved = [0, 1, 2, 3].map(|i| midpoint[i] + delta[i]);
        write_lanes_00cfaed0(e, frame.addr() + 0x60, moved);
        e.mem.set_u32(frame.addr() + 0x80, flag);
        let half_span = [0, 1, 2, 3].map(|i| half * (second[i] - first[i]));
        write_lanes_00cfaed0(e, frame.addr() + 0x70, half_span);
        e.vcall(object.addr(), 0x6c, &args![frame.addr() + 0x50, this]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cfaed0,
        hkp_world_linear_caster_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn hkp_world_linear_caster_linear_cast_stores_the_cast_and_calls_the_collector() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x010e_c000, 0x1000);
        e.set_global(HALF_01016248, 0.5f32);
        for i in 0..4 {
            e.set_global(ONES_010EC7F0 + 4 * i, 1.0f32);
        }
        // The collidable: [collidable] holds a vtable whose slot 0x1c fills
        // the two vectors; its +8 word is the body whose +0x30 is its origin.
        let mut slots = vec![0u32; 8];
        slots[0x1c / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder, 0x0090_2000);
        e.mem.set_u32(holder + 0xc, 0xabcd);
        let body = e.mem.alloc(0x40);
        for (i, value) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
            e.mem.set_f32(body + 0x30 + 4 * i as u32, *value);
        }
        let collidable = e.mem.alloc(0x10);
        e.mem.set_u32(collidable, holder);
        e.mem.set_u32(collidable + 8, body);
        e.register(0x0090_5000, |e, a| {
            // this, body, time, out: va = (1, 2, 3, 4), vb = (3, 2, 1, 0).
            assert_eq!(a[2], 0.25f32.to_bits());
            for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0, 0.0]
                .iter()
                .enumerate()
            {
                e.mem.set_f32(a[3] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        // The collector object: slot 0x6c receives (block, caster).
        let mut collector_slots = vec![0u32; 0x20];
        collector_slots[0x6c / 4] = 0x0090_5100;
        e.put_vtable(0x0090_3000, &collector_slots);
        let collector = e.mem.alloc(8);
        e.mem.set_u32(collector, 0x0090_3000);
        let block = Rc::new(RefCell::new(Vec::new()));
        let block_log = Rc::clone(&block);
        e.register_double(0x0090_5100, move |e, a| {
            let words: Vec<u32> = (0..0x24).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            block_log.borrow_mut().push((a.to_vec(), words));
            Ret::default()
        });
        // The input: position (5, 5, 5, 0), speed 9 at +0x10, time 0.25 at +0x14.
        let input = e.mem.alloc(0x20);
        for (i, v) in [5.0f32, 5.0, 5.0, 0.0, 9.0, 0.25].iter().enumerate() {
            e.mem.set_f32(input + 4 * i as u32, *v);
        }
        let config = e.mem.alloc(0x50);
        for i in 0..0x14 {
            e.mem.set_u32(config + 4 * i, 0x100 + i);
        }
        let this = e.mem.alloc(0x90);
        e.call(
            0x00cf_aed0,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(collector),
                Ptr::<()>::new(collidable),
                Ptr::<()>::new(input),
                11u32,
                Ptr::<()>::new(config),
                33u32,
                44u32,
                55u32,
                66u32
            ],
        );
        assert_eq!(e.mem.u32(this + 4), input);
        assert_eq!(e.mem.u32(this + 8), 11);
        assert_eq!(e.mem.u32(this + 0xc), 55);
        assert_eq!(e.mem.u32(this + 0x10), 66);
        assert_eq!(e.mem.u32(this + 0x14), collidable);
        assert_eq!(e.mem.u32(this + 0x18), 0xabcd);
        assert_eq!(e.mem.u32(this + 0x20), 0x100);
        assert_eq!(e.mem.u32(this + 0x30), 0x104);
        assert_eq!(e.mem.f32(this + 0x28), 0.25);
        assert_eq!(e.mem.u32(this + 0x40), 0x108);
        assert_eq!(e.mem.u32(this + 0x60), 0x110);
        assert_eq!(e.mem.u32(this + 0x88), 33);
        assert_eq!(e.mem.f32(this + 0x80), 9.0);
        // delta = (4, 3, 2, 0): length sqrt(29).
        assert_eq!(e.mem.f32(this + 0x70), 4.0);
        assert_eq!(e.mem.f32(this + 0x84), 29.0f32.sqrt());
        let calls = block.borrow();
        assert_eq!(calls.len(), 1);
        let (call_args, words) = &calls[0];
        assert_eq!(call_args[0], collector);
        assert_eq!(call_args[2], this);
        let lanes = |from: usize| [0, 1, 2, 3].map(|i| f32::from_bits(words[from + i]));
        // midpoint = 0.5 * va + 0.5 * vb, then plus delta, then 0.5 * (vb - va).
        assert_eq!(lanes(0), [2.0, 2.0, 2.0, 2.0]);
        assert_eq!(lanes(4), [6.0, 5.0, 4.0, 2.0]);
        assert_eq!(lanes(8), [1.0, 0.0, -1.0, -2.0]);
        assert_eq!(words[0xc], 44);
    }
}
