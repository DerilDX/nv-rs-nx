//! `NiCollision/niboxbv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nicollision::nicollisiondata::{
    add_viewer_string, c_string_length, float_operand, load_float,
};
use crate::units::nicollision::nispherebv::negated_bits;

layout! {
    /// `NiBoxBV` (Xbox PDB), 0x40 bytes (the PC code allocates 0x40): the
    /// vtable, then `m_kBox`, an `NiBox` (Xbox PDB, 0x3c bytes):
    /// `m_kCenter` at +4, the three unit axes `m_akAxis` at +0x10, +0x1c
    /// and +0x28, the three half extents `m_afExtent` at +0x34.
    pub struct NiBoxBV: 0x40 {
        /// `m_kBox.m_kCenter` (Xbox PDB), x.
        0x04 m_kBox_center_x: u32,
        /// `m_kBox.m_akAxis[0]` (Xbox PDB), x.
        0x10 m_kBox_axis_x: u32,
        /// `m_kBox.m_afExtent[0]` (Xbox PDB).
        0x34 m_kBox_extent: u32,
    }
}

/// The offsets inside the volume of `m_kBox`'s centre, first axis and
/// first extent (see [`NiBoxBV`]).
const CENTER: u32 = 0x04;
const AXES: u32 = 0x10;
const EXTENTS: u32 = 0x34;
/// `NiBoxBV`'s vtable (`010f1814`) and `NiBoundingVolume`'s (`010f16f4`),
/// from the RTTI before them.
const NI_BOX_BV_VTABLE: u32 = 0x010f_1814;
const NI_BOUNDING_VOLUME_VTABLE: u32 = 0x010f_16f4;
/// The memory manager instance (`this` of its allocate and free).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, `thiscall` on [`MEMORY_MANAGER`],
/// the size) and its deallocation (`00aa4060`, the block).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The defaults `Create` and the stream loader build a box from: the
/// extents (`011a9490`, 1, 1, 1 in the image), the centre (`011f426c`, zero),
/// and the axes (`011a946c`, `011a9478`, `011a9484`: the unit x, y and z
/// in the image).
const DEFAULT_EXTENTS: u32 = 0x011a_9490;
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
const DEFAULT_AXIS_X: u32 = 0x011a_946c;
const DEFAULT_AXIS_Y: u32 = 0x011a_9478;
const DEFAULT_AXIS_Z: u32 = 0x011a_9484;
/// `NiBox::operator==`, `NiBox::LoadBinary`, `NiBox::SaveBinary` (Xbox
/// PDB; `00eae520`, `00eae5d0`, `00eae630`; `thiscall` on the box, one
/// word).
const NI_BOX_EQUAL: u32 = 0x00ea_e520;
const NI_BOX_LOAD_BINARY: u32 = 0x00ea_e5d0;
const NI_BOX_SAVE_BINARY: u32 = 0x00ea_e630;
/// `NiBoundingVolume`'s `LoadBinary` part (`00a89af0`, `thiscall`, the
/// stream) and `NiBoundingVolume::SaveBinary` (`00e9cb10`).
const NI_BOUNDING_VOLUME_LOAD_BINARY: u32 = 0x00a8_9af0;
const NI_BOUNDING_VOLUME_SAVE_BINARY: u32 = 0x00e9_cb10;
/// The allocator `00aa1070` (cdecl, size) and `NiSprintf` (`00a305b0`,
/// cdecl: buffer, size, format, then the format's arguments).
const ALLOCATE: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `"%s = BOX_BV"`.
const BOX_BV_FORMAT: u32 = 0x010f_1844;
/// The `float`s `0.0001` (`010141cc`) and `0.001` (`01017d00`) handed to
/// the intersectors as the pseudodistance threshold and the derivative
/// time step.
const PSEUDODISTANCE_THRESHOLD: u32 = 0x0101_41cc;
const DERIVATIVE_TIME_STEP: u32 = 0x0101_7d00;
/// The intersector constructors the box tests use (`thiscall`: two
/// volumes, then the maximum time, the derivative time step, the
/// pseudodistance threshold and the iteration limit; `RET 0x18`):
/// `00eb3c40` with a sphere volume, `00eb3890` with a capsule volume.
const INTERSECTOR_CONSTRUCTOR_00EB3C40: u32 = 0x00eb_3c40;
const INTERSECTOR_CONSTRUCTOR_00EB3890: u32 = 0x00eb_3890;
/// `NiIntersector::Test` (`00eae6e0`) and `::Find` (`00eae7a0`), each
/// `thiscall` with the two velocities; the contact time (`00eae6f0`, ST0),
/// point (`00eae700`) and normal (`00eae710`); the intersection type
/// (`00be0aa0`); the destructor (`00eae6d0`).
const INTERSECTOR_TEST: u32 = 0x00ea_e6e0;
const INTERSECTOR_FIND: u32 = 0x00ea_e7a0;
const INTERSECTOR_CONTACT_TIME: u32 = 0x00ea_e6f0;
const INTERSECTOR_CONTACT_POINT: u32 = 0x00ea_e700;
const INTERSECTOR_CONTACT_NORMAL: u32 = 0x00ea_e710;
const INTERSECTOR_TYPE: u32 = 0x00be_0aa0;
const INTERSECTOR_DESTRUCTOR: u32 = 0x00ea_e6d0;
/// The float `GetNormal` compares the projections with (`0104da4c`;
/// `FLT_EPSILON` in the image) and the unitize it ends with (`004a0c10`,
/// `thiscall` on the point).
const FLT_EPSILON: u32 = 0x0104_da4c;
const UNITIZE_004A0C10: u32 = 0x004a_0c10;

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 00e9e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `NiBoxBV::operator_` (`thiscall`; the other volume;
/// `RET 4`): by its body `operator!=`, the negation of this volume's
/// virtual slot +0x24 (`operator==` in the Xbox PDB's vtable) on the other
/// volume.
pub fn ni_box_bv_operator_(e: &mut Engine, this: Ptr<NiBoxBV>, other: Ptr) -> bool {
    !e.vcall(this.addr(), 0x24, &args![other]).bool()
}

// Translated from 00ea07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::operator==` (engine map `operator__ov2`; `thiscall`; the other
/// volume; `RET 4`): the other volume's `Type` (virtual slot +0xc) must be
/// 1 and its box must equal this one's (`NiBox::operator==` called on the
/// other volume's box with this one's as the argument).
pub fn ni_box_bv_operator_ov2(e: &mut Engine, this: Ptr<NiBoxBV>, other: Ptr) -> bool {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 1 {
        return false;
    }
    e.call(
        NI_BOX_EQUAL,
        &args![other.addr() + CENTER, this.addr() + CENTER],
    )
    .bool()
}

// Translated from 00ea07e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::LoadBinary` (Xbox PDB; `thiscall`; the stream; `RET 4`): the
/// base volume's part (`00a89af0`), then the box (`NiBox::LoadBinary`).
pub fn ni_box_bv_load_binary(e: &mut Engine, this: Ptr<NiBoxBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_LOAD_BINARY, &args![this, stream]);
    e.call(NI_BOX_LOAD_BINARY, &args![this.addr() + CENTER, stream]);
}

// Translated from 00ea0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::SaveBinary` (Xbox PDB; `thiscall`; the stream; `RET 4`):
/// `NiBoundingVolume::SaveBinary`, then the box (`NiBox::SaveBinary`).
pub fn ni_box_bv_save_binary(e: &mut Engine, this: Ptr<NiBoxBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_SAVE_BINARY, &args![this, stream]);
    e.call(NI_BOX_SAVE_BINARY, &args![this.addr() + CENTER, stream]);
}

// Translated from 00ea0820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoxBV` constructor from parts (no name in the engine map;
/// `thiscall`; the extents, the centre and the three axes, each the
/// address of three floats; `RET 0x14`): the vtable `010f1814`, the
/// centre's words, the extents through `FLD`/`FSTP` (a signalling NaN
/// quieted), then the axes' words. Returns `this`.
pub fn fn_00ea0820(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    extents: Ptr,
    center: Ptr,
    axis_x: Ptr,
    axis_y: Ptr,
    axis_z: Ptr,
) -> Ptr<NiBoxBV> {
    let base = this.addr();
    e.mem.set_u32(base, NI_BOX_BV_VTABLE);
    copy_point(e, base + CENTER, center.addr());
    for axis in 0..3u32 {
        let word = x87_quiet_bits(e.mem.u32(extents.addr() + 4 * axis));
        e.mem.set_u32(base + EXTENTS + 4 * axis, word);
    }
    copy_point(e, base + AXES, axis_x.addr());
    copy_point(e, base + AXES + 0xc, axis_y.addr());
    copy_point(e, base + AXES + 0x18, axis_z.addr());
    this
}

// Translated from 00ea08a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::Copy` (Xbox PDB; `thiscall`; the other volume; `RET 4`): the
/// 15 words of the other volume's box (`REP MOVSD`).
pub fn ni_box_bv_copy(e: &mut Engine, this: Ptr<NiBoxBV>, other: Ptr<NiBoxBV>) {
    for word in 0..15u32 {
        let value = e.mem.u32(other.addr() + CENTER + 4 * word);
        e.mem.set_u32(this.addr() + CENTER + 4 * word, value);
    }
}

// Translated from 00ea08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::UpdateWorldData` (Xbox PDB; `thiscall`; the model volume and
/// the transform; `RET 8`): this volume becomes the model volume moved by
/// the transform (a row-major 3x3 rotation at +0, the translation at
/// +0x24, the scale at +0x30). The centre is rotated (each row's three
/// products summed in the executable's order and rounded), scaled (the
/// scale itself through `FLD`/`FSTP`) and translated; each axis is rotated
/// only and each extent multiplied by the scale. Every component is read
/// before it is written, as in the executable, so the model may be this
/// volume.
pub fn ni_box_bv_update_world_data(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    model: Ptr<NiBoxBV>,
    transform: Ptr,
) {
    let (out, m, t) = (this.addr(), model.addr(), transform.addr());
    let ld = |e: &Engine, a: u32| load_float(e, a);
    let op = |e: &Engine, a: u32| float_operand(e, a);
    // The rotated centre: (y r1 + x r0) + z r2 per row, in the code's
    // operand order.
    let c0 = x87_store32(x87_add(
        x87_add(
            x87_mul(ld(e, m + 8), op(e, t + 4)),
            x87_mul(ld(e, m + 4), op(e, t)),
        ),
        x87_mul(ld(e, m + 0xc), op(e, t + 8)),
    ));
    let c1 = x87_store32(x87_add(
        x87_add(
            x87_mul(ld(e, m + 8), op(e, t + 0x10)),
            x87_mul(ld(e, t + 0xc), op(e, m + 4)),
        ),
        x87_mul(ld(e, m + 0xc), op(e, t + 0x14)),
    ));
    let c2 = x87_store32(x87_add(
        x87_add(
            x87_mul(ld(e, m + 4), op(e, t + 0x18)),
            x87_mul(ld(e, t + 0x1c), op(e, m + 8)),
        ),
        x87_mul(ld(e, m + 0xc), op(e, t + 0x20)),
    ));
    // `FLD scale; FSTP` (into the model argument's stack slot), then the
    // products with the scale on the stack.
    let scale = x87_load32(x87_quiet_bits(e.mem.u32(t + 0x30)));
    let s0 = x87_store32(x87_mul(x87_load32(c0), scale));
    let s1 = x87_store32(x87_mul(x87_load32(c1), scale));
    let s2 = x87_store32(x87_mul(scale, x87_m32(c2)));
    let c0 = x87_store32(x87_add(ld(e, t + 0x24), x87_m32(s0)));
    let c1 = x87_store32(x87_add(ld(e, t + 0x28), x87_m32(s1)));
    let c2 = x87_store32(x87_add(ld(e, t + 0x2c), x87_m32(s2)));
    e.mem.set_u32(out + 4, c0);
    e.mem.set_u32(out + 8, c1);
    e.mem.set_u32(out + 0xc, c2);

    // The first axis (+0x10).
    let a = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x14), op(e, t + 4)),
                x87_mul(ld(e, m + 0x10), op(e, t)),
            ),
            x87_mul(ld(e, t + 8), op(e, m + 0x18)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x10), op(e, t + 0xc)),
                x87_mul(ld(e, m + 0x14), op(e, t + 0x10)),
            ),
            x87_mul(ld(e, m + 0x18), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x10), op(e, t + 0x18)),
                x87_mul(ld(e, m + 0x14), op(e, t + 0x1c)),
            ),
            x87_mul(ld(e, t + 0x20), op(e, m + 0x18)),
        )),
    ];
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(out + 0x10 + 4 * i as u32, *word);
    }
    let extent = x87_store32(x87_mul(ld(e, m + 0x34), op(e, t + 0x30)));
    e.mem.set_u32(out + 0x34, extent);

    // The second axis (+0x1c).
    let a = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t), op(e, m + 0x1c)),
                x87_mul(ld(e, m + 0x20), op(e, t + 4)),
            ),
            x87_mul(ld(e, t + 8), op(e, m + 0x24)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, m + 0x20), op(e, t + 0x10)),
                x87_mul(ld(e, t + 0xc), op(e, m + 0x1c)),
            ),
            x87_mul(ld(e, m + 0x24), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0x1c), op(e, m + 0x20)),
                x87_mul(ld(e, m + 0x1c), op(e, t + 0x18)),
            ),
            x87_mul(ld(e, t + 0x20), op(e, m + 0x24)),
        )),
    ];
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(out + 0x1c + 4 * i as u32, *word);
    }
    let extent = x87_store32(x87_mul(ld(e, m + 0x38), op(e, t + 0x30)));
    e.mem.set_u32(out + 0x38, extent);

    // The third axis (+0x28).
    let a = [
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t), op(e, m + 0x28)),
                x87_mul(ld(e, t + 4), op(e, m + 0x2c)),
            ),
            x87_mul(ld(e, m + 0x30), op(e, t + 8)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0xc), op(e, m + 0x28)),
                x87_mul(ld(e, t + 0x10), op(e, m + 0x2c)),
            ),
            x87_mul(ld(e, m + 0x30), op(e, t + 0x14)),
        )),
        x87_store32(x87_add(
            x87_add(
                x87_mul(ld(e, t + 0x1c), op(e, m + 0x2c)),
                x87_mul(ld(e, m + 0x28), op(e, t + 0x18)),
            ),
            x87_mul(ld(e, m + 0x30), op(e, t + 0x20)),
        )),
    ];
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(out + 0x28 + 4 * i as u32, *word);
    }
    let extent = x87_store32(x87_mul(ld(e, m + 0x3c), op(e, t + 0x30)));
    e.mem.set_u32(out + 0x3c, extent);
}

/// The intersector `Find` the two box finds run: an intersector on the
/// stack made by `constructor` from the two volumes, the time, `0.001`,
/// `0.0001` (both read from the exe's data through `FLD`/`FSTP`) and 0x20
/// iterations; `Find` with the two velocities; the contact time to
/// `out_time` and, for an intersection type of 3 or 2, the contact point to
/// `out_point` and (when `want_normals`) the contact normal to
/// `out_normal` and its negation to `out_normal_negated`. The intersector
/// is destroyed on both paths.
#[allow(clippy::too_many_arguments)]
fn intersector_find(
    e: &mut Engine,
    constructor: u32,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    let threshold = x87_quiet_bits(e.mem.u32(PSEUDODISTANCE_THRESHOLD));
    let step = x87_quiet_bits(e.mem.u32(DERIVATIVE_TIME_STEP));
    let maximum_time = x87_quiet_bits(time.to_bits());
    // The intersector at `[ESP+0xc]` of the 0x5c-byte frame.
    e.with_stack(0x50, |e, intersector| {
        e.call(
            constructor,
            &args![
                intersector,
                first,
                second,
                maximum_time,
                step,
                threshold,
                0x20u32
            ],
        );
        e.call(
            INTERSECTOR_FIND,
            &args![intersector, first_velocity, second_velocity],
        );
        let contact_time = e.call(INTERSECTOR_CONTACT_TIME, &args![intersector]).f64();
        e.mem.set_u32(out_time.addr(), x87_store32(contact_time));
        if e.call(INTERSECTOR_TYPE, &args![intersector]).u32() != 3
            && e.call(INTERSECTOR_TYPE, &args![intersector]).u32() != 2
        {
            e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
            return false;
        }
        let point = e.call(INTERSECTOR_CONTACT_POINT, &args![intersector]).u32();
        copy_point(e, out_point.addr(), point);
        if want_normals {
            let normal = e
                .call(INTERSECTOR_CONTACT_NORMAL, &args![intersector])
                .u32();
            copy_point(e, out_normal.addr(), normal);
            for axis in 0..3u32 {
                let word = e.mem.u32(out_normal.addr().wrapping_add(4 * axis));
                e.mem.set_u32(
                    out_normal_negated.addr().wrapping_add(4 * axis),
                    negated_bits(word),
                );
            }
        }
        e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
        true
    })
}

// Translated from 00ea3950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// row's sphere column of the find table set up at `00e99129`): the box
/// `Find` test against a sphere volume (cdecl, ten words: the time, the
/// box and its velocity, the other volume and its velocity, `out_time`,
/// `out_point`, `want_normals` (a byte), `out_normal`,
/// `out_normal_negated`), through the intersector constructor `00eb3c40`;
/// see [`intersector_find`].
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea3950(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    intersector_find(
        e,
        INTERSECTOR_CONSTRUCTOR_00EB3C40,
        time,
        first,
        first_velocity,
        second,
        second_velocity,
        out_time,
        out_point,
        want_normals,
        out_normal,
        out_normal_negated,
    )
}

// Translated from 00ea3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// row's capsule column of the find table, `00e99137`): [`fn_00ea3950`]
/// with the intersector constructor `00eb3890`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea3c50(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    intersector_find(
        e,
        INTERSECTOR_CONSTRUCTOR_00EB3890,
        time,
        first,
        first_velocity,
        second,
        second_velocity,
        out_time,
        out_point,
        want_normals,
        out_normal,
        out_normal_negated,
    )
}

/// The absolute value the box tests take of a projection: `FSTP` (the
/// sum rounded to a float), `FLD`, `FABS`, `FSTP`, `FLD`.
fn abs_rounded(value: f64) -> f64 {
    x87_load32(x87_quiet_bits(x87_store32(value)) & 0x7fff_ffff)
}

/// The three projections of `d` (the other volume's centre minus the
/// box's) on the box's axes, in the box tests' operand order, each
/// compared with its extent: whether all three are within (`FCOMPP`
/// against the extent; a NaN is outside).
fn center_inside(e: &Engine, b: u32, d: [f64; 3]) -> bool {
    let op = |a: u32| float_operand(e, a);
    let ld = |a: u32| load_float(e, a);
    let p0 = x87_add(
        x87_add(x87_mul(ld(b + 0x14), d[1]), x87_mul(d[0], op(b + 0x10))),
        x87_mul(ld(b + 0x18), d[2]),
    );
    if !(ld(b + 0x34) >= abs_rounded(p0)) {
        return false;
    }
    let p1 = x87_add(
        x87_add(x87_mul(ld(b + 0x20), d[1]), x87_mul(ld(b + 0x1c), d[0])),
        x87_mul(ld(b + 0x24), d[2]),
    );
    if !(ld(b + 0x38) >= abs_rounded(p1)) {
        return false;
    }
    let p2 = x87_add(
        x87_add(x87_mul(d[1], ld(b + 0x2c)), x87_mul(d[0], op(b + 0x28))),
        x87_mul(ld(b + 0x30), d[2]),
    );
    ld(b + 0x3c) >= abs_rounded(p2)
}

