//! `Havok SDK/hkpshapecollection.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpShapeCollection` embeds a shape container (an `hkpShapeContainer` interface with
//! its own vtable) at +0x10. The container's virtual slots used here: `+8` first key,
//! `+0xC` next key (`-1` ends the walk), `+0x14` child shape of a key (key, scratch buffer)
//! and, on the child shape, `+0xC` (the supporting projection) and `+0x20` (the ray cast).
//!
//! Havok's profiling records (`RDTSC` time stamps appended to the thread's monitor stream,
//! found with `TlsGetValue` of the index at `0126835c`) are written as the exe does; the
//! engine has no model of `RDTSC`, so the host clock stands in for the time stamps.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// The end-of-timer label and the start labels of the two timed functions.
const LABEL_END: u32 = 0x010c_41f8;
const LABEL_SHAPE_COLLECT: u32 = 0x010c_e1f8;
const LABEL_MAXIMUM_PROJECTION: u32 = 0x010c_e228;
/// The `float` (about `-FLT_MAX`) a maximum starts from (`0104e148`).
const MINIMUM_FLOAT: u32 = 0x0104_e148;
/// The key that ends a walk of the container.
const NO_KEY: u32 = 0xffff_ffff;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d213f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape collection (no map name) that does nothing and takes two
/// arguments.
pub fn fn_00d213f0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) {}

