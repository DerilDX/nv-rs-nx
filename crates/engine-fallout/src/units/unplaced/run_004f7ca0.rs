//! `(unplaced)/run_004f7ca0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions work on one object whose first eight bytes are the head
//! node of a list of points (node: item at +0, next at +4), with a byte at
//! +0xC that says the list owns its items, the cached bounds at +0x10
//! (smallest of the points' first coordinate), +0x14 (smallest second),
//! +0x18 (largest first), +0x1C (largest second), and the point count at
//! +0x24. The two coordinate getters are `006a7f50` and `006b9130` (each a
//! float in ST0). Together they form a polygon: bounds, a point-inside test
//! and a test over every four consecutive points (`004f7310`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0059bb30` (named `D3DTexture_LockRect` in the engine map, a folded
/// body): the word at +0x24 of its `this` (the point count).
const COUNT_0059BB30: u32 = 0x0059_bb30;
/// `006815c0` (unnamed in the engine map): returns its `this` (a node; the
/// item is its first word).
const NODE_ITEM_006815C0: u32 = 0x0068_15c0;
/// `00726070` (unnamed in the engine map): the node's next (word at +4).
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
/// `0063f7b0` (unnamed in the engine map): removes the head node's item
/// from the list.
const REMOVE_HEAD_0063F7B0: u32 = 0x0063_f7b0;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `006a7f50` (unnamed in the engine map): a point's first coordinate (ST0).
const POINT_FIRST_006A7F50: u32 = 0x006a_7f50;
/// `006b9130` (unnamed in the engine map): a point's second coordinate (ST0).
const POINT_SECOND_006B9130: u32 = 0x006b_9130;
/// `004f72a0` (unnamed in the engine map; cdecl, two points and a float):
/// the first coordinate where the edge between the two points meets the
/// given second coordinate (ST0); used by the inside test.
const EDGE_CROSSING_004F72A0: u32 = 0x004f_72a0;
/// `004f7310` (unnamed in the engine map; cdecl, four points): the test
/// run on each four consecutive points.
const FOUR_POINT_TEST_004F7310: u32 = 0x004f_7310;
/// `FLT_MAX` (float), the bound a cleared smallest coordinate starts at.
const FLOAT_MAX: u32 = 0x0101_6970;
/// `-FLT_MAX` (float), the bound a cleared largest coordinate starts at.
const FLOAT_LOWEST: u32 = 0x0101_5f5c;
/// `FLT_MAX` as a double: a cached smallest bound below it is valid.
const DOUBLE_FLOAT_MAX: u32 = 0x0102_31b0;
/// `-FLT_MAX` as a double: a cached largest bound above it is valid.
const DOUBLE_FLOAT_LOWEST: u32 = 0x0102_41b0;

fn count(e: &mut Engine, this: Ptr) -> u32 {
    e.call(COUNT_0059BB30, &args![this]).u32()
}

fn item_of(e: &mut Engine, node: u32) -> u32 {
    let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
    e.mem.u32(item)
}

fn next_of(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_FOLLOWING_00726070, &args![node]).u32()
}

/// A coordinate getter's ST0 result as `FSTP dword` leaves it, read back
/// with `FLD dword`.
fn coordinate(e: &mut Engine, getter: u32, point: u32) -> f64 {
    let value = e.call(getter, &args![point]).f64();
    x87_load32(x87_store32(value))
}

/// The shared body of the four bound getters: the cached bound at `offset`
/// when it is valid (below `FLT_MAX` for a smallest bound, above `-FLT_MAX`
/// for a largest one), else the extreme of `getter` over the list's items,
/// from `start` (the cache is not written). Each improvement calls the
/// getter a second time for the value it stores, as the exe does.
fn bound(e: &mut Engine, this: Ptr, offset: u32, getter: u32, smallest: bool) -> f64 {
    let (start, limit) = if smallest {
        (FLOAT_MAX, DOUBLE_FLOAT_MAX)
    } else {
        (FLOAT_LOWEST, DOUBLE_FLOAT_LOWEST)
    };
    let mut best = x87_store32(x87_load32(e.global::<u32>(start)));
    let cached = x87_load32(e.mem.u32(this.addr() + offset));
    let limit = e.global::<f64>(limit);
    let valid = if smallest {
        cached < limit
    } else {
        cached > limit
    };
    if valid {
        return cached;
    }
    let mut node = this.addr();
    while node != 0 && item_of(e, node) != 0 {
        let point = item_of(e, node);
        let value = e.call(getter, &args![point]).f64();
        let current = x87_load32(best);
        let better = if smallest {
            current > value
        } else {
            current < value
        };
        if better {
            let point = item_of(e, node);
            let value = e.call(getter, &args![point]).f64();
            best = x87_store32(value);
        }
        node = next_of(e, node);
    }
    x87_load32(best)
}

