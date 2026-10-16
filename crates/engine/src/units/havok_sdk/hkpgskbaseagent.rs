//! `Havok SDK/hkpgskbaseagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Timer labels: the start (`010cf22c`) and the end (`010c41f8`) of `staticGetClosestPoints`.
const LABEL_START: u32 = 0x010c_f22c;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkTransform::setMulInverseMul` (Xbox PDB, `00cb22f0`, `this` = result, `(a, b)`).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// The two builders of the GSK cache (`00dafe70` when the second shape's type word at +0xc is
/// 3, else `00dafcd0`; `this` = the cache, `(shape a, shape b, transform)`; `RET 0xc`).
const GSK_INIT_TYPE_3: u32 = 0x00da_fe70;
const GSK_INIT_OTHER: u32 = 0x00da_fcd0;
/// The GSK solver (`this` = a local GSK object): `00daad40(shape a, shape b, transform,
/// result)` returns non-zero for no result, `00da9880(result, out)` turns a result into the
/// closest points and `00da9790(cache)` refreshes the cache.
const GSK_SOLVE: u32 = 0x00da_ad40;
const GSK_CLOSEST_POINTS: u32 = 0x00da_9880;
const GSK_REFRESH_CACHE: u32 = 0x00da_9790;
/// `hkVector4::setRotatedDir` (Xbox PDB name unconfirmed, `00c7f700`, `this` = result,
/// `(matrix, vector)`).
const ROTATE_VECTOR: u32 = 0x00c7_f700;
/// The `-1.0` float in the exe image (`01012054`).
const MINUS_ONE: u32 = 0x0101_2054;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte timer record `(label, time stamp)` to the thread's monitor stream while
/// its cursor (+4) is below its limit (+0xC).
fn monitor_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 come from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: [f32; 4], src: [f32; 4], imm: u32) -> [f32; 4] {
    [
        dst[(imm & 3) as usize],
        dst[((imm >> 2) & 3) as usize],
        src[((imm >> 4) & 3) as usize],
        src[((imm >> 6) & 3) as usize],
    ]
}

/// `UNPCKLPS dst, src`.
fn unpcklps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [dst[0], src[0], dst[1], src[1]]
}

/// `UNPCKHPS dst, src`.
fn unpckhps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [dst[2], src[2], dst[3], src[3]]
}

/// `MOVLHPS dst, src`: the low half of `src` becomes the high half of `dst`.
fn movlhps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [dst[0], dst[1], src[0], src[1]]
}

/// `MOVHLPS dst, src`: the high half of `src` becomes the low half of `dst`.
fn movhlps(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [src[2], src[3], dst[2], dst[3]]
}

// Translated from 00dabf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::hkpGskBaseAgent` (Xbox PDB; the vtable `010d51cc` it stores has the RTTI
/// name `hkpGskBaseAgent`). `this` is ECX; `body_a` and `body_b` are collision bodies (shape
/// at +0, motion at +8, parent chain at +0xc); `third` is stored at `this + 8`. Sets the
/// 16-bit word at +6 to 1, builds the relative transform of the two motions (`00cb22f0`) and
/// the GSK cache at `this + 0xc` (`00dafe70` when the second shape's word at +0xc is 3,
/// otherwise `00dafcd0`), clears the 16 bytes at +0x20, stores the `-1.0` constant at +0x2c and
/// +0x18, and puts at +0x1c the smaller of the floats at +0x4c of the root bodies of the two
/// chains (the second wins unless it is above the first, and when the compare is unordered).
/// Returns `this`.
pub fn hkp_gsk_base_agent_hkp_gsk_base_agent(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    third: u32,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, third);
    e.mem.set_u32(t, 0x010d_51cc);
    let shape_b = e.mem.u32(body_b.addr());
    let motion_b = e.mem.u32(body_b.addr() + 8);
    let motion_a = e.mem.u32(body_a.addr() + 8);
    let shape_a = e.mem.u32(body_a.addr());
    e.with_stack(0x40, |e, transform| {
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, motion_a, motion_b],
        );
        let init = if e.mem.u32(shape_b + 0xc) == 3 {
            GSK_INIT_TYPE_3
        } else {
            GSK_INIT_OTHER
        };
        e.call(init, &args![t + 0xc, shape_a, shape_b, transform]);
    });
    let minus_one = x87_quiet(e.mem.f32(MINUS_ONE));
    set_lanes(e, t + 0x20, [0.0; 4]);
    e.mem.set_f32(t + 0x2c, minus_one);
    e.mem.set_f32(t + 0x18, minus_one);
    let mut root_a = body_a.addr();
    while e.mem.u32(root_a + 0xc) != 0 {
        root_a = e.mem.u32(root_a + 0xc);
    }
    let first = x87_quiet(e.mem.f32(root_a + 0x4c));
    let mut root_b = body_b.addr();
    while e.mem.u32(root_b + 0xc) != 0 {
        root_b = e.mem.u32(root_b + 0xc);
    }
    let second = x87_quiet(e.mem.f32(root_b + 0x4c));
    let smaller = if f64::from(second) > f64::from(first) {
        first
    } else {
        second
    };
    e.mem.set_f32(t + 0x1c, smaller);
    this
}