// Translated from 00d21400 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape collection (no map name) that does nothing and takes one
/// argument.
pub fn fn_00d21400(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d21450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape collection (no map name; the timer record it writes is
/// labelled with the string `TtrcShpCollect` at `010ce1f8`): casts a ray through every child
/// the filter lets through and reports whether one hit. Bumps `output + 0x40`, then walks
/// the container's keys (`+8` first, `+0xC` next); for each key, when `input + 0x24` holds
/// a filter object, its slot 0 `(out hkBool, input, this, container, key)` must answer true;
/// then the child shape (`+0x14` of the container, with a scratch buffer) is asked slot
/// `0x20` `(out hit flag, input, output)` and a hit remembers the key. Afterwards the
/// counter is dropped again and the last hit key is stored at `output + 0x20 + 4 *
/// counter`. Writes `key != -1` to the byte `out` and returns `out`.
pub fn fn_00d21450(e: &mut Engine, this: Ptr, out: Ptr, input: Ptr, output: Ptr) -> Ptr {
    profile_record(e, LABEL_SHAPE_COLLECT);
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_add(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    let container = this.addr() + 0x10;
    let filter = e.mem.u32(input.addr() + 0x24);
    let hit_key = e.with_stack(0x240, |e, base| {
        // the function's stack frame: the hit flag byte, the remembered key, the filter's
        // answer byte and the child shape's scratch buffer
        let hit_flag = base.addr() + 0x13;
        let remembered = base.addr() + 0x14;
        let filter_answer = base.addr() + 0x1b;
        let buffer = base.addr() + 0x20;
        e.mem.set_u32(remembered, NO_KEY);
        let mut key = e.vcall(container, 8, &args![]).u32();
        while key != NO_KEY {
            let accepted = if filter == 0 {
                true
            } else {
                let answer = e
                    .vcall(
                        filter,
                        0,
                        &args![filter_answer, input, this, container, key],
                    )
                    .u32();
                e.mem.u8(answer) != 0
            };
            if accepted {
                let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
                e.vcall(child, 0x20, &args![hit_flag, input, output]);
                if e.mem.u8(hit_flag) != 0 {
                    e.mem.set_u32(remembered, key);
                }
            }
            key = e.vcall(container, 0xc, &args![key]).u32();
        }
        e.mem.u32(remembered)
    });
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_sub(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    if hit_key != NO_KEY {
        e.mem
            .set_u32(output.addr() + 0x20 + depth.wrapping_mul(4), hit_key);
    }
    profile_record(e, LABEL_END);
    e.mem.set_u8(out.addr(), u8::from(hit_key != NO_KEY));
    out
}

// Translated from 00d216e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollection::getMaximumProjection` (Xbox PDB): the largest of the children's
/// projections on `direction` (child slot `0xC`, result in `ST0`), starting from the
/// `float` at `0104e148`; a child replaces the maximum unless its value is below it
/// (so a NaN replaces it). Timed with the profiling records.
pub fn hkp_shape_collection_get_maximum_projection(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
) -> f32 {
    profile_record(e, LABEL_MAXIMUM_PROJECTION);
    let mut maximum = x87_quiet(e.global::<f32>(MINIMUM_FLOAT));
    let container = this.addr() + 0x10;
    e.with_stack(0x210, |e, buffer| {
        let mut key = e.vcall(container, 8, &args![]).u32();
        while key != NO_KEY {
            let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
            let projection = e.vcall(child, 0xc, &args![direction]).f32();
            if !(projection < maximum) {
                maximum = x87_quiet(projection);
            }
            key = e.vcall(container, 0xc, &args![key]).u32();
        }
    });
    profile_record(e, LABEL_END);
    maximum
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_13f0, fn_00d213f0(Ptr, u32, u32)),
        entry!(0x00d2_1400, fn_00d21400(Ptr, u32)),
        entry!(0x00d2_1450, fn_00d21450(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00d2_16e0,
            hkp_shape_collection_get_maximum_projection(Ptr, Ptr) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CONTAINER_VTABLE: u32 = 0x0a00_0000;
    const FIRST_KEY: u32 = 0x0a10_0001;
    const NEXT_KEY: u32 = 0x0a10_0002;
    const GET_CHILD: u32 = 0x0a10_0003;
    const CHILD_VTABLE: u32 = 0x0a00_1000;
    const CHILD_PROJECTION: u32 = 0x0a10_0011;
    const CHILD_CAST: u32 = 0x0a10_0012;
    const FILTER_VTABLE: u32 = 0x0a00_2000;
    const FILTER_SLOT: u32 = 0x0a10_0021;

    /// A collection of `children` keys `0..children` (keys are walked in order) with
    /// the profiling stream in place. Child `k` is one shared child object; the doubles
    /// below look at the key.
    fn rig(e: &mut Engine, children: u32) -> (u32, u32) {
        e.map(0x0104_e000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        let mut slots = vec![0u32; 8];
        slots[8 / 4] = FIRST_KEY;
        slots[0xc / 4] = NEXT_KEY;
        slots[0x14 / 4] = GET_CHILD;
        e.put_vtable(CONTAINER_VTABLE, &slots);
        let mut child_slots = vec![0u32; 0x10];
        child_slots[0xc / 4] = CHILD_PROJECTION;
        child_slots[0x20 / 4] = CHILD_CAST;
        e.put_vtable(CHILD_VTABLE, &child_slots);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, CONTAINER_VTABLE);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, CHILD_VTABLE);
        e.register_double(FIRST_KEY, move |_, _| Ret {
            eax: if children == 0 { NO_KEY } else { 0 },
            ..Ret::default()
        });
        // the next key needs the child count: keep it in a global of the double
        e.register_double(NEXT_KEY, move |_, a| Ret {
            eax: if a[1] + 1 < children {
                a[1] + 1
            } else {
                NO_KEY
            },
            ..Ret::default()
        });
        e.register_double(GET_CHILD, move |_, _| Ret {
            eax: child,
            ..Ret::default()
        });
        // profiling stream: cursor at +4, limit at +0xC
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0xe0);
        e.mem.set_u32(MONITOR_SLOT, 3);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (this, stream)
    }

    #[test]
    fn empty_methods_take_their_arguments_and_return() {
        let mut e = Engine::new();
        e.call(0x00d2_13f0, &args![1u32, 2u32, 3u32]);
        e.call(0x00d2_1400, &args![1u32, 2u32]);
    }

    #[test]
    fn maximum_projection_keeps_the_largest_and_records_the_timer() {
        let mut e = Engine::new();
        let (this, stream) = rig(&mut e, 3);
        e.mem.set_f32(MINIMUM_FLOAT, -3.4e38);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        let values = [2.0f32, 5.0, 3.0];
        let next = Rc::new(RefCell::new(0usize));
        e.register_double(CHILD_PROJECTION, move |_, a| {
            log.borrow_mut().push(a[1]);
            let mut n = next.borrow_mut();
            let value = values[*n];
            *n += 1;
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(0x10);
        let got = e.call(0x00d2_16e0, &args![this, direction]).f32();
        assert_eq!(got, 5.0);
        assert_eq!(*seen.borrow(), vec![direction; 3]);
        // two records: the start label and the end label
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20 + 24);
        assert_eq!(e.mem.u32(stream + 0x20), LABEL_MAXIMUM_PROJECTION);
        assert_eq!(e.mem.u32(stream + 0x2c), LABEL_END);
    }

    #[test]
    fn maximum_projection_without_children_is_the_starting_float() {
        let mut e = Engine::new();
        let (this, _) = rig(&mut e, 0);
        e.mem.set_f32(MINIMUM_FLOAT, -1.5);
        let direction = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d2_16e0, &args![this, direction]).f32(), -1.5);
    }

    #[test]
    fn maximum_projection_lets_a_nan_replace_the_maximum() {
        let mut e = Engine::new();
        let (this, _) = rig(&mut e, 2);
        e.mem.set_f32(MINIMUM_FLOAT, 0.0);
        let next = Rc::new(RefCell::new(0usize));
        e.register_double(CHILD_PROJECTION, move |_, _| {
            let mut n = next.borrow_mut();
            *n += 1;
            Ret {
                st0: if *n == 1 { 4.0 } else { f64::NAN },
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(0x10);
        assert!(e.call(0x00d2_16e0, &args![this, direction]).f32().is_nan());
    }

    #[test]
    fn walk_without_a_filter_remembers_the_last_hitting_key() {
        let mut e = Engine::new();
        let (this, stream) = rig(&mut e, 3);
        let input = e.mem.alloc(0x40);
        let output = e.mem.alloc(0x80);
        e.mem.set_u32(output + 0x40, 5);
        let casts = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&casts);
        let keys_seen = Rc::new(RefCell::new(0u32));
        e.register_double(CHILD_CAST, move |e, a| {
            // a = [child, hit flag, input, output]; the counter is already bumped
            log.borrow_mut().push((a[2], a[3], e.mem.u32(a[3] + 0x40)));
            let mut n = keys_seen.borrow_mut();
            // children 0 and 1 hit, child 2 does not
            e.mem.set_u8(a[1], u8::from(*n < 2));
            *n += 1;
            Ret::default()
        });
        let out = e.mem.alloc(4);
        let got = e.call(0x00d2_1450, &args![this, out, input, output]).u32();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(
            *casts.borrow(),
            vec![(input, output, 6), (input, output, 6), (input, output, 6)]
        );
        // the counter is back at 5 and the last hit key (1) sits at output + 0x20 + 4 * 5
        assert_eq!(e.mem.u32(output + 0x40), 5);
        assert_eq!(e.mem.u32(output + 0x20 + 20), 1);
        assert_eq!(e.mem.u32(stream + 0x20), LABEL_SHAPE_COLLECT);
        assert_eq!(e.mem.u32(stream + 0x2c), LABEL_END);
    }

    #[test]
    fn walk_with_a_filter_casts_only_through_accepted_keys() {
        let mut e = Engine::new();
        let (this, _) = rig(&mut e, 3);
        let mut slots = vec![0u32; 2];
        slots[0] = FILTER_SLOT;
        e.put_vtable(FILTER_VTABLE, &slots);
        let filter = e.mem.alloc(8);
        e.mem.set_u32(filter, FILTER_VTABLE);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input + 0x24, filter);
        let output = e.mem.alloc(0x80);
        let asked = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&asked);
        e.register_double(FILTER_SLOT, move |e, a| {
            // a = [filter, out, input, this, container, key]
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], u8::from(a[5] == 1));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let cast_keys = Rc::new(RefCell::new(0u32));
        let count = Rc::clone(&cast_keys);
        e.register_double(CHILD_CAST, move |e, a| {
            *count.borrow_mut() += 1;
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        e.call(0x00d2_1450, &args![this, out, input, output]);
        let asked = asked.borrow();
        assert_eq!(asked.len(), 3);
        assert_eq!(asked[0][2..], [input, this, this + 0x10, 0]);
        assert_eq!(*cast_keys.borrow(), 1);
        // no child hit: the result byte is 0 and nothing is stored in the output
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u32(output + 0x20), 0);
        assert_eq!(e.mem.u32(output + 0x40), 0);
    }
}
