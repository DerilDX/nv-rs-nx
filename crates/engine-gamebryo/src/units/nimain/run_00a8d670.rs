//! `NiMain/run_00a8d670` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual tables the constructors store (RTTI `NiImageReader`, `NiTGAReader`,
/// `NiBMPReader`).
const NI_IMAGE_READER_VTABLE: u32 = 0x0109_f3d0;
const NI_TGA_READER_VTABLE: u32 = 0x0109_f4b0;
const NI_BMP_READER_VTABLE: u32 = 0x0109_f4c4;
/// `InitializeCriticalSection` (import slot) and `NiPixelFormat::NiPixelFormat` (engine map,
/// `00a7cca0`; `thiscall`, no stack words).
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
/// The pixel format both readers copy over their own (0x11 words at `011aa2e8`).
const PIXEL_FORMAT_SOURCE_011AA2E8: u32 = 0x011a_a2e8;
const PIXEL_FORMAT_WORDS: u32 = 0x11;
/// Where each reader keeps its pixel format.
const TGA_PIXEL_FORMAT: u32 = 0x11c;
const BMP_PIXEL_FORMAT: u32 = 0x108;

/// The start both readers share: the `NiImageReader` virtual table, its critical section at
/// +0x80, the reader's own virtual table, and its pixel format constructed. Answers the pixel
/// format's address.
fn reader_start(e: &mut Engine, base: u32, vtable: u32, pixel_format: u32) -> u32 {
    e.mem.set_u32(base, NI_IMAGE_READER_VTABLE);
    e.call(
        IMPORT_INITIALIZE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.mem.set_u32(base, vtable);
    let format = base.wrapping_add(pixel_format);
    e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
    format
}

/// The `REP MOVSD` of the 0x11 words at `011aa2e8` over the pixel format.
fn copy_pixel_format(e: &mut Engine, format: u32) {
    for word in 0..PIXEL_FORMAT_WORDS {
        let value = e.mem.u32(PIXEL_FORMAT_SOURCE_011AA2E8 + 4 * word);
        e.mem.set_u32(format.wrapping_add(4 * word), value);
    }
}

// Translated from 00a8d750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a8d670`): the `NiTGAReader` constructor (the RTTI of the
/// virtual table it stores), `thiscall`, no stack words. The shared reader start (pixel format
/// at +0x11c), the 16-bit fields +0x104, +0x10a, +0x10e, +0x106, +0x10c and +0x110 and the
/// bytes +0x100 to +0x102, +0x108 and +0x112 to +0x118 cleared, the pixel format at `011aa2e8`
/// copied over its own, then the words +0x160 to +0x174 and the bytes +0x178 to +0x17c
/// cleared. Answers `this`.
pub fn fn_00a8d750(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let format = reader_start(e, base, NI_TGA_READER_VTABLE, TGA_PIXEL_FORMAT);
    for offset in [0x104u32, 0x10a, 0x10e] {
        e.mem.set_u16(base.wrapping_add(offset), 0);
    }
    for offset in [
        0x100u32, 0x101, 0x102, 0x108, 0x112, 0x113, 0x114, 0x115, 0x116, 0x117, 0x118,
    ] {
        e.mem.set_u8(base.wrapping_add(offset), 0);
    }
    for offset in [0x106u32, 0x10c, 0x110] {
        e.mem.set_u16(base.wrapping_add(offset), 0);
    }
    copy_pixel_format(e, format);
    for offset in (0x160u32..=0x174).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in 0x178u32..=0x17c {
        e.mem.set_u8(base.wrapping_add(offset), 0);
    }
    this
}

// Translated from 00a8e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a8d670`): the `NiBMPReader` constructor (the RTTI of the
/// virtual table it stores), `thiscall`, no stack words. The shared reader start (pixel format
/// at +0x108), +0x100 and +0x104 cleared, the pixel format at `011aa2e8` copied over its own,
/// then +0x150 and +0x154, the bytes +0x158 and +0x159 and the 16-bit +0x14c cleared. Answers
/// `this`.
pub fn fn_00a8e410(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let format = reader_start(e, base, NI_BMP_READER_VTABLE, BMP_PIXEL_FORMAT);
    e.mem.set_u32(base.wrapping_add(0x100), 0);
    e.mem.set_u32(base.wrapping_add(0x104), 0);
    copy_pixel_format(e, format);
    e.mem.set_u32(base.wrapping_add(0x150), 0);
    e.mem.set_u32(base.wrapping_add(0x154), 0);
    e.mem.set_u8(base.wrapping_add(0x158), 0);
    e.mem.set_u8(base.wrapping_add(0x159), 0);
    e.mem.set_u16(base.wrapping_add(0x14c), 0);
    this
}

/// The `DeleteCriticalSection` import slot.
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The memory manager (`011f6238`) and its `Deallocate` (`00aa4060`,
/// `thiscall`, `RET 4`), and the Gamebryo free (`00aa10f0`, `cdecl`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a8e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor of an image reader
/// with nothing of its own to free (`thiscall`, `RET 4`: the flags): stores
/// `NiImageReader`'s vtable, deletes the critical section at `+0x80` and, with
/// bit 0 of the flags, gives the object back to the memory manager. Answers
/// `this`.
pub fn fn_00a8e480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, NI_IMAGE_READER_VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![t + 0x80]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, t]);
    }
    this
}