// Translated from 00dac450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskBaseAgent::staticGetClosestPoints` (Xbox PDB), cdecl
/// `(body_a, body_b, input, collector)`; the roles are described from their use. Brackets the
/// work with timer records (start and end labels in the monitor stream). Builds the relative
/// transform of the two bodies' motions (`00cb22f0`), the GSK cache (`00dafe70` or `00dafcd0`,
/// as the constructor does) and asks both shapes (vtable offset `0x3c`) to convert the cache's
/// vertex ids into vertices; initialises a local GSK object from the cache's counts and
/// runs the solver (`00daad40`); when it succeeds, `00da9880` gives the closest points, which
/// are turned into body A's space (`00c7f700`). When the gap (the distance minus both shape
/// radii, the floats at +0x10 of the shapes) is below the float at `input + 8`, the contact
/// point and normal are put in body-local form, optionally refined by the second shape's
/// virtual function at offset `0x40` (only when its type word at +0xc is 3 and the word at
/// `input + 4` is set; a result of 0 drops the contact, 1 replaces the normal), and passed
/// to the collector's virtual function at offset 4 as a 16-float record (point, normal with
/// the gap in its last lane, the two bodies).
pub fn hkp_gsk_base_agent_static_get_closest_points(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    monitor_record(e, LABEL_START);
    let (a, b) = (body_a.addr(), body_b.addr());
    let motion_a = e.mem.u32(a + 8);
    let motion_b = e.mem.u32(b + 8);
    let shape_a = e.mem.u32(a);
    let shape_b = e.mem.u32(b);
    e.with_stack(0x290, |e, frame| {
        let x = frame.addr();
        e.mem.set_u32(x + 0xc, shape_a);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![x + 0x80, motion_a, motion_b],
        );
        let init = if e.mem.u32(shape_b + 0xc) == 3 {
            GSK_INIT_TYPE_3
        } else {
            GSK_INIT_OTHER
        };
        e.call(init, &args![x + 0x10, shape_a, shape_b, x + 0x80]);
        let count_a = u32::from(e.mem.u8(x + 0x18));
        let count_b = u32::from(e.mem.u8(x + 0x19));
        let flags = u32::from(e.mem.u8(x + 0x1a));
        e.mem.set_u32(x + 0x104, count_b);
        e.mem.set_u32(x + 0x108, flags & 0xf);
        e.mem.set_u32(x + 0x100, count_a);
        e.mem.set_u32(x + 0x10c, flags >> 4);
        e.mem.set_u32(x + 0x110, 0);
        e.mem.set_u32(x + 0x114, 0);
        e.vcall(shape_a, 0x3c, &args![x + 0x10, count_a, x + 0x120]);
        let second_ids = x + 0x10 + count_a * 2;
        e.vcall(shape_b, 0x3c, &args![second_ids, count_b, x + 0x1e0]);
        let failed = e
            .call(
                GSK_SOLVE,
                &args![x + 0x100, shape_a, shape_b, x + 0x80, x + 0xc0],
            )
            .u32();
        if failed == 0 {
            e.call(GSK_CLOSEST_POINTS, &args![x + 0x100, x + 0xc0, x + 0x30]);
            e.call(ROTATE_VECTOR, &args![x + 0x70, motion_a, x + 0x30]);
            let radius_a = e.mem.f32(shape_a + 0x10);
            let radius_b = e.mem.f32(shape_b + 0x10);
            let raw = e.mem.f32(x + 0x50);
            let gap = ((f64::from(raw) - f64::from(radius_a)) - f64::from(radius_b)) as f32;
            e.mem.set_f32(x + 0xc, gap);
            let gap = e.mem.f32(x + 0xc);
            e.mem.set_f32(x + 0x50, gap);
            let tolerance = e.mem.f32(input.addr() + 8);
            if f64::from(tolerance) > f64::from(gap) {
                emit_contact(
                    e,
                    x,
                    a,
                    b,
                    input.addr(),
                    collector.addr(),
                    shape_b,
                    gap,
                    radius_a,
                );
            }
        }
    });
    monitor_record(e, LABEL_END);
}