// Translated from 004f7dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the point list. While the count
/// (`0059bb30`) is non-zero: takes the head item, removes it
/// (`0063f7b0`), decrements the count at +0x24, and frees the item when it
/// is non-null and the owning byte at +0xC is set. Then the count is
/// cleared and the bounds reset: +0x14 and +0x10 to `FLT_MAX`, +0x1C and
/// +0x18 to `-FLT_MAX` (copied through the FPU, in that order).
pub fn fn_004f7dd0(e: &mut Engine, this: Ptr) {
    while count(e, this) != 0 {
        let item = item_of(e, this.addr());
        e.call(REMOVE_HEAD_0063F7B0, &args![this]);
        let remaining = e.mem.u32(this.addr() + 0x24).wrapping_sub(1);
        e.mem.set_u32(this.addr() + 0x24, remaining);
        if item != 0 && e.mem.u8(this.addr() + 0xc) != 0 {
            e.call(DEALLOCATE, &args![item]);
        }
    }
    e.mem.set_u32(this.addr() + 0x24, 0);
    let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
    e.mem.set_u32(this.addr() + 0x14, max);
    let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
    e.mem.set_u32(this.addr() + 0x10, max);
    let lowest = x87_quiet_bits(e.global::<u32>(FLOAT_LOWEST));
    e.mem.set_u32(this.addr() + 0x1c, lowest);
    let lowest = x87_quiet_bits(e.global::<u32>(FLOAT_LOWEST));
    e.mem.set_u32(this.addr() + 0x18, lowest);
}

// Translated from 004f7e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the smallest first coordinate (ST0): the
/// cached +0x10 when below `FLT_MAX`, else the minimum of `006a7f50` over
/// the items (`FLT_MAX` for none).
pub fn fn_004f7e80(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x10, POINT_FIRST_006A7F50, true)
}

// Translated from 004f7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the largest first coordinate (ST0): the
/// cached +0x18 when above `-FLT_MAX`, else the maximum of `006a7f50` over
/// the items (`-FLT_MAX` for none).
pub fn fn_004f7f10(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x18, POINT_FIRST_006A7F50, false)
}

// Translated from 004f7fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the smallest second coordinate (ST0): the
/// cached +0x14 when below `FLT_MAX`, else the minimum of `006b9130` over
/// the items.
pub fn fn_004f7fa0(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x14, POINT_SECOND_006B9130, true)
}

// Translated from 004f8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the largest second coordinate (ST0): the
/// cached +0x1C when above `-FLT_MAX`, else the maximum of `006b9130` over
/// the items.
pub fn fn_004f8030(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x1c, POINT_SECOND_006B9130, false)
}

// Translated from 004f81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): with `check_count` set
/// and fewer than 3 points, false; otherwise true exactly when the
/// four-point walk `004f8220` (with its byte argument 1) finds nothing.
pub fn fn_004f81e0(e: &mut Engine, this: Ptr, check_count: u8) -> bool {
    if check_count != 0 && count(e, this) < 3 {
        return false;
    }
    !fn_004f8220(e, this, 1)
}