/// The other volume's centre (+4) minus the box's, each component rounded
/// to a float (`FLD`, `FSUB dword`, `FSTP`), loaded back.
fn center_offset(e: &Engine, b: u32, other: u32) -> [f64; 3] {
    [0u32, 4, 8].map(|offset| {
        x87_load32(x87_store32(x87_sub(
            load_float(e, other + 4 + offset),
            float_operand(e, b + 4 + offset),
        )))
    })
}

// Translated from 00ea2030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the box row's capsule column of the test
/// table set up at `00e9908d`): the box test against a capsule volume
/// (cdecl, five words: the time, the box, its velocity, the other volume,
/// its velocity). True when the other volume's centre (+4) lies within the
/// box (its offset's projection on each axis, rounded and made absolute,
/// not above the extent); otherwise the intersector made by `00eb3890`
/// (the time, `0.001`, `0.0001`, 0x20 iterations) is run
/// (`NiIntersector::Test` with the two velocities) and the answer is
/// whether its intersection type is 3 or 2. The intersector is destroyed
/// on both paths.
pub fn fn_00ea2030(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
) -> bool {
    let d = center_offset(e, first.addr(), second.addr());
    if center_inside(e, first.addr(), d) {
        return true;
    }
    let threshold = x87_quiet_bits(e.mem.u32(PSEUDODISTANCE_THRESHOLD));
    let step = x87_quiet_bits(e.mem.u32(DERIVATIVE_TIME_STEP));
    let maximum_time = x87_quiet_bits(time.to_bits());
    e.with_stack(0x50, |e, intersector| {
        e.call(
            INTERSECTOR_CONSTRUCTOR_00EB3890,
            &args![
                intersector,
                first,
                second,
                maximum_time,
                step,
                threshold,
                0x20u32
            ],
        );
        e.call(
            INTERSECTOR_TEST,
            &args![intersector, first_velocity, second_velocity],
        );
        let hit = e.call(INTERSECTOR_TYPE, &args![intersector]).u32() == 3
            || e.call(INTERSECTOR_TYPE, &args![intersector]).u32() == 2;
        e.call(INTERSECTOR_DESTRUCTOR, &args![intersector]);
        hit
    })
}

// Translated from 00ea3a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::GetNormal` (Xbox PDB; `thiscall`; the out point, the three
/// face signs and the velocity's three floats; `RET 0x1c`): `out` starts as
/// the zero point at `011f426c`; each axis whose sign is 1 is added to it
/// and each whose sign is -1 subtracted (component by component, rounded),
/// but only when the velocity is the zero point or the velocity's
/// projection on that axis is, rounded and made absolute, above the float
/// at `0104da4c` (`FLT_EPSILON` in the image). Unless exactly one axis was
/// applied, `out` is unitized (`004a0c10`). Returns `out`.
#[allow(clippy::too_many_arguments)]
pub fn ni_box_bv_get_normal(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    out: Ptr,
    first: i32,
    second: i32,
    third: i32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
) -> Ptr {
    let (b, out) = (this.addr(), out.addr());
    copy_point(e, out, ZERO_POINT_011F426C);
    let vx = x87_load32(velocity_x.to_bits());
    let vy = x87_load32(velocity_y.to_bits());
    let vz = x87_load32(velocity_z.to_bits());
    // `FUCOMPP` with the zero point's floats: equal, NaN unequal.
    let still = load_float(e, ZERO_POINT_011F426C) == vx
        && load_float(e, ZERO_POINT_011F426C + 4) == vy
        && load_float(e, ZERO_POINT_011F426C + 8) == vz;
    let epsilon = load_float(e, FLT_EPSILON);
    let ld = |e: &Engine, a: u32| load_float(e, a);
    let op = |e: &Engine, a: u32| float_operand(e, a);
    let mut applied = 0u32;
    let signs = [first, second, third];
    for (axis, sign) in signs.iter().enumerate() {
        let a = b + AXES + 0xc * axis as u32;
        let projection = match axis {
            0 => x87_add(
                x87_add(x87_mul(ld(e, a + 4), vy), x87_mul(ld(e, a), vx)),
                x87_mul(ld(e, a + 8), vz),
            ),
            1 => x87_add(
                x87_add(x87_mul(ld(e, a + 4), vy), x87_mul(vx, op(e, a))),
                x87_mul(ld(e, a + 8), vz),
            ),
            _ => x87_add(
                x87_add(x87_mul(vy, ld(e, a + 4)), x87_mul(vx, ld(e, a))),
                x87_mul(vz, ld(e, a + 8)),
            ),
        };
        // Without a still velocity the projection is tested (`TEST
        // AH,0x41` or `0x5`: on when it is above the epsilon, ordered).
        if !still && !(abs_rounded(projection) > epsilon) {
            continue;
        }
        if *sign == 1 {
            let x = x87_store32(x87_add(ld(e, out), op(e, a)));
            e.mem.set_u32(out, x);
            let y = x87_store32(x87_add(ld(e, a + 4), op(e, out + 4)));
            e.mem.set_u32(out + 4, y);
            let z = x87_store32(x87_add(ld(e, a + 8), op(e, out + 8)));
            e.mem.set_u32(out + 8, z);
            applied += 1;
        } else if *sign == -1 {
            for offset in [0u32, 4, 8] {
                let value = x87_store32(x87_sub(ld(e, out + offset), op(e, a + offset)));
                e.mem.set_u32(out + offset, value);
            }
            applied += 1;
        }
    }
    if applied != 1 {
        e.call(UNITIZE_004A0C10, &args![out]);
    }
    Ptr::new(out)
}

// Translated from 00ea7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// loader of the table at `0126fa80`, set at `00e9901d`): the box volume's
/// stream loader (cdecl, the stream). Makes a box volume as
/// [`ni_box_bv_create`] does and runs its `LoadBinary` (virtual slot 0) on
/// the stream. When the allocation fails the code still makes the virtual
/// call through the null pointer (it reads the vtable at address 0).
/// Answers the volume.
pub fn fn_00ea7490(e: &mut Engine, stream: Ptr) -> Ptr<NiBoxBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    let volume = if block != 0 {
        construct_default(e, block)
    } else {
        Ptr::NULL
    };
    e.vcall(volume.addr(), 0, &args![stream]);
    volume
}

/// [`fn_00ea0820`] on `block` with the default extents, centre and axes.
fn construct_default(e: &mut Engine, block: u32) -> Ptr<NiBoxBV> {
    fn_00ea0820(
        e,
        Ptr::new(block),
        Ptr::new(DEFAULT_EXTENTS),
        Ptr::new(ZERO_POINT_011F426C),
        Ptr::new(DEFAULT_AXIS_X),
        Ptr::new(DEFAULT_AXIS_Y),
        Ptr::new(DEFAULT_AXIS_Z),
    )
}

// Translated from 00ea74f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::Create` (Xbox PDB): 0x40 bytes from the memory manager and,
/// when that gives a block, a box volume with the default extents,
/// centre and axes ([`fn_00ea0820`]). Answers the volume, or null.
pub fn ni_box_bv_create(e: &mut Engine) -> Ptr<NiBoxBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_default(e, block)
}

// Translated from 00ea7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::Clone` (Xbox PDB; `thiscall`): 0x40 bytes from the memory
/// manager, the vtable `010f1814` and this volume's 15 box words copied
/// (`REP MOVSD`). Answers the copy, or null.
pub fn ni_box_bv_clone(e: &mut Engine, this: Ptr<NiBoxBV>) -> Ptr<NiBoxBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, NI_BOX_BV_VTABLE);
    for word in 0..15u32 {
        let value = e.mem.u32(this.addr() + CENTER + 4 * word);
        e.mem.set_u32(block + CENTER + 4 * word, value);
    }
    Ptr::new(block)
}

// Translated from 00eab7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxBV::AddViewerStrings` (Xbox PDB; `thiscall`, `this` not read; the
/// name and the strings array; `RET 8`): appends `"<name> = BOX_BV"` (a
/// block of `strlen(name) + 0xa` bytes from `00aa1070`, formatted by
/// `NiSprintf`) to the strings array.
pub fn ni_box_bv_add_viewer_strings(e: &mut Engine, _this: Ptr<NiBoxBV>, name: Ptr, strings: Ptr) {
    let size = c_string_length(e, name.addr()).wrapping_add(0xa);
    let line = e.call(ALLOCATE, &args![size]).u32();
    e.call(NI_SPRINTF, &args![line, size, BOX_BV_FORMAT, name]);
    add_viewer_string(e, strings, line);
}

// Translated from 00eac1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoxBV` scalar deleting destructor (engine map; `thiscall`;
/// flags; `RET 4`): the `NiBoundingVolume` vtable (`010f16f4`, the base
/// destructor inlined), then the block goes back to the memory manager
/// (`00aa4060`) when bit 0 of `flags` is set. Returns `this`.
pub fn ni_box_bv_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBoxBV>,
    flags: u32,
) -> Ptr<NiBoxBV> {
    e.mem.set_u32(this.addr(), NI_BOUNDING_VOLUME_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}
/// `00eb42e0` (cdecl: a point, a box, three out floats; ST0) and `00eb50f0`
/// (cdecl: a segment, a box, four out floats; ST0): the distances the
/// box-sphere test compares with the squared radius.
const POINT_BOX_DISTANCE_00EB42E0: u32 = 0x00eb_42e0;
const SEGMENT_BOX_DISTANCE_00EB50F0: u32 = 0x00eb_50f0;

/// [`center_inside`] with the operand order of [`fn_00ea7530`] (two of the
/// products take the axis component as a loaded value or a memory operand
/// the other way round).
fn center_inside_7530(e: &Engine, b: u32, d: [f64; 3]) -> bool {
    let op = |a: u32| float_operand(e, a);
    let ld = |a: u32| load_float(e, a);
    let p0 = x87_add(
        x87_add(x87_mul(ld(b + 0x14), d[1]), x87_mul(ld(b + 0x10), d[0])),
        x87_mul(ld(b + 0x18), d[2]),
    );
    if !(ld(b + 0x34) >= abs_rounded(p0)) {
        return false;
    }
    let p1 = x87_add(
        x87_add(x87_mul(ld(b + 0x20), d[1]), x87_mul(ld(b + 0x1c), d[0])),
        x87_mul(ld(b + 0x24), d[2]),
    );
    if !(ld(b + 0x38) >= abs_rounded(p1)) {
        return false;
    }
    let p2 = x87_add(
        x87_add(x87_mul(d[1], ld(b + 0x2c)), x87_mul(d[0], ld(b + 0x28))),
        x87_mul(d[2], op(b + 0x30)),
    );
    ld(b + 0x3c) >= abs_rounded(p2)
}

// Translated from 00ea7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the box row's sphere column of the test
/// table set up at `00e9907f`): the box test against a sphere volume
/// (cdecl, five words: the time, the box, its velocity, the sphere volume,
/// its velocity; the sphere's centre at +4, radius at +0x10). True when the
/// sphere's centre lies within the box (as in [`fn_00ea2030`]). Otherwise
/// the relative velocity (the sphere's minus the box's, each component
/// rounded) decides: when it equals the zero point at `011f426c`
/// (`FUCOMPP`) the distance is `00eb42e0` of the centre and the box,
/// otherwise `00eb50f0` of the segment from the centre along `time *
/// velocity` (each component rounded) and the box. The answer is whether
/// that distance, rounded, is at most the radius squared (`FCOMPP`,
/// ordered).
pub fn fn_00ea7530(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
) -> bool {
    let (b, s) = (first.addr(), second.addr());
    let d = center_offset(e, b, s);
    if center_inside_7530(e, b, d) {
        return true;
    }
    // The frame: the relative velocity at `[ESP+0x10]` (then the scaled
    // components), the segment (centre, direction) at `[ESP+0x1c]`, two
    // out floats at `[ESP+0xc]` and `[ESP+0x10]`, and two more in the
    // argument slots of the box and the sphere.
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        let relative = [0u32, 4, 8].map(|offset| {
            x87_store32(x87_sub(
                load_float(e, second_velocity.addr() + offset),
                float_operand(e, first_velocity.addr() + offset),
            ))
        });
        let [rx, ry, rz] = relative.map(x87_load32);
        let still = load_float(e, ZERO_POINT_011F426C) == rx
            && load_float(e, ZERO_POINT_011F426C + 4) == ry
            && load_float(e, ZERO_POINT_011F426C + 8) == rz;
        let (out_a, out_b, out_c, out_d) = (f, f + 4, f + 8, f + 0xc);
        let distance = if still {
            e.call(
                POINT_BOX_DISTANCE_00EB42E0,
                &args![s + 4, b + 4, out_a, out_c, out_d],
            )
            .f64()
        } else {
            let t = x87_load32(time.to_bits());
            let segment = f + 0x10;
            copy_point(e, segment, s + 4);
            let direction = [
                x87_store32(x87_mul(rx, t)),
                x87_store32(x87_mul(ry, t)),
                x87_store32(x87_mul(t, rz)),
            ];
            for (axis, word) in direction.iter().enumerate() {
                e.mem.set_u32(segment + 0xc + 4 * axis as u32, *word);
            }
            e.call(
                SEGMENT_BOX_DISTANCE_00EB50F0,
                &args![segment, b + 4, out_b, out_a, out_c, out_d],
            )
            .f64()
        };
        let distance = x87_load32(x87_store32(distance));
        let radius = load_float(e, s + 0x10);
        // `FCOMPP; TEST AH,0x41; JP`: true when below or equal.
        distance <= x87_mul(radius, radius)
    })
}

/// `00eab8a0` (`thiscall` on the sphere volume; the radius, the centre's
/// address; `RET 8`): builds an `NiSphereBV` (its vtable `010f1854` by the
/// RTTI) on the stack for [`fn_00ea7730`].
const SPHERE_BV_CONSTRUCT_00EAB8A0: u32 = 0x00ea_b8a0;
/// The radius of that sphere (`01013ea4`, a `float`; 0.01 in the image).
const PROBE_RADIUS: u32 = 0x0101_3ea4;
/// The `double` (0.99 in the image, `0106d508`) the extents are scaled by
/// before a projection is clamped to them.
const CLAMP_FACTOR: u32 = 0x0106_d508;

/// The clamp of [`fn_00ea7730`] for one axis: the projection `p` (a loaded
/// float) above `factor` times the extent (`FCOMP`, ordered) becomes the
/// extent; below `factor` times the negated extent (the negation stored
/// and reloaded) the negated extent; otherwise (a NaN included) it stays.
fn clamp_to_extent(e: &Engine, p: f64, extent: u32, factor: f64) -> f64 {
    let full = load_float(e, extent);
    // `TEST AH,0x5; JP`: on unless the scaled extent is below `p`.
    if x87_mul(full, factor) < p {
        return x87_load32(x87_store32(full));
    }
    let negated = x87_load32(x87_store32(-full));
    // `TEST AH,0x41; JNZ`: kept unless the scaled negation is above `p`.
    if x87_mul(negated, factor) > p {
        return x87_load32(x87_store32(negated));
    }
    p
}

/// One half of [`fn_00ea7730`]: moves the point at `point` into the box
/// `b` moved by its velocity `v` times the contact time at `time_at`: the
/// offset of the point from the moved centre is projected on the three
/// axes (rounded), each projection clamped ([`clamp_to_extent`]), and the
/// point becomes the moved centre plus the axes times the clamped
/// projections (each product and sum rounded, the three axes added one
/// after the other). `second` selects the operand forms of the second
/// half (a few products take a memory operand where the first loads it).
fn clamp_point_into_box(e: &mut Engine, b: u32, v: u32, time_at: u32, point: u32, second: bool) {
    let ld = |e: &Engine, a: u32| load_float(e, a);
    let op = |e: &Engine, a: u32| float_operand(e, a);
    let st = |x: f64| x87_load32(x87_store32(x));
    let tt = x87_load32(x87_quiet_bits(e.mem.u32(time_at)));
    let moved = [
        x87_store32(x87_mul(ld(e, v), tt)),
        x87_store32(x87_mul(ld(e, v + 4), tt)),
        x87_store32(x87_mul(tt, op(e, v + 8))),
    ];
    let c = [
        st(x87_add(x87_load32(moved[0]), op(e, b + 4))),
        st(x87_add(ld(e, b + 8), x87_m32(moved[1]))),
        st(x87_add(ld(e, b + 0xc), x87_m32(moved[2]))),
    ];
    // The moved centre's y and z come back as memory operands: the same
    // values (stored floats are never signalling).
    let d = [
        st(x87_sub(ld(e, point), c[0])),
        st(x87_sub(ld(e, point + 4), c[1])),
        st(x87_sub(ld(e, point + 8), c[2])),
    ];
    let a = |i: u32| b + 0x10 + 4 * i;
    let (p0, p1, p2) = if second {
        (
            x87_add(
                x87_mul(ld(e, a(2)), d[2]),
                x87_add(x87_mul(ld(e, a(1)), d[1]), x87_mul(d[0], op(e, a(0)))),
            ),
            x87_add(
                x87_add(x87_mul(ld(e, a(4)), d[1]), x87_mul(ld(e, a(3)), d[0])),
                x87_mul(ld(e, a(5)), d[2]),
            ),
            x87_add(
                x87_mul(d[2], ld(e, a(8))),
                x87_add(x87_mul(d[1], ld(e, a(7))), x87_mul(d[0], op(e, a(6)))),
            ),
        )
    } else {
        (
            x87_add(
                x87_mul(ld(e, a(2)), d[2]),
                x87_add(x87_mul(ld(e, a(0)), d[0]), x87_mul(ld(e, a(1)), d[1])),
            ),
            x87_add(
                x87_add(x87_mul(ld(e, a(4)), d[1]), x87_mul(d[0], op(e, a(3)))),
                x87_mul(ld(e, a(5)), d[2]),
            ),
            x87_add(
                x87_add(x87_mul(d[0], ld(e, a(6))), x87_mul(d[1], ld(e, a(7)))),
                x87_mul(d[2], op(e, a(8))),
            ),
        )
    };
    let factor = x87_load64(e.mem.u64(CLAMP_FACTOR));
    let p0 = clamp_to_extent(e, st(p0), b + 0x34, factor);
    let p1 = clamp_to_extent(e, st(p1), b + 0x38, factor);
    let p2 = clamp_to_extent(e, st(p2), b + 0x3c, factor);
    let q2 = if second {
        [
            st(x87_mul(p2, op(e, a(6)))),
            st(x87_mul(ld(e, a(7)), p2)),
            st(x87_mul(p2, op(e, a(8)))),
        ]
    } else {
        [
            st(x87_mul(ld(e, a(6)), p2)),
            st(x87_mul(ld(e, a(7)), p2)),
            st(x87_mul(p2, op(e, a(8)))),
        ]
    };
    let q1 = [
        st(x87_mul(ld(e, a(3)), p1)),
        st(x87_mul(ld(e, a(4)), p1)),
        st(x87_mul(p1, ld(e, a(5)))),
    ];
    let q0 = [
        st(x87_mul(ld(e, a(0)), p0)),
        st(x87_mul(ld(e, a(1)), p0)),
        st(x87_mul(p0, if second { op(e, a(2)) } else { ld(e, a(2)) })),
    ];
    let r = [
        st(x87_add(c[0], q0[0])),
        st(x87_add(q0[1], c[1])),
        st(x87_add(q0[2], c[2])),
    ];
    for axis in 0..3 {
        let s = st(x87_add(r[axis], q1[axis]));
        let word = x87_store32(x87_add(s, q2[axis]));
        e.mem.set_u32(point + 4 * axis as u32, word);
    }
}

