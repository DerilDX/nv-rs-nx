//! `BSMain/run_00c4c2d0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::NiTArray;

/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall`, the result's
/// address): the inverse transform (rotation 3x3, translation, scale; 0x34
/// bytes).
const TRANSFORM_INVERT_NON_UNIFORM: u32 = 0x004b_4880;
/// `D3DXMatrixInverse` (`00ee6dda`, `stdcall`, `(out, determinant, in)`).
const D3DX_MATRIX_INVERSE: u32 = 0x00ee_6dda;
/// The translation `00c4c2d0` puts in the matrix it inverts (three floats).
const FIXED_TRANSLATION: u32 = 0x011f_474c;
/// `00aa1070` (`cdecl`, one word): the block allocation; `00aa10f0`
/// (`cdecl`, one word): its free.
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// The renderer singleton pointer, its Direct3D device (+0x288) and the
/// device methods used here by vtable offset (COM, `this` first): +0x1d8
/// with `(9, &value)`; on the object `00c4ed40` holds, +0x08 (no
/// arguments); on the one `00c4ed70` uses, +0x18 with `(2)`.
const RENDERER: u32 = 0x011f_4748;
const RENDERER_DEVICE: u32 = 0x288;
const SLOT_DEVICE_QUERY: u32 = 0x1d8;
const SLOT_RELEASE: u32 = 0x08;
const SLOT_ISSUE: u32 = 0x18;
/// The byte `00c4ed70` sets the first time.
const ISSUED_ONCE: u32 = 0x0120_30a4;

/// A float copied through the x87 stack and multiplied by `scale` (a float
/// memory operand), stored as a float.
fn scaled(value: u32, scale: u32) -> u32 {
    x87_store32(x87_mul(x87_load32(value), x87_m32(scale)))
}

// Translated from 00c4c2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`; `cdecl`): the inverse of
/// a transform as a 4x4 row-major matrix at `out`. With `fixed` zero:
/// `NiTransform::InvertNonUniform` of `transform`, then the inverse
/// rotation's columns times its scale as the first three rows (each
/// element copied through a float temporary, then multiplied), 0.0 in the
/// fourth column, its translation as the last row and 1.0. Otherwise
/// `D3DXMatrixInverse` of the identity with the translation at `011f474c`
/// (the transform is not read).
pub fn fn_00c4c2d0(e: &mut Engine, transform: Ptr, fixed: u32, out: Ptr) {
    let out = out.addr();
    if fixed == 0 {
        e.with_stack(0x34, |e, inverse| {
            let inverse = inverse.addr();
            e.call(TRANSFORM_INVERT_NON_UNIFORM, &args![transform, inverse]);
            let scale = e.mem.u32(inverse + 0x30);
            for column in 0..3u32 {
                for row in 0..3u32 {
                    let element = e.mem.u32(inverse + row * 12 + column * 4);
                    let copied = x87_quiet_bits(element);
                    let value = scaled(copied, scale);
                    e.mem
                        .set_u32(out.wrapping_add(column * 16 + row * 4), value);
                }
                e.mem.set_f32(out.wrapping_add(column * 16 + 12), 0.0);
            }
            for k in 0..3u32 {
                let value = x87_quiet_bits(e.mem.u32(inverse + 0x24 + 4 * k));
                e.mem.set_u32(out.wrapping_add(0x30 + 4 * k), value);
            }
            e.mem.set_f32(out.wrapping_add(0x3c), 1.0);
        });
        return;
    }
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        for k in 0..16u32 {
            let value = if k % 5 == 0 { 1.0 } else { 0.0 };
            e.mem.set_f32(matrix + 4 * k, value);
        }
        for k in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(FIXED_TRANSLATION + 4 * k));
            e.mem.set_u32(matrix + 0x30 + 4 * k, value);
        }
        e.call(D3DX_MATRIX_INVERSE, &args![out, 0u32, matrix]);
    });
}

