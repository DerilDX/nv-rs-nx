//! `BSShader/bscubemapcamera.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa13e0` (`cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// The vtable of `BSCubeMapCamera` (`010bc470`, no RTTI).
const VTABLE: u32 = 0x010b_c470;
/// `NiCamera`'s constructor (`00a712f0`) and destructor (`00a6fae0`),
/// `thiscall`, no stack words.
const CAMERA_CONSTRUCTOR: u32 = 0x00a7_12f0;
const CAMERA_DESTRUCTOR: u32 = 0x00a6_fae0;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, five words:
/// array, element size, count, constructor, destructor) and `eh vector
/// destructor iterator` (`00ec5fce`, four words), both cleaned by the
/// callee, with the `NiPointer` element constructor `00b54620` and
/// destructor `0045cec0`.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const POINTER_CONSTRUCT: u32 = 0x00b5_4620;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `NiFrustum::NiFrustum` (`00a71b70`, `thiscall`, one word: orthographic).
const FRUSTUM_CONSTRUCTOR: u32 = 0x00a7_1b70;
/// The CRT's `tan` (`00eca5d0`, the angle in ST0, here a leading `f64`).
const TAN: u32 = 0x00ec_a5d0;
/// The `double` angle (`0103b100`, a quarter turn's half: pi/4 as a
/// `float`) and the `float` far distance (`01018bfc`, 2048.0) of the view.
const HALF_FIELD_OF_VIEW: u32 = 0x0103_b100;
const FAR_DISTANCE: u32 = 0x0101_8bfc;
/// `BSShaderAccumulator::BSShaderAccumulator` (`00b660d0`, `thiscall`,
/// three words).
const ACCUMULATOR_CONSTRUCTOR: u32 = 0x00b6_60d0;

/// The camera's fields on PC: the six face `NiPointer`s (+0x118), the words
/// +0x114 (the constructor's argument), the `NiPointer`s +0x130, +0x138 and
/// the accumulator +0x13C; `NiCamera::m_kViewFrustum` (Xbox PDB) +0xDC.
const FACES: u32 = 0x118;
const OWNER: u32 = 0x114;
const POINTER_130: u32 = 0x130;
const POINTER_138: u32 = 0x138;
const ACCUMULATOR: u32 = 0x13c;
const VIEW_FRUSTUM: u32 = 0xdc;

/// An `NiPointer`'s release: the count at +4 decremented, the object
/// deleted through its vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An `NiPointer` member's release and clear.
fn release_and_clear(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(member, 0);
    }
}

/// An `NiPointer` member's release without clearing.
fn release_member(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
    }
}

// Translated from 00bd74a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`BSCubeMapCamera`'s destructor by its vtable and fields): the
/// vtable, the six faces released and cleared, +0x130 and +0x13C released
/// and cleared, then the member destructors of +0x13C, +0x138 and +0x130,
/// the faces' vector destructor and `NiCamera`'s destructor.
// Translation incomplete: 00bd74a0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2d00a, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bd74a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for face in 0..6 {
        release_and_clear(e, base + FACES + 4 * face);
    }
    release_and_clear(e, base + POINTER_130);
    release_and_clear(e, base + ACCUMULATOR);
    release_member(e, base + ACCUMULATOR);
    release_member(e, base + POINTER_138);
    release_member(e, base + POINTER_130);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + FACES, 4u32, 6u32, POINTER_DESTRUCT],
    );
    e.call(CAMERA_DESTRUCTOR, &args![this]);
}