// Translated from 00ea7730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, seven words: the time, box `a`, its
/// velocity, box `b`, its velocity, the address of the contact time, the
/// contact point): moves the contact point into both boxes. A small sphere
/// volume is built on the stack (`00eab8a0` with radius 1.0 and the zero
/// point, then the radius from `01013ea4` and the point as its centre);
/// when the box-sphere test ([`fn_00ea7530`], the sphere not moving) says
/// it does not touch box `a`, the point is clamped into box `a` moved to
/// the contact time ([`clamp_point_into_box`]); then the same for box `b`
/// (the sphere still at the original point). The function answers nothing
/// its callers read (`EAX` holds the last test's byte or a point word).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea7730(
    e: &mut Engine,
    time: f32,
    a: Ptr,
    a_velocity: Ptr,
    b: Ptr,
    b_velocity: Ptr,
    contact_time: Ptr,
    point: Ptr,
) {
    let time = f32::from_bits(x87_quiet_bits(time.to_bits()));
    // The sphere volume `[ESP+0x4c]` (vtable, centre, radius).
    e.with_stack(0x14, |e, sphere| {
        let s = sphere.addr();
        e.call(
            SPHERE_BV_CONSTRUCT_00EAB8A0,
            &args![s, 1.0f32, ZERO_POINT_011F426C],
        );
        let radius = x87_quiet_bits(e.mem.u32(PROBE_RADIUS));
        e.mem.set_u32(s + 0x10, radius);
        copy_point(e, s + 4, point.addr());
        let zero = Ptr::new(ZERO_POINT_011F426C);
        if !fn_00ea7530(e, time, a, a_velocity, sphere, zero) {
            clamp_point_into_box(
                e,
                a.addr(),
                a_velocity.addr(),
                contact_time.addr(),
                point.addr(),
                false,
            );
        }
        if !fn_00ea7530(e, time, b, b_velocity, sphere, zero) {
            clamp_point_into_box(
                e,
                b.addr(),
                b_velocity.addr(),
                contact_time.addr(),
                point.addr(),
                true,
            );
        }
    });
}

/// `NiPoint3::Dot` (engine map, `004b6190`; `thiscall` on one point, the
/// other point's address; `RET 4`; ST0).
const POINT_DOT: u32 = 0x004b_6190;
/// `004b3800` (`thiscall` on a point `p`; out, `q`; `RET 8`): `p x q` into
/// `out`.
const POINT_CROSS_004B3800: u32 = 0x004b_3800;

/// `FSTP float` then `FLD float`: the value rounded to a float, loaded back.
fn rounded(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The separating-axis decision the box-box test makes for each axis: `r`
/// the summed radii (a stored float), `d0` the centre offset's projection
/// and `d1` the projection at the end of the motion. Separated when both
/// are above `r`, or, when `d0` is not above `r` (a NaN included), when
/// `d0` and then `d1` (asked only then) are below `-r` (stored and
/// reloaded). `d1_above` gives `d1` for the first case, `d1_below` for the
/// second (they differ only in how some axes compute it).
fn axis_separated(
    e: &mut Engine,
    r: f64,
    d0: f64,
    d1_above: impl FnOnce(&mut Engine) -> f64,
    d1_below: impl FnOnce(&mut Engine) -> f64,
) -> bool {
    // `TEST AH,0x5; JP`: the second case unless `r` is below `d0`.
    if r < d0 {
        let d1 = d1_above(e);
        return d1 > r;
    }
    let negated = rounded(-r);
    // `TEST AH,0x41; JNZ`: on to the next axis unless `-r` is above `d0`.
    if !(negated > d0) {
        return false;
    }
    let d1 = d1_below(e);
    negated > d1
}

// Translated from 00ea0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created; the box
/// row's box column of the test table set up at `00e99086`): the moving
/// box-box test (cdecl, five words: the time, box `a`, its velocity, box
/// `b`, its velocity). With `T` the centre offset (`b - a`), `V` the
/// relative velocity (`b`'s minus `a`'s) and `T1 = V * time + T` (every
/// component rounded), the boxes are apart when one of these axes
/// separates them ([`axis_separated`], all values rounded where the code
/// stores them): `a`'s three axes, `b`'s three axes, the nine axis cross
/// products (with `T1` projected in extended precision), and the six
/// products of `V` with the axes (with `|L . (V x T)|` against the summed
/// radii, `NiPoint3::Dot` and `004b3800`). Answers false as soon as one
/// separates, true otherwise.
pub fn fn_00ea0ad0(
    e: &mut Engine,
    time: f32,
    a: Ptr,
    a_velocity: Ptr,
    b: Ptr,
    b_velocity: Ptr,
) -> bool {
    let (a, av, b, bv) = (a.addr(), a_velocity.addr(), b.addr(), b_velocity.addr());
    // The frame: `[ESP+0x34]` the three floats of `T1` (later `V x T`),
    // `[ESP+0x9c]` a copy of `V`, `[ESP+0xa8]` of `T1`, `[ESP+0xb4]` of `T`.
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let (block, v_copy, t1_copy, t_copy) = (f, f + 0xc, f + 0x18, f + 0x24);
        let ld = |e: &Engine, x: u32| load_float(e, x);
        let op = |e: &Engine, x: u32| float_operand(e, x);
        let mul = x87_mul;
        let add = x87_add;
        let sub = x87_sub;
        let v = [0u32, 4, 8].map(|k| rounded(sub(ld(e, bv + k), op(e, av + k))));
        let t = [0u32, 4, 8].map(|k| rounded(sub(ld(e, b + 4 + k), op(e, a + 4 + k))));
        for k in 0..3 {
            e.mem.set_u32(v_copy + 4 * k, x87_store32(v[k as usize]));
            e.mem.set_u32(t_copy + 4 * k, x87_store32(t[k as usize]));
        }
        let time = x87_load32(time.to_bits());
        let moved = [
            rounded(mul(v[0], time)),
            rounded(mul(v[1], time)),
            rounded(mul(time, v[2])),
        ];
        let t1 = [0usize, 1, 2].map(|k| rounded(add(moved[k], t[k])));
        for k in 0..3 {
            let bits = x87_store32(t1[k as usize]);
            e.mem.set_u32(block + 4 * k, bits);
            e.mem.set_u32(t1_copy + 4 * k, bits);
        }
        let axis_a = |i: u32| a + 0x10 + 0xc * i;
        let axis_b = |j: u32| b + 0x10 + 0xc * j;
        let extent_a = |i: u32| a + 0x34 + 4 * i;
        let extent_b = |j: u32| b + 0x34 + 4 * j;
        // `(x + y) + z` of three products, each `FLD` of the first address
        // times the second as a memory operand.
        let dot = |e: &Engine, terms: [(u32, u32); 3]| {
            add(
                add(
                    mul(ld(e, terms[0].0), op(e, terms[0].1)),
                    mul(ld(e, terms[1].0), op(e, terms[1].1)),
                ),
                mul(ld(e, terms[2].0), op(e, terms[2].1)),
            )
        };
        let (a0, a1, a2) = (axis_a(0), axis_a(1), axis_a(2));
        let (b0, b1, b2) = (axis_b(0), axis_b(1), axis_b(2));
        // The axis dot products `c[i][j] = Ai . Bj`, in the code's operand
        // forms.
        let c = [
            [
                rounded(dot(e, [(b0, a0), (b0 + 4, a0 + 4), (b0 + 8, a0 + 8)])),
                rounded(dot(e, [(b1 + 4, a0 + 4), (b1, a0), (b1 + 8, a0 + 8)])),
                rounded(dot(e, [(b2 + 4, a0 + 4), (b2, a0), (b2 + 8, a0 + 8)])),
            ],
            [
                rounded(dot(e, [(a1 + 4, b0 + 4), (b0, a1), (a1 + 8, b0 + 8)])),
                rounded(dot(e, [(b1 + 4, a1 + 4), (a1, b1), (b1 + 8, a1 + 8)])),
                rounded(dot(e, [(b2 + 4, a1 + 4), (b2, a1), (b2 + 8, a1 + 8)])),
            ],
            [
                rounded(dot(e, [(a2 + 4, b0 + 4), (a2, b0), (a2 + 8, b0 + 8)])),
                rounded(dot(e, [(b1 + 4, a2 + 4), (b1, a2), (b1 + 8, a2 + 8)])),
                rounded(dot(e, [(b2 + 4, a2 + 4), (b2, a2), (b2 + 8, a2 + 8)])),
            ],
        ];
        let abs_c = c.map(|row| row.map(|value| rounded(value.abs())));
        // `T . Ai` and `T1 . Ai`: the first axis takes x and y as memory
        // operands and loads z; the others load y and z.
        let project_a = |e: &Engine, i: u32, p: [f64; 3]| {
            let axis = axis_a(i);
            if i == 0 {
                add(
                    add(mul(p[0], op(e, axis)), mul(p[1], op(e, axis + 4))),
                    mul(ld(e, axis + 8), p[2]),
                )
            } else {
                add(
                    add(mul(ld(e, axis + 4), p[1]), mul(p[0], op(e, axis))),
                    mul(ld(e, axis + 8), p[2]),
                )
            }
        };
        let d_a = [0, 1, 2].map(|i| rounded(project_a(e, i, t)));
        let d1_a = [0, 1, 2].map(|i| rounded(project_a(e, i, t1)));
        for i in 0..3usize {
            let radius_b = rounded(add(
                add(
                    mul(ld(e, extent_b(1)), abs_c[i][1]),
                    mul(ld(e, extent_b(0)), abs_c[i][0]),
                ),
                mul(abs_c[i][2], op(e, extent_b(2))),
            ));
            let r = rounded(add(radius_b, op(e, extent_a(i as u32))));
            let d1 = d1_a[i];
            if axis_separated(e, r, d_a[i], |_| d1, |_| d1) {
                return false;
            }
        }
        // `b`'s axes: `T . Bj` and, when needed, `T1 . Bj` (on the second
        // side of `b`'s second and third axes through `NiPoint3::Dot` on the
        // copy of `T1`, unrounded).
        let project_b = |e: &Engine, j: u32, p: [f64; 3]| {
            let axis = axis_b(j);
            match j {
                0 => add(
                    add(mul(p[1], op(e, axis + 4)), mul(p[0], op(e, axis))),
                    mul(ld(e, axis + 8), p[2]),
                ),
                1 => add(
                    add(mul(ld(e, axis + 4), p[1]), mul(p[0], op(e, axis))),
                    mul(ld(e, axis + 8), p[2]),
                ),
                _ => add(
                    add(mul(p[0], ld(e, axis)), mul(p[1], ld(e, axis + 4))),
                    mul(ld(e, axis + 8), p[2]),
                ),
            }
        };
        for j in 0..3u32 {
            let ju = j as usize;
            let d0 = rounded(project_b(e, j, t));
            let radius_a = rounded(add(
                add(
                    mul(ld(e, extent_a(1)), abs_c[1][ju]),
                    mul(ld(e, extent_a(0)), abs_c[0][ju]),
                ),
                mul(ld(e, extent_a(2)), abs_c[2][ju]),
            ));
            let r = rounded(add(radius_a, op(e, extent_b(j))));
            let above = |e: &mut Engine| rounded(project_b(e, j, t1));
            let separated = if j == 0 {
                axis_separated(e, r, d0, above, |e| rounded(project_b(e, 0, t1)))
            } else {
                axis_separated(e, r, d0, above, |e| {
                    e.call(POINT_DOT, &args![axis_b(j), t1_copy]).f64()
                })
            };
            if separated {
                return false;
            }
        }
        // The nine cross products `Ai x Bj`: `d0` rounded, `d1` in extended
        // precision; the radii as the two rounded sums of the code.
        let (da, d1a) = (d_a, d1_a);
        let e_a = [0u32, 1, 2].map(extent_a);
        let e_b = [0u32, 1, 2].map(extent_b);
        // (d0, d1, the first radius term, the second) per axis.
        type Cross = (f64, f64, f64, f64);
        let crosses: [Cross; 9] = [
            (
                rounded(sub(mul(da[2], c[1][0]), mul(c[2][0], da[1]))),
                sub(mul(d1a[2], c[1][0]), mul(c[2][0], d1a[1])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[0][2]),
                    mul(abs_c[0][1], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[1][0]),
                    mul(ld(e, e_a[1]), abs_c[2][0]),
                )),
            ),
            (
                rounded(sub(mul(da[2], c[1][1]), mul(c[2][1], da[1]))),
                sub(mul(d1a[2], c[1][1]), mul(c[2][1], d1a[1])),
                rounded(add(
                    mul(ld(e, e_b[0]), abs_c[0][2]),
                    mul(abs_c[0][0], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[1][1]),
                    mul(ld(e, e_a[1]), abs_c[2][1]),
                )),
            ),
            (
                rounded(sub(mul(da[2], c[1][2]), mul(c[2][2], da[1]))),
                sub(mul(d1a[2], c[1][2]), mul(c[2][2], d1a[1])),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[1][2]),
                    mul(ld(e, e_a[1]), abs_c[2][2]),
                )),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[0][0]),
                    mul(ld(e, e_b[0]), abs_c[0][1]),
                )),
            ),
            (
                rounded(sub(mul(c[2][0], da[0]), mul(da[2], c[0][0]))),
                sub(mul(c[2][0], d1a[0]), mul(d1a[2], c[0][0])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[1][2]),
                    mul(abs_c[1][1], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[0][0]),
                    mul(ld(e, e_a[0]), abs_c[2][0]),
                )),
            ),
            (
                rounded(sub(mul(c[2][1], da[0]), mul(da[2], c[0][1]))),
                sub(mul(c[2][1], d1a[0]), mul(d1a[2], c[0][1])),
                rounded(add(
                    mul(ld(e, e_b[0]), abs_c[1][2]),
                    mul(abs_c[1][0], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[0][1]),
                    mul(ld(e, e_a[0]), abs_c[2][1]),
                )),
            ),
            (
                rounded(sub(mul(c[2][2], da[0]), mul(da[2], c[0][2]))),
                sub(mul(c[2][2], d1a[0]), mul(d1a[2], c[0][2])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[1][0]),
                    mul(ld(e, e_b[0]), abs_c[1][1]),
                )),
                rounded(add(
                    mul(ld(e, e_a[2]), abs_c[0][2]),
                    mul(ld(e, e_a[0]), abs_c[2][2]),
                )),
            ),
            (
                rounded(sub(mul(da[1], c[0][0]), mul(c[1][0], da[0]))),
                sub(mul(c[0][0], d1a[1]), mul(c[1][0], d1a[0])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[2][2]),
                    mul(abs_c[2][1], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[1]), abs_c[0][0]),
                    mul(ld(e, e_a[0]), abs_c[1][0]),
                )),
            ),
            (
                rounded(sub(mul(da[1], c[0][1]), mul(c[1][1], da[0]))),
                sub(mul(c[0][1], d1a[1]), mul(c[1][1], d1a[0])),
                rounded(add(
                    mul(ld(e, e_b[0]), abs_c[2][2]),
                    mul(abs_c[2][0], op(e, e_b[2])),
                )),
                rounded(add(
                    mul(ld(e, e_a[1]), abs_c[0][1]),
                    mul(ld(e, e_a[0]), abs_c[1][1]),
                )),
            ),
            (
                rounded(sub(mul(da[1], c[0][2]), mul(da[0], c[1][2]))),
                sub(mul(c[0][2], d1a[1]), mul(d1a[0], c[1][2])),
                rounded(add(
                    mul(ld(e, e_b[1]), abs_c[2][0]),
                    mul(ld(e, e_b[0]), abs_c[2][1]),
                )),
                rounded(add(
                    mul(ld(e, e_a[1]), abs_c[0][2]),
                    mul(ld(e, e_a[0]), abs_c[1][2]),
                )),
            ),
        ];
        for (d0, d1, first, second) in crosses {
            let r = rounded(add(first, second));
            if axis_separated(e, r, d0, |_| d1, |_| d1) {
                return false;
            }
        }
        // The motion axes: `N = V x T` (into the first block), the axes'
        // products with `V`, and for each axis `L` the test of
        // `|L . N|` against the radii.
        e.call(POINT_CROSS_004B3800, &args![v_copy, block, t_copy]);
        let dot_call =
            |e: &mut Engine, this: u32, other: u32| e.call(POINT_DOT, &args![this, other]).f64();
        let abs_rounded_value = |value: f64| rounded(value.abs());
        // `a`'s axes.
        let n1 = rounded(dot_call(e, v_copy, a1));
        let n2 = rounded(dot_call(e, v_copy, a2));
        let abs_n2 = abs_rounded_value(n2);
        let abs_n1 = abs_rounded_value(n1);
        let reach = abs_rounded_value(dot_call(e, a0, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(n2, c[1][1]), mul(n1, c[2][1])))),
            abs_rounded_value(rounded(sub(mul(n2, c[1][0]), mul(n1, c[2][0])))),
            abs_rounded_value(rounded(sub(mul(n2, c[1][2]), mul(n1, c[2][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_b[1])), mul(k[1], op(e, e_b[0]))),
                mul(k[2], op(e, e_b[2])),
            )),
            rounded(add(mul(ld(e, e_a[2]), abs_n1), mul(ld(e, e_a[1]), abs_n2))),
        ));
        // `TEST AH,0x5; JNP`: separated when the radius is below.
        if radius < reach {
            return false;
        }
        let n0 = rounded(dot_call(e, v_copy, a0));
        let abs_n0 = abs_rounded_value(n0);
        let reach = abs_rounded_value(dot_call(e, a1, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(n0, c[2][1]), mul(n2, c[0][1])))),
            abs_rounded_value(rounded(sub(mul(n0, c[2][0]), mul(n2, c[0][0])))),
            abs_rounded_value(rounded(sub(mul(n0, c[2][2]), mul(n2, c[0][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_b[1])), mul(k[1], op(e, e_b[0]))),
                mul(k[2], op(e, e_b[2])),
            )),
            rounded(add(mul(abs_n2, op(e, e_a[0])), mul(ld(e, e_a[2]), abs_n0))),
        ));
        if radius < reach {
            return false;
        }
        let reach = abs_rounded_value(dot_call(e, a2, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(n1, c[0][1]), mul(n0, c[1][1])))),
            abs_rounded_value(rounded(sub(mul(n1, c[0][0]), mul(n0, c[1][0])))),
            abs_rounded_value(rounded(sub(mul(n1, c[0][2]), mul(n0, c[1][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_b[1])), mul(k[1], op(e, e_b[0]))),
                mul(k[2], op(e, e_b[2])),
            )),
            rounded(add(mul(abs_n1, op(e, e_a[0])), mul(ld(e, e_a[1]), abs_n0))),
        ));
        if radius < reach {
            return false;
        }
        // `b`'s axes.
        let m1 = rounded(dot_call(e, v_copy, b1));
        let m2 = rounded(dot_call(e, v_copy, b2));
        let abs_m2 = abs_rounded_value(m2);
        let abs_m1 = abs_rounded_value(m1);
        let reach = abs_rounded_value(dot_call(e, b0, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(m2, c[1][1]), mul(m1, c[1][2])))),
            abs_rounded_value(rounded(sub(mul(m2, c[0][1]), mul(m1, c[0][2])))),
            abs_rounded_value(rounded(sub(mul(m2, c[2][1]), mul(m1, c[2][2])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_a[1])), mul(k[1], op(e, e_a[0]))),
                mul(k[2], op(e, e_a[2])),
            )),
            rounded(add(mul(abs_m1, op(e, e_b[2])), mul(abs_m2, op(e, e_b[1])))),
        ));
        if radius < reach {
            return false;
        }
        let m0 = rounded(dot_call(e, v_copy, b0));
        let abs_m0 = abs_rounded_value(m0);
        let reach = abs_rounded_value(dot_call(e, b1, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(m0, c[1][2]), mul(m2, c[1][0])))),
            abs_rounded_value(rounded(sub(mul(m0, c[0][2]), mul(m2, c[0][0])))),
            abs_rounded_value(rounded(sub(mul(m0, c[2][2]), mul(m2, c[2][0])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_a[1])), mul(k[1], op(e, e_a[0]))),
                mul(k[2], op(e, e_a[2])),
            )),
            rounded(add(mul(abs_m0, op(e, e_b[2])), mul(abs_m2, op(e, e_b[0])))),
        ));
        if radius < reach {
            return false;
        }
        let reach = abs_rounded_value(dot_call(e, b2, block));
        let k = [
            abs_rounded_value(rounded(sub(mul(m1, c[1][0]), mul(m0, c[1][1])))),
            abs_rounded_value(rounded(sub(mul(m1, c[0][0]), mul(m0, c[0][1])))),
            abs_rounded_value(rounded(sub(mul(m1, c[2][0]), mul(m0, c[2][1])))),
        ];
        let radius = rounded(add(
            rounded(add(
                add(mul(k[0], op(e, e_a[1])), mul(k[1], op(e, e_a[0]))),
                mul(k[2], op(e, e_a[2])),
            )),
            rounded(add(mul(abs_m0, op(e, e_b[1])), mul(abs_m1, op(e, e_b[0])))),
        ));
        !(radius < reach)
    })
}