// Translated from 00c4c920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): an `NiTArray`'s
/// `SetSize` (the template instance in this unit). Nothing when `size` is
/// the capacity. A size below the used size clears the slots from `size`
/// on (counting each non-null one off) and becomes the used size. Then the
/// capacity becomes `size`: a new block of that many words (`00aa1070`)
/// takes the used slots and zeros for the rest, or no block for 0; the old
/// block is freed (`00aa10f0`).
pub fn fn_00c4c920(e: &mut Engine, this: Ptr<NiTArray>, size: u32) {
    let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if size == capacity {
        return;
    }
    if size < u32::from(e.get(this, NiTArray::m_usSize)) {
        let mut i = size as u16;
        while i < e.get(this, NiTArray::m_usSize) {
            let base = e.get(this, NiTArray::m_pBase);
            let slot = base.wrapping_add(u32::from(i) * 4);
            if e.mem.u32(slot) != 0 {
                e.mem.set_u32(slot, 0);
                let count = e.get(this, NiTArray::m_usESize).wrapping_sub(1);
                e.set(this, NiTArray::m_usESize, count);
            }
            i = i.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, size as u16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, size as u16);
    if size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let block = e.call(NI_ALLOC, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, block);
        let mut i = 0u16;
        while i < e.get(this, NiTArray::m_usSize) {
            let word = e.mem.u32(old.wrapping_add(u32::from(i) * 4));
            let base = e.get(this, NiTArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(u32::from(i) * 4), word);
            i = i.wrapping_add(1);
        }
        let mut i = e.get(this, NiTArray::m_usSize);
        while i < e.get(this, NiTArray::m_usMaxSize) {
            let base = e.get(this, NiTArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(u32::from(i) * 4), 0);
            i = i.wrapping_add(1);
        }
    }
    e.call(NI_FREE, &args![old]);
}

// Translated from 00c4ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): clears the word at
/// `this`, then has the renderer's Direct3D device (the pointer at
/// `011f4748`, +0x288) fill it through its method at vtable +0x1d8 with 9.
/// Returns `this`.
pub fn fn_00c4ecf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let renderer = e.mem.u32(RENDERER);
    let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
    e.vcall(device, SLOT_DEVICE_QUERY, &args![9u32, this]);
    this
}

// Translated from 00c4ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): releases the COM object
/// the word at `this` holds (its method at vtable +0x08), when there is
/// one.
pub fn fn_00c4ed40(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.vcall(object, SLOT_RELEASE, &args![]);
    }
}

