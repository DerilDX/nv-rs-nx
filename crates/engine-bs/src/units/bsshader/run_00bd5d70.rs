//! `BSShader/run_00bd5d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00bd6ec0` (`run_00bd5d70`, unnamed; `thiscall`, no stack words), the
/// `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word), the set-up
/// `00bd6ae0` (unnamed; `thiscall` on +0x58, one word) and `00b8bc20` (unit
/// `imagespacemanager.cpp`, unnamed; `thiscall`, no stack words).
const BASE_CONSTRUCT_00BD6EC0: u32 = 0x00bd_6ec0;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const SET_UP_00BD6AE0: u32 = 0x00bd_6ae0;
const CALL_00B8BC20: u32 = 0x00b8_bc20;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// This effect's vtable (`010bc410`; no RTTI locator before it).
const VTABLE_010BC410: u32 = 0x010b_c410;

// Translated from 00bd7120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00bd6ec0`, stores the vtable
/// `010bc410`, clears the bytes +0x7c to +0x7e and the words +0x80 to +0x88, constructs the
/// three 16-byte textures at +0x8c (`eh vector constructor iterator` with `004ebb70` and
/// `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 6 (`00a90180`),
/// stores 0.0 at +0x68 to +0x70, clears +0x74, runs `00bd6ae0` on +0x58 with 4 and `00b8bc20`.
/// Answers `this`.
// Translation incomplete: 00bd7120 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00bd7120(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00BD6EC0, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC410);
    for offset in [0x7cu32, 0x7d, 0x7e] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x80u32, 0x84, 0x88] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0x8c,
            0x10u32,
            3u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 6u32]);
    for offset in [0x68u32, 0x6c, 0x70, 0x74] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(SET_UP_00BD6AE0, &args![base + 0x58, 4u32]);
    e.call(CALL_00B8BC20, &args![base]);
    this
}

