//! `BSMain/bstestobjects.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton (`011f6238`) and its `Allocate` (`00aa3e40`, `thiscall`,
/// the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00439ef0` (unnamed; `thiscall` on a point: the result's address and another point;
/// answers the result): what `CreateBox` gets its first direction from (`end`, `start`).
const POINT_OPERATION_00439EF0: u32 = 0x0043_9ef0;
/// `00439e90` (unnamed; `thiscall` on a point: the result's address and another point;
/// answers the result): what makes the last corner from its sum and the lower offset.
const POINT_OPERATION_00439E90: u32 = 0x0043_9e90;
/// `004b3800` (unnamed; `thiscall` on a point: the result's address and another point):
/// what the direction is turned with the vector (0, 0, 1).
const POINT_OPERATION_004B3800: u32 = 0x004b_3800;
/// `004a0c10` (unnamed; `thiscall` on the point, no stack words): run on the turned
/// direction after its z is cleared.
const POINT_OPERATION_004A0C10: u32 = 0x004a_0c10;
/// The compiler's `vector constructor iterator` (`00401050`, `stdcall`: array, element
/// size, count, constructor) and the colour constructor it is given (`004a7800`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const COLOR_CONSTRUCT: u32 = 0x004a_7800;
/// `memcpy` (`00ec44d0`, `cdecl`) and the box's 0x48-byte triangle list it copies
/// (`010c2c38`, twelve triangles of three `u16`s).
const MEMCPY: u32 = 0x00ec_44d0;
const BOX_TRIANGLES: u32 = 0x010c_2c38;
/// `NiNew` (`00aa13e0`, `cdecl`, the size) and the `NiTriShape` constructor `00a74410`
/// (`thiscall`, nine words: vertex count, vertices, normals, colours, texture sets, texture
/// set count, data flags, triangle count, triangles).
const NI_NEW: u32 = 0x00aa_13e0;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// The new shape's vtable slot `CreateBox` calls last (+0xE8, no stack words).
const SHAPE_SLOT_E8: u32 = 0xe8;

/// A vertex as the x87 code makes it: `sum = offset + base` per component (`FLD dword`
/// offset, `FADD dword` base, `FSTP dword`), then `z + sum` (`FLD dword` z, `FADD dword`
/// sum, `FSTP dword`), stored at `out`. `base` is read from memory component by component
/// as the code reads it.
fn corner(e: &mut Engine, out: u32, offset: [u32; 3], base: u32, z: [u32; 3]) {
    let mut sum = [0u32; 3];
    for i in 0..3u32 {
        let index = i as usize;
        let base_word = e.mem.u32(base.wrapping_add(4 * i));
        sum[index] = x87_store32(x87_add(x87_load32(offset[index]), x87_m32(base_word)));
    }
    let mut vertex = [0u32; 3];
    for i in 0..3 {
        vertex[i] = x87_store32(x87_add(x87_load32(z[i]), x87_m32(sum[i])));
    }
    for (i, word) in vertex.iter().enumerate() {
        e.mem.set_u32(out + 4 * i as u32, *word);
    }
}

/// `direction * factor` per component (`FLD dword`, `FMUL dword`, `FSTP dword`).
fn scaled(direction: [u32; 3], factor: u32) -> [u32; 3] {
    direction.map(|d| x87_store32(x87_mul(x87_load32(d), x87_m32(factor))))
}