// Translated from 004f8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): false with fewer than 4
/// points. Otherwise, for each first node `a` with second node `b` (a node
/// that has a next), and each later pair `c`, `d` after `b` (`d` moving on
/// through the list; at the end of the list, with `wrap` set and `a` not
/// the head, `d` becomes the head once), `004f7310(a, b, c, d)` on the four
/// items (all four nodes non-null) answering true makes the result true.
/// After the inner walk, `a` moves to `b`. False when nothing answered.
pub fn fn_004f8220(e: &mut Engine, this: Ptr, wrap: u8) -> bool {
    if count(e, this) < 4 {
        return false;
    }
    let head = this.addr();
    let mut first = head;
    let mut second = next_of(e, first);
    let third = next_of(e, second);
    let _ = next_of(e, third);
    loop {
        if next_of(e, second) == 0 {
            return false;
        }
        let mut third = next_of(e, second);
        let mut fourth = if third != 0 { next_of(e, third) } else { 0 };
        loop {
            let mut last = false;
            if fourth == 0 {
                if wrap != 0 && first != head {
                    fourth = head;
                }
                last = true;
            }
            if first != 0 && second != 0 && third != 0 && fourth != 0 {
                let d = item_of(e, fourth);
                let c = item_of(e, third);
                let b = item_of(e, second);
                let a = item_of(e, first);
                if e.call(FOUR_POINT_TEST_004F7310, &args![a, b, c, d]).u8() != 0 {
                    return true;
                }
            }
            third = fourth;
            if fourth != 0 {
                fourth = next_of(e, fourth);
            }
            if last {
                break;
            }
        }
        first = second;
        second = next_of(e, second);
    }
}