/// The Ni allocation `00aa1070` and free `00aa10f0` (`cdecl`, one word each).
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00bd6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the set-size of an `NiTArray` of bytes (base +4,
/// max size +8, size +0xa, element count +0xc, all 16-bit but the base), `thiscall` with one
/// stack word: the new size (compared whole with the 16-bit fields). Nothing when it equals the
/// max size. A size below the current size clears the set bytes from the new size up (each one
/// lowering the element count) and becomes the size. The max size takes the new size; with 0
/// the base is cleared, else a new block of that many bytes (`00aa1070`) takes the old bytes up
/// to the size and zeros up to the max size. The old block is freed (`00aa10f0`).
pub fn fn_00bd6ae0(e: &mut Engine, this: Ptr, size: u32) {
    let base = this.addr();
    if size == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    let narrow = size as u16;
    if size < u32::from(e.mem.u16(base + 0xa)) {
        let mut index = narrow;
        if index < e.mem.u16(base + 0xa) {
            loop {
                let byte = e.mem.u32(base + 4).wrapping_add(u32::from(index));
                if e.mem.u8(byte) != 0 {
                    e.mem.set_u8(byte, 0);
                    let count = e.mem.u16(base + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(base + 0xc, count);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(base + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(base + 0xa, narrow);
    }
    let old = e.mem.u32(base + 4);
    e.mem.set_u16(base + 8, narrow);
    if size == 0 {
        e.mem.set_u32(base + 4, 0);
        e.call(NI_FREE_00AA10F0, &args![old]);
        return;
    }
    let block = e
        .call(NI_ALLOCATE_00AA1070, &args![u32::from(narrow)])
        .u32();
    e.mem.set_u32(base + 4, block);
    let mut index = 0u16;
    while index < e.mem.u16(base + 0xa) {
        let byte = e.mem.u8(old.wrapping_add(u32::from(index)));
        let block = e.mem.u32(base + 4);
        e.mem.set_u8(block.wrapping_add(u32::from(index)), byte);
        index = index.wrapping_add(1);
    }
    let mut index = e.mem.u16(base + 0xa);
    while index < e.mem.u16(base + 8) {
        let block = e.mem.u32(base + 4);
        e.mem.set_u8(block.wrapping_add(u32::from(index)), 0);
        index = index.wrapping_add(1);
    }
    e.call(NI_FREE_00AA10F0, &args![old]);
}

/// The image-space effect base constructor `00ba4080` (`thiscall`, no stack words), this
/// effect's vtable (`010bc3f0`) and its byte array's (`010b9d24`) (no RTTI locators before
/// them).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BC3F0: u32 = 0x010b_c3f0;
const ARRAY_VTABLE_010B9D24: u32 = 0x010b_9d24;

// Translated from 00bd6ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the base constructor of `00bd7120`'s effect,
/// `thiscall`, no stack words. Runs the image-space effect base constructor `00ba4080`, stores
/// the vtable `010bc3f0` and makes the empty array at +0x58 (max size +0x60 and element count
/// +0x64 0, base +0x5c null, grow-by +0x66 1, size +0x62 0, vtable `010b9d24`, in that order).
/// Answers `this`.
// Translation incomplete: 00bd6ec0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bd6ec0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC3F0);
    e.mem.set_u16(base + 0x60, 0);
    e.mem.set_u16(base + 0x64, 0);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u16(base + 0x66, 1);
    e.mem.set_u16(base + 0x62, 0);
    e.mem.set_u32(base + 0x58, ARRAY_VTABLE_010B9D24);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_7120, fn_00bd7120(Ptr) -> Ptr),
        entry!(0x00bd_6ae0, fn_00bd6ae0(Ptr, u32)),
        entry!(0x00bd_6ec0, fn_00bd6ec0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd7120_builds_the_effect() {
        let mut e = Engine::new();
        for target in [
            BASE_CONSTRUCT_00BD6EC0,
            ARRAY_SET_SIZE_00A90180,
            SET_UP_00BD6AE0,
            CALL_00B8BC20,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        e.mem.write(this, &[0xcc; 0xbc]);
        assert_eq!(e.call(0x00bd_7120, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC410);
        assert_eq!(e.mem.bytes(this + 0x68, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.bytes(this + 0x7c, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x7f), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x80, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BASE_CONSTRUCT_00BD6EC0, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0x8c,
                        0x10,
                        3,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 6]),
                (SET_UP_00BD6AE0, vec![this + 0x58, 4]),
                (CALL_00B8BC20, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod b000n_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An array of max size 4 and size 4 holding 1, 0, 2, 3 (element count 3).
    fn array(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let old = e.mem.alloc(4);
        e.mem.write(old, &[1, 0, 2, 3]);
        e.mem.set_u32(this + 4, old);
        e.mem.set_u16(this + 8, 4);
        e.mem.set_u16(this + 0xa, 4);
        e.mem.set_u16(this + 0xc, 3);
        (this, old)
    }

    #[test]
    fn fn_00bd6ae0_shrinks_and_grows_the_byte_array() {
        let mut e = engine();
        let (this, old) = array(&mut e);
        e.call(0x00bd_6ae0, &args![this, 2u32]);
        // Bytes 2 and 3 cleared (both set): count 1, size 2, max 2, a new block of 2.
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 2);
        let block = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(block, 2), vec![1, 0]);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070), vec![vec![2]]);
        assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![old]]);
        // Growing keeps the size and zeros the rest.
        e.mem.write(block, &[7, 8]);
        e.call(0x00bd_6ae0, &args![this, 5u32]);
        let grown = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(grown, 5), vec![7, 8, 0, 0, 0]);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 5);
        // The same max size: nothing.
        e.call(0x00bd_6ae0, &args![this, 5u32]);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070).len(), 2);
        // Size 0: base cleared, the old block freed.
        e.call(0x00bd_6ae0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(calls(&e, NI_FREE_00AA10F0).last().unwrap(), &vec![grown]);
    }

    #[test]
    fn fn_00bd6ec0_builds_the_effect_and_its_array() {
        let mut e = engine();
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00bd_6ec0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC3F0);
        assert_eq!(e.mem.u32(this + 0x58), ARRAY_VTABLE_010B9D24);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xa), vec![0; 0xa]);
        assert_eq!(e.mem.u16(this + 0x66), 1);
        assert_eq!(calls(&e, EFFECT_CONSTRUCT_00BA4080), vec![vec![this]]);
    }
}