// Translated from 00c4ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): only the first time (the
/// byte at `012030a4`), calls the method at vtable +0x18 of the COM object
/// the word at `this` holds with 2, and returns true; false afterwards.
pub fn fn_00c4ed70(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(ISSUED_ONCE) != 0 {
        return false;
    }
    e.mem.set_u8(ISSUED_ONCE, 1);
    let object = e.mem.u32(this.addr());
    e.vcall(object, SLOT_ISSUE, &args![2u32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_c2d0, fn_00c4c2d0(Ptr, u32, Ptr)),
        entry!(0x00c4_c920, fn_00c4c920(Ptr<NiTArray>, u32)),
        entry!(0x00c4_ecf0, fn_00c4ecf0(Ptr) -> Ptr),
        entry!(0x00c4_ed40, fn_00c4ed40(Ptr)),
        entry!(0x00c4_ed70, fn_00c4ed70(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const QUERY: u32 = 0x00ee_0100;
    const RELEASE: u32 = 0x00ee_0200;
    const ISSUE: u32 = 0x00ee_0300;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    #[test]
    fn fn_00c4c2d0_builds_the_inverse_matrix() {
        let mut e = Engine::new();
        e.register(TRANSFORM_INVERT_NON_UNIFORM, |e, a| {
            for k in 0..9 {
                e.mem.set_f32(a[1] + 4 * k, k as f32 + 1.0);
            }
            for k in 0..3 {
                e.mem.set_f32(a[1] + 0x24 + 4 * k, 10.0 + k as f32);
            }
            e.mem.set_f32(a[1] + 0x30, 2.0);
            Ret::default()
        });
        let out = e.mem.alloc(0x40);
        e.call(0x00c4_c2d0, &args![0x5000u32, 0u32, out]);
        let matrix: Vec<f32> = (0..16).map(|k| e.mem.f32(out + 4 * k)).collect();
        assert_eq!(
            matrix,
            [
                2.0, 8.0, 14.0, 0.0, 4.0, 10.0, 16.0, 0.0, 6.0, 12.0, 18.0, 0.0, 10.0, 11.0, 12.0,
                1.0
            ]
        );
        // The fixed form inverts the identity with the fixed translation.
        let log = Log::default();
        let l = log.clone();
        e.register_double(D3DX_MATRIX_INVERSE, move |e, a| {
            let input: Vec<u32> = (0..16).map(|k| e.mem.u32(a[2] + 4 * k)).collect();
            l.borrow_mut().push((a[0], input));
            Ret::default()
        });
        e.map(FIXED_TRANSLATION, 12);
        e.mem.set_f32(FIXED_TRANSLATION, 7.0);
        e.mem.set_f32(FIXED_TRANSLATION + 8, 9.0);
        e.call(0x00c4_c2d0, &args![0x5000u32, 1u32, out]);
        let one = 1.0f32.to_bits();
        assert_eq!(
            *log.borrow(),
            [(
                out,
                vec![
                    one,
                    0,
                    0,
                    0,
                    0,
                    one,
                    0,
                    0,
                    0,
                    0,
                    one,
                    0,
                    7.0f32.to_bits(),
                    0,
                    9.0f32.to_bits(),
                    one
                ]
            )]
        );
    }

    #[test]
    fn fn_00c4c920_resizes_the_array() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x20);
        logged(&mut e, &log, NI_ALLOC, block);
        logged(&mut e, &log, NI_FREE, 0);
        let old = e.mem.alloc(0x20);
        for k in 0..5 {
            e.mem
                .set_u32(old + 4 * k, if k == 1 { 0 } else { 0x100 + k });
        }
        let this: Ptr<NiTArray> = e.new_object();
        e.set(this, NiTArray::m_pBase, old);
        e.set(this, NiTArray::m_usMaxSize, 5);
        e.set(this, NiTArray::m_usSize, 5);
        e.set(this, NiTArray::m_usESize, 4);
        // Shrinking to 3 clears slots 3 and 4.
        e.call(0x00c4_c920, &args![this, 3u32]);
        assert_eq!(e.get(this, NiTArray::m_usSize), 3);
        assert_eq!(e.get(this, NiTArray::m_usESize), 2);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(this, NiTArray::m_pBase), block);
        assert_eq!(
            (0..3).map(|k| e.mem.u32(block + 4 * k)).collect::<Vec<_>>(),
            [0x100, 0, 0x102]
        );
        assert_eq!(*log.borrow(), [(NI_ALLOC, vec![12]), (NI_FREE, vec![old])]);
        // Growing to 6 copies the used slots and zeroes the rest.
        let bigger = e.mem.alloc(0x20);
        e.mem.write(bigger, &[0xaa; 0x20]);
        logged(&mut e, &log, NI_ALLOC, bigger);
        e.call(0x00c4_c920, &args![this, 6u32]);
        assert_eq!(e.mem.u32(bigger + 8), 0x102);
        assert_eq!(e.mem.u32(bigger + 20), 0);
        assert_eq!(e.mem.u32(bigger + 24), 0xaaaa_aaaa);
        // The same capacity: nothing; zero: no block.
        log.borrow_mut().clear();
        e.call(0x00c4_c920, &args![this, 6u32]);
        assert!(log.borrow().is_empty());
        e.set(this, NiTArray::m_usSize, 0);
        e.call(0x00c4_c920, &args![this, 0u32]);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(*log.borrow(), [(NI_FREE, vec![bigger])]);
    }

    #[test]
    fn device_helpers_call_their_com_methods() {
        let mut e = Engine::new();
        let log = Log::default();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_DEVICE_QUERY, QUERY);
        e.mem.set_u32(vtable + SLOT_RELEASE, RELEASE);
        e.mem.set_u32(vtable + SLOT_ISSUE, ISSUE);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        e.map(RENDERER, 4);
        e.mem.set_u32(RENDERER, renderer);
        let l = log.clone();
        e.register_double(QUERY, move |e, a| {
            l.borrow_mut().push((QUERY, a.to_vec()));
            e.mem.set_u32(a[2], 0x55);
            Ret::default()
        });
        logged(&mut e, &log, RELEASE, 0);
        logged(&mut e, &log, ISSUE, 0);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0xffff);
        assert_eq!(e.call(0x00c4_ecf0, &args![value]).u32(), value);
        assert_eq!(e.mem.u32(value), 0x55);
        e.mem.set_u32(value, device);
        e.call(0x00c4_ed40, &args![value]);
        e.map(ISSUED_ONCE, 4);
        assert!(e.call(0x00c4_ed70, &args![value]).bool());
        assert!(!e.call(0x00c4_ed70, &args![value]).bool());
        assert_eq!(
            *log.borrow(),
            [
                (QUERY, vec![device, 9, value]),
                (RELEASE, vec![device]),
                (ISSUE, vec![device, 2]),
            ]
        );
        e.mem.set_u32(value, 0);
        e.call(0x00c4_ed40, &args![value]);
        assert_eq!(log.borrow().len(), 3);
    }
}