// Translated from 00c55d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c55d50 - the FS:[0] exception frame (unwind states 0 and 1 for the colour array and the shape allocation) is not translated
/// `BSTestObjects::CreateBox_ov2` (engine map; `cdecl`, eight words: two points, four
/// floats, two colours): a box `NiTriShape` along the segment from `start` to `end`.
///
/// The direction is `00439ef0` on `start` with `end`, turned by `004b3800` with (0, 0, 1),
/// its z cleared and `004a0c10` run on it; the side offsets are the direction times
/// `right` and times `-left`, the vertical offsets (0, 0, `up`) and (0, 0, `-down`). The
/// eight vertices (0x60 bytes from the memory manager; null when that fails, and the code
/// then writes through null) are, each side offset plus a point plus a vertical offset:
/// `-left`/`end`/up, `right`/`end`/up, `-left`/`end`/down, `right`/`end`/down, then the same
/// four with `start`, the last one's vertical offset added by `00439e90`. With either colour
/// given, 8 colours (0x80 bytes, constructed by the `vector constructor iterator` with
/// `004a7800`; null when the allocation fails) take the four words of `second_color` (the
/// first when only that is given) for the first four vertices and of `first_color` (the
/// second when only that is given) for the others. The triangles are a copy of the 0x48
/// bytes at `010c2c38`. The shape is `NiNew(0xC4)` constructed by `00a74410` with 8
/// vertices, no normals, the colours, no texture sets, data flags 0 and 12 triangles (null
/// when the allocation fails; the code then calls through null), then its vtable slot
/// +0xE8. Answers the shape.
#[allow(clippy::too_many_arguments)]
pub fn bs_test_objects_create_box_ov2(
    e: &mut Engine,
    start: Ptr,
    end: Ptr,
    right: f32,
    left: f32,
    up: f32,
    down: f32,
    first_color: Ptr,
    second_color: Ptr,
) -> u32 {
    let vertices = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x60u32])
        .u32();
    // A non-null block is walked by the (empty) constructor loop eight times; nothing is
    // written.
    let start = start.addr();
    let end = end.addr();
    // The locals passed by address: the first direction (+0), (0, 0, 1) (+0xC), the turned
    // direction (+0x18), the lower offset (+0x24), the last sum (+0x30) and the last
    // corner (+0x3C).
    e.with_stack(0x48, |e, locals| {
        let l = locals.addr();
        e.call(POINT_OPERATION_00439EF0, &args![start, l, end]);
        e.mem.set_u32(l + 0xc, x87_store32(0.0));
        e.mem.set_u32(l + 0x10, x87_store32(0.0));
        e.mem.set_u32(l + 0x14, x87_store32(1.0));
        e.call(POINT_OPERATION_004B3800, &args![l, l + 0x18, l + 0xc]);
        e.mem.set_u32(l + 0x20, x87_store32(0.0));
        e.call(POINT_OPERATION_004A0C10, &args![l + 0x18]);
        let direction = [
            e.mem.u32(l + 0x18),
            e.mem.u32(l + 0x1c),
            e.mem.u32(l + 0x20),
        ];
        let right_offset = scaled(direction, right.to_bits());
        let minus_left = x87_store32(-x87_load32(left.to_bits()));
        let left_offset = scaled(direction, minus_left);
        let upper = [0, 0, x87_quiet_bits(up.to_bits())];
        let lower = [0, 0, x87_store32(-x87_load32(down.to_bits()))];
        e.mem.set_u32(l + 0x24, lower[0]);
        e.mem.set_u32(l + 0x28, lower[1]);
        e.mem.set_u32(l + 0x2c, lower[2]);
        corner(e, vertices, left_offset, end, upper);
        corner(e, vertices + 0xc, right_offset, end, upper);
        corner(e, vertices + 0x18, left_offset, end, lower);
        corner(e, vertices + 0x24, right_offset, end, lower);
        corner(e, vertices + 0x30, left_offset, start, upper);
        corner(e, vertices + 0x3c, right_offset, start, upper);
        corner(e, vertices + 0x48, left_offset, start, lower);
        for i in 0..3u32 {
            let index = i as usize;
            let base_word = e.mem.u32(start.wrapping_add(4 * i));
            let sum = x87_add(x87_load32(right_offset[index]), x87_m32(base_word));
            e.mem.set_u32(l + 0x30 + 4 * i, x87_store32(sum));
        }
        let last = e
            .call(
                POINT_OPERATION_00439E90,
                &args![l + 0x30, l + 0x3c, l + 0x24],
            )
            .u32();
        for i in 0..3u32 {
            let word = e.mem.u32(last.wrapping_add(4 * i));
            e.mem.set_u32(vertices + 0x54 + 4 * i, word);
        }
    });
    let mut colors = 0u32;
    let mut first_color = first_color.addr();
    let mut second_color = second_color.addr();
    if first_color != 0 || second_color != 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x80u32])
            .u32();
        colors = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block, 0x10u32, 8u32, COLOR_CONSTRUCT],
            );
            block
        } else {
            0
        };
        if first_color != 0 && second_color == 0 {
            second_color = first_color;
        } else if second_color != 0 && first_color == 0 {
            first_color = second_color;
        }
        for index in 0..4u32 {
            let to = (index << 4).wrapping_add(colors);
            for word in 0..4u32 {
                let value = e.mem.u32(second_color.wrapping_add(4 * word));
                e.mem.set_u32(to.wrapping_add(4 * word), value);
            }
        }
        for index in 4..8u32 {
            let to = (index << 4).wrapping_add(colors);
            for word in 0..4u32 {
                let value = e.mem.u32(first_color.wrapping_add(4 * word));
                e.mem.set_u32(to.wrapping_add(4 * word), value);
            }
        }
    }
    let triangles = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x48u32])
        .u32();
    e.call(MEMCPY, &args![triangles, BOX_TRIANGLES, 0x48u32]);
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(
            TRI_SHAPE_CONSTRUCT,
            &args![block, 8u32, vertices, 0u32, colors, 0u32, 0u32, 0u32, 0xcu32, triangles],
        )
        .u32()
    } else {
        0
    };
    e.vcall(shape, SHAPE_SLOT_E8, &args![]);
    shape
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c5_5d50,
        bs_test_objects_create_box_ov2(Ptr, Ptr, f32, f32, f32, f32, Ptr, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHAPE_VTABLE: u32 = 0x0e60_0000;
    const SHAPE_FINISH: u32 = 0x0e61_0000;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0xc);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
        at
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    /// Doubles: the allocations from the heap, the first direction (1, 0, 0), the turn
    /// keeping it, the unitize doing nothing, `00439e90` adding, `memcpy`, `NiNew` and the
    /// shape constructor storing the shape vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(POINT_OPERATION_00439EF0, |e, a| {
            e.mem.set_u32(a[1], 1.0f32.to_bits());
            e.mem.set_u32(a[1] + 4, 0);
            e.mem.set_u32(a[1] + 8, 0);
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_004B3800, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            // A z the code clears afterwards.
            e.mem.set_u32(a[1] + 8, 9.0f32.to_bits());
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_004A0C10, |_, _| Ret::default());
        e.register(POINT_OPERATION_00439E90, |e, a| {
            for i in 0..3 {
                let sum = f32::from_bits(e.mem.u32(a[0] + 4 * i))
                    + f32::from_bits(e.mem.u32(a[2] + 4 * i));
                e.mem.set_u32(a[1] + 4 * i, sum.to_bits());
            }
            a[1].into_ret()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.map(BOX_TRIANGLES, 0x48);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TRI_SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], SHAPE_VTABLE);
            a[0].into_ret()
        });
        e.put_vtable(SHAPE_VTABLE, &[0; 0x3c]);
        e.mem.set_u32(SHAPE_VTABLE + SHAPE_SLOT_E8, SHAPE_FINISH);
        e.register(SHAPE_FINISH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn box_corners_and_shape() {
        let mut e = engine();
        e.mem.set_u16(BOX_TRIANGLES + 2, 7);
        let start = point(&mut e, [10.0, 20.0, 30.0]);
        let end = point(&mut e, [100.0, 200.0, 300.0]);
        let shape = e
            .call(
                0x00c5_5d50,
                &args![start, end, 2.0f32, 3.0f32, 5.0f32, 7.0f32, 0u32, 0u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, POINT_OPERATION_00439EF0)[0][2], end);
        assert_eq!(calls(&log, POINT_OPERATION_00439EF0)[0][0], start);
        let made = calls(&log, TRI_SHAPE_CONSTRUCT);
        assert_eq!(made.len(), 1);
        let words = &made[0];
        assert_eq!(words[0], shape);
        assert_eq!(words[1], 8);
        assert_eq!([words[3], words[4], words[5]], [0, 0, 0]);
        assert_eq!([words[6], words[7], words[8]], [0, 0, 0xc]);
        let vertices = words[2];
        assert_eq!(
            floats(&e, vertices, 24),
            vec![
                97.0, 200.0, 305.0, 102.0, 200.0, 305.0, 97.0, 200.0, 293.0, 102.0, 200.0, 293.0,
                7.0, 20.0, 35.0, 12.0, 20.0, 35.0, 7.0, 20.0, 23.0, 12.0, 20.0, 23.0
            ]
        );
        // The triangles are a copy of the exe's list.
        assert_eq!(e.mem.u16(words[9] + 2), 7);
        assert_eq!(calls(&log, SHAPE_FINISH), vec![vec![shape]]);
        assert!(calls(&log, VECTOR_CONSTRUCT).is_empty());
        // The turned direction's z was cleared before the unitize.
        let turned = calls(&log, POINT_OPERATION_004A0C10)[0][0];
        assert_eq!(calls(&log, POINT_OPERATION_004B3800)[0][1], turned);
    }

    #[test]
    fn box_colours_fill_in_for_each_other() {
        let mut e = engine();
        let start = point(&mut e, [0.0; 3]);
        let end = point(&mut e, [1.0, 0.0, 0.0]);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(first + 4 * i, 0x100 + i);
            e.mem.set_u32(second + 4 * i, 0x200 + i);
        }
        let colours_of = |e: &mut Engine, a: u32, b: u32| -> Vec<u32> {
            e.call_log = Some(vec![]);
            e.call(
                0x00c5_5d50,
                &args![start, end, 1.0f32, 1.0f32, 1.0f32, 1.0f32, a, b],
            );
            let log = e.call_log.take().unwrap();
            let made = calls(&log, TRI_SHAPE_CONSTRUCT);
            let constructed = calls(&log, VECTOR_CONSTRUCT);
            assert_eq!(
                constructed,
                vec![vec![made[0][4], 0x10, 8, COLOR_CONSTRUCT]]
            );
            (0..8).map(|i| e.mem.u32(made[0][4] + 0x10 * i)).collect()
        };
        assert_eq!(
            colours_of(&mut e, first, second),
            vec![0x200, 0x200, 0x200, 0x200, 0x100, 0x100, 0x100, 0x100]
        );
        assert_eq!(colours_of(&mut e, first, 0), vec![0x100; 8]);
        assert_eq!(colours_of(&mut e, 0, second), vec![0x200; 8]);
    }

    #[test]
    fn box_signs_follow_the_x87() {
        let mut e = engine();
        let start = point(&mut e, [0.0; 3]);
        let end = point(&mut e, [0.0; 3]);
        // A signalling NaN for `up` is stored quiet; `-down` of a NaN flips its sign.
        let shape = e
            .call(
                0x00c5_5d50,
                &args![
                    start,
                    end,
                    1.0f32,
                    1.0f32,
                    0x7f80_0001u32,
                    0x7fc0_0002u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        assert_ne!(shape, 0);
        let log = e.call_log.take().unwrap();
        let vertices = calls(&log, TRI_SHAPE_CONSTRUCT)[0][2];
        assert_eq!(e.mem.u32(vertices + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(vertices + 0x20), 0xffc0_0002);
    }
}