/// The tail of `staticGetClosestPoints` after the gap test (the part of the exe from
/// `00dac5e9`): `x` is the frame, `gap` the stored gap and `radius_a` the first shape's
/// radius.
#[allow(clippy::too_many_arguments)]
fn emit_contact(
    e: &mut Engine,
    x: u32,
    a: u32,
    b: u32,
    input: u32,
    collector: u32,
    shape_b: u32,
    gap: f32,
    radius_a: f32,
) {
    let normal = lanes(e, x + 0x30);
    let point = lanes(e, x + 0x40);
    let negative_radius = -x87_quiet(radius_a);
    let moved = [0, 1, 2, 3].map(|i| negative_radius * normal[i] + point[i]);
    set_lanes(e, x + 0x40, moved);
    let negative_gap = -gap;
    let local = [0, 1, 2, 3].map(|i| negative_gap * normal[i] + moved[i]);
    let offset = lanes(e, x + 0xb0);
    let delta = [0, 1, 2, 3].map(|i| local[i] - offset[i]);
    let (m0, m1, m2) = (lanes(e, x + 0x80), lanes(e, x + 0x90), lanes(e, x + 0xa0));
    let low = unpcklps(m0, m1);
    let high = unpckhps(m0, m1);
    let column0 = movlhps(low, m2);
    let column1 = shufps(movhlps(column0, low), m2, 0xd4);
    let column2 = shufps(high, m2, 0xe4);
    let by_y = shufps(delta, delta, 0x55);
    let by_x = shufps(delta, delta, 0x00);
    let by_z = shufps(delta, delta, 0xaa);
    let mut world = [0.0f32; 4];
    for i in 0..4 {
        world[i] = (by_y[i] * column1[i] + by_x[i] * column0[i]) + column2[i] * by_z[i];
    }
    set_lanes(e, x + 0x60, world);
    if e.mem.u32(shape_b + 0xc) == 3 && e.mem.u32(input + 4) != 0 {
        if e.mem.u32(x + 0x114) != 0 {
            e.call(GSK_REFRESH_CACHE, &args![x + 0x100, x + 0x10]);
        }
        let direction = lanes(e, x + 0x70);
        set_lanes(e, x + 0x20, direction);
        let second_motion = e.mem.u32(b + 8);
        let first_motion = e.mem.u32(a + 8);
        let count_a = u32::from(e.mem.u8(x + 0x18));
        let second_ids = x + 0x10 + count_a * 2;
        e.mem.set_u32(x + 0xc, second_motion);
        e.mem.set_u32(x + 0x1c, second_ids);
        let verdict = e
            .vcall(
                shape_b,
                0x40,
                &args![
                    second_ids,
                    x + 0x19,
                    x + 0x60,
                    second_motion,
                    e.mem.u32(a),
                    first_motion,
                    x + 0x20
                ],
            )
            .u32();
        if verdict == 0 {
            return;
        }
        if verdict == 1 {
            let refined = lanes(e, x + 0x20);
            set_lanes(e, x + 0x70, refined);
        }
    }
    let world = lanes(e, x + 0x60);
    let second_motion = e.mem.u32(b + 8);
    let rows = [
        lanes(e, second_motion),
        lanes(e, second_motion + 0x10),
        lanes(e, second_motion + 0x20),
        lanes(e, second_motion + 0x30),
    ];
    let spread_y = shufps(world, world, 0x55);
    let spread_x = shufps(world, world, 0x00);
    let spread_z = shufps(world, world, 0xaa);
    let mut moved_point = [0.0f32; 4];
    for i in 0..4 {
        let sum = (spread_y[i] * rows[1][i] + spread_x[i] * rows[0][i])
            + (spread_z[i] * rows[2][i] + rows[3][i]);
        moved_point[i] = sum;
    }
    set_lanes(e, x + 0xd0, moved_point);
    let normal = lanes(e, x + 0x70);
    set_lanes(e, x + 0xe0, normal);
    e.mem.set_f32(x + 0xec, x87_quiet(e.mem.f32(x + 0x50)));
    e.mem.set_u32(x + 0xf0, a);
    e.mem.set_u32(x + 0xf4, b);
    let table = e.mem.u32(collector);
    let callback = e.mem.u32(table + 4);
    e.call(callback, &args![collector, x + 0xd0]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_bf10, hkp_gsk_base_agent_hkp_gsk_base_agent(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00da_c450,
            hkp_gsk_base_agent_static_get_closest_points(Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// A body: shape at +0 (type word at shape + 0xc), motion at +8, parent at +0xc and the
    /// float at +0x4c.
    fn body(e: &mut Engine, shape_type: u32, motion: u32, parent: u32, value: f32) -> u32 {
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, shape_type);
        let b = e.mem.alloc(0x60);
        e.mem.set_u32(b, shape);
        e.mem.set_u32(b + 8, motion);
        e.mem.set_u32(b + 0xc, parent);
        e.mem.set_f32(b + 0x4c, value);
        b
    }

    #[test]
    fn the_constructor_builds_the_cache_and_takes_the_smaller_root_value() {
        let mut e = test_engine();
        e.mem.set_f32(MINUS_ONE, -1.0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, GSK_INIT_TYPE_3, 0);
        log_double(&mut e, &log, GSK_INIT_OTHER, 0);
        let root_a = body(&mut e, 1, 0x100, 0, 5.0);
        let a = body(&mut e, 1, 0x1000, root_a, 99.0);
        let root_b = body(&mut e, 1, 0x200, 0, 2.0);
        let b = body(&mut e, 3, 0x2000, root_b, 99.0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, 0xdead);
        let got = e.call(0x00da_bf10, &args![this, a, b, 0x77u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010d_51cc);
        assert_eq!(e.mem.u32(this + 8), 0x77);
        assert_eq!(lanes(&e, this + 0x20), [0.0, 0.0, 0.0, -1.0]);
        assert_eq!(e.mem.f32(this + 0x18), -1.0);
        // The second root's value (2.0) is below the first's (5.0).
        assert_eq!(e.mem.f32(this + 0x1c), 2.0);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1..], [0x1000, 0x2000]);
        // The second shape has type 3, so the type-3 builder runs with (cache, shape a, shape b).
        assert_eq!(calls[1].0, GSK_INIT_TYPE_3);
        assert_eq!(calls[1].1[0], this + 0xc);
        assert_eq!(calls[1].1[1], e.mem.u32(a));
        assert_eq!(calls[1].1[2], e.mem.u32(b));
        assert_eq!(calls.len(), 2);
        // With the first value smaller, it wins instead.
        drop(calls);
        e.mem.set_f32(root_a + 0x4c, 1.0);
        let other = body(&mut e, 4, 0x2000, root_b, 0.0);
        e.call(0x00da_bf10, &args![this, a, other, 0x77u32]);
        assert_eq!(e.mem.f32(this + 0x1c), 1.0);
        assert_eq!(log.borrow().last().unwrap().0, GSK_INIT_OTHER);
    }

    /// Doubles for everything `staticGetClosestPoints` calls; returns the collector log.
    fn closest_points_rig(solver_result: u32) -> (Engine, Log, u32, u32, u32, u32) {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        // Monitor stream: a record buffer.
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.set_global(MONITOR_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        // The relative transform: identity rows, zero translation.
        e.register_double(TRANSFORM_SET_MUL_INVERSE_MUL, |e, a| {
            let rows = [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ];
            for (i, row) in rows.into_iter().enumerate() {
                set_lanes(e, a[0] + 0x10 * i as u32, row);
            }
            Ret::default()
        });
        // The cache builder: one id each side.
        e.register_double(GSK_INIT_OTHER, |e, a| {
            e.mem.set_u8(a[0] + 8, 1);
            e.mem.set_u8(a[0] + 9, 1);
            e.mem.set_u8(a[0] + 0xa, 0);
            Ret::default()
        });
        log_double(&mut e, &log, GSK_SOLVE, solver_result);
        e.register_double(GSK_CLOSEST_POINTS, |e, a| {
            set_lanes(e, a[2], [0.0, 0.0, 1.0, 0.0]);
            set_lanes(e, a[2] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            e.mem.set_f32(a[2] + 0x20, 0.5);
            Ret::default()
        });
        e.register_double(ROTATE_VECTOR, |e, a| {
            let v = lanes(e, a[2]);
            set_lanes(e, a[0], v);
            Ret::default()
        });
        // Shape vtables with the vertex conversion at 0x3c.
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x3c, 0x0900_003c);
        log_double(&mut e, &log, 0x0900_003c, 0);
        let make_shape = |e: &mut Engine, radius: f32| {
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape, vtable);
            e.mem.set_f32(shape + 0x10, radius);
            e.mem.set_u32(shape + 0xc, 1);
            shape
        };
        let shape_a = make_shape(&mut e, 0.1);
        let shape_b = make_shape(&mut e, 0.2);
        let motion_a = e.mem.alloc(0x40);
        let motion_b = e.mem.alloc(0x40);
        for (i, row) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 0.0, 0.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            set_lanes(&mut e, motion_b + 0x10 * i as u32, row);
        }
        let a = e.mem.alloc(0x20);
        e.mem.set_u32(a, shape_a);
        e.mem.set_u32(a + 8, motion_a);
        let b = e.mem.alloc(0x20);
        e.mem.set_u32(b, shape_b);
        e.mem.set_u32(b + 8, motion_b);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 1.0);
        // The collector records the point record it receives.
        let collector_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(collector_vtable + 4, 0x0900_0004);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, collector_vtable);
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..16).map(|i| e.mem.u32(a[1] + 4 * i)));
            sink.borrow_mut().push((0x0900_0004, words));
            Ret::default()
        });
        (e, log, a, b, input, collector)
    }

    #[test]
    fn a_close_pair_reports_a_contact_point() {
        let (mut e, log, a, b, input, collector) = closest_points_rig(0);
        e.call(0x00da_c450, &args![a, b, input, collector]);
        let calls = log.borrow();
        let record = calls.iter().find(|c| c.0 == 0x0900_0004).unwrap();
        let floats: Vec<f32> = record.1[2..].iter().map(|w| f32::from_bits(*w)).collect();
        // gap = 0.5 - 0.1 - 0.2; the world point is the point moved back by radius and gap
        // along the normal, then placed by body b's motion (translation 10 on x).
        let gap = ((f64::from(0.5f32) - f64::from(0.1f32)) - f64::from(0.2f32)) as f32;
        assert!((floats[0] - 11.0).abs() < 1e-5);
        assert!((floats[1] - 2.0).abs() < 1e-5);
        assert!((floats[2] - (3.0 - 0.1 - gap)).abs() < 1e-5);
        // The normal and the gap in the last lane.
        assert_eq!(floats[6], 1.0);
        assert_eq!(floats[7], gap);
        // The two bodies.
        assert_eq!(record.1[2 + 8], a);
        assert_eq!(record.1[2 + 9], b);
        // The monitor stream got two records (start and end labels).
        drop(calls);
        let stream = e.call(TLS_GET_VALUE, &args![3u32]).u32();
        let cursor = e.mem.u32(stream + 4);
        assert_eq!(cursor + 0x100 - e.mem.u32(stream + 0xc), 24);
    }

    #[test]
    fn a_failed_solve_or_a_wide_gap_reports_nothing() {
        let (mut e, log, a, b, input, collector) = closest_points_rig(1);
        e.call(0x00da_c450, &args![a, b, input, collector]);
        assert!(log.borrow().iter().all(|c| c.0 != 0x0900_0004));
        // A tolerance below the gap drops the contact too.
        let (mut e, log, a, b, input, collector) = closest_points_rig(0);
        e.mem.set_f32(input + 8, 0.1);
        e.call(0x00da_c450, &args![a, b, input, collector]);
        assert!(log.borrow().iter().all(|c| c.0 != 0x0900_0004));
    }
}