/// One of the box's own axes in [`fn_00ea2190`]: `p` the projection of
/// the first vertex's offset from the centre, `q1` and `q2` those of the
/// two edges from that vertex, `w` the relative velocity's (all stored
/// floats) and `extent` the box's half extent on the axis (loaded). Above
/// the extent: the lowest vertex (`p` plus `0`, `q1` or `q2`, the sum
/// rounded) must be above it, then that vertex moved by `w` times the time
/// (extended precision); below the negated extent (stored and reloaded)
/// the same with the highest vertex. The comparisons take a NaN the way
/// the code's flag tests do: a NaN `q1` or `q2` picks the other sum.
fn triangle_box_axis_separated(p: f64, q1: f64, q2: f64, w: f64, extent: f64, time: f64) -> bool {
    let moved = |x: f64| x87_add(x, x87_mul(w, time));
    // `TEST AH,0x5; JP`: the negative side unless the extent is below `p`.
    if extent < p {
        let lowest = if q1 > 0.0 {
            if q2 >= 0.0 {
                // The first vertex is the lowest: `p` itself, already above.
                return extent < moved(p);
            }
            rounded(x87_add(p, q2))
        } else if q2 <= q1 {
            rounded(x87_add(p, q2))
        } else {
            rounded(x87_add(p, q1))
        };
        return extent < lowest && extent < moved(lowest);
    }
    let negated = rounded(-extent);
    // `TEST AH,0x41; JNZ`: on to the next axis unless the negated extent
    // is above `p`.
    if !(negated > p) {
        return false;
    }
    let highest = if q1 <= 0.0 {
        if q2 <= 0.0 {
            return moved(p) < negated;
        }
        rounded(x87_add(p, q2))
    } else if q2 >= q1 {
        rounded(x87_add(p, q2))
    } else {
        rounded(x87_add(p, q1))
    };
    highest < negated && moved(highest) < negated
}

/// The other axes of [`fn_00ea2190`] (the triangle's normal and the nine
/// products of a box axis with an edge): `d` the first vertex's offset
/// projected, `k` how far the one vertex off the first edge's line lies
/// from it on the axis (the other vertex projects to `d`; zero for the
/// normal), `radius` the box's projected radius and `w` the relative
/// velocity's projection (stored floats). Above the radius the lower end
/// (`d`, or `d + k` rounded when `k` is negative or a NaN) must be above
/// it, then that end moved by `w` times the time; below the negated radius
/// (stored and reloaded) the same with the higher end.
fn triangle_edge_axis_separated(d: f64, k: f64, radius: f64, w: f64, time: f64) -> bool {
    let moved = |x: f64| x87_add(x, x87_mul(w, time));
    // `TEST AH,0x5; JP`: the negative side unless the radius is below `d`.
    if radius < d {
        if k >= 0.0 {
            return moved(d) > radius;
        }
        let lowest = rounded(x87_add(d, k));
        return lowest > radius && moved(lowest) > radius;
    }
    let negated = rounded(-radius);
    // `TEST AH,0x41; JNZ`: on to the next axis unless the negated radius
    // is above `d`.
    if !(negated > d) {
        return false;
    }
    if k <= 0.0 {
        return moved(d) < negated;
    }
    let highest = rounded(x87_add(d, k));
    highest < negated && moved(highest) < negated
}

// Translated from 00ea2190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the moving box-triangle test (cdecl, seven
/// words: the time, the box volume, its velocity, the triangle's three
/// vertices, the triangle's velocity). With `W` the relative velocity
/// (the triangle's minus the box's), `E1` and `E2` the edges from the
/// first vertex, `N = E1 x E2` and `D` the first vertex's offset from the
/// box centre (every component rounded), the two are apart when one of
/// these axes separates them: the normal ([`triangle_edge_axis_separated`]
/// with the radius `|e1 (N . A1)| + |e0 (N . A0)| + |e2 (N . A2)|`), the
/// box's three axes ([`triangle_box_axis_separated`]), then for each box
/// axis its products with `E1`, `E2` and `E2 - E1` (the first axis's
/// computed in place, the others through `004b3800`, the second axis's
/// last products' projections through `NiPoint3::Dot`; the third edge's
/// projections the differences of the first two's). Every dot product is
/// `(x + y) + z` and rounded where the code stores it. Answers false as
/// soon as one axis separates, true otherwise.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea2190(
    e: &mut Engine,
    time: f32,
    bx: Ptr,
    box_velocity: Ptr,
    v0: Ptr,
    v1: Ptr,
    v2: Ptr,
    triangle_velocity: Ptr,
) -> bool {
    let (b, bv, tv) = (bx.addr(), box_velocity.addr(), triangle_velocity.addr());
    let (v0, v1, v2) = (v0.addr(), v1.addr(), v2.addr());
    // The frame: `[ESP+0x28]` a copy of the current product axis (`L`),
    // `[ESP+0x34]` `W`, `[ESP+0x74]` the product `004b3800` writes (first
    // `E1`, unused by then), `[ESP+0x84]` a copy of `E1`, `[ESP+0x90]` of
    // `E2`, `[ESP+0x9c]` of `D`.
    e.with_stack(0x48, |e, frame| {
        let f = frame.addr();
        let (l_copy, w_at, out, e1_copy, e2_copy, d_copy) =
            (f, f + 0xc, f + 0x18, f + 0x24, f + 0x30, f + 0x3c);
        let ld = |e: &Engine, x: u32| load_float(e, x);
        let (add, sub, mul) = (x87_add, x87_sub, x87_mul);
        let dot = |p: [f64; 3], q: [f64; 3]| {
            rounded(add(add(mul(p[0], q[0]), mul(p[1], q[1])), mul(p[2], q[2])))
        };
        let cross = |p: [f64; 3], q: [f64; 3]| {
            [
                rounded(sub(mul(p[1], q[2]), mul(p[2], q[1]))),
                rounded(sub(mul(p[2], q[0]), mul(p[0], q[2]))),
                rounded(sub(mul(p[0], q[1]), mul(p[1], q[0]))),
            ]
        };
        let difference = |e: &Engine, p: u32, q: u32| {
            [0u32, 4, 8].map(|k| rounded(sub(ld(e, p + k), ld(e, q + k))))
        };
        let store = |e: &mut Engine, at: u32, v: [f64; 3]| {
            for (k, x) in v.iter().enumerate() {
                e.mem.set_u32(at + 4 * k as u32, x87_store32(*x));
            }
        };
        let w = difference(e, tv, bv);
        let e1 = difference(e, v1, v0);
        let e2 = difference(e, v2, v0);
        store(e, w_at, w);
        store(e, e1_copy, e1);
        store(e, e2_copy, e2);
        let n = cross(e1, e2);
        let d = difference(e, v0, b + CENTER);
        store(e, d_copy, d);
        let axis = |e: &Engine, i: u32| {
            let a = b + AXES + 0xc * i;
            [ld(e, a), ld(e, a + 4), ld(e, a + 8)]
        };
        let extent = |e: &Engine, i: u32| ld(e, b + EXTENTS + 4 * i);
        let t = x87_load32(time.to_bits());
        let axes = [axis(e, 0), axis(e, 1), axis(e, 2)];
        let ext = [extent(e, 0), extent(e, 1), extent(e, 2)];
        let na = axes.map(|a| dot(a, n));
        // The normal: every vertex projects to `D . N`.
        let term = |x: f64, y: f64| rounded(rounded(mul(x, y)).abs());
        let radius = rounded(add(
            add(term(ext[1], na[1]), term(ext[0], na[0])),
            term(na[2], ext[2]),
        ));
        if triangle_edge_axis_separated(dot(d, n), 0.0, radius, dot(w, n), t) {
            return false;
        }
        // The box's axes.
        let mut q1 = [0.0; 3];
        let mut q2 = [0.0; 3];
        for i in 0..3 {
            let a = axes[i];
            let p = dot(a, d);
            q1[i] = dot(a, e1);
            q2[i] = dot(a, e2);
            if triangle_box_axis_separated(p, q1[i], q2[i], dot(a, w), ext[i], t) {
                return false;
            }
        }
        let q3 = [0usize, 1, 2].map(|i| rounded(sub(q2[i], q1[i])));
        // The box axis `i` times an edge: the radius from the edge's
        // projections on the other two axes.
        let edge_radius = |i: usize, q: [f64; 3]| {
            let (j, k) = match i {
                0 => (1, 2),
                1 => (0, 2),
                _ => (0, 1),
            };
            rounded(add(term(ext[j], q[k]), term(ext[k], q[j])))
        };
        // The product axis as `004b3800` returns it, copied to `L`.
        let product = |e: &mut Engine, i: u32, edge: u32| {
            let ret = e.call(POINT_CROSS_004B3800, &args![b + AXES + 0xc * i, out, edge]);
            let at = ret.eax;
            for k in 0..3 {
                let word = e.mem.u32(at + 4 * k);
                e.mem.set_u32(l_copy + 4 * k, word);
            }
            [ld(e, l_copy), ld(e, l_copy + 4), ld(e, l_copy + 8)]
        };
        for i in 0..3usize {
            // `k` for `E1` is `N . Ai`, for `E2` and `E2 - E1` its negation.
            let k = na[i];
            let (d1, w1) = if i == 0 {
                let l = cross(axes[0], e1);
                (dot(l, d), dot(l, w))
            } else {
                let l = product(e, i as u32, e1_copy);
                (dot(l, d), dot(l, w))
            };
            if triangle_edge_axis_separated(d1, k, edge_radius(i, q1), w1, t) {
                return false;
            }
            let (d2, w2) = match i {
                0 => {
                    let l = cross(axes[0], e2);
                    (dot(l, d), dot(l, w))
                }
                1 => {
                    let l = product(e, 1, e2_copy);
                    (dot(l, d), dot(l, w))
                }
                _ => {
                    product(e, 2, e2_copy);
                    let d2 = e.call(POINT_DOT, &args![l_copy, d_copy]).f64();
                    let w2 = e.call(POINT_DOT, &args![l_copy, w_at]).f64();
                    (rounded(d2), rounded(w2))
                }
            };
            if triangle_edge_axis_separated(d2, -k, edge_radius(i, q2), w2, t) {
                return false;
            }
            let d3 = rounded(sub(d2, d1));
            let w3 = rounded(sub(w2, w1));
            if triangle_edge_axis_separated(d3, -k, edge_radius(i, q3), w3, t) {
                return false;
            }
        }
        true
    })
}

/// The `double` (`010c9300`; 2^-23 in the image) a new contact time must
/// exceed the current one by before [`fn_00ea7d40`] takes its axis.
const TIME_SLACK_010C9300: u32 = 0x010c_9300;
/// The `float` 0.5 (`01016248` in the image) the overlap case halves the
/// sum of the two centres with.
const HALF_01016248: u32 = 0x0101_6248;
/// The `double` 1.0 (`01012070` in the image) the normal's squared length
/// is compared with.
const ONE_01012070: u32 = 0x0101_2070;
/// The `float` `1e-6` (`010959f4` in the image): an edge case whose
/// `|1 - c^2|` is not above it takes its own axis's offset as zero.
const PARALLEL_LIMIT_010959F4: u32 = 0x0109_59f4;
/// `00439e90` and `00439ef0` (`thiscall` on a point `p`; out, `q`; `RET 8`;
/// no Xbox name): by their bodies `p + q` and `p - q` into `out`,
/// returning `out`.
const POINT_SUM_00439E90: u32 = 0x0043_9e90;
const POINT_DIFFERENCE_00439EF0: u32 = 0x0043_9ef0;
/// `004a3760` (cdecl: out, a `float`, a point; no Xbox name): by its body
/// the point times the float into `out`, returning `out`.
const POINT_SCALE_004A3760: u32 = 0x004a_3760;
/// `004a7290` (`thiscall` on a point; ST0; no Xbox name): by its body the
/// point's squared length.
const POINT_LENGTH_SQUARED_004A7290: u32 = 0x004a_7290;

/// The bytes of [`fn_00ea7d40`]'s frame (`SUB ESP,0x1e8` under the four
/// saved registers; the locals sit at `[ESP+0x10]` to `[ESP+0x1f7]`) and the
/// locals whose address it hands to its callees: the copy of the relative
/// velocity `V` (`[ESP+0xd8]`), the point `D` the edge cases project
/// (`[ESP+0x54]`), and the overlap case's sum of the centres (`[ESP+0xe4]`)
/// and half of it (`[ESP+0xa8]`).
const FIND_FRAME: u32 = 0x1f8;
const FIND_V_COPY: u32 = 0xd8;
const FIND_D_POINT: u32 = 0x54;
const FIND_CENTRE_SUM: u32 = 0xe4;
const FIND_HALF_SUM: u32 = 0xa8;

/// What [`fn_00ea7d40`] has found so far and what its contact cases read:
/// the arguments, the frame, the axis dot products `c[i][j] = Ai . Bj`, the
/// centre offset `T`, the contact axis (0 when none was taken) and its side
/// (1 or -1, 0 when none).
struct BoxFind {
    time: u32,
    a: u32,
    va: u32,
    b: u32,
    vb: u32,
    out_time: u32,
    point: u32,
    want_normals: bool,
    normal: u32,
    negated: u32,
    frame: u32,
    c: [[f64; 3]; 3],
    t: [f64; 3],
    axis: u32,
    side: i32,
}

/// One separating axis of [`fn_00ea7d40`]: `r` the summed projected radii,
/// `d0` the centre offset's projection now and `d1` (asked only when
/// needed; the face axes have it computed before, the same value) its
/// projection at the end of the motion. Above `r` (`TEST AH,0x5; JP` on
/// `r` against `d0`): apart when `d1` is above `r` too, else the boxes meet
/// at `(r - d0) * time_above / (d1 - d0)` (rounded); below `-r` (stored and
/// reloaded) the same with `(r + d0) * -time / (d1 - d0)`. A NaN takes
/// neither side. A meeting time above the current contact time plus
/// [`TIME_SLACK_010C9300`] (the current one is the `0.0` of `FLDZ` for the
/// first axis, `*out_time` after) becomes the contact time, with this axis
/// and side. Returns true when the axis separates the boxes.
#[allow(clippy::too_many_arguments)]
fn find_axis(
    e: &mut Engine,
    s: &mut BoxFind,
    axis: u32,
    r: f64,
    d0: f64,
    d1: impl FnOnce() -> f64,
    time_above: f64,
    time: f64,
) -> bool {
    let (t, side) = if r < d0 {
        let d1 = d1();
        // `TEST AH,0x41; JNZ`: on unless `d1` is above `r`.
        if d1 > r {
            return true;
        }
        let t = rounded(x87_div(
            x87_mul(x87_sub(r, d0), time_above),
            x87_sub(d1, d0),
        ));
        (t, 1)
    } else {
        let negated = rounded(-r);
        // `TEST AH,0x41; JNZ`: on to the next axis unless `-r` is above `d0`.
        if !(negated > d0) {
            return false;
        }
        let d1 = d1();
        if negated > d1 {
            return true;
        }
        let t = rounded(x87_div(x87_mul(x87_add(r, d0), -time), x87_sub(d1, d0)));
        (t, -1)
    };
    let current = if axis == 1 {
        0.0
    } else {
        load_float(e, s.out_time)
    };
    let slack = f64::from_bits(e.mem.u64(TIME_SLACK_010C9300));
    // `FCOMPP; TEST AH,0x5; JP`: taken only when the slack sum is below.
    if x87_add(current, slack) < t {
        e.mem.set_u32(s.out_time, x87_store32(t));
        s.axis = axis;
        s.side = side;
    }
    false
}

/// The offset along one box axis of a contact point: `c` above zero gives
/// `positive` (`FILD` of the side or its negation) times the half extent at
/// `extent` (a memory operand), below zero the other sign, zero or a NaN
/// `0.0` (the `FLDZ` stored), each product rounded.
fn signed_extent(e: &Engine, c: f64, positive: i32, extent: u32) -> f64 {
    if c > 0.0 {
        rounded(x87_mul(f64::from(positive), float_operand(e, extent)))
    } else if c < 0.0 {
        rounded(x87_mul(
            f64::from(positive.wrapping_neg()),
            float_operand(e, extent),
        ))
    } else {
        0.0
    }
}

/// The contact point of the face and edge cases: box `bx`'s centre moved by
/// `velocity` times the contact time, plus its three axes times `k`, added
/// in that order (`(((ot v + centre) + k0 A0) + k1 A1) + k2 A2`) and
/// stored component by component into `point`. The x of the first axis
/// and the z of the axes `z_memory` names are memory operands, the other
/// axis components loaded; `velocity_x_loaded` is the code that loads the
/// velocity's x and takes the contact time as the operand (the faces of
/// `a`), the others load the time.
fn store_contact_point(
    e: &mut Engine,
    s: &BoxFind,
    bx: u32,
    velocity: u32,
    k: [f64; 3],
    velocity_x_loaded: bool,
    z_memory: [bool; 3],
) {
    for component in 0..3u32 {
        let moved = if component == 0 && velocity_x_loaded {
            x87_mul(load_float(e, velocity), float_operand(e, s.out_time))
        } else {
            x87_mul(
                load_float(e, s.out_time),
                float_operand(e, velocity + 4 * component),
            )
        };
        let mut sum = x87_add(moved, float_operand(e, bx + CENTER + 4 * component));
        for (axis, &offset) in k.iter().enumerate() {
            let field = bx + AXES + 0xc * axis as u32 + 4 * component;
            let memory = match component {
                0 => axis == 0,
                1 => false,
                _ => z_memory[axis],
            };
            let value = if memory {
                float_operand(e, field)
            } else {
                load_float(e, field)
            };
            sum = x87_add(sum, x87_mul(offset, value));
        }
        e.mem.set_u32(s.point + 4 * component, x87_store32(sum));
    }
}