// Translated from 00a8d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiTGAReader`'s scalar deleting destructor
/// (`thiscall`, `RET 4`: the flags): stores its vtable, frees the block at
/// `+0x164` (`00aa10f0`) and gives the one at `+0x16c` back to the memory
/// manager, stores `NiImageReader`'s vtable, deletes the critical section at
/// `+0x80` and, with bit 0 of the flags, gives the object back. Answers
/// `this`.
pub fn fn_00a8d850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    let block = e.mem.u32(t + 0x164);
    e.mem.set_u32(t, NI_TGA_READER_VTABLE);
    e.call(NI_FREE, &args![block]);
    let other = e.mem.u32(t + 0x16c);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, other]);
    e.mem.set_u32(t, NI_IMAGE_READER_VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![t + 0x80]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, t]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_d750, fn_00a8d750(Ptr) -> Ptr),
        entry!(0x00a8_e410, fn_00a8e410(Ptr) -> Ptr),
        entry!(0x00a8_e480, fn_00a8e480(Ptr, u32) -> Ptr),
        entry!(0x00a8_d850, fn_00a8d850(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(PIXEL_FORMAT_SOURCE_011AA2E8, 4 * PIXEL_FORMAT_WORDS);
        for word in 0..PIXEL_FORMAT_WORDS {
            e.mem
                .set_u32(PIXEL_FORMAT_SOURCE_011AA2E8 + 4 * word, 0x200 + word);
        }
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A block of 0x180 bytes all 0xff.
    fn filled(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x180);
        for i in 0..0x60 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        this
    }

    #[test]
    fn fn_00a8d750_builds_the_tga_reader() {
        let mut e = rig();
        let this = filled(&mut e);
        assert_eq!(e.call(0x00a8_d750, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TGA_READER_VTABLE);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(NI_PIXEL_FORMAT_CONSTRUCT, vec![this + TGA_PIXEL_FORMAT])));
        assert!(log.contains(&(IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])));
        // +0x100 to +0x118 cleared except the bytes +0x103 and +0x109, which it leaves.
        for offset in 0x100..0x119 {
            let expected = if offset == 0x103 || offset == 0x109 {
                0xff
            } else {
                0
            };
            assert_eq!(e.mem.u8(this + offset), expected, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u16(this + 0x11a), 0xffff);
        assert_eq!(e.mem.u32(this + TGA_PIXEL_FORMAT), 0x200);
        assert_eq!(e.mem.u32(this + TGA_PIXEL_FORMAT + 0x40), 0x210);
        for offset in 0x160..0x17d {
            assert_eq!(e.mem.u8(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x17d), 0xff);
    }

    #[test]
    fn fn_00a8e410_builds_the_bmp_reader() {
        let mut e = rig();
        let this = filled(&mut e);
        assert_eq!(e.call(0x00a8_e410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BMP_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        assert_eq!(e.mem.u32(this + 0x104), 0);
        assert_eq!(e.mem.u32(this + BMP_PIXEL_FORMAT), 0x200);
        assert_eq!(e.mem.u32(this + BMP_PIXEL_FORMAT + 0x40), 0x210);
        assert_eq!(e.mem.u16(this + 0x14c), 0);
        assert_eq!(e.mem.u16(this + 0x14e), 0xffff);
        assert_eq!(e.mem.u32(this + 0x150), 0);
        assert_eq!(e.mem.u32(this + 0x154), 0);
        assert_eq!(e.mem.u16(this + 0x158), 0);
        assert_eq!(e.mem.u8(this + 0x15a), 0xff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for f in [
            IMPORT_INITIALIZE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            NI_PIXEL_FORMAT_CONSTRUCT,
            DEALLOCATE,
            NI_FREE,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.map(PIXEL_FORMAT_SOURCE_011AA2E8, 0x44);
        e.mem.set_u32(PIXEL_FORMAT_SOURCE_011AA2E8, 0x2e8);
        e
    }

    #[test]
    fn fn_00a8e410_builds_the_bmp_reader() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x150, 9);
        e.mem.set_u16(this + 0x14c, 9);
        assert_eq!(e.call(0x00a8_e410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BMP_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x108), 0x2e8);
        assert_eq!(e.mem.u32(this + 0x150), 0);
        assert_eq!(e.mem.u16(this + 0x14c), 0);
    }

    #[test]
    fn fn_00a8d750_builds_the_tga_reader() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        for at in (0x100..0x180).step_by(4) {
            e.mem.set_u32(this + at, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_d750, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TGA_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x11c), 0x2e8);
        assert_eq!(e.mem.u32(this + 0x100), 0xff00_0000);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x17c) & 0xff, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (NI_PIXEL_FORMAT_CONSTRUCT, vec![this + 0x11c]));
    }

    #[test]
    fn fn_00a8e480_frees_on_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_e480, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (DELETE_CRITICAL_SECTION, vec![this + 0x80]),
                (DEALLOCATE, vec![MEMORY_MANAGER, this]),
            ]
        );
        assert_eq!(e.mem.u32(this), NI_IMAGE_READER_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00a8_e480, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_00a8d850_frees_the_buffers() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x164, 0x700);
        e.mem.set_u32(this + 0x16c, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x00a8_d850, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (NI_FREE, vec![0x700]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x800]),
                (DELETE_CRITICAL_SECTION, vec![this + 0x80]),
                (DEALLOCATE, vec![MEMORY_MANAGER, this]),
            ]
        );
    }
}