// Translated from 00bd8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCubeMapCamera::BSCubeMapCamera` (Xbox PDB): `NiCamera`'s constructor,
/// the six face pointers constructed, the vtable, the pointers +0x130,
/// +0x138 and +0x13C cleared, `owner` kept at +0x114, the faces and +0x130,
/// +0x138 released and cleared (null here); the view frustum is a default
/// `NiFrustum` with left and bottom `-tan(pi/4)`, right and top `tan(pi/4)`
/// (the `float`s through the x87 stack and the argument's slot), near 1 and
/// far 2048; then a new `BSShaderAccumulator` (0x280 bytes; `(0x63, 1,
/// 0x2F7)`; null when the allocation fails) becomes the accumulator +0x13C.
/// Returns `this`.
// Translation incomplete: 00bd8290 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2d13c, state variable) and the security-cookie bookkeeping are not translated.
pub fn bs_cube_map_camera_bs_cube_map_camera(e: &mut Engine, this: Ptr, owner: u32) -> Ptr {
    let base = this.addr();
    e.call(CAMERA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, VTABLE);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + FACES,
            4u32,
            6u32,
            POINTER_CONSTRUCT,
            POINTER_DESTRUCT
        ],
    );
    e.mem.set_u32(base + POINTER_130, 0);
    e.mem.set_u32(base + POINTER_138, 0);
    e.mem.set_u32(base + ACCUMULATOR, 0);
    e.mem.set_u32(base + OWNER, owner);
    for face in 0..6 {
        release_and_clear(e, base + FACES + 4 * face);
    }
    release_and_clear(e, base + POINTER_130);
    release_and_clear(e, base + POINTER_138);
    e.with_stack(0x1c, |e, frustum| {
        e.call(FRUSTUM_CONSTRUCTOR, &args![frustum, 0u32]);
        let angle = f64::from_bits(e.global::<u64>(HALF_FIELD_OF_VIEW));
        let tangent = e.call(TAN, &args![angle]).f64();
        let tangent = x87_store32(tangent);
        let tangent = x87_store32(x87_load32(tangent));
        let negative = x87_store32(-x87_load32(tangent));
        let negative = x87_quiet_bits(negative);
        let f = frustum.addr();
        e.mem.set_u32(f, negative);
        e.mem.set_u32(f + 0xc, negative);
        e.mem.set_u32(f + 4, x87_quiet_bits(tangent));
        e.mem.set_u32(f + 8, x87_quiet_bits(tangent));
        e.mem.set_u32(f + 0x10, 1.0f32.to_bits());
        e.mem
            .set_u32(f + 0x14, x87_quiet_bits(e.global::<u32>(FAR_DISTANCE)));
        let bytes = e.mem.bytes(f, 0x1c);
        e.mem.write(base + VIEW_FRUSTUM, &bytes);
    });
    let block = e.call(NI_NEW, &args![0x280u32]).u32();
    let accumulator = if block == 0 {
        0
    } else {
        e.call(
            ACCUMULATOR_CONSTRUCTOR,
            &args![block, 0x63u32, 1u32, 0x2f7u32],
        )
        .u32()
    };
    let old = e.mem.u32(base + ACCUMULATOR);
    if old != accumulator {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(base + ACCUMULATOR, accumulator);
        if accumulator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![accumulator + 4]);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_74a0, fn_00bd74a0(Ptr)),
        entry!(
            0x00bd_8290,
            bs_cube_map_camera_bs_cube_map_camera(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        for quiet in [
            CAMERA_CONSTRUCTOR,
            CAMERA_DESTRUCTOR,
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            FRUSTUM_CONSTRUCTOR,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_9000, &[0, 0x0040_9004]);
        e.register(0x0040_9004, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_sets_the_frustum_and_the_accumulator() {
        let mut e = rig();
        e.map(0x0103_b000, 0x1000);
        e.map(0x0101_8000, 0x1000);
        e.set_global(HALF_FIELD_OF_VIEW, std::f64::consts::FRAC_PI_4);
        e.set_global(FAR_DISTANCE, 2048.0f32);
        e.register(TAN, |_, a| Ret {
            st0: f64::take(a, &mut 0).tan(),
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ACCUMULATOR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0090_9000);
            a[0].into_ret()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x140));
        assert_eq!(
            e.call(0x00bd_8290, &args![this, 0x7777u32]).u32(),
            this.addr()
        );
        let base = this.addr();
        assert_eq!(e.mem.u32(base), VTABLE);
        assert_eq!(e.mem.u32(base + OWNER), 0x7777);
        let tangent = std::f64::consts::FRAC_PI_4.tan() as f32;
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM), -tangent);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 4), tangent);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 0xc), -tangent);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 0x10), 1.0);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 0x14), 2048.0);
        let accumulator = e.mem.u32(base + ACCUMULATOR);
        assert_ne!(accumulator, 0);
        assert_eq!(e.mem.u32(accumulator + 4), 1);
        assert_eq!(
            calls_to(&e, ACCUMULATOR_CONSTRUCTOR),
            vec![vec![accumulator, 0x63, 1, 0x2f7]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![
                base + FACES,
                4,
                6,
                POINTER_CONSTRUCT,
                POINTER_DESTRUCT
            ]]
        );
    }

    #[test]
    fn destructor_releases_faces_and_pointers() {
        let mut e = rig();
        let this: Ptr = Ptr::new(e.mem.alloc(0x140));
        let base = this.addr();
        let face = e.mem.alloc(8);
        e.mem.set_u32(face, 0x0090_9000);
        e.mem.set_u32(face + 4, 1);
        e.mem.set_u32(base + FACES + 8, face);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, 0x0090_9000);
        e.mem.set_u32(other + 4, 2);
        e.mem.set_u32(base + POINTER_138, other);
        e.call(0x00bd_74a0, &args![this]);
        assert_eq!(e.mem.u32(base + FACES + 8), 0);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![face]]);
        // +0x138 only gets its member destructor: not cleared.
        assert_eq!(e.mem.u32(base + POINTER_138), other);
        assert_eq!(e.mem.u32(other + 4), 1);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![base + FACES, 4, 6, POINTER_DESTRUCT]]
        );
        assert_eq!(calls_to(&e, CAMERA_DESTRUCTOR), vec![vec![base]]);
    }
}