/// `FLD; FCHS; FSTP` of the three floats at `from` into `to` (all three
/// read before the first is written).
fn store_negated_point(e: &mut Engine, to: u32, from: u32) {
    let words = [0u32, 4, 8].map(|k| negated_bits(e.mem.u32(from + k)));
    for (k, word) in words.iter().enumerate() {
        e.mem.set_u32(to + 4 * k as u32, *word);
    }
}

/// The time argument of [`fn_00ea7730`] as [`fn_00ea7d40`] passes it
/// (`FLD; FSTP` onto the stack: a signalling NaN quieted).
fn passed_time(s: &BoxFind) -> f32 {
    f32::from_bits(x87_quiet_bits(s.time))
}

/// The end of [`fn_00ea7d40`] for the overlap and edge cases (`00ea96d5`):
/// without `want_normals` nothing. Otherwise, when the normal's squared
/// length (`004a7290`, rounded) differs from 1.0 by more than `FLT_EPSILON`
/// (`|len2 - 1|` rounded; ordered compares), the normal is unitized
/// (`004a0c10`) when the squared length is above `FLT_EPSILON`, else
/// replaced by the default x axis (`011a946c`, three words). Then the
/// negated normal is stored. Answers true.
fn finish_normal(e: &mut Engine, s: &BoxFind) -> bool {
    if !s.want_normals {
        return true;
    }
    let length = rounded(
        e.call(POINT_LENGTH_SQUARED_004A7290, &args![s.normal])
            .f64(),
    );
    let one = x87_load64(e.mem.u64(ONE_01012070));
    let off = rounded(rounded(x87_sub(length, one)).abs());
    let epsilon = load_float(e, FLT_EPSILON);
    if epsilon < off {
        if epsilon < length {
            e.call(UNITIZE_004A0C10, &args![s.normal]);
        } else {
            copy_point(e, s.normal, DEFAULT_AXIS_X);
        }
    }
    store_negated_point(e, s.negated, s.normal);
    true
}

/// Case 0 of [`fn_00ea7d40`] (no axis was taken: the boxes overlap at the
/// start; the contact time stays 0): the contact point is half the sum of
/// the centres (`00439e90` into `[ESP+0xe4]`, then `004a3760` with the
/// float at `01016248` into `[ESP+0xa8]`, three words copied), the normal
/// the point minus `a`'s centre (`00439ef0` into the copy of `V`), finished
/// by [`finish_normal`].
fn find_overlap(e: &mut Engine, s: &BoxFind) -> bool {
    let f = s.frame;
    let sum = e
        .call(
            POINT_SUM_00439E90,
            &args![s.a + CENTER, f + FIND_CENTRE_SUM, s.b + CENTER],
        )
        .u32();
    let half = x87_quiet_bits(e.mem.u32(HALF_01016248));
    let point = e
        .call(POINT_SCALE_004A3760, &args![f + FIND_HALF_SUM, half, sum])
        .u32();
    copy_point(e, s.point, point);
    if !s.want_normals {
        return true;
    }
    let normal = e
        .call(
            POINT_DIFFERENCE_00439EF0,
            &args![s.point, f + FIND_V_COPY, s.a + CENTER],
        )
        .u32();
    copy_point(e, s.normal, normal);
    finish_normal(e, s)
}

/// Cases 1 to 3 of [`fn_00ea7d40`] (`a`'s axis `i` separated longest): the
/// contact point is on `b`: its offsets along its axes `j` are
/// [`signed_extent`] of `c[i][j]` with the negated side, `b` moved to the
/// contact time ([`store_contact_point`]), then [`fn_00ea7730`] (`b`
/// first). The normal is `a`'s axis `i` (the three words, or negated when
/// the side is negative), the other output its negation; the normal is not
/// unitized. Answers true.
fn find_face_of_a(e: &mut Engine, s: &BoxFind, i: usize) -> bool {
    let k = [0usize, 1, 2].map(|j| {
        signed_extent(
            e,
            s.c[i][j],
            s.side.wrapping_neg(),
            s.b + EXTENTS + 4 * j as u32,
        )
    });
    store_contact_point(e, s, s.b, s.vb, k, true, [false, false, true]);
    let p = |x: u32| Ptr::new(x);
    fn_00ea7730(
        e,
        passed_time(s),
        p(s.b),
        p(s.vb),
        p(s.a),
        p(s.va),
        p(s.out_time),
        p(s.point),
    );
    if !s.want_normals {
        return true;
    }
    let axis = s.a + AXES + 0xc * i as u32;
    if s.side < 0 {
        store_negated_point(e, s.normal, axis);
    } else {
        copy_point(e, s.normal, axis);
    }
    store_negated_point(e, s.negated, s.normal);
    true
}

/// Cases 4 to 6 of [`fn_00ea7d40`] (`b`'s axis `j`): as
/// [`find_face_of_a`] with the boxes' roles exchanged: the point on `a`
/// (offsets [`signed_extent`] of `c[i][j]` with the side itself), the
/// normal `b`'s axis `j`.
fn find_face_of_b(e: &mut Engine, s: &BoxFind, j: usize) -> bool {
    let k =
        [0usize, 1, 2].map(|i| signed_extent(e, s.c[i][j], s.side, s.a + EXTENTS + 4 * i as u32));
    store_contact_point(e, s, s.a, s.va, k, false, [false, false, true]);
    let p = |x: u32| Ptr::new(x);
    fn_00ea7730(
        e,
        passed_time(s),
        p(s.a),
        p(s.va),
        p(s.b),
        p(s.vb),
        p(s.out_time),
        p(s.point),
    );
    if !s.want_normals {
        return true;
    }
    let axis = s.b + AXES + 0xc * j as u32;
    if s.side < 0 {
        store_negated_point(e, s.normal, axis);
    } else {
        copy_point(e, s.normal, axis);
    }
    store_negated_point(e, s.negated, s.normal);
    true
}

/// One edge case of [`fn_00ea7d40`] (`a`'s axis `i` times `b`'s axis `j`):
/// `a_terms` and `b_terms` are the two other axes of each box, each with
/// the `c` entry ([row, column]) whose sign gives its offset and whether a
/// positive entry takes the negated side, in the code's order; `scale` the
/// local `004a3760` writes `V` times the contact time into, `negative` and
/// `positive` the locals `004b3800` writes the normal into for each side,
/// `z_memory` the contact point's operand forms
/// ([`store_contact_point`]).
struct EdgeCase {
    i: usize,
    j: usize,
    a_terms: [(usize, [usize; 2], bool); 2],
    b_terms: [(usize, [usize; 2], bool); 2],
    scale: u32,
    negative: u32,
    positive: u32,
    z_memory: [bool; 3],
}

/// Cases 7 to 15 of [`fn_00ea7d40`], in order (`a`'s axis 0 with `b`'s
/// axes 0, 1, 2, then `a`'s axes 1 and 2).
const EDGE_CASES: [EdgeCase; 9] = [
    EdgeCase {
        i: 0,
        j: 0,
        a_terms: [(1, [2, 0], true), (2, [1, 0], false)],
        b_terms: [(1, [0, 2], true), (2, [0, 1], false)],
        scale: 0x1d4,
        negative: 0x9c,
        positive: 0x54,
        z_memory: [false, false, true],
    },
    EdgeCase {
        i: 0,
        j: 1,
        a_terms: [(1, [2, 1], true), (2, [1, 1], false)],
        b_terms: [(0, [0, 2], false), (2, [0, 0], true)],
        scale: 0xf0,
        negative: 0x44,
        positive: 0x1c,
        z_memory: [false, false, true],
    },
    EdgeCase {
        i: 0,
        j: 2,
        a_terms: [(1, [2, 2], true), (2, [1, 2], false)],
        b_terms: [(0, [0, 1], true), (1, [0, 0], false)],
        scale: 0x1ec,
        negative: 0x28,
        positive: 0x108,
        z_memory: [false, false, true],
    },
    EdgeCase {
        i: 1,
        j: 0,
        a_terms: [(0, [2, 0], false), (2, [0, 0], true)],
        b_terms: [(1, [1, 2], true), (2, [1, 1], false)],
        scale: 0x180,
        negative: 0x120,
        positive: 0x1e0,
        z_memory: [false, true, true],
    },
    EdgeCase {
        i: 1,
        j: 1,
        a_terms: [(0, [2, 1], false), (2, [0, 1], true)],
        b_terms: [(0, [1, 2], false), (2, [1, 0], true)],
        scale: 0x138,
        negative: 0x198,
        positive: 0x150,
        z_memory: [false, true, true],
    },
    EdgeCase {
        i: 1,
        j: 2,
        a_terms: [(0, [2, 2], false), (2, [0, 2], true)],
        b_terms: [(0, [1, 1], true), (1, [1, 0], false)],
        scale: 0x1c8,
        negative: 0x168,
        positive: 0x1b0,
        z_memory: [false, true, true],
    },
    EdgeCase {
        i: 2,
        j: 0,
        a_terms: [(0, [1, 0], true), (1, [0, 0], false)],
        b_terms: [(1, [2, 2], true), (2, [2, 1], false)],
        scale: 0xfc,
        negative: 0x114,
        positive: 0x12c,
        z_memory: [false, false, false],
    },
    EdgeCase {
        i: 2,
        j: 1,
        a_terms: [(0, [1, 1], true), (1, [0, 1], false)],
        b_terms: [(0, [2, 2], false), (2, [2, 0], true)],
        scale: 0x144,
        negative: 0x15c,
        positive: 0x174,
        z_memory: [false, false, false],
    },
    EdgeCase {
        i: 2,
        j: 2,
        a_terms: [(0, [1, 2], true), (1, [0, 2], false)],
        b_terms: [(0, [2, 1], true), (1, [2, 0], false)],
        scale: 0x18c,
        negative: 0x1a4,
        positive: 0x1bc,
        z_memory: [false, false, false],
    },
];

/// An edge case of [`fn_00ea7d40`] (see [`EdgeCase`]): the offsets of the
/// four other axes are [`signed_extent`]s; `a`'s own offset along axis `i`
/// solves for the closest point of the two edges when `|1 - c[i][j]^2|`
/// (rounded) is above the float at `010959f4`: with `D = T + V * time`
/// (`004a3760` on the copy of `V` with the contact time, each sum rounded,
/// stored at `[ESP+0x54]`), `q` the sum of `a`'s other offsets times their
/// `c[.][j]` (a `double`), `r1 = q - Bj . D` (rounded) and the
/// `NiPoint3::Dot`s with `D`, it is
/// `((Ai . D + r1 c[i][j]) + kb c[i][.] + kb' c[i][.]) / (1 - c[i][j]^2)`;
/// otherwise 0. The contact point is on `a` moved to the contact time, then
/// [`fn_00ea7730`]. The normal (with `want_normals`) is `Ai x Bj`
/// (`004b3800`), negated for a negative side, finished by
/// [`finish_normal`].
fn find_edge(e: &mut Engine, s: &BoxFind, edge: &EdgeCase) -> bool {
    let (i, j, c, f) = (edge.i, edge.j, s.c, s.frame);
    let a_axis = |n: usize| s.a + AXES + 0xc * n as u32;
    let b_axis = |n: usize| s.b + AXES + 0xc * n as u32;
    let mut ka = [0.0f64; 3];
    let mut kb = [0.0f64; 3];
    for (n, [row, column], negated) in edge.a_terms {
        let positive = if negated {
            s.side.wrapping_neg()
        } else {
            s.side
        };
        ka[n] = signed_extent(e, c[row][column], positive, s.a + EXTENTS + 4 * n as u32);
    }
    for (n, [row, column], negated) in edge.b_terms {
        let positive = if negated {
            s.side.wrapping_neg()
        } else {
            s.side
        };
        kb[n] = signed_extent(e, c[row][column], positive, s.b + EXTENTS + 4 * n as u32);
    }
    let denominator = rounded(x87_sub(1.0, x87_mul(c[i][j], c[i][j])));
    // `FCOMP float [010959f4]; TEST AH,0x41; JNZ`: solved only when above.
    ka[i] = if rounded(denominator.abs()) > float_operand(e, PARALLEL_LIMIT_010959F4) {
        let contact = x87_quiet_bits(e.mem.u32(s.out_time));
        let moved = e
            .call(
                POINT_SCALE_004A3760,
                &args![f + edge.scale, contact, f + FIND_V_COPY],
            )
            .u32();
        let d = [
            rounded(x87_add(s.t[0], float_operand(e, moved))),
            rounded(x87_add(load_float(e, moved + 4), s.t[1])),
            rounded(x87_add(load_float(e, moved + 8), s.t[2])),
        ];
        for (k, x) in d.iter().enumerate() {
            e.mem
                .set_u32(f + FIND_D_POINT + 4 * k as u32, x87_store32(*x));
        }
        let [(p0, _, _), (p1, _, _)] = edge.a_terms;
        let q = x87_add(x87_mul(ka[p0], c[p0][j]), x87_mul(ka[p1], c[p1][j]));
        let dot_b = e.call(POINT_DOT, &args![b_axis(j), f + FIND_D_POINT]).f64();
        let r1 = rounded(x87_sub(q, dot_b));
        let r1c = x87_mul(r1, c[i][j]);
        let dot_a = e.call(POINT_DOT, &args![a_axis(i), f + FIND_D_POINT]).f64();
        let [(s0, _, _), (s1, _, _)] = edge.b_terms;
        let sum = x87_add(
            x87_add(x87_add(dot_a, r1c), x87_mul(kb[s0], c[i][s0])),
            x87_mul(kb[s1], c[i][s1]),
        );
        rounded(x87_div(sum, denominator))
    } else {
        0.0
    };
    store_contact_point(e, s, s.a, s.va, ka, false, edge.z_memory);
    let p = |x: u32| Ptr::new(x);
    fn_00ea7730(
        e,
        passed_time(s),
        p(s.a),
        p(s.va),
        p(s.b),
        p(s.vb),
        p(s.out_time),
        p(s.point),
    );
    if !s.want_normals {
        return true;
    }
    if s.side < 0 {
        let normal = e
            .call(
                POINT_CROSS_004B3800,
                &args![a_axis(i), f + edge.negative, b_axis(j)],
            )
            .u32();
        store_negated_point(e, s.normal, normal);
    } else {
        let normal = e
            .call(
                POINT_CROSS_004B3800,
                &args![a_axis(i), f + edge.positive, b_axis(j)],
            )
            .u32();
        copy_point(e, s.normal, normal);
    }
    finish_normal(e, s)
}