// Translated from 004f8360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `point` lies
/// inside the polygon. False for a null point, fewer than 3 points, or a
/// point outside the bounds (first coordinate below `004f7e80` or above
/// `004f7f10`, second below `004f7fa0` or above `004f8030`). Otherwise each
/// edge (each node and its next, the last closing to the head) is a
/// candidate when the point's first coordinate is below both ends' first
/// coordinates, or below one of them and below the edge's crossing
/// (`004f72a0` at the point's second coordinate). A candidate counts when
/// the point's second coordinate is below the first end's and the second
/// end's is not above it, or is not below the first end's (ordered) and the
/// second end's is above it. An odd count is inside.
pub fn fn_004f8360(e: &mut Engine, this: Ptr, point: u32) -> bool {
    if point == 0 || count(e, this) < 3 {
        return false;
    }
    let px = coordinate(e, POINT_FIRST_006A7F50, point);
    let py_bits = x87_store32(e.call(POINT_SECOND_006B9130, &args![point]).f64());
    let py = x87_load32(py_bits);
    if px < fn_004f7e80(e, this) {
        return false;
    }
    if px > fn_004f7f10(e, this) {
        return false;
    }
    if py < fn_004f7fa0(e, this) {
        return false;
    }
    if py > fn_004f8030(e, this) {
        return false;
    }
    let head = this.addr();
    let mut crossings: i32 = 0;
    let mut node = head;
    loop {
        let mut next = next_of(e, node);
        if next == 0 {
            next = head;
        }
        let a = item_of(e, node);
        let b = item_of(e, next);
        let ax = coordinate(e, POINT_FIRST_006A7F50, a);
        let bx = coordinate(e, POINT_FIRST_006A7F50, b);
        let hit = if px < ax && px < bx {
            true
        } else if px < ax || px < bx {
            let pushed = x87_store32(x87_load32(py_bits));
            let crossing = e.call(EDGE_CROSSING_004F72A0, &args![a, b, pushed]).f64();
            px < crossing
        } else {
            false
        };
        if hit {
            let ay = coordinate(e, POINT_SECOND_006B9130, a);
            let by = coordinate(e, POINT_SECOND_006B9130, b);
            if (py < ay && by <= py) || (py >= ay && by > py) {
                crossings = crossings.wrapping_add(1);
            }
        }
        node = next;
        if node == head {
            break;
        }
    }
    crossings % 2 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_7dd0, fn_004f7dd0(Ptr)),
        entry!(0x004f_7e80, fn_004f7e80(Ptr) -> f64),
        entry!(0x004f_7f10, fn_004f7f10(Ptr) -> f64),
        entry!(0x004f_7fa0, fn_004f7fa0(Ptr) -> f64),
        entry!(0x004f_8030, fn_004f8030(Ptr) -> f64),
        entry!(0x004f_81e0, fn_004f81e0(Ptr, u8) -> bool),
        entry!(0x004f_8220, fn_004f8220(Ptr, u8) -> bool),
        entry!(0x004f_8360, fn_004f8360(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the four constants and the list and point doubles.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1_0000);
        e.set_global::<f32>(FLOAT_MAX, f32::MAX);
        e.set_global::<f32>(FLOAT_LOWEST, f32::MIN);
        e.set_global::<f64>(DOUBLE_FLOAT_MAX, f64::from(f32::MAX));
        e.set_global::<f64>(DOUBLE_FLOAT_LOWEST, f64::from(f32::MIN));
        e.register(COUNT_0059BB30, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(NODE_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(POINT_FIRST_006A7F50, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(POINT_SECOND_006B9130, |e, a| e.mem.f32(a[0] + 4).into_ret());
        e
    }

    /// A polygon object over `points` (the head node is the object
    /// itself), bounds cleared, count set.
    fn polygon(e: &mut Engine, points: &[(f32, f32)]) -> u32 {
        let this = e.mem.alloc(0x28);
        let mut node = this;
        for (i, (x, y)) in points.iter().enumerate() {
            let point = e.mem.alloc(8);
            e.mem.set_f32(point, *x);
            e.mem.set_f32(point + 4, *y);
            e.mem.set_u32(node, point);
            if i + 1 < points.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.mem.set_f32(this + 0x10, f32::MAX);
        e.mem.set_f32(this + 0x14, f32::MAX);
        e.mem.set_f32(this + 0x18, f32::MIN);
        e.mem.set_f32(this + 0x1c, f32::MIN);
        e.mem.set_u32(this + 0x24, points.len() as u32);
        this
    }

    #[test]
    fn at_004f7dd0_empties_the_list_and_resets_the_bounds() {
        let mut e = engine();
        e.register(REMOVE_HEAD_0063F7B0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = polygon(&mut e, &[(1.0, 2.0), (3.0, 4.0)]);
        let first = e.mem.u32(this);
        e.mem.set_f32(this + 0x10, 5.0);
        e.mem.set_u8(this + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_7dd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.f32(this + 0x10), f32::MAX);
        assert_eq!(e.mem.f32(this + 0x14), f32::MAX);
        assert_eq!(e.mem.f32(this + 0x18), f32::MIN);
        assert_eq!(e.mem.f32(this + 0x1c), f32::MIN);
        let freed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == DEALLOCATE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(freed.len(), 2);
        assert_eq!(freed[0], first);

        // Not owning: nothing freed.
        let this = polygon(&mut e, &[(1.0, 2.0)]);
        e.call_log = Some(vec![]);
        e.call(0x004f_7dd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != DEALLOCATE));
    }

    #[test]
    fn at_004f7e80_family_returns_cache_or_extremes() {
        let mut e = engine();
        let this = polygon(&mut e, &[(3.0, -2.0), (-1.5, 7.0), (2.0, 0.5)]);
        assert_eq!(e.call(0x004f_7e80, &args![this]).f64(), -1.5);
        assert_eq!(e.call(0x004f_7f10, &args![this]).f64(), 3.0);
        assert_eq!(e.call(0x004f_7fa0, &args![this]).f64(), -2.0);
        assert_eq!(e.call(0x004f_8030, &args![this]).f64(), 7.0);
        // Valid caches win.
        e.mem.set_f32(this + 0x10, 10.0);
        e.mem.set_f32(this + 0x18, -10.0);
        e.mem.set_f32(this + 0x14, 11.0);
        e.mem.set_f32(this + 0x1c, -11.0);
        assert_eq!(e.call(0x004f_7e80, &args![this]).f64(), 10.0);
        assert_eq!(e.call(0x004f_7f10, &args![this]).f64(), -10.0);
        assert_eq!(e.call(0x004f_7fa0, &args![this]).f64(), 11.0);
        assert_eq!(e.call(0x004f_8030, &args![this]).f64(), -11.0);
        // A NaN cache is not valid: the walk runs.
        e.mem.set_u32(this + 0x10, 0x7fc0_0000);
        assert_eq!(e.call(0x004f_7e80, &args![this]).f64(), -1.5);
        // An empty list gives the start values.
        let empty = polygon(&mut e, &[]);
        assert_eq!(
            e.call(0x004f_7e80, &args![empty]).f64(),
            f64::from(f32::MAX)
        );
        assert_eq!(
            e.call(0x004f_8030, &args![empty]).f64(),
            f64::from(f32::MIN)
        );
    }

    #[test]
    fn at_004f7e80_calls_the_getter_again_for_each_improvement() {
        let mut e = engine();
        let this = polygon(&mut e, &[(3.0, 0.0), (1.0, 0.0), (2.0, 0.0)]);
        e.call_log = Some(vec![]);
        e.call(0x004f_7e80, &args![this]);
        let log = e.call_log.take().unwrap();
        // Improvements at 3.0 and 1.0: 3 reads + 2 rereads.
        assert_eq!(
            log.iter().filter(|c| c.0 == POINT_FIRST_006A7F50).count(),
            5
        );
    }

    #[test]
    fn at_004f8220_needs_four_points_and_tests_each_window() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| Ret::default());
        let small = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8220, &args![small, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FOUR_POINT_TEST_004F7310));

        let this = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let items: Vec<u32> = {
            let mut v = vec![];
            let mut node = this;
            while node != 0 {
                v.push(e.mem.u32(node));
                node = e.mem.u32(node + 4);
            }
            v
        };
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8220, &args![this, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        let windows: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == FOUR_POINT_TEST_004F7310)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(windows, vec![vec![items[0], items[1], items[2], items[3]]]);

        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8220, &args![this, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        let windows: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == FOUR_POINT_TEST_004F7310)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            windows,
            vec![
                vec![items[0], items[1], items[2], items[3]],
                vec![items[1], items[2], items[3], items[0]],
            ]
        );
    }

    #[test]
    fn at_004f8220_returns_true_on_the_first_hit() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| 1u32.into_ret());
        let this = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_8220, &args![this, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == FOUR_POINT_TEST_004F7310)
                .count(),
            1
        );
    }

    #[test]
    fn at_004f81e0_checks_the_count_then_inverts_the_walk() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| Ret::default());
        let three = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
        assert!(e.call(0x004f_81e0, &args![three, 1u32]).bool());
        let two = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0)]);
        assert!(!e.call(0x004f_81e0, &args![two, 1u32]).bool());
        assert!(e.call(0x004f_81e0, &args![two, 0u32]).bool());
        e.register(FOUR_POINT_TEST_004F7310, |_, _| 1u32.into_ret());
        let four = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        assert!(!e.call(0x004f_81e0, &args![four, 0u32]).bool());
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn at_004f8360_tests_points_against_a_square() {
        let mut e = engine();
        // The crossing of the edge a-b at second coordinate y.
        e.register(EDGE_CROSSING_004F72A0, |e, a| {
            let (ax, ay) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            let (bx, by) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let y = f32::from_bits(a[2]);
            (ax + (y - ay) * (bx - ax) / (by - ay)).into_ret()
        });
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let inside = point(&mut e, 1.0, 2.0);
        assert!(e.call(0x004f_8360, &args![this, inside]).bool());
        let outside = point(&mut e, 5.0, 2.0);
        assert!(!e.call(0x004f_8360, &args![this, outside]).bool());
        let below = point(&mut e, 1.0, -1.0);
        assert!(!e.call(0x004f_8360, &args![this, below]).bool());
        assert!(!e.call(0x004f_8360, &args![this, 0u32]).bool());
        let two = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0)]);
        assert!(!e.call(0x004f_8360, &args![two, inside]).bool());
    }

    #[test]
    fn at_004f8360_asks_for_the_crossing_of_a_slanted_edge() {
        let mut e = engine();
        e.register(EDGE_CROSSING_004F72A0, |e, a| {
            let (ax, ay) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            let (bx, by) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let y = f32::from_bits(a[2]);
            (ax + (y - ay) * (bx - ax) / (by - ay)).into_ret()
        });
        // A triangle; the point is right of the slanted edge's crossing.
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (0.0, 4.0)]);
        let inside = point(&mut e, 1.0, 1.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_8360, &args![this, inside]).bool());
        let log = e.call_log.take().unwrap();
        let crossings: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EDGE_CROSSING_004F72A0)
            .collect();
        assert!(!crossings.is_empty());
        assert_eq!(crossings[0].1[2], 1.0f32.to_bits());
        let outside = point(&mut e, 3.0, 3.0);
        assert!(!e.call(0x004f_8360, &args![this, outside]).bool());
    }
}