// Translated from 00ea7d40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (a function Ghidra never created; the box row's box column of
/// the find table set up at `00e99130`): the moving box-box `Find` (cdecl,
/// ten words as [`fn_00ea3950`]: the time, box `a` and its velocity, box
/// `b` and its velocity, `out_time`, `out_point`, `want_normals` (a byte),
/// `out_normal`, `out_normal_negated`). With `V` the relative velocity
/// (`b`'s minus `a`'s), `T` the centre offset (`b - a`), `T1 = T + V time`
/// and `c[i][j] = Ai . Bj` (every value rounded where the code stores it),
/// the fifteen separating axes are tried in turn ([`find_axis`]): `a`'s
/// three axes, `b`'s three, then the nine products `Ai x Bj` (`a`'s axis
/// major); the first that keeps the boxes apart for the whole motion
/// answers false. Otherwise the axis whose meeting time is the latest (by
/// more than the slack) is the contact axis, its time `*out_time` (0 when
/// none was taken), and a `switch` on it (a jump table after the body)
/// stores the contact point and, with `want_normals`, the normal and its
/// negation: [`find_overlap`] (0), [`find_face_of_a`] (1 to 3),
/// [`find_face_of_b`] (4 to 6), [`find_edge`] (7 to 15). Answers true.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ea7d40(
    e: &mut Engine,
    time: f32,
    a: Ptr,
    a_velocity: Ptr,
    b: Ptr,
    b_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    want_normals: bool,
    out_normal: Ptr,
    out_normal_negated: Ptr,
) -> bool {
    e.with_stack(FIND_FRAME, |e, frame| {
        let f = frame.addr();
        let (a, va, b, vb) = (a.addr(), a_velocity.addr(), b.addr(), b_velocity.addr());
        let ot = out_time.addr();
        let ld = |e: &Engine, x: u32| load_float(e, x);
        let op = |e: &Engine, x: u32| float_operand(e, x);
        let time_l = x87_load32(time.to_bits());
        let time_m = x87_m32(time.to_bits());
        let v = [
            rounded(x87_sub(ld(e, vb), op(e, va))),
            rounded(x87_sub(ld(e, vb + 4), op(e, va + 4))),
            rounded(x87_sub(ld(e, vb + 8), op(e, va + 8))),
        ];
        for (k, x) in v.iter().enumerate() {
            e.mem
                .set_u32(f + FIND_V_COPY + 4 * k as u32, x87_store32(*x));
        }
        let t = [
            rounded(x87_sub(ld(e, b + 4), op(e, a + 4))),
            rounded(x87_sub(ld(e, b + 8), op(e, a + 8))),
            rounded(x87_sub(ld(e, b + 0xc), op(e, a + 0xc))),
        ];
        let m = [
            rounded(x87_mul(v[0], time_l)),
            rounded(x87_mul(v[1], time_l)),
            rounded(x87_mul(time_l, v[2])),
        ];
        let t1 = [
            rounded(x87_add(t[0], m[0])),
            rounded(x87_add(m[1], t[1])),
            rounded(x87_add(m[2], t[2])),
        ];
        // `FLDZ; FSTP [EDI]`.
        e.mem.set_u32(ot, 0);
        let mut s = BoxFind {
            time: time.to_bits(),
            a,
            va,
            b,
            vb,
            out_time: ot,
            point: out_point.addr(),
            want_normals,
            normal: out_normal.addr(),
            negated: out_normal_negated.addr(),
            frame: f,
            c: [[0.0; 3]; 3],
            t,
            axis: 0,
            side: 0,
        };
        let mut c = [[0.0f64; 3]; 3];
        let mut ac = [[0.0f64; 3]; 3];
        let mut da = [0.0f64; 3];
        let mut d1a = [0.0f64; 3];
        // `a`'s first axis.
        c[0] = [
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x14), op(e, a + 0x14)),
                    x87_mul(ld(e, b + 0x10), op(e, a + 0x10)),
                ),
                x87_mul(ld(e, b + 0x18), op(e, a + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x20), op(e, a + 0x14)),
                    x87_mul(ld(e, b + 0x1c), op(e, a + 0x10)),
                ),
                x87_mul(ld(e, b + 0x24), op(e, a + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x2c), op(e, a + 0x14)),
                    x87_mul(ld(e, a + 0x10), op(e, b + 0x28)),
                ),
                x87_mul(ld(e, b + 0x30), op(e, a + 0x18)),
            )),
        ];
        da[0] = rounded(x87_add(
            x87_add(
                x87_mul(t[0], op(e, a + 0x10)),
                x87_mul(t[1], op(e, a + 0x14)),
            ),
            x87_mul(t[2], op(e, a + 0x18)),
        ));
        d1a[0] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x10), t1[0]),
                x87_mul(t1[1], op(e, a + 0x14)),
            ),
            x87_mul(t1[2], op(e, a + 0x18)),
        ));
        // `FABS` of a stored float, stored and reloaded twice: exact.
        ac[0] = c[0].map(f64::abs);
        let rb = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x38), ac[0][1]),
                x87_mul(ld(e, b + 0x34), ac[0][0]),
            ),
            x87_mul(ac[0][2], op(e, b + 0x3c)),
        ));
        let r = rounded(x87_add(rb, op(e, a + 0x34)));
        let d1 = d1a[0];
        if find_axis(e, &mut s, 1, r, da[0], || d1, time_l, time_l) {
            return false;
        }
        // `a`'s second axis.
        c[1] = [
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, a + 0x20), op(e, b + 0x14)),
                    x87_mul(ld(e, b + 0x10), op(e, a + 0x1c)),
                ),
                x87_mul(ld(e, a + 0x24), op(e, b + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x20), op(e, a + 0x20)),
                    x87_mul(ld(e, b + 0x1c), op(e, a + 0x1c)),
                ),
                x87_mul(ld(e, b + 0x24), op(e, a + 0x24)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x2c), op(e, a + 0x20)),
                    x87_mul(ld(e, b + 0x28), op(e, a + 0x1c)),
                ),
                x87_mul(ld(e, b + 0x30), op(e, a + 0x24)),
            )),
        ];
        da[1] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x20), t[1]),
                x87_mul(t[0], op(e, a + 0x1c)),
            ),
            x87_mul(ld(e, a + 0x24), t[2]),
        ));
        d1a[1] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x20), t1[1]),
                x87_mul(ld(e, a + 0x1c), t1[0]),
            ),
            x87_mul(ld(e, a + 0x24), t1[2]),
        ));
        ac[1] = c[1].map(f64::abs);
        let rb = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x38), ac[1][1]),
                x87_mul(ld(e, b + 0x34), ac[1][0]),
            ),
            x87_mul(ac[1][2], op(e, b + 0x3c)),
        ));
        let r = rounded(x87_add(rb, op(e, a + 0x38)));
        let d1 = d1a[1];
        if find_axis(e, &mut s, 2, r, da[1], || d1, time_l, time_l) {
            return false;
        }
        // `a`'s third axis.
        c[2] = [
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, a + 0x2c), op(e, b + 0x14)),
                    x87_mul(ld(e, a + 0x28), op(e, b + 0x10)),
                ),
                x87_mul(ld(e, a + 0x30), op(e, b + 0x18)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x20), op(e, a + 0x2c)),
                    x87_mul(ld(e, b + 0x1c), op(e, a + 0x28)),
                ),
                x87_mul(ld(e, b + 0x24), op(e, a + 0x30)),
            )),
            rounded(x87_add(
                x87_add(
                    x87_mul(ld(e, b + 0x2c), op(e, a + 0x2c)),
                    x87_mul(ld(e, b + 0x28), op(e, a + 0x28)),
                ),
                x87_mul(ld(e, b + 0x30), op(e, a + 0x30)),
            )),
        ];
        da[2] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x2c), t[1]),
                x87_mul(t[0], op(e, a + 0x28)),
            ),
            x87_mul(ld(e, a + 0x30), t[2]),
        ));
        d1a[2] = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x2c), t1[1]),
                x87_mul(ld(e, a + 0x28), t1[0]),
            ),
            x87_mul(ld(e, a + 0x30), t1[2]),
        ));
        ac[2] = c[2].map(f64::abs);
        let rb = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x38), ac[2][1]),
                x87_mul(ld(e, b + 0x34), ac[2][0]),
            ),
            x87_mul(ac[2][2], op(e, b + 0x3c)),
        ));
        let r = rounded(x87_add(rb, op(e, a + 0x3c)));
        let d1 = d1a[2];
        if find_axis(e, &mut s, 3, r, da[2], || d1, time_l, time_l) {
            return false;
        }
        s.c = c;
        // `b`'s axes (the meeting time of the first two takes the time as a
        // memory operand).
        let db0 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x10), t[0]),
                x87_mul(t[1], op(e, b + 0x14)),
            ),
            x87_mul(t[2], op(e, b + 0x18)),
        ));
        let d1b0 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x10), t1[0]),
                x87_mul(t1[1], op(e, b + 0x14)),
            ),
            x87_mul(t1[2], op(e, b + 0x18)),
        ));
        let ra = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x38), ac[1][0]),
                x87_mul(ac[0][0], op(e, a + 0x34)),
            ),
            x87_mul(ld(e, a + 0x3c), ac[2][0]),
        ));
        let r = rounded(x87_add(ra, op(e, b + 0x34)));
        if find_axis(e, &mut s, 4, r, db0, || d1b0, time_m, time_l) {
            return false;
        }
        let db1 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x20), t[1]),
                x87_mul(t[0], op(e, b + 0x1c)),
            ),
            x87_mul(ld(e, b + 0x24), t[2]),
        ));
        let d1b1 = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, b + 0x20), t1[1]),
                x87_mul(ld(e, b + 0x1c), t1[0]),
            ),
            x87_mul(ld(e, b + 0x24), t1[2]),
        ));
        let ra = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x38), ac[1][1]),
                x87_mul(ac[0][1], op(e, a + 0x34)),
            ),
            x87_mul(ld(e, a + 0x3c), ac[2][1]),
        ));
        let r = rounded(x87_add(ra, op(e, b + 0x38)));
        if find_axis(e, &mut s, 5, r, db1, || d1b1, time_m, time_l) {
            return false;
        }
        let db2 = rounded(x87_add(
            x87_mul(t[2], ld(e, b + 0x30)),
            x87_add(
                x87_mul(t[0], ld(e, b + 0x28)),
                x87_mul(t[1], ld(e, b + 0x2c)),
            ),
        ));
        let d1b2 = rounded(x87_add(
            x87_add(
                x87_mul(t1[1], ld(e, b + 0x2c)),
                x87_mul(t1[0], op(e, b + 0x28)),
            ),
            x87_mul(ld(e, b + 0x30), t1[2]),
        ));
        let ra = rounded(x87_add(
            x87_add(
                x87_mul(ld(e, a + 0x38), ac[1][2]),
                x87_mul(ac[0][2], op(e, a + 0x34)),
            ),
            x87_mul(ld(e, a + 0x3c), ac[2][2]),
        ));
        let r = rounded(x87_add(ra, op(e, b + 0x3c)));
        if find_axis(e, &mut s, 6, r, db2, || d1b2, time_l, time_l) {
            return false;
        }
        // The nine products `Ai x Bj`: `d0` and `d1` from `a`'s projections,
        // the radii as two rounded sums (each pair in the code's operand
        // forms).
        let cross = |p: f64, x: f64, q: f64, y: f64| rounded(x87_sub(x87_mul(p, x), x87_mul(q, y)));
        let (eb, ea) = (|k: u32| b + EXTENTS + 4 * k, |k: u32| a + EXTENTS + 4 * k);
        let pair = |x: f64, y: f64| rounded(x87_add(x, y));
        // A0 x B0.
        let d0 = cross(da[2], c[1][0], c[2][0], da[1]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[0][2]),
                x87_mul(ac[0][1], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[2][0]),
                x87_mul(ld(e, ea(2)), ac[1][0]),
            ),
        );
        let d1 = || cross(c[1][0], d1a[2], c[2][0], d1a[1]);
        if find_axis(e, &mut s, 7, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A0 x B1.
        let d0 = cross(da[2], c[1][1], c[2][1], da[1]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(1)), ac[2][1]),
                x87_mul(ld(e, ea(2)), ac[1][1]),
            ),
            pair(
                x87_mul(ac[0][2], ld(e, eb(0))),
                x87_mul(ac[0][0], op(e, eb(2))),
            ),
        );
        let d1 = || cross(c[1][1], d1a[2], c[2][1], d1a[1]);
        if find_axis(e, &mut s, 8, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A0 x B2.
        let d0 = cross(da[2], c[1][2], c[2][2], da[1]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(1)), ac[2][2]),
                x87_mul(ld(e, ea(2)), ac[1][2]),
            ),
            pair(
                x87_mul(ld(e, eb(1)), ac[0][0]),
                x87_mul(ld(e, eb(0)), ac[0][1]),
            ),
        );
        let d1 = || cross(c[1][2], d1a[2], c[2][2], d1a[1]);
        if find_axis(e, &mut s, 9, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A1 x B0.
        let d0 = cross(c[2][0], da[0], da[2], c[0][0]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[1][2]),
                x87_mul(ac[1][1], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(2)), ac[0][0]),
                x87_mul(ac[2][0], op(e, ea(0))),
            ),
        );
        let d1 = || cross(c[2][0], d1a[0], c[0][0], d1a[2]);
        if find_axis(e, &mut s, 10, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A1 x B1.
        let d0 = cross(c[2][1], da[0], da[2], c[0][1]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(2)), ac[0][1]),
                x87_mul(ac[2][1], op(e, ea(0))),
            ),
            pair(
                x87_mul(ld(e, eb(0)), ac[1][2]),
                x87_mul(ac[1][0], op(e, eb(2))),
            ),
        );
        let d1 = || cross(c[2][1], d1a[0], c[0][1], d1a[2]);
        if find_axis(e, &mut s, 11, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A1 x B2.
        let d0 = cross(c[2][2], da[0], da[2], c[0][2]);
        let r = pair(
            pair(
                x87_mul(ld(e, ea(2)), ac[0][2]),
                x87_mul(ac[2][2], op(e, ea(0))),
            ),
            pair(
                x87_mul(ld(e, eb(1)), ac[1][0]),
                x87_mul(ld(e, eb(0)), ac[1][1]),
            ),
        );
        let d1 = || cross(c[2][2], d1a[0], c[0][2], d1a[2]);
        if find_axis(e, &mut s, 12, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A2 x B0.
        let d0 = cross(da[1], c[0][0], c[1][0], da[0]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[2][2]),
                x87_mul(ac[2][1], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[0][0]),
                x87_mul(ac[1][0], op(e, ea(0))),
            ),
        );
        let d1 = || cross(d1a[1], c[0][0], c[1][0], d1a[0]);
        if find_axis(e, &mut s, 13, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A2 x B1.
        let d0 = cross(da[1], c[0][1], c[1][1], da[0]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(0)), ac[2][2]),
                x87_mul(ac[2][0], op(e, eb(2))),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[0][1]),
                x87_mul(ac[1][1], op(e, ea(0))),
            ),
        );
        let d1 = || cross(d1a[1], c[0][1], c[1][1], d1a[0]);
        if find_axis(e, &mut s, 14, r, d0, d1, time_l, time_l) {
            return false;
        }
        // A2 x B2.
        let d0 = cross(da[1], c[0][2], da[0], c[1][2]);
        let r = pair(
            pair(
                x87_mul(ld(e, eb(1)), ac[2][0]),
                x87_mul(ld(e, eb(0)), ac[2][1]),
            ),
            pair(
                x87_mul(ld(e, ea(1)), ac[0][2]),
                x87_mul(ac[1][2], op(e, ea(0))),
            ),
        );
        let d1 = || cross(d1a[1], c[0][2], d1a[0], c[1][2]);
        if find_axis(e, &mut s, 15, r, d0, d1, time_l, time_l) {
            return false;
        }
        // The jump table at `00eab768` (the compiler's bound check `JBE` on
        // 15 lets a larger value fall through to the normal's end, which
        // cannot happen: the axis is one of the fifteen or 0).
        match s.axis {
            0 => find_overlap(e, &s),
            1..=3 => find_face_of_a(e, &s, s.axis as usize - 1),
            4..=6 => find_face_of_b(e, &s, s.axis as usize - 4),
            7..=15 => find_edge(e, &s, &EDGE_CASES[s.axis as usize - 7]),
            _ => finish_normal(e, &s),
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_e840, ni_box_bv_operator_(Ptr<NiBoxBV>, Ptr) -> bool),
        entry!(
            0x00ea_07a0,
            ni_box_bv_operator_ov2(Ptr<NiBoxBV>, Ptr) -> bool
        ),
        entry!(0x00ea_07e0, ni_box_bv_load_binary(Ptr<NiBoxBV>, Ptr)),
        entry!(0x00ea_0800, ni_box_bv_save_binary(Ptr<NiBoxBV>, Ptr)),
        entry!(
            0x00ea_0820,
            fn_00ea0820(Ptr<NiBoxBV>, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr<NiBoxBV>
        ),
        entry!(0x00ea_08a0, ni_box_bv_copy(Ptr<NiBoxBV>, Ptr<NiBoxBV>)),
        entry!(
            0x00ea_08c0,
            ni_box_bv_update_world_data(Ptr<NiBoxBV>, Ptr<NiBoxBV>, Ptr)
        ),
        entry!(0x00ea_2030, fn_00ea2030(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00ea_3950,
            fn_00ea3950(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00ea_3a60,
            ni_box_bv_get_normal(Ptr<NiBoxBV>, Ptr, i32, i32, i32, f32, f32, f32) -> Ptr
        ),
        entry!(
            0x00ea_3c50,
            fn_00ea3c50(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
        entry!(0x00ea_7490, fn_00ea7490(Ptr) -> Ptr<NiBoxBV>),
        entry!(0x00ea_74f0, ni_box_bv_create() -> Ptr<NiBoxBV>),
        entry!(0x00ea_7530, fn_00ea7530(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00ea_7d10, ni_box_bv_clone(Ptr<NiBoxBV>) -> Ptr<NiBoxBV>),
        entry!(
            0x00ea_b7b0,
            ni_box_bv_add_viewer_strings(Ptr<NiBoxBV>, Ptr, Ptr)
        ),
        entry!(
            0x00ea_c1d0,
            ni_box_bv_scalar_deleting_destructor(Ptr<NiBoxBV>, u32) -> Ptr<NiBoxBV>
        ),
        entry!(0x00ea_7730, fn_00ea7730(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00ea_0ad0, fn_00ea0ad0(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00ea_2190,
            fn_00ea2190(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00ea_7d40,
            fn_00ea7d40(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, bool, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::units::nicollision::nicollisiondata::{VIEWER_STRINGS_GROW, VIEWER_STRINGS_SET_AT};

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    fn read(e: &Engine, p: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(p + 4 * i)).collect()
    }

    /// A box volume: centre, three axes, three extents.
    fn box_volume(e: &mut Engine, values: [f32; 15]) -> u32 {
        let b = e.mem.alloc(0x40);
        e.mem.set_u32(b, NI_BOX_BV_VTABLE);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(b + 4 + 4 * i as u32, *v);
        }
        b
    }

    /// An axis-aligned box at `center` with half extents `extent`.
    fn aligned(e: &mut Engine, center: [f32; 3], extent: [f32; 3]) -> u32 {
        box_volume(
            e,
            [
                center[0], center[1], center[2], 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                extent[0], extent[1], extent[2],
            ],
        )
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Data pages the box functions read: the zero point and default
    /// axes, the extents, the epsilon and the intersector floats.
    fn data(e: &mut Engine) {
        e.map(0x011f_4000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        for (i, v) in [
            1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_f32(DEFAULT_AXIS_X + 4 * i as u32, *v);
        }
        e.map(0x0104_d000, 0x1000);
        e.mem.set_u32(FLT_EPSILON, 0x3400_0000);
        e.map(0x0101_4000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_f32(PSEUDODISTANCE_THRESHOLD, 0.0001);
        e.mem.set_f32(DERIVATIVE_TIME_STEP, 0.001);
    }

    #[test]
    fn operators_compare_types_and_boxes() {
        let mut e = Engine::new();
        // The other volume's `Type` (slot +0xc of a test vtable) answers its
        // word at +0x40.
        e.register(0x0f00_000c, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.put_vtable(0x0f00_1000, &[0, 0, 0, 0x0f00_000c]);
        e.register(NI_BOX_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let this = e.mem.alloc(0x44);
        let other = e.mem.alloc(0x44);
        e.mem.set_u32(other, 0x0f00_1000);
        e.mem.set_u32(other + 0x40, 1);
        e.mem.set_u32(this + 4, 7);
        e.mem.set_u32(other + 4, 7);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ea_07a0, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NI_BOX_EQUAL), vec![vec![other + 4, this + 4]]);
        e.mem.set_u32(other + 4, 8);
        assert!(!e.call(0x00ea_07a0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 7);
        e.mem.set_u32(other + 0x40, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ea_07a0, &args![this, other]).bool());
        assert!(calls(&e.call_log.take().unwrap(), NI_BOX_EQUAL).is_empty());
        // `operator!=`: the negation of slot +0x24 on this volume.
        e.register(0x0f00_0024, |_, a| u32::from(a[1] == 0x1234).into_ret());
        e.put_vtable(0x0f00_2000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f00_0024]);
        e.mem.set_u32(this, 0x0f00_2000);
        assert!(!e.call(0x00e9_e840, &args![this, 0x1234u32]).bool());
        assert!(e.call(0x00e9_e840, &args![this, 0x1235u32]).bool());
    }

    #[test]
    fn binary_io_runs_the_base_then_the_box() {
        for (addr, base, part) in [
            (
                0x00ea_07e0u32,
                NI_BOUNDING_VOLUME_LOAD_BINARY,
                NI_BOX_LOAD_BINARY,
            ),
            (
                0x00ea_0800,
                NI_BOUNDING_VOLUME_SAVE_BINARY,
                NI_BOX_SAVE_BINARY,
            ),
        ] {
            let mut e = Engine::new();
            e.register(base, nothing);
            e.register(part, nothing);
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x5000u32, 0x6000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [(base, vec![0x5000, 0x6000]), (part, vec![0x5004, 0x6000])]
            );
        }
    }

    #[test]
    fn constructor_copy_and_clone() {
        let mut e = Engine::new();
        let extents = floats(&mut e, &[1.0, 2.0, f32::from_bits(0x7f80_0001)]);
        let center = floats(&mut e, &[4.0, 5.0, 6.0]);
        let x = floats(&mut e, &[1.0, 0.0, 0.0]);
        let y = floats(&mut e, &[0.0, 1.0, 0.0]);
        let z = floats(&mut e, &[0.0, 0.0, 1.0]);
        let this = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00ea_0820, &args![this, extents, center, x, y, z])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), NI_BOX_BV_VTABLE);
        assert_eq!(read(&e, this + 4, 3), vec![4.0, 5.0, 6.0]);
        assert_eq!(
            read(&e, this + 0x10, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 0x34, 2), vec![1.0, 2.0]);
        // The extents go through FLD/FSTP: the signalling NaN is quieted.
        assert_eq!(e.mem.u32(this + 0x3c), 0x7fc0_0001);
        // Copy: the 15 words, the vtable kept.
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other, 0xdead);
        e.call(0x00ea_08a0, &args![other, this]);
        assert_eq!(e.mem.u32(other), 0xdead);
        for word in 1..16u32 {
            assert_eq!(e.mem.u32(other + 4 * word), e.mem.u32(this + 4 * word));
        }
        // Clone: a block from the memory manager with the vtable and words.
        e.register(MEMORY_ALLOCATE, |e, a| {
            assert_eq!(a, &[MEMORY_MANAGER, 0x40]);
            e.mem.alloc(0x40).into_ret()
        });
        let copy = e.call(0x00ea_7d10, &args![other]).u32();
        assert_eq!(e.mem.u32(copy), NI_BOX_BV_VTABLE);
        for word in 1..16u32 {
            assert_eq!(e.mem.u32(copy + 4 * word), e.mem.u32(this + 4 * word));
        }
        e.register(MEMORY_ALLOCATE, nothing);
        assert_eq!(e.call(0x00ea_7d10, &args![other]).u32(), 0);
    }

    #[test]
    fn create_and_load_build_the_default_box() {
        let mut e = Engine::new();
        data(&mut e);
        e.register(MEMORY_ALLOCATE, |e, _| e.mem.alloc(0x40).into_ret());
        let made = e.call(0x00ea_74f0, &[]).u32();
        assert_eq!(e.mem.u32(made), NI_BOX_BV_VTABLE);
        assert_eq!(read(&e, made + 4, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(
            read(&e, made + 0x10, 12),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]
        );
        // The loader makes the same box and runs its `LoadBinary` (slot 0).
        e.register(0x0f00_0000, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.put_vtable(NI_BOX_BV_VTABLE, &[0x0f00_0000]);
        let loaded = e.call(0x00ea_7490, &args![0x6000u32]).u32();
        assert_eq!(e.mem.u32(loaded + 4), 0x6000);
        assert_eq!(e.mem.f32(loaded + 0x34), 1.0);
        e.register(MEMORY_ALLOCATE, nothing);
        assert_eq!(e.call(0x00ea_74f0, &[]).u32(), 0);
    }

    #[test]
    fn update_world_data_moves_the_box() {
        let mut e = Engine::new();
        // A rotation by 90 degrees about z, translation (10, 20, 30),
        // scale 2.
        let transform = floats(
            &mut e,
            &[
                0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 2.0,
            ],
        );
        let model = box_volume(
            &mut e,
            [
                1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, 1.5, 2.5,
            ],
        );
        let this = e.mem.alloc(0x40);
        e.call(0x00ea_08c0, &args![this, model, transform]);
        // Centre: R (1,2,3) = (-2, 1, 3), times 2, plus the translation.
        assert_eq!(read(&e, this + 4, 3), vec![6.0, 22.0, 36.0]);
        assert_eq!(
            read(&e, this + 0x10, 9),
            vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 0x34, 3), vec![1.0, 3.0, 5.0]);
        // In place (the model is the volume itself) gives the same.
        e.call(0x00ea_08c0, &args![model, model, transform]);
        for word in 1..16u32 {
            assert_eq!(e.mem.u32(model + 4 * word), e.mem.u32(this + 4 * word));
        }
    }

    #[test]
    fn get_normal_adds_the_signed_axes() {
        let mut e = Engine::new();
        data(&mut e);
        e.register(UNITIZE_004A0C10, nothing);
        let this = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let out = e.mem.alloc(0xc);
        // A still velocity: every signed axis counts; two of them: unitized.
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x00ea_3a60,
                &args![this, out, 1i32, -1i32, 0i32, 0.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        assert_eq!(back, out);
        assert_eq!(read(&e, out, 3), vec![1.0, -1.0, 0.0]);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), UNITIZE_004A0C10),
            vec![vec![out]]
        );
        // Moving along x: only the x axis is above the epsilon; one axis,
        // not unitized.
        e.call_log = Some(vec![]);
        e.call(
            0x00ea_3a60,
            &args![this, out, -1i32, 1i32, 1i32, 2.0f32, 0.0f32, 0.0f32],
        );
        assert_eq!(read(&e, out, 3), vec![-1.0, 0.0, 0.0]);
        assert!(calls(&e.call_log.take().unwrap(), UNITIZE_004A0C10).is_empty());
        // No sign of 1 or -1: nothing applied, unitized (of zero).
        e.call_log = Some(vec![]);
        e.call(
            0x00ea_3a60,
            &args![this, out, 2i32, 0i32, 5i32, 0.0f32, 0.0f32, 1.0f32],
        );
        assert_eq!(read(&e, out, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), UNITIZE_004A0C10).len(),
            1
        );
        // A NaN velocity is not still and no projection is above.
        e.call(
            0x00ea_3a60,
            &args![this, out, 1i32, 1i32, 1i32, f32::NAN, 0.0f32, 0.0f32],
        );
        assert_eq!(read(&e, out, 3), vec![0.0, 0.0, 0.0]);
    }

    /// Doubles for an intersector at `[ESP+0xc]`: the constructor records
    /// nothing, `Test`/`Find` nothing, the contact time 0.5, the type the
    /// first word of `kind`, the point and normal two fixed triples.
    fn intersector(e: &mut Engine, kind: u32) -> (u32, u32) {
        let point = floats(e, &[1.0, 2.0, 3.0]);
        let normal = floats(e, &[0.0, 1.0, f32::from_bits(0x7f80_0001)]);
        for addr in [
            INTERSECTOR_CONSTRUCTOR_00EB3C40,
            INTERSECTOR_CONSTRUCTOR_00EB3890,
            INTERSECTOR_TEST,
            INTERSECTOR_FIND,
            INTERSECTOR_DESTRUCTOR,
        ] {
            e.register(addr, nothing);
        }
        e.register(INTERSECTOR_CONTACT_TIME, |_, _| st0(0.5));
        e.register_double(INTERSECTOR_TYPE, move |_, _| kind.into_ret());
        e.register_double(INTERSECTOR_CONTACT_POINT, move |_, _| point.into_ret());
        e.register_double(INTERSECTOR_CONTACT_NORMAL, move |_, _| normal.into_ret());
        (point, normal)
    }

    #[test]
    fn finds_run_their_intersector() {
        for (addr, constructor) in [
            (0x00ea_3950u32, INTERSECTOR_CONSTRUCTOR_00EB3C40),
            (0x00ea_3c50, INTERSECTOR_CONSTRUCTOR_00EB3890),
        ] {
            for (kind, hit) in [(3u32, true), (2, true), (1, false)] {
                let mut e = Engine::new();
                data(&mut e);
                intersector(&mut e, kind);
                let outs = e.mem.alloc(0x28);
                let (time, point, normal, negated) = (outs, outs + 4, outs + 0x10, outs + 0x1c);
                e.call_log = Some(vec![]);
                let result = e
                    .call(
                        addr,
                        &args![
                            2.0f32, 0x5000u32, 0x5100u32, 0x5200u32, 0x5300u32, time, point, true,
                            normal, negated
                        ],
                    )
                    .bool();
                assert_eq!(result, hit);
                let log = e.call_log.take().unwrap();
                let made = calls(&log, constructor);
                assert_eq!(
                    &made[0][1..],
                    &[
                        0x5000,
                        0x5200,
                        2.0f32.to_bits(),
                        0.001f32.to_bits(),
                        0.0001f32.to_bits(),
                        0x20
                    ]
                );
                assert_eq!(
                    calls(&log, INTERSECTOR_FIND),
                    vec![vec![made[0][0], 0x5100, 0x5300]]
                );
                assert_eq!(calls(&log, INTERSECTOR_DESTRUCTOR).len(), 1);
                assert_eq!(e.mem.f32(time), 0.5);
                if hit {
                    assert_eq!(read(&e, point, 3), vec![1.0, 2.0, 3.0]);
                    assert_eq!(read(&e, normal, 2), vec![0.0, 1.0]);
                    assert_eq!(e.mem.u32(negated), 0x8000_0000);
                    assert_eq!(e.mem.f32(negated + 4), -1.0);
                    // FLD; FCHS; FSTP: the signalling NaN quieted, negated.
                    assert_eq!(e.mem.u32(negated + 8), 0xffc0_0001);
                }
            }
        }
    }

    #[test]
    fn find_without_normals_leaves_them() {
        let mut e = Engine::new();
        data(&mut e);
        intersector(&mut e, 3);
        let outs = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00ea_3950,
                &args![
                    1.0f32,
                    0x5000u32,
                    0x5100u32,
                    0x5200u32,
                    0x5300u32,
                    outs,
                    outs + 4,
                    0x100u32,
                    outs + 0x10,
                    outs + 0x1c
                ],
            )
            .bool());
        assert!(calls(&e.call_log.take().unwrap(), INTERSECTOR_CONTACT_NORMAL).is_empty());
        assert_eq!(e.mem.u32(outs + 0x10), 0);
    }

    #[test]
    fn capsule_test_accepts_an_inside_centre_else_asks_the_intersector() {
        let mut e = Engine::new();
        data(&mut e);
        intersector(&mut e, 1);
        let b = aligned(&mut e, [0.0; 3], [1.0, 2.0, 3.0]);
        let other = floats(&mut e, &[0.0, 0.5, -1.5, 2.5]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        assert!(calls(&e.call_log.take().unwrap(), INTERSECTOR_TEST).is_empty());
        // On the face (|offset| equal to the extent) still counts.
        e.mem.set_f32(other + 0xc, 3.0);
        assert!(e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        // Outside on z: the intersector decides (type 1: no).
        e.mem.set_f32(other + 0xc, 3.5);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let made = calls(&log, INTERSECTOR_CONSTRUCTOR_00EB3890);
        assert_eq!(made[0][1..3], [b, other]);
        assert_eq!(
            calls(&log, INTERSECTOR_TEST),
            vec![vec![made[0][0], 0x5100, 0x5300]]
        );
        assert_eq!(calls(&log, INTERSECTOR_DESTRUCTOR).len(), 1);
        intersector(&mut e, 2);
        assert!(e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
        // A NaN offset is outside.
        e.mem.set_u32(other + 4, 0x7fc0_0000);
        intersector(&mut e, 0);
        assert!(!e
            .call(0x00ea_2030, &args![1.0f32, b, 0x5100u32, other, 0x5300u32])
            .bool());
    }

    #[test]
    fn sphere_test_measures_the_distance_when_outside() {
        let mut e = Engine::new();
        data(&mut e);
        let b = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // A sphere volume: vtable, centre at +4, radius at +0x10.
        let sphere = floats(&mut e, &[0.0, 0.5, 0.5, 0.5, 2.0]);
        let still = floats(&mut e, &[1.0, 1.0, 1.0]);
        // Inside: true without a distance.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_7530, &args![1.0f32, b, still, sphere, still])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, POINT_BOX_DISTANCE_00EB42E0).is_empty());
        // Outside and still (equal velocities): the point distance, 4.0 is
        // the radius squared exactly.
        e.mem.set_f32(sphere + 4, 5.0);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(4.0));
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_7530, &args![1.0f32, b, still, sphere, still])
            .bool());
        let log = e.call_log.take().unwrap();
        let measured = calls(&log, POINT_BOX_DISTANCE_00EB42E0);
        assert_eq!(measured[0][0..2], [sphere + 4, b + 4]);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(4.000_001));
        assert!(!e
            .call(0x00ea_7530, &args![1.0f32, b, still, sphere, still])
            .bool());
        // Moving: the segment from the centre along time * (sphere - box).
        let moving = floats(&mut e, &[1.0, 3.0, 1.0]);
        e.register(SEGMENT_BOX_DISTANCE_00EB50F0, |e, a| {
            let segment: Vec<f32> = (0..6).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            assert_eq!(segment, vec![5.0, 0.5, 0.5, 0.0, 4.0, 0.0]);
            st0(1.0)
        });
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00ea_7530, &args![2.0f32, b, still, sphere, moving])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SEGMENT_BOX_DISTANCE_00EB50F0)[0][1], b + 4);
        assert!(calls(&log, POINT_BOX_DISTANCE_00EB42E0).is_empty());
    }

    #[test]
    fn viewer_strings_and_destructor() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, nothing);
        e.register(VIEWER_STRINGS_GROW, nothing);
        e.register(VIEWER_STRINGS_SET_AT, nothing);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"bound");
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ea_b7b0, &args![0u32, name, strings]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ALLOCATE), vec![vec![15]]);
        let printed = calls(&log, NI_SPRINTF);
        assert_eq!(printed[0][1..], [15, BOX_BV_FORMAT, name]);
        assert!(calls(&log, VIEWER_STRINGS_GROW).is_empty());
        assert_eq!(calls(&log, VIEWER_STRINGS_SET_AT)[0][1], 2);
        // The scalar deleting destructor.
        e.register(MEMORY_DEALLOCATE, nothing);
        let volume = e.mem.alloc(0x40);
        e.mem.set_u32(volume, NI_BOX_BV_VTABLE);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ea_c1d0, &args![volume, flags]).u32(), volume);
            assert_eq!(e.mem.u32(volume), NI_BOUNDING_VOLUME_VTABLE);
            let log = e.call_log.take().unwrap();
            let freed = calls(&log, MEMORY_DEALLOCATE);
            assert_eq!(freed.len(), frees);
            if frees == 1 {
                assert_eq!(freed[0], vec![MEMORY_MANAGER, volume]);
            }
        }
    }

    #[test]
    fn contact_point_is_clamped_into_both_boxes() {
        let mut e = Engine::new();
        data(&mut e);
        e.map(PROBE_RADIUS & !0xfff, 0x1000);
        e.mem.set_f32(PROBE_RADIUS, 0.01);
        e.map(CLAMP_FACTOR & !0xfff, 0x1000);
        e.mem.set_u64(CLAMP_FACTOR, 0x3fef_ae14_8000_0000);
        e.register(SPHERE_BV_CONSTRUCT_00EAB8A0, |e, a| {
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        // Far from the probe sphere: never touching.
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(100.0));
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let b = aligned(&mut e, [3.0, 0.0, 0.0], [1.0; 3]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let time = floats(&mut e, &[0.5]);
        let point = floats(&mut e, &[5.0, 0.25, -3.0]);
        e.call(0x00ea_7730, &args![1.0f32, a, still, b, still, time, point]);
        // Into a: x clamped to 1 (above 0.99), z to -1; then into b: x
        // clamped to 3 - 1 (below -0.99 of b's extent).
        assert_eq!(read(&e, point, 3), vec![2.0, 0.25, -1.0]);
    }

    #[test]
    fn contact_point_moves_with_the_box_and_stays_when_touching() {
        let mut e = Engine::new();
        data(&mut e);
        e.map(PROBE_RADIUS & !0xfff, 0x1000);
        e.mem.set_f32(PROBE_RADIUS, 0.01);
        e.map(CLAMP_FACTOR & !0xfff, 0x1000);
        e.mem.set_u64(CLAMP_FACTOR, 0x3fef_ae14_8000_0000);
        e.register(SPHERE_BV_CONSTRUCT_00EAB8A0, nothing);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(100.0));
        e.register(SEGMENT_BOX_DISTANCE_00EB50F0, |_, _| st0(100.0));
        // Box a moves by (2, 0, 0) per unit; at contact time 0.5 its centre
        // is at x 1: the point (5, 0, 0) is clamped to x 2.
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        let moving = floats(&mut e, &[2.0, 0.0, 0.0]);
        // Box b contains the original point: left alone.
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let time = floats(&mut e, &[0.5]);
        let point = floats(&mut e, &[5.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(
            0x00ea_7730,
            &args![1.0f32, a, moving, b, still, time, point],
        );
        assert_eq!(read(&e, point, 3), vec![2.0, 0.0, 0.0]);
        // Box a moves relative to the probe: the segment distance; the
        // original point is inside box b: no distance for it.
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SEGMENT_BOX_DISTANCE_00EB50F0).len(), 1);
        assert!(calls(&log, POINT_BOX_DISTANCE_00EB42E0).is_empty());
    }

    /// `NiPoint3::Dot` and the cross product `004b3800` over plain floats.
    fn vector_doubles(e: &mut Engine) {
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[1], 3));
            st0(f64::from(p[0] * q[0] + p[1] * q[1] + p[2] * q[2]))
        });
        e.register(POINT_CROSS_004B3800, |e, a| {
            let (p, q) = (read(e, a[0], 3), read(e, a[2], 3));
            let out = [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ];
            for (i, v) in out.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            a[1].into_ret()
        });
    }

    fn box_box(e: &mut Engine, b: u32, t: f32, velocity: [f32; 3]) -> (bool, Vec<(u32, Vec<u32>)>) {
        let a = aligned(e, [0.0; 3], [1.0; 3]);
        let still = floats(e, &[0.0, 0.0, 0.0]);
        let moving = floats(e, &velocity);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ea_0ad0, &args![t, a, still, b, moving]).bool();
        let log = e.call_log.take().unwrap();
        (result, log)
    }

    #[test]
    fn box_box_test_separates_on_the_first_axis() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let b = aligned(&mut e, [3.0, 0.0, 0.0], [1.0; 3]);
        let (hit, log) = box_box(&mut e, b, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_DOT).is_empty());
        // Also on the negative side.
        let b = aligned(&mut e, [-3.0, 0.0, 0.0], [1.0; 3]);
        assert!(!box_box(&mut e, b, 1.0, [0.0; 3]).0);
    }

    #[test]
    fn box_box_test_overlap_and_motion() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // Overlapping: every axis passes, the motion axes included (with a
        // zero velocity the cross product is zero).
        let b = aligned(&mut e, [1.0, 0.5, 0.0], [1.0; 3]);
        let (hit, log) = box_box(&mut e, b, 1.0, [0.0; 3]);
        assert!(hit);
        assert_eq!(calls(&log, POINT_CROSS_004B3800).len(), 1);
        assert_eq!(calls(&log, POINT_DOT).len(), 12);
        // Moving through the box along x: apart at the start (3 > 2) and on
        // the other side at the end: not separated.
        let b = aligned(&mut e, [3.0, 0.0, 0.0], [1.0; 3]);
        assert!(box_box(&mut e, b, 1.0, [-6.0, 0.0, 0.0]).0);
        // Moving along x but 3 apart in y: separated on the second axis.
        let b = aligned(&mut e, [0.0, 3.0, 0.0], [1.0; 3]);
        let (hit, log) = box_box(&mut e, b, 1.0, [5.0, 0.0, 0.0]);
        assert!(!hit);
        assert!(calls(&log, POINT_CROSS_004B3800).is_empty());
        // Moving towards it but not far enough in the time: still apart.
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        assert!(!box_box(&mut e, b, 1.0, [-2.0, 0.0, 0.0]).0);
        assert!(box_box(&mut e, b, 2.0, [-2.0, 0.0, 0.0]).0);
    }

    #[test]
    fn box_box_test_separates_on_the_other_boxs_axes() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let c = std::f32::consts::FRAC_1_SQRT_2;
        // A thin strip along the anti-diagonal: only its own first axis
        // (the diagonal) separates it.
        let strip = box_volume(
            &mut e,
            [
                2.2, 2.2, 0.0, c, c, 0.0, -c, c, 0.0, 0.0, 0.0, 1.0, 0.1, 3.0, 1.0,
            ],
        );
        let (hit, log) = box_box(&mut e, strip, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_DOT).is_empty());
        // The same strip with its axes swapped, on the negative side: the
        // second axis separates, its end projection through `Dot`.
        let strip = box_volume(
            &mut e,
            [
                -2.2, -2.2, 0.0, -c, c, 0.0, c, c, 0.0, 0.0, 0.0, 1.0, 3.0, 0.1, 1.0,
            ],
        );
        let (hit, log) = box_box(&mut e, strip, 1.0, [0.0; 3]);
        assert!(!hit);
        let dots = calls(&log, POINT_DOT);
        assert_eq!(dots.len(), 1);
        assert_eq!(dots[0][0], strip + 0x1c);
    }

    fn box_triangle(
        e: &mut Engine,
        vertices: [[f32; 3]; 3],
        t: f32,
        velocity: [f32; 3],
    ) -> (bool, Vec<(u32, Vec<u32>)>) {
        let b = aligned(e, [0.0; 3], [1.0; 3]);
        let still = floats(e, &[0.0, 0.0, 0.0]);
        let moving = floats(e, &velocity);
        let [p0, p1, p2] = vertices.map(|v| floats(e, &v));
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00ea_2190, &args![t, b, still, p0, p1, p2, moving])
            .bool();
        let log = e.call_log.take().unwrap();
        (result, log)
    }

    #[test]
    fn box_triangle_test_separates_on_the_normal_and_the_box_axes() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // A triangle in the plane x = 3 facing the box: its normal separates.
        let facing = [[3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [3.0, 0.0, 1.0]];
        let (hit, log) = box_triangle(&mut e, facing, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_CROSS_004B3800).is_empty());
        // Edge-on (normal along y): the box's first axis separates, on the
        // positive side and on the negative side.
        let edge_on = [[3.0, 0.0, 0.0], [4.0, 0.0, 0.0], [3.0, 0.0, 1.0]];
        let (hit, log) = box_triangle(&mut e, edge_on, 1.0, [0.0; 3]);
        assert!(!hit);
        assert!(calls(&log, POINT_CROSS_004B3800).is_empty());
        let behind = [[-3.0, 0.0, 0.0], [-4.0, 0.0, 0.0], [-3.0, 0.0, 1.0]];
        assert!(!box_triangle(&mut e, behind, 1.0, [0.0; 3]).0);
    }

    #[test]
    fn box_triangle_test_follows_the_motion() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // Moving through the box: no axis separates; every product axis is
        // asked, the second axis's last ones through `Dot`.
        let facing = [[3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [3.0, 0.0, 1.0]];
        let (hit, log) = box_triangle(&mut e, facing, 1.0, [-5.0, 0.0, 0.0]);
        assert!(hit);
        let crosses = calls(&log, POINT_CROSS_004B3800);
        assert_eq!(crosses.len(), 4);
        assert_eq!(calls(&log, POINT_DOT).len(), 2);
        // `004b3800` on the box's second axis with `E1`, a copy of
        // `v1 - v0`.
        let box_at = crosses[0][0] - 0x1c;
        assert_eq!(read(&e, crosses[0][2], 3), vec![0.0, 1.0, 0.0]);
        assert_eq!(crosses[2][0], box_at + 0x28);
        let edge_on = [[3.0, 0.0, 0.0], [4.0, 0.0, 0.0], [3.0, 0.0, 1.0]];
        assert!(box_triangle(&mut e, edge_on, 1.0, [-5.0, 0.0, 0.0]).0);
        // Too slow to reach the box in the time.
        assert!(!box_triangle(&mut e, edge_on, 1.0, [-1.0, 0.0, 0.0]).0);
        assert!(box_triangle(&mut e, edge_on, 2.0, [-1.0, 0.0, 0.0]).0);
    }

    #[test]
    fn box_triangle_test_separates_on_an_edge_product() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        // A flat triangle past the box's corner edge at x = y = 1: only the
        // box's third axis times the first edge, `(10, 10, 0)`, separates.
        let corner = [[-3.75, 6.25, 0.0], [6.25, -3.75, 0.0], [3.0, 3.0, 0.0]];
        let (hit, log) = box_triangle(&mut e, corner, 1.0, [0.0; 3]);
        assert!(!hit);
        assert_eq!(calls(&log, POINT_CROSS_004B3800).len(), 3);
        assert!(calls(&log, POINT_DOT).is_empty());
        // Moving towards the box it is no longer apart at the end.
        let (hit, log) = box_triangle(&mut e, corner, 1.0, [-5.0, -5.0, 0.0]);
        assert!(hit);
        assert_eq!(calls(&log, POINT_DOT).len(), 2);
    }

    fn read3(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn write3(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (k, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * k as u32, *x);
        }
    }

    /// Everything `fn_00ea7d40` and the contact point clamp read: the data
    /// floats, the vector helpers over plain floats, and the probe tests
    /// answering "touching" (distance 0), so the point is never clamped.
    fn find_world(e: &mut Engine) {
        data(e);
        vector_doubles(e);
        for page in [
            0x010c_9000u32,
            0x0101_6000,
            0x0101_2000,
            0x0109_5000,
            0x0101_3000,
            0x0106_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(TIME_SLACK_010C9300, 0x3e80_0000_0000_0000);
        e.mem.set_f32(HALF_01016248, 0.5);
        e.mem.set_u64(ONE_01012070, 1.0f64.to_bits());
        e.mem.set_f32(PARALLEL_LIMIT_010959F4, 1e-6);
        e.mem.set_f32(PROBE_RADIUS, 0.01);
        e.mem.set_u64(CLAMP_FACTOR, 0x3fef_ae14_8000_0000);
        e.register(SPHERE_BV_CONSTRUCT_00EAB8A0, nothing);
        e.register(POINT_BOX_DISTANCE_00EB42E0, |_, _| st0(0.0));
        e.register(SEGMENT_BOX_DISTANCE_00EB50F0, |_, _| st0(0.0));
        e.register(POINT_SUM_00439E90, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_DIFFERENCE_00439EF0, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_SCALE_004A3760, |e, a| {
            let (s, p) = (f32::from_bits(a[1]), read3(e, a[2]));
            write3(e, a[0], [s * p[0], s * p[1], s * p[2]]);
            a[0].into_ret()
        });
        e.register(POINT_LENGTH_SQUARED_004A7290, |e, a| {
            let p = read3(e, a[0]);
            st0(f64::from(p[2] * p[2] + p[1] * p[1] + p[0] * p[0]))
        });
        e.register(UNITIZE_004A0C10, |e, a| {
            let p = read3(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            write3(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            Ret::default()
        });
    }

    /// What one find leaves: the answer, the contact time, point, normal
    /// and negated normal (words; `0x7777_7777` where nothing was stored)
    /// and the call log.
    struct Found {
        hit: bool,
        time: f32,
        point: [f32; 3],
        normal: [u32; 3],
        negated: [u32; 3],
        log: Vec<(u32, Vec<u32>)>,
    }

    fn find(
        e: &mut Engine,
        time: f32,
        a: u32,
        va: [f32; 3],
        b: u32,
        vb: [f32; 3],
        want: bool,
    ) -> Found {
        let (va, vb) = (floats(e, &va), floats(e, &vb));
        let outs = e.mem.alloc(0x28);
        for word in 0..10 {
            e.mem.set_u32(outs + 4 * word, 0x7777_7777);
        }
        e.call_log = Some(vec![]);
        let hit = e
            .call(
                0x00ea_7d40,
                &args![
                    time,
                    a,
                    va,
                    b,
                    vb,
                    outs,
                    outs + 4,
                    want,
                    outs + 0x10,
                    outs + 0x1c
                ],
            )
            .bool();
        let words = |at: u32| [0u32, 4, 8].map(|k| e.mem.u32(at + k));
        Found {
            hit,
            time: e.mem.f32(outs),
            point: read3(e, outs + 4),
            normal: words(outs + 0x10),
            negated: words(outs + 0x1c),
            log: e.call_log.take().unwrap(),
        }
    }

    fn bits(v: [f32; 3]) -> [u32; 3] {
        v.map(f32::to_bits)
    }

    fn close(found: [f32; 3], want: [f32; 3]) -> bool {
        found.iter().zip(want).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    const STILL: [f32; 3] = [0.0; 3];

    #[test]
    fn box_box_find_separated_answers_false() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // Apart on `a`'s first axis, on both sides, and still apart after
        // a motion too short to close the gap.
        for (centre, velocity) in [
            ([5.0, 0.0, 0.0], STILL),
            ([-5.0, 0.0, 0.0], STILL),
            ([5.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
            ([-5.0, 0.0, 0.0], [2.5, 0.0, 0.0]),
            ([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
        ] {
            let b = aligned(&mut e, centre, [1.0; 3]);
            let found = find(&mut e, 1.0, a, STILL, b, velocity, true);
            assert!(!found.hit);
            // The contact time is cleared first; nothing else is stored.
            assert_eq!(found.time, 0.0);
            assert_eq!(found.normal, [0x7777_7777; 3]);
            assert!(calls(&found.log, POINT_DOT).is_empty());
        }
    }

    #[test]
    fn box_box_find_overlap_takes_the_midpoint() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // Overlapping at the start: case 0, the contact time stays 0.
        let b = aligned(&mut e, [1.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, true);
        assert!(found.hit);
        assert_eq!(found.time, 0.0);
        assert_eq!(found.point, [0.5, 0.0, 0.0]);
        // The normal is the point minus `a`'s centre, unitized (its squared
        // length 0.25 is off 1 and above the epsilon).
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        let log = &found.log;
        let added = calls(log, POINT_SUM_00439E90);
        assert_eq!([added[0][0], added[0][2]], [a + 4, b + 4]);
        let scaled = calls(log, POINT_SCALE_004A3760);
        assert_eq!(scaled[0][1], 0.5f32.to_bits());
        assert_eq!(scaled[0][2], added[0][1]);
        assert_eq!(calls(log, POINT_DIFFERENCE_00439EF0)[0][2], a + 4);
        assert_eq!(calls(log, UNITIZE_004A0C10).len(), 1);
        // A normal of length 1 is left alone.
        let b = aligned(&mut e, [2.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, true);
        assert_eq!(found.point, [1.0, 0.0, 0.0]);
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert!(calls(&found.log, UNITIZE_004A0C10).is_empty());
        // Coinciding centres: a zero normal becomes the default x axis.
        let b = aligned(&mut e, [0.0; 3], [0.5; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, true);
        assert_eq!(found.point, [0.0; 3]);
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        assert!(calls(&found.log, UNITIZE_004A0C10).is_empty());
        // Without normals: the point only.
        let b = aligned(&mut e, [1.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, STILL, false);
        assert_eq!(found.point, [0.5, 0.0, 0.0]);
        assert_eq!(found.normal, [0x7777_7777; 3]);
        assert!(calls(&found.log, POINT_DIFFERENCE_00439EF0).is_empty());
    }

    #[test]
    fn box_box_find_face_of_the_first_box() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = aligned(&mut e, [0.0; 3], [1.0; 3]);
        // `b` comes along x and meets `a`'s face x = 1 at 3/4 of the time
        // (`b`'s own x axis meets at the same time: not later by the slack).
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert_eq!(found.time, 0.75);
        // On `b`'s face: its centre at the contact time minus its extent.
        assert_eq!(found.point, [1.0, 0.0, 0.0]);
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        assert!(calls(&found.log, POINT_CROSS_004B3800).is_empty());
        assert!(calls(&found.log, POINT_LENGTH_SQUARED_004A7290).is_empty());
        // From the other side: the negative side, the negated axis.
        let b = aligned(&mut e, [-5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert_eq!(found.time, 0.75);
        assert_eq!(found.point, [-1.0, 0.0, 0.0]);
        assert_eq!(found.normal, bits([-1.0, -0.0, -0.0]));
        assert_eq!(found.negated, bits([1.0, 0.0, 0.0]));
        // `a` moving instead, on y, for twice the time: the point on the
        // still `b`'s face.
        let b = aligned(&mut e, [0.0, 5.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 2.0, a, [0.0, 2.0, 0.0], b, STILL, false);
        assert!(found.hit);
        assert_eq!(found.time, 1.5);
        assert_eq!(found.point, [0.0, 4.0, 0.0]);
        assert_eq!(found.normal, [0x7777_7777; 3]);
    }

    /// `a` turned 45 degrees about z and then about x (no axis of it is
    /// perpendicular to x), extents 1, at the origin.
    fn tilted(e: &mut Engine) -> u32 {
        let h = std::f32::consts::FRAC_1_SQRT_2;
        box_volume(
            e,
            [
                0.0, 0.0, 0.0, h, h, 0.0, -0.5, 0.5, h, 0.5, -0.5, h, 1.0, 1.0, 1.0,
            ],
        )
    }

    #[test]
    fn box_box_find_face_of_the_second_box() {
        let mut e = Engine::new();
        find_world(&mut e);
        let a = tilted(&mut e);
        // `b` (aligned) comes along x: `a`'s corner hits `b`'s face x = -1.
        let b = aligned(&mut e, [5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        let reach = std::f32::consts::FRAC_1_SQRT_2 + 1.0;
        assert!((found.time - (4.0 - reach) / 4.0).abs() < 1e-6);
        // The corner of `a` reaching furthest along x: A0 - A1 + A2.
        let h = std::f32::consts::FRAC_1_SQRT_2;
        assert!(close(found.point, [h + 1.0, h - 1.0, 0.0]));
        assert_eq!(found.normal, bits([1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([-1.0, -0.0, -0.0]));
        assert!(calls(&found.log, POINT_CROSS_004B3800).is_empty());
        // From the other side: the corner A1 - A0 - A2, the negated axis.
        let b = aligned(&mut e, [-5.0, 0.0, 0.0], [1.0; 3]);
        let found = find(&mut e, 1.0, a, STILL, b, [4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert!(close(found.point, [-h - 1.0, 1.0 - h, 0.0]));
        assert_eq!(found.normal, bits([-1.0, -0.0, -0.0]));
    }

    /// `a` turned 45 degrees about z (an edge along z leading in x), `b`
    /// 45 degrees about y (an edge along y leading in -x), `b` at `centre`.
    fn crossed_boxes(e: &mut Engine, centre: [f32; 3]) -> (u32, u32) {
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let a = box_volume(
            e,
            [
                0.0, 0.0, 0.0, h, h, 0.0, -h, h, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
            ],
        );
        let b = box_volume(
            e,
            [
                centre[0], centre[1], centre[2], h, 0.0, -h, 0.0, 1.0, 0.0, h, 0.0, h, 1.0, 1.0,
                1.0,
            ],
        );
        (a, b)
    }

    #[test]
    fn box_box_find_edge_against_edge() {
        let mut e = Engine::new();
        find_world(&mut e);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        // The edges meet when `b` has moved 5 - 2 sqrt 2: on the axis
        // A2 x B1 = -x, the negative side (case 14).
        let (a, b) = crossed_boxes(&mut e, [5.0, 0.0, 0.5]);
        let found = find(&mut e, 1.0, a, STILL, b, [-4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert!((found.time - (5.0 - 4.0 * h) / 4.0).abs() < 1e-6);
        // On `a`'s edge, at `b`'s edge's height: `a`'s own offset along z
        // solved from the two edges (0.5).
        assert!(close(found.point, [2.0 * h, 0.0, 0.5]));
        let log = &found.log;
        let scaled = calls(log, POINT_SCALE_004A3760);
        assert_eq!(scaled.len(), 1);
        assert_eq!(scaled[0][1], found.time.to_bits());
        let dots = calls(log, POINT_DOT);
        assert_eq!([dots[0][0], dots[1][0]], [b + 0x1c, a + 0x28]);
        assert_eq!(dots[0][1], scaled[0][0] - 0x144 + 0x54);
        // The normal: -(A2 x B1) through `004b3800`, already of length 1.
        let crossed = calls(log, POINT_CROSS_004B3800);
        assert_eq!([crossed[0][0], crossed[0][2]], [a + 0x28, b + 0x1c]);
        assert_eq!(crossed[0][1], scaled[0][0] - 0x144 + 0x15c);
        assert_eq!(found.normal, bits([1.0, -0.0, -0.0]));
        assert_eq!(found.negated, bits([-1.0, 0.0, 0.0]));
        assert!(calls(log, UNITIZE_004A0C10).is_empty());
        // From the other side: the positive side, the product itself.
        let (a, b) = crossed_boxes(&mut e, [-5.0, 0.0, -0.5]);
        let found = find(&mut e, 1.0, a, STILL, b, [4.0, 0.0, 0.0], true);
        assert!(found.hit);
        assert!(close(found.point, [-2.0 * h, 0.0, -0.5]));
        assert_eq!(found.normal, bits([-1.0, 0.0, 0.0]));
        assert_eq!(found.negated, bits([1.0, -0.0, -0.0]));
        let crossed = calls(&found.log, POINT_CROSS_004B3800);
        let scaled = calls(&found.log, POINT_SCALE_004A3760);
        assert_eq!(crossed[0][1], scaled[0][0] - 0x144 + 0x174);
    }

    #[test]
    fn box_box_find_parallel_edges_skip_the_solve() {
        let mut e = Engine::new();
        find_world(&mut e);
        // Case 7 with `a`'s and `b`'s first axes parallel (`1 - c00^2` is
        // 0): `a`'s offset along it is 0, nothing is projected, and the zero
        // product of the axes becomes the default normal.
        let a = aligned(&mut e, [1.0, 2.0, 3.0], [1.0; 3]);
        let b = aligned(&mut e, [4.0, 0.0, 0.0], [1.0; 3]);
        let (va, vb, ot, point) = (
            floats(&mut e, &[2.0, 0.0, 0.0]),
            floats(&mut e, &[0.0; 3]),
            floats(&mut e, &[0.5]),
            e.mem.alloc(0x24),
        );
        let s = BoxFind {
            time: 1.0f32.to_bits(),
            a,
            va,
            b,
            vb,
            out_time: ot,
            point,
            want_normals: true,
            normal: point + 0xc,
            negated: point + 0x18,
            frame: e.mem.alloc(FIND_FRAME),
            c: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            t: [3.0, -2.0, -3.0],
            axis: 7,
            side: 1,
        };
        e.call_log = Some(vec![]);
        assert!(find_edge(&mut e, &s, &EDGE_CASES[0]));
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, POINT_SCALE_004A3760).is_empty());
        assert!(calls(&log, POINT_DOT).is_empty());
        let crossed = calls(&log, POINT_CROSS_004B3800);
        assert_eq!(crossed, vec![vec![a + 0x10, s.frame + 0x54, b + 0x10]]);
        // `a`'s centre moved by half its velocity; every offset is 0.
        assert_eq!(read3(&e, point), [2.0, 2.0, 3.0]);
        assert_eq!(read3(&e, point + 0xc), [1.0, 0.0, 0.0]);
        assert_eq!(read3(&e, point + 0x18), [-1.0, -0.0, -0.0]);
    }

    #[test]
    fn box_box_find_signed_extents_and_the_slack() {
        let mut e = Engine::new();
        let x = e.mem.alloc(8);
        e.mem.set_f32(x, 2.0);
        e.mem.set_u32(x + 4, 0x7f80_0001);
        assert_eq!(signed_extent(&e, 0.5, 1, x), 2.0);
        assert_eq!(signed_extent(&e, -0.5, 1, x), -2.0);
        assert_eq!(signed_extent(&e, 0.5, -1, x), -2.0);
        assert_eq!(signed_extent(&e, 0.0, 1, x), 0.0);
        assert_eq!(signed_extent(&e, f64::NAN, 1, x), 0.0);
        // A signalling NaN extent comes back quiet.
        assert_eq!(x87_store32(signed_extent(&e, 1.0, 1, x + 4)), 0x7fc0_0001);
        // `find_axis`: a later meeting not later by the slack keeps the axis.
        find_world(&mut e);
        let ot = floats(&mut e, &[0.5]);
        let mut s = BoxFind {
            time: 0,
            a: 0,
            va: 0,
            b: 0,
            vb: 0,
            out_time: ot,
            point: 0,
            want_normals: false,
            normal: 0,
            negated: 0,
            frame: 0,
            c: [[0.0; 3]; 3],
            t: [0.0; 3],
            axis: 3,
            side: 1,
        };
        // (2 - 4) / (0 - 4) = 0.5: equal, not taken.
        assert!(!find_axis(&mut e, &mut s, 5, 2.0, 4.0, || 0.0, 1.0, 1.0));
        assert_eq!((s.axis, s.side), (3, 1));
        // From below: (2 - 5) * -1 / (-1 + 5) = 0.75: taken, side -1.
        assert!(!find_axis(&mut e, &mut s, 6, 2.0, -5.0, || -1.0, 1.0, 1.0));
        assert_eq!((s.axis, s.side), (6, -1));
        assert_eq!(e.mem.f32(ot), 0.75);
        // Apart from below for the whole motion: separated.
        assert!(find_axis(&mut e, &mut s, 8, 2.0, -5.0, || -3.0, 1.0, 1.0));
        // A NaN offset takes neither side.
        assert!(!find_axis(
            &mut e,
            &mut s,
            9,
            2.0,
            f64::NAN,
            || 0.0,
            1.0,
            1.0
        ));
        assert_eq!(s.axis, 6);
    }
}
