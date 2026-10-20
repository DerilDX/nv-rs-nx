//! `FaceGen/algs.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00acbb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `algs.obj`). Thiscall that ignores
/// `this` and returns 0.
pub fn fn_00acbb70(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// The functions below are the C++ runtime's stream and locale code the map places in this
// unit (Ghidra's library matches name some of them; the RTTI of the vtables they store names
// the classes: `basic_streambuf<char>` `010a30fc`, `basic_filebuf<char>` `010a3138`,
// `basic_ostream<char>` `010a318c`, `basic_ios<char>` `010a2fa4`, `ios_base` `010a2f9c`,
// `ctype<char>` `010a31c4`, `ctype_base` `010a31f4`, `locale::facet` `010a31fc`,
// `runtime_error` `010a307c`, `ios_base::failure` `010a3070`).

/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::_Lockit::_Lockit` (`00ec32e3`, thiscall, one word: the lock kind) and its destructor
/// (`00ec330b`, thiscall).
const LOCKIT_CTOR: u32 = 0x00ec_32e3;
const LOCKIT_DTOR: u32 = 0x00ec_330b;
/// `_CxxThrowException` (`00ec7894`, two words: the object and the throw information); it
/// does not return in the game.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `0044a4c0` (thiscall, one word: a C string): constructs a string from it.
const STRING_FROM_C_0044A4C0: u32 = 0x0044_a4c0;
/// `0044a500` (thiscall, one word: the string to copy): copy-constructs a string.
const STRING_COPY_0044A500: u32 = 0x0044_a500;
/// `0044a730` (thiscall, two words): the string tidy the stream code calls with `(0, 0)` on a
/// new string and `(1, 0)` to destroy one.
const STRING_TIDY_0044A730: u32 = 0x0044_a730;
/// `0044a580` (thiscall, three words: source string, offset, count): assigns part of a string.
const STRING_ASSIGN_0044A580: u32 = 0x0044_a580;
/// `npos` of the string class (`.rdata`).
const STRING_NPOS_010173F0: u32 = 0x0101_73f0;
/// `00ec78e0` (thiscall, no stack words): the `std::exception` base constructor of the
/// stream failure object.
const EXCEPTION_CTOR_00EC78E0: u32 = 0x00ec_78e0;
/// The vtables `ios_base::clear` stores in its failure object (RTTI `runtime_error`,
/// `ios_base::failure`) and the failure's throw information.
const VTABLE_RUNTIME_ERROR_010A307C: u32 = 0x010a_307c;
const VTABLE_IOS_FAILURE_010A3070: u32 = 0x010a_3070;
const IOS_FAILURE_THROW_INFO_0116C3A4: u32 = 0x0116_c3a4;
/// The messages: `"ios_base::badbit set"`, `"ios_base::failbit set"`, `"ios_base::eofbit set"`.
const MESSAGE_BADBIT_010A3084: u32 = 0x010a_3084;
const MESSAGE_FAILBIT_010A3054: u32 = 0x010a_3054;
const MESSAGE_EOFBIT_010A303C: u32 = 0x010a_303c;
/// `std::locale::_Init` (`00ec36a6`, cdecl, no arguments) and the global locale getter
/// `00ec34d4` (cdecl, no arguments).
const LOCALE_INIT: u32 = 0x00ec_36a6;
const GLOBAL_LOCALE_00EC34D4: u32 = 0x00ec_34d4;
/// Vtables stored by the stream buffer and file buffer destructors and constructors (RTTI
/// `basic_streambuf<char>`, `basic_filebuf<char>`).
const VTABLE_STREAMBUF_010A30FC: u32 = 0x010a_30fc;
const VTABLE_FILEBUF_010A3138: u32 = 0x010a_3138;
/// Vtables of `ctype<char>`, `ctype_base` and `locale::facet`.
const VTABLE_CTYPE_010A31C4: u32 = 0x010a_31c4;
const VTABLE_CTYPE_BASE_010A31F4: u32 = 0x010a_31f4;
const VTABLE_FACET_010A31FC: u32 = 0x010a_31fc;

// Translated from 00ac9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this body `std::getline<...>` (`cg`), but it is a forwarder (thiscall, one
/// word: a string; `RET 4`): the string's characters are inline at `+4` when its capacity
/// (`+0x18`) is below 0x10, else at the pointer at `+4`; it calls `00ac6100` (thiscall, four
/// words) on `this` with `(0, this[+0x14], characters, length)`, the length being the
/// string's `+0x14`, and returns that call's `EAX`.
pub fn fn_00ac9da0(e: &mut Engine, this: Ptr, string: Ptr) -> u32 {
    let string = string.addr();
    let length = e.mem.u32(string + 0x14);
    let characters = if e.mem.u32(string + 0x18) < 0x10 {
        string + 4
    } else {
        e.mem.u32(string + 4)
    };
    let own = e.mem.u32(this.addr() + 0x14);
    e.call(0x00ac_6100, &args![this, 0u32, own, characters, length])
        .u32()
}

/// The throw at the end of [`ios_base_clear`]: builds the message string (0x1c bytes) and the
/// `ios_base::failure` object (0x28 bytes, its string at `+0xc`) on the stack and throws the
/// object. The game does not return from the throw; the code after it destroys the message
/// string (`0044a730(1, 0)`) and leaves.
fn throw_ios_failure(e: &mut Engine, message: u32) {
    e.with_stack(0x44, |e, frame| {
        let string = frame.addr();
        let object = frame.addr() + 0x1c;
        e.call(STRING_FROM_C_0044A4C0, &args![string, message]);
        e.call(EXCEPTION_CTOR_00EC78E0, &args![object]);
        e.mem.set_u32(object, VTABLE_RUNTIME_ERROR_010A307C);
        e.call(STRING_COPY_0044A500, &args![object + 0xc, string]);
        e.mem.set_u32(object, VTABLE_IOS_FAILURE_010A3070);
        e.call(
            CXX_THROW_EXCEPTION,
            &args![object, IOS_FAILURE_THROW_INFO_0116C3A4],
        );
        e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
    });
}

// Translated from 00ac9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9ff0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `std::ios_base::clear(state, reraise)` (the map's name; thiscall, two words, `RET 8`):
/// stores `state & 0x17` at `+8`; returns when it has no bit in common with the exception mask
/// at `+0xc`. Otherwise, with the `reraise` byte set it rethrows (`_CxxThrowException(0, 0)`);
/// else it throws an `ios_base::failure` saying `badbit` (state bit 4), `failbit` (bit 2) or
/// `eofbit`. The throws do not return in the game; the code after each returns.
pub fn ios_base_clear(e: &mut Engine, this: Ptr, state: u32, reraise: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 8, state & 0x17);
    if e.mem.u32(base + 8) & e.mem.u32(base + 0xc) == 0 {
        return;
    }
    if reraise as u8 != 0 {
        e.call(CXX_THROW_EXCEPTION, &args![0u32, 0u32]);
        return;
    }
    if e.mem.u32(base + 8) & e.mem.u32(base + 0xc) & 4 != 0 {
        throw_ios_failure(e, MESSAGE_BADBIT_010A3084);
        return;
    }
    if e.mem.u32(base + 8) & e.mem.u32(base + 0xc) & 2 != 0 {
        throw_ios_failure(e, MESSAGE_FAILBIT_010A3054);
        return;
    }
    throw_ios_failure(e, MESSAGE_EOFBIT_010A303C);
}

// Translated from 00acb6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf<char>` destructor body (thiscall). Stores the vtable
/// `010a30fc`; when the locale holder at `+0x38` is set it releases its facet
/// ([`fn_00acb720`]) and frees the holder (`operator delete`; the compiler's always-true
/// delete flag is kept as such); then destroys the mutex at `+4` (`00ec3260`, thiscall).
pub fn fn_00acb6b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_STREAMBUF_010A30FC);
    let holder = e.mem.u32(base + 0x38);
    if holder != 0 {
        fn_00acb720(e, Ptr::new(holder));
        // `MOV ECX,1; AND ECX,1; JZ`: the delete flag is the constant 1.
        let delete_flag = std::hint::black_box(1u32);
        if delete_flag & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![holder]);
        }
    }
    e.call(0x00ec_3260, &args![base + 4]);
}

// Translated from 00acb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the locale destructor shape (thiscall on a one-word holder). When the facet
/// pointer at `+0` is set it drops a reference ([`facet_decref`]); when that returns the facet
/// (no references left) it calls the facet's slot 0 (its deleting destructor) with 1.
pub fn fn_00acb720(e: &mut Engine, this: Ptr) {
    let facet = e.mem.u32(this.addr());
    if facet != 0 {
        let released = facet_decref(e, Ptr::new(facet));
        if released.addr() != 0 {
            e.vcall(released.addr(), 0, &args![1u32]);
        }
    }
}

// Translated from 00acb770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::facet::_Decref` (Ghidra library match; thiscall). Under a `_Lockit(0)` (4
/// bytes on the stack) it decrements the reference count at `+4` unless it is 0 or
/// `0xffffffff`; returns `this` when the count is then 0, else 0.
pub fn facet_decref(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.with_stack(4, |e, lock| {
        e.call(LOCKIT_CTOR, &args![lock, 0u32]);
        let count = e.mem.u32(base + 4);
        if count > 0 && count < 0xffff_ffff {
            e.mem.set_u32(base + 4, count - 1);
        }
        let result = if e.mem.u32(base + 4) == 0 { base } else { 0 };
        e.call(LOCKIT_DTOR, &args![lock]);
        Ptr::new(result)
    })
}

// Translated from 00acb7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: thiscall that ignores `this` and returns `0xffffffff` (`OR EAX,-1`).
pub fn fn_00acb7e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0xffff_ffff
}

// Translated from 00acc970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acc970 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_filebuf<char>` destructor body (thiscall). Stores the vtable `010a3138`;
/// when the byte at `+0x48` is set calls `00acfb90` (thiscall) on `this`; then runs the
/// stream-buffer destructor body [`fn_00acb6b0`].
pub fn fn_00acc970(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_FILEBUF_010A3138);
    if e.mem.u8(base + 0x48) != 0 {
        e.call(0x00ac_fb90, &args![base]);
    }
    fn_00acb6b0(e, this);
}

// Translated from 00ace2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a `basic_ostream<char>` destructor with its virtual base (thiscall). Reads the
/// virtual-base offset from `[[this] + 4]` and stores the vtable `010a318c` at `this + offset`;
/// then stores `010a2fa4` and `010a2f9c` (RTTI `basic_ios`, `ios_base`) at `+4` and destroys
/// the `ios_base` at `+4` (`00ec3205`, cdecl, one word).
pub fn fn_00ace2d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let offset = e.mem.u32(e.mem.u32(base).wrapping_add(4));
    e.mem.set_u32(base.wrapping_add(offset), 0x010a_318c);
    e.mem.set_u32(base + 4, 0x010a_2fa4);
    e.mem.set_u32(base + 4, 0x010a_2f9c);
    e.call(0x00ec_3205, &args![base + 4]);
}

// Translated from 00ace750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::getloc` shape (thiscall, one word: the locale it fills; `RET 4`).
/// Copies the word of the locale held at `+0x24` into `[out]`, adds a reference to the object
/// it points at ([`facet_incref`]) and returns `out`.
pub fn fn_00ace750(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let holder = e.mem.u32(this.addr() + 0x24);
    let facet = e.mem.u32(holder);
    e.mem.set_u32(out.addr(), facet);
    let facet = e.mem.u32(out.addr());
    facet_incref(e, Ptr::new(facet));
    out
}

// Translated from 00ace790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::facet::_Incref` (Ghidra library match; thiscall). Under a `_Lockit(0)` it
/// increments the reference count at `+4` unless it is `0xffffffff`.
pub fn facet_incref(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, lock| {
        e.call(LOCKIT_CTOR, &args![lock, 0u32]);
        let count = e.mem.u32(base + 4);
        if count < 0xffff_ffff {
            e.mem.set_u32(base + 4, count + 1);
        }
        e.call(LOCKIT_DTOR, &args![lock]);
    });
}

/// The word a file buffer copies to `+0x44` when it opens (`.data`).
const INITIAL_STATE_011F6D8C: u32 = 0x011f_6d8c;

// Translated from 00acfc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf` `_Init(file, mode)` shape (thiscall, two words, `RET 8`).
/// Sets the byte at `+0x48` to `mode == 1` and the byte at `+0x41` to 0 and resets the buffer
/// pointers ([`fn_00ad0720`]); with a file it points the pointer slots at the file's own
/// (`+0x10`/`+0x14` at `file + 8`, `+0x20`/`+0x24` at `file`, `+0x30`/`+0x34` at `file + 4`;
/// the compiler's always-true test is kept). Stores the file at `+0x4c`, the word at
/// `011f6d8c` at `+0x44` and 0 at `+0x3c`.
pub fn fn_00acfc10(e: &mut Engine, this: Ptr, file: u32, mode: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 0x48, (mode == 1) as u8);
    e.mem.set_u8(base + 0x41, 0);
    fn_00ad0720(e, this);
    // `MOV EAX,1; TEST EAX,EAX; JZ`: the second condition is the constant 1.
    let always = std::hint::black_box(1u32);
    if file != 0 && always != 0 {
        let pointers = file.wrapping_add(8);
        let middle = file.wrapping_add(4);
        e.mem.set_u32(base + 0x10, pointers);
        e.mem.set_u32(base + 0x14, pointers);
        e.mem.set_u32(base + 0x20, file);
        e.mem.set_u32(base + 0x24, file);
        e.mem.set_u32(base + 0x30, middle);
        e.mem.set_u32(base + 0x34, middle);
    }
    e.mem.set_u32(base + 0x4c, file);
    let state = e.global::<u32>(INITIAL_STATE_011F6D8C);
    e.mem.set_u32(base + 0x44, state);
    e.mem.set_u32(base + 0x3c, 0);
}

// Translated from 00ad0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_ios::init(buffer, add_standard)` shape (thiscall, two words, `RET 8`).
/// Initialises the `ios_base` part ([`fn_00ad0550`]), stores the stream buffer at `+0x28` and 0
/// at `+0x2c`, and the fill character `widen(' ')` ([`fn_00ad30c0`] with 0x20) as the byte at
/// `+0x30`. Without a buffer it sets `badbit` (`clear(state | 4, 0)`; the compiler's
/// always-true test is kept). With the `add_standard` byte set it registers the stream
/// (`00ec31b3`, cdecl, one word); otherwise it clears `+4`.
pub fn fn_00ad0470(e: &mut Engine, this: Ptr, buffer: u32, add_standard: u32) {
    let base = this.addr();
    fn_00ad0550(e, this);
    e.mem.set_u32(base + 0x28, buffer);
    e.mem.set_u32(base + 0x2c, 0);
    let fill = fn_00ad30c0(e, this, 0x20);
    e.mem.set_u8(base + 0x30, fill as u8);
    // `MOV EAX,4; TEST EAX,EAX; JZ`: the second condition is the constant 4.
    let always = std::hint::black_box(4u32);
    if e.mem.u32(base + 0x28) == 0 && always != 0 {
        let state = e.mem.u32(base + 8) | 4;
        let state = if e.mem.u32(base + 0x28) == 0 {
            state | 4
        } else {
            state
        };
        ios_base_clear(e, this, state, 0);
    }
    if add_standard as u8 != 0 {
        e.call(0x00ec_31b3, &args![base]);
    } else {
        e.mem.set_u32(base + 4, 0);
    }
}

/// The 4-byte locale both stream constructors make: `operator new(4)`; when that succeeds its
/// word is `locale::_Init()` and the global locale (`00ec34d4`) gets a reference
/// ([`facet_incref`]). Returns the block, or 0.
fn new_locale(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
    if block == 0 {
        return 0;
    }
    let implementation = e.call(LOCALE_INIT, &args![]).u32();
    e.mem.set_u32(block, implementation);
    let global = e.call(GLOBAL_LOCALE_00EC34D4, &args![]).u32();
    facet_incref(e, Ptr::new(global));
    block
}

// Translated from 00ad0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::_Init` shape (thiscall). Clears `+0x24`, `+4` and `+0xc`, stores
/// the format flags 0x201 at `+0x10` and the precision 6 at `+0x14`, clears `+0x18` to `+0x20`,
/// runs `clear(0, 0)` ([`ios_base_clear`]) and stores a new locale (see [`new_locale`]) at
/// `+0x24`.
pub fn fn_00ad0550(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0x201);
    e.mem.set_u32(base + 0x14, 6);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    ios_base_clear(e, this, 0, 0);
    let locale = new_locale(e);
    e.mem.set_u32(base + 0x24, locale);
}

// Translated from 00ad0630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad0630 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_streambuf<char>` constructor (thiscall). Stores the vtable `010a30fc`,
/// constructs the mutex at `+4` (`00ec3246`, thiscall), stores a new locale (see
/// [`new_locale`]) at `+0x38` and resets the buffer pointers ([`fn_00ad0720`]). Returns
/// `this`.
pub fn fn_00ad0630(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_STREAMBUF_010A30FC);
    e.call(0x00ec_3246, &args![base + 4]);
    let locale = new_locale(e);
    e.mem.set_u32(base + 0x38, locale);
    fn_00ad0720(e, this);
    this
}

// Translated from 00ad0720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the stream buffer's `_Init()` (thiscall). Points the six pointer slots at its own
/// storage (`+0x10` at `+8`, `+0x14` at `+0xc`, `+0x20` at `+0x18`, `+0x24` at `+0x1c`, `+0x30`
/// at `+0x28`, `+0x34` at `+0x2c`), then clears the words they point at, in the exe's order
/// (through `+0x14`, `+0x24`, `+0x34`, `+0x10`, `+0x20`, `+0x30`).
pub fn fn_00ad0720(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x10, base + 8);
    e.mem.set_u32(base + 0x14, base + 0xc);
    e.mem.set_u32(base + 0x20, base + 0x18);
    e.mem.set_u32(base + 0x24, base + 0x1c);
    e.mem.set_u32(base + 0x30, base + 0x28);
    e.mem.set_u32(base + 0x34, base + 0x2c);
    for slot in [0x14u32, 0x24, 0x34, 0x10, 0x20, 0x30] {
        let target = e.mem.u32(base + slot);
        e.mem.set_u32(target, 0);
    }
}

// Translated from 00ad30c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad30c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// The map names this body `std::ios_base::eof` (`cg`); it is the `basic_ios::widen(ch)` shape
/// (thiscall, one word, `RET 4`): gets the stream's locale into a 4-byte local
/// ([`fn_00ace750`]), its `ctype<char>` facet ([`fn_00ad38d0`]), releases the local
/// ([`fn_00acb720`]) and returns the facet's slot `0x18` called with the character byte
/// (zero-extended).
pub fn fn_00ad30c0(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    e.with_stack(4, |e, local| {
        let locale = fn_00ace750(e, this, local);
        let facet = fn_00ad38d0(e, locale);
        fn_00acb720(e, local);
        e.vcall(facet.addr(), 0x18, &args![character & 0xff]).u32()
    })
}

// Translated from 00ad33c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad33c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed (thiscall, one stack word it does not read, `RET 4`): allocates one word through
/// `0044d670` (cdecl; called with the two words 1 and 0), stores `this` in it when the
/// allocation succeeded, and stores the block (or 0) at `+0`. Returns `this`.
pub fn fn_00ad33c0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    let block = e.call(0x0044_d670, &args![1u32, 0u32]).u32();
    let stored = if block != 0 {
        e.mem.set_u32(block, this.addr());
        block
    } else {
        0
    };
    e.mem.set_u32(this.addr(), stored);
    this
}

/// The cached `ctype<char>` facet, its facet id and the facet id counter (`.data`).
const CACHED_CTYPE_011F6D90: u32 = 0x011f_6d90;
const CTYPE_ID_0126FBA4: u32 = 0x0126_fba4;
const FACET_ID_COUNTER_0126FBA0: u32 = 0x0126_fba0;

// Translated from 00ad38d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad38d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `std::use_facet<ctype<char>>(locale)` (cdecl, one word). Under a `_Lockit(0)` it
/// reads the cached facet `011f6d90`; when the facet id `0126fba4` is 0 it gives it the next
/// id from `0126fba0` (checked again under a second `_Lockit(0)`). It looks the id up in the
/// locale ([`locale_getfacet`]); when that finds none it takes the cached facet, or when there
/// is none makes one ([`fn_00ad3aa0`] on the cache local): a result of -1 throws `bad_cast`
/// (`00ec79c4` with `"bad cast"` `010a31b4`, throw information `0116cdb8`; the game does not
/// return, the code after it returns 0), otherwise the new facet becomes the cached one, gets
/// a reference ([`facet_incref`]) and is registered (`00ec3599`, cdecl). Returns the facet.
pub fn fn_00ad38d0(e: &mut Engine, locale: Ptr) -> Ptr {
    e.with_stack(0x20, |e, frame| {
        // Frame: the inner lock at +0, the `bad_cast` object at +8 (0xc bytes), the cache local
        // at +0x18 and the outer lock at +0x1c.
        let inner_lock = frame.addr();
        let bad_cast = frame.addr() + 8;
        let cached_at = frame.addr() + 0x18;
        let outer_lock = frame.addr() + 0x1c;
        e.call(LOCKIT_CTOR, &args![outer_lock, 0u32]);
        let cached = e.global::<u32>(CACHED_CTYPE_011F6D90);
        e.mem.set_u32(cached_at, cached);
        if e.global::<u32>(CTYPE_ID_0126FBA4) == 0 {
            e.call(LOCKIT_CTOR, &args![inner_lock, 0u32]);
            if e.global::<u32>(CTYPE_ID_0126FBA4) == 0 {
                let next = e.global::<u32>(FACET_ID_COUNTER_0126FBA0).wrapping_add(1);
                e.set_global(FACET_ID_COUNTER_0126FBA0, next);
                let id = e.global::<u32>(FACET_ID_COUNTER_0126FBA0);
                e.set_global(CTYPE_ID_0126FBA4, id);
            }
            e.call(LOCKIT_DTOR, &args![inner_lock]);
        }
        let id = e.global::<u32>(CTYPE_ID_0126FBA4);
        let mut facet = locale_getfacet(e, locale, id);
        if facet == 0 {
            let cached = e.mem.u32(cached_at);
            if cached != 0 {
                facet = cached;
            } else if fn_00ad3aa0(e, Ptr::new(cached_at), locale) == 0xffff_ffff {
                e.call(0x00ec_79c4, &args![bad_cast, 0x010a_31b4u32]);
                e.call(CXX_THROW_EXCEPTION, &args![bad_cast, 0x0116_cdb8u32]);
            } else {
                facet = e.mem.u32(cached_at);
                let made = e.mem.u32(cached_at);
                e.set_global(CACHED_CTYPE_011F6D90, made);
                let made = e.mem.u32(cached_at);
                facet_incref(e, Ptr::new(made));
                e.call(0x00ec_3599, &args![made]);
            }
        }
        e.call(LOCKIT_DTOR, &args![outer_lock]);
        Ptr::new(facet)
    })
}

// Translated from 00ad3a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::_Getfacet(id)` (Ghidra library match; thiscall, one word, `RET 4`): the
/// entry `id` of the implementation's facet table (`+8`, count `+0xc`) when `id` is below the
/// count, else 0; when that is 0 and the implementation's byte at `+0x14` is set, the same
/// lookup in the global locale implementation (`00ec34d4`).
pub fn locale_getfacet(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let found = if id < e.mem.u32(implementation + 0xc) {
        let table = e.mem.u32(implementation + 8);
        e.mem.u32(table.wrapping_add(id.wrapping_mul(4)))
    } else {
        0
    };
    if found != 0 || e.mem.u8(e.mem.u32(this.addr()) + 0x14) == 0 {
        return found;
    }
    let global = e.call(GLOBAL_LOCALE_00EC34D4, &args![]).u32();
    if id < e.mem.u32(global + 0xc) {
        let table = e.mem.u32(global + 8);
        e.mem.u32(table.wrapping_add(id.wrapping_mul(4)))
    } else {
        0
    }
}

// Translated from 00ad3aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3aa0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `ctype<char>::_Getcat(slot, locale)` (cdecl, two words). When the slot is set and
/// holds 0 it allocates 0x18 bytes; when that succeeds it takes the locale's name
/// ([`fn_00ad3e60`] into a stack string), builds a `_Locinfo` from it ([`fn_00ad3ca0`]), stores
/// the vtables `010a31fc`, `010a31f4`, `010a31c4` (0 at `+4` after the first) and copies the
/// four words of `__Getctype` (`00ec38a9` on a stack `_Ctypevec`) to `+8`. The block (or 0)
/// goes into the slot; the `_Locinfo` ([`fn_00ad3db0`]) and the string (`0044a730(1, 0)`) are
/// destroyed when they were built. Returns 2.
pub fn fn_00ad3aa0(e: &mut Engine, slot: Ptr, locale: Ptr) -> u32 {
    if slot.addr() == 0 || e.mem.u32(slot.addr()) != 0 {
        return 2;
    }
    e.with_stack(0xf4, |e, frame| {
        // Frame: the `_Ctypevec` at +0 (0x10 bytes), the name string at +0x64, the `_Locinfo`
        // at +0x80.
        let ctype_vector = frame.addr();
        let string = frame.addr() + 0x64;
        let locinfo = frame.addr() + 0x80;
        let mut built = 0u32;
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        let made = if block != 0 {
            let name = fn_00ad3e60(e, locale, Ptr::new(string));
            built |= 1;
            fn_00ad3ca0(e, Ptr::new(locinfo), name.addr());
            built |= 2;
            e.mem.set_u32(block, VTABLE_FACET_010A31FC);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block, VTABLE_CTYPE_BASE_010A31F4);
            e.mem.set_u32(block, VTABLE_CTYPE_010A31C4);
            let vector = e.call(0x00ec_38a9, &args![ctype_vector]).u32();
            let words = [
                e.mem.u32(vector),
                e.mem.u32(vector + 4),
                e.mem.u32(vector + 8),
                e.mem.u32(vector + 0xc),
            ];
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(block + 8 + 4 * i as u32, *word);
            }
            block
        } else {
            0
        };
        e.mem.set_u32(slot.addr(), made);
        if built & 2 != 0 {
            built &= !2;
            fn_00ad3db0(e, Ptr::new(locinfo));
        }
        if built & 1 != 0 {
            e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        }
    });
    2
}

// Translated from 00ad3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3ca0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `_Locinfo(name)` constructor (thiscall, one word: the name string; `RET 4`).
/// Constructs its `_Lockit(0)` at `+0`, initialises the strings at `+4`, `+0x20`, `+0x3c` and
/// `+0x58` (`0044a730(0, 0)`), then runs `_Locinfo_ctor(this, name)` (`00ec3740`, cdecl).
/// Returns `this`.
pub fn fn_00ad3ca0(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    let base = this.addr();
    e.call(LOCKIT_CTOR, &args![base, 0u32]);
    for offset in [4u32, 0x20, 0x3c, 0x58] {
        e.call(STRING_TIDY_0044A730, &args![base + offset, 0u32, 0u32]);
    }
    e.call(0x00ec_3740, &args![base, name]);
    this
}

// Translated from 00ad3db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3db0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `_Locinfo` destructor (thiscall): `_Locinfo_dtor(this)` (`00ec35d7`, cdecl),
/// destroys the strings at `+0x58`, `+0x3c`, `+0x20` and `+4` (`0044a730(1, 0)`) and the
/// `_Lockit` at `+0`.
pub fn fn_00ad3db0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x00ec_35d7, &args![base]);
    for offset in [0x58u32, 0x3c, 0x20, 4] {
        e.call(STRING_TIDY_0044A730, &args![base + offset, 1u32, 0u32]);
    }
    e.call(LOCKIT_DTOR, &args![base]);
}

// Translated from 00ad3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `locale::name()` (thiscall, one word: the string it fills; `RET 4`). Initialises
/// the string (`0044a730(0, 0)`), assigns it the implementation's name string at `+0x18`
/// (`0044a580(source, 0, npos)`, `npos` read from `010173f0`) and returns it.
pub fn fn_00ad3e60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let source = e.mem.u32(this.addr()).wrapping_add(0x18);
    e.call(STRING_TIDY_0044A730, &args![out, 0u32, 0u32]);
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(STRING_ASSIGN_0044A580, &args![out, source, 0u32, npos]);
    out
}

// Translated from 00ad4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, one word, `RET 4`): returns the low byte of its argument (`MOV AL`), the
/// shape of `ctype<char>::do_widen`.
pub fn fn_00ad4000(_e: &mut Engine, _this: Ptr, value: u32) -> u8 {
    value as u8
}

// Translated from 00ad4110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad4110 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `ctype<char>` scalar deleting destructor (thiscall, one word, `RET 4`). Stores
/// the vtable `010a31c4`, frees the table ([`ctype_tidy`]), stores `010a31f4` and `010a31fc`
/// and with flag bit 0 frees `this` (`operator delete`). Returns `this`.
pub fn fn_00ad4110(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_CTYPE_010A31C4);
    ctype_tidy(e, this);
    e.mem.set_u32(base, VTABLE_CTYPE_BASE_010A31F4);
    e.mem.set_u32(base, VTABLE_FACET_010A31FC);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

// Translated from 00ad41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::_Tidy` (Ghidra library match; thiscall): with a positive delete flag at
/// `+0x14` frees the table at `+0x10` with `_free` (`00ecd291`), with a negative one with
/// `operator delete`.
pub fn ctype_tidy(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let flag = e.mem.u32(base + 0x14) as i32;
    if flag > 0 {
        let table = e.mem.u32(base + 0x10);
        e.call(0x00ec_d291, &args![table]);
    } else if flag < 0 {
        let table = e.mem.u32(base + 0x10);
        e.call(OPERATOR_DELETE, &args![table]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00acbb70, fn_00acbb70(Ptr) -> u32),
        entry!(0x00ac_9da0, fn_00ac9da0(Ptr, Ptr) -> u32),
        entry!(0x00ac_9ff0, ios_base_clear(Ptr, u32, u32)),
        entry!(0x00ac_b6b0, fn_00acb6b0(Ptr)),
        entry!(0x00ac_b720, fn_00acb720(Ptr)),
        entry!(0x00ac_b770, facet_decref(Ptr) -> Ptr),
        entry!(0x00ac_b7e0, fn_00acb7e0(Ptr) -> u32),
        entry!(0x00ac_c970, fn_00acc970(Ptr)),
        entry!(0x00ac_e2d0, fn_00ace2d0(Ptr)),
        entry!(0x00ac_e750, fn_00ace750(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_e790, facet_incref(Ptr)),
        entry!(0x00ac_fc10, fn_00acfc10(Ptr, u32, u32)),
        entry!(0x00ad_0470, fn_00ad0470(Ptr, u32, u32)),
        entry!(0x00ad_0550, fn_00ad0550(Ptr)),
        entry!(0x00ad_0630, fn_00ad0630(Ptr) -> Ptr),
        entry!(0x00ad_0720, fn_00ad0720(Ptr)),
        entry!(0x00ad_30c0, fn_00ad30c0(Ptr, u32) -> u32),
        entry!(0x00ad_33c0, fn_00ad33c0(Ptr, u32) -> Ptr),
        entry!(0x00ad_38d0, fn_00ad38d0(Ptr) -> Ptr),
        entry!(0x00ad_3a10, locale_getfacet(Ptr, u32) -> u32),
        entry!(0x00ad_3aa0, fn_00ad3aa0(Ptr, Ptr) -> u32),
        entry!(0x00ad_3ca0, fn_00ad3ca0(Ptr, u32) -> Ptr),
        entry!(0x00ad_3db0, fn_00ad3db0(Ptr)),
        entry!(0x00ad_3e60, fn_00ad3e60(Ptr, Ptr) -> Ptr),
        entry!(0x00ad_4000, fn_00ad4000(Ptr, u32) -> u8),
        entry!(0x00ad_4110, fn_00ad4110(Ptr, u32) -> Ptr),
        entry!(0x00ad_41c0, ctype_tidy(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ac_bb70, &args![Ptr::<()>::new(0x10)]).u32(), 0);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    #[test]
    fn b046_00ac9da0_passes_the_inline_or_heap_characters() {
        let mut e = Engine::new();
        e.register(0x00ac_6100, |_, _| 0x77u32.into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5555);
        let string = e.mem.alloc(0x1c);
        e.mem.set_u32(string + 0x14, 3);
        e.mem.set_u32(string + 0x18, 0xf);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ac_9da0, &args![this, string]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(
            calls_to(&e, 0x00ac_6100),
            vec![vec![this, 0, 0x5555, string + 4, 3]]
        );
        e.mem.set_u32(string + 0x18, 0x10);
        e.mem.set_u32(string + 4, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x00ac_9da0, &args![this, string]);
        assert_eq!(
            calls_to(&e, 0x00ac_6100),
            vec![vec![this, 0, 0x5555, 0x9000, 3]]
        );
    }

    #[test]
    fn b046_00ac9ff0_clear_stores_the_state_and_throws_by_bit() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                STRING_FROM_C_0044A4C0,
                EXCEPTION_CTOR_00EC78E0,
                STRING_COPY_0044A500,
                CXX_THROW_EXCEPTION,
                STRING_TIDY_0044A730,
            ],
        );
        let this = e.mem.alloc(0x10);
        // No bit in common with the mask: nothing thrown.
        e.mem.set_u32(this + 0xc, 0x2);
        e.call_log = Some(vec![]);
        e.call(0x00ac_9ff0, &args![this, 0xffff_ff0du32, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0x05);
        assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
        // Reraise: `_CxxThrowException(0, 0)` only.
        e.call_log = Some(vec![]);
        e.call(0x00ac_9ff0, &args![this, 2u32, 0x101u32]);
        assert_eq!(calls_to(&e, CXX_THROW_EXCEPTION), vec![vec![0, 0]]);
        assert!(calls_to(&e, STRING_FROM_C_0044A4C0).is_empty());
        // Only the low byte of the reraise word counts.
        for (state, mask, message) in [
            (4u32, 6u32, MESSAGE_BADBIT_010A3084),
            (6, 2, MESSAGE_FAILBIT_010A3054),
            (1, 1, MESSAGE_EOFBIT_010A303C),
        ] {
            e.mem.set_u32(this + 0xc, mask);
            e.call_log = Some(vec![]);
            e.call(0x00ac_9ff0, &args![this, state, 0x100u32]);
            let made = calls_to(&e, STRING_FROM_C_0044A4C0);
            assert_eq!(made.len(), 1);
            assert_eq!(made[0][1], message);
            let string = made[0][0];
            let object = string + 0x1c;
            assert_eq!(calls_to(&e, EXCEPTION_CTOR_00EC78E0), vec![vec![object]]);
            assert_eq!(
                calls_to(&e, STRING_COPY_0044A500),
                vec![vec![object + 0xc, string]]
            );
            assert_eq!(
                calls_to(&e, CXX_THROW_EXCEPTION),
                vec![vec![object, IOS_FAILURE_THROW_INFO_0116C3A4]]
            );
            assert_eq!(calls_to(&e, STRING_TIDY_0044A730), vec![vec![string, 1, 0]]);
        }
    }

    fn facet(e: &mut Engine, count: u32) -> u32 {
        let facet = e.mem.alloc(8);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(facet, 0x0300_0000);
        e.mem.set_u32(facet + 4, count);
        facet
    }

    #[test]
    fn b046_00acb6b0_releases_the_locale_and_the_mutex() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[OPERATOR_DELETE, 0x00ec_3260, LOCKIT_CTOR, LOCKIT_DTOR],
        );
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b6b0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_STREAMBUF_010A30FC);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![this + 4]]);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(this + 0x38, holder);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b6b0, &args![this]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![holder]]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![this + 4]]);
    }

    #[test]
    fn b046_00acb720_deletes_the_facet_when_its_last_reference_goes() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR, 0x0300_1000]);
        let holder = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b720, &args![holder]);
        assert!(calls_to(&e, LOCKIT_CTOR).is_empty());
        let shared = facet(&mut e, 2);
        e.mem.set_u32(holder, shared);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b720, &args![holder]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(e.mem.u32(shared + 4), 1);
        e.call(0x00ac_b720, &args![holder]);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![shared, 1]]);
    }

    #[test]
    fn b046_00acb770_decref_keeps_zero_and_the_immortal_count() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR]);
        for (count, after, released) in [
            (2u32, 1u32, false),
            (1, 0, true),
            (0, 0, true),
            (0xffff_ffff, 0xffff_ffff, false),
        ] {
            let shared = facet(&mut e, count);
            e.call_log = Some(vec![]);
            let result = e.call(0x00ac_b770, &args![shared]).u32();
            assert_eq!(e.mem.u32(shared + 4), after);
            assert_eq!(result, if released { shared } else { 0 });
            let lock = calls_to(&e, LOCKIT_CTOR);
            assert_eq!(lock.len(), 1);
            assert_eq!(lock[0][1], 0);
            assert_eq!(calls_to(&e, LOCKIT_DTOR), vec![vec![lock[0][0]]]);
        }
    }

    #[test]
    fn b046_00acb7e0_returns_all_ones() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ac_b7e0, &args![0x10u32]).u32(), 0xffff_ffff);
    }

    #[test]
    fn b046_00acc970_closes_an_open_file_then_runs_the_base() {
        let mut e = Engine::new();
        stub(&mut e, &[0x00ac_fb90, 0x00ec_3260]);
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x00ac_c970, &args![this]);
        assert!(calls_to(&e, 0x00ac_fb90).is_empty());
        assert_eq!(e.mem.u32(this), VTABLE_STREAMBUF_010A30FC);
        e.mem.set_u8(this + 0x48, 1);
        e.register(0x00ac_fb90, |e, a| {
            assert_eq!(e.mem.u32(a[0]), VTABLE_FILEBUF_010A3138);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ac_c970, &args![this]);
        assert_eq!(calls_to(&e, 0x00ac_fb90), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![this + 4]]);
    }

    #[test]
    fn b046_00ace2d0_stores_the_vtables_and_destroys_the_base() {
        let mut e = Engine::new();
        stub(&mut e, &[0x00ec_3205]);
        let this = e.mem.alloc(0x40);
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x18);
        e.mem.set_u32(this, vbtable);
        e.call_log = Some(vec![]);
        e.call(0x00ac_e2d0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x18), 0x010a_318c);
        assert_eq!(e.mem.u32(this + 4), 0x010a_2f9c);
        assert_eq!(calls_to(&e, 0x00ec_3205), vec![vec![this + 4]]);
    }

    #[test]
    fn b046_00ace750_and_00ace790_copy_and_reference_the_locale() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR]);
        let shared = facet(&mut e, 5);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, shared);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x24, holder);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ac_e750, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u32(out), shared);
        assert_eq!(e.mem.u32(shared + 4), 6);
        let immortal = facet(&mut e, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00ac_e790, &args![immortal]);
        assert_eq!(e.mem.u32(immortal + 4), 0xffff_ffff);
        assert_eq!(calls_to(&e, LOCKIT_CTOR).len(), 1);
        assert_eq!(calls_to(&e, LOCKIT_DTOR).len(), 1);
    }

    #[test]
    fn b046_00acfc10_points_the_buffer_at_the_file() {
        let mut e = Engine::new();
        e.map(INITIAL_STATE_011F6D8C, 4);
        e.set_global(INITIAL_STATE_011F6D8C, 0x1234u32);
        let this = e.mem.alloc(0x50);
        e.mem.write(this, &[0xcc; 0x50]);
        e.call(0x00ac_fc10, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u8(this + 0x48), 1);
        assert_eq!(e.mem.u8(this + 0x41), 0);
        assert_eq!(e.mem.u32(this + 0x10), this + 8);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0x1234);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        e.call(0x00ac_fc10, &args![this, 0x7000u32, 2u32]);
        assert_eq!(e.mem.u8(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x7008);
        assert_eq!(e.mem.u32(this + 0x14), 0x7008);
        assert_eq!(e.mem.u32(this + 0x20), 0x7000);
        assert_eq!(e.mem.u32(this + 0x24), 0x7000);
        assert_eq!(e.mem.u32(this + 0x30), 0x7004);
        assert_eq!(e.mem.u32(this + 0x34), 0x7004);
        assert_eq!(e.mem.u32(this + 0x4c), 0x7000);
    }

    #[test]
    fn b046_00ad0720_resets_the_six_pointers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        e.mem.write(this, &[0xcc; 0x38]);
        e.call(0x00ad_0720, &args![this]);
        for (slot, target) in [
            (0x10, 8),
            (0x14, 0xc),
            (0x20, 0x18),
            (0x24, 0x1c),
            (0x30, 0x28),
            (0x34, 0x2c),
        ] {
            assert_eq!(e.mem.u32(this + slot), this + target);
            assert_eq!(e.mem.u32(this + target), 0);
        }
    }

    /// A stream whose locale finds no `ctype` facet but the cached one: the cached facet's slot
    /// `0x18` (widen) returns the character plus 1.
    fn stream_world(e: &mut Engine) -> (u32, u32, u32) {
        stub(
            e,
            &[
                LOCKIT_CTOR,
                LOCKIT_DTOR,
                0x00ec_31b3,
                CXX_THROW_EXCEPTION,
                0x0300_1000,
            ],
        );
        e.map(CACHED_CTYPE_011F6D90, 4);
        e.map(FACET_ID_COUNTER_0126FBA0, 8);
        // The locale implementation (a facet itself: vtable, reference count, an empty table).
        let implementation = e.mem.alloc(0x18);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(implementation, 0x0300_0000);
        e.mem.set_u32(implementation + 4, 5);
        let global = facet(e, 1);
        e.register_double(LOCALE_INIT, move |_, _| implementation.into_ret());
        e.register_double(GLOBAL_LOCALE_00EC34D4, move |_, _| global.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let ctype = e.mem.alloc(8);
        e.put_vtable(0x0310_0000, &[0, 0, 0, 0, 0, 0, 0x0310_1000]);
        e.mem.set_u32(ctype, 0x0310_0000);
        e.mem.set_u32(ctype + 4, 0xffff_ffff);
        e.register(0x0310_1000, |_, a| (a[1] + 1).into_ret());
        e.set_global(CACHED_CTYPE_011F6D90, ctype);
        (implementation, global, ctype)
    }

    #[test]
    fn b046_00ad0550_initialises_the_base_and_its_locale() {
        let mut e = Engine::new();
        let (implementation, global, _) = stream_world(&mut e);
        let this = e.mem.alloc(0x34);
        e.mem.write(this, &[0xcc; 0x34]);
        e.call(0x00ad_0550, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x201);
        assert_eq!(e.mem.u32(this + 0x14), 6);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let locale = e.mem.u32(this + 0x24);
        assert_eq!(e.mem.u32(locale), implementation);
        assert_eq!(e.mem.u32(global + 4), 2);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call(0x00ad_0550, &args![this]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(global + 4), 2);
    }

    #[test]
    fn b046_00ad0470_and_00ad30c0_widen_the_fill_and_mark_a_missing_buffer() {
        let mut e = Engine::new();
        let (_, global, _) = stream_world(&mut e);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00ad_0470, &args![this, 0u32, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x30), 0x21);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 8), 4);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&e, 0x00ec_31b3).is_empty());
        // The widen's locale reference was taken and dropped again.
        assert_eq!(e.mem.u32(global + 4), 2);
        e.call_log = Some(vec![]);
        e.call(0x00ad_0470, &args![this, 0x7000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x7000);
        assert_eq!(calls_to(&e, 0x00ec_31b3), vec![vec![this]]);
        assert_eq!(e.call(0x00ad_30c0, &args![this, 0x141u32]).u32(), 0x42);
    }

    #[test]
    fn b046_00ad0630_constructs_the_stream_buffer() {
        let mut e = Engine::new();
        let (implementation, _, _) = stream_world(&mut e);
        stub(&mut e, &[0x00ec_3246]);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_0630, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_STREAMBUF_010A30FC);
        assert_eq!(calls_to(&e, 0x00ec_3246), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x38)), implementation);
        assert_eq!(e.mem.u32(this + 0x34), this + 0x2c);
    }

    #[test]
    fn b046_00ad33c0_stores_the_back_pointer_block() {
        let mut e = Engine::new();
        let block = e.mem.alloc(4);
        e.register_double(0x0044_d670, move |_, _| block.into_ret());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_33c0, &args![this, 9u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0044_d670), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(this), block);
        assert_eq!(e.mem.u32(block), this);
        e.register(0x0044_d670, |_, _| 0u32.into_ret());
        e.call(0x00ad_33c0, &args![this, 9u32]);
        assert_eq!(e.mem.u32(this), 0);
    }

    fn locale_with(e: &mut Engine, table: &[u32], fallback: bool) -> u32 {
        let implementation = e.mem.alloc(0x18);
        let entries = e.mem.alloc(4 * table.len() as u32 + 4);
        for (i, entry) in table.iter().enumerate() {
            e.mem.set_u32(entries + 4 * i as u32, *entry);
        }
        e.mem.set_u32(implementation + 8, entries);
        e.mem.set_u32(implementation + 0xc, table.len() as u32);
        e.mem.set_u8(implementation + 0x14, fallback as u8);
        let locale = e.mem.alloc(4);
        e.mem.set_u32(locale, implementation);
        locale
    }

    #[test]
    fn b046_00ad3a10_looks_in_the_locale_then_the_global_one() {
        let mut e = Engine::new();
        let global = e.mem.alloc(0x18);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(entries + 4, 0x4444);
        e.mem.set_u32(global + 8, entries);
        e.mem.set_u32(global + 0xc, 2);
        e.register_double(GLOBAL_LOCALE_00EC34D4, move |_, _| global.into_ret());
        let own = locale_with(&mut e, &[0, 0x1111], false);
        assert_eq!(e.call(0x00ad_3a10, &args![own, 1u32]).u32(), 0x1111);
        assert_eq!(e.call(0x00ad_3a10, &args![own, 0u32]).u32(), 0);
        let falls_back = locale_with(&mut e, &[0], true);
        assert_eq!(e.call(0x00ad_3a10, &args![falls_back, 1u32]).u32(), 0x4444);
        assert_eq!(e.call(0x00ad_3a10, &args![falls_back, 2u32]).u32(), 0);
    }

    fn ctype_world(e: &mut Engine) -> u32 {
        stub(
            e,
            &[
                LOCKIT_CTOR,
                LOCKIT_DTOR,
                STRING_TIDY_0044A730,
                STRING_ASSIGN_0044A580,
                0x00ec_3740,
                0x00ec_35d7,
                0x00ec_3599,
            ],
        );
        e.map(STRING_NPOS_010173F0, 4);
        e.set_global(STRING_NPOS_010173F0, 0xffff_ffffu32);
        e.map(CACHED_CTYPE_011F6D90, 4);
        e.map(FACET_ID_COUNTER_0126FBA0, 8);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let vector = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(vector + 4 * i, 0xa0 + i);
        }
        e.register_double(0x00ec_38a9, move |_, _| vector.into_ret());
        vector
    }

    #[test]
    fn b046_00ad3aa0_makes_a_ctype_facet_once() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        let locale = locale_with(&mut e, &[], false);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3aa0, &args![slot, locale]).u32(), 2);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        e.mem.set_u32(slot, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3aa0, &args![slot, locale]).u32(), 2);
        let made = e.mem.u32(slot);
        assert_eq!(e.mem.u32(made), VTABLE_CTYPE_010A31C4);
        assert_eq!(e.mem.u32(made + 4), 0);
        for i in 0..4 {
            assert_eq!(e.mem.u32(made + 8 + 4 * i), 0xa0 + i);
        }
        assert_eq!(calls_to(&e, 0x00ec_35d7).len(), 1);
        let tidy = calls_to(&e, STRING_TIDY_0044A730);
        assert_eq!(tidy.last().unwrap()[1..], [1, 0]);
        assert!(e.call(0x00ad_3aa0, &args![0u32, locale]).u32() == 2);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.mem.set_u32(slot, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ad_3aa0, &args![slot, locale]);
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls_to(&e, 0x00ec_35d7).is_empty());
        assert!(calls_to(&e, STRING_TIDY_0044A730).is_empty());
    }

    #[test]
    fn b046_00ad38d0_finds_caches_or_makes_the_facet() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        e.set_global(FACET_ID_COUNTER_0126FBA0, 4u32);
        // Found in the locale: the id is assigned (5) and the entry returned.
        let mut table = vec![0u32; 6];
        table[5] = 0x5151;
        let locale = locale_with(&mut e, &table, false);
        assert_eq!(e.call(0x00ad_38d0, &args![locale]).u32(), 0x5151);
        assert_eq!(e.global::<u32>(CTYPE_ID_0126FBA4), 5);
        assert_eq!(e.global::<u32>(FACET_ID_COUNTER_0126FBA0), 5);
        // Not found, cached: the cached one.
        let empty = locale_with(&mut e, &[], false);
        e.set_global(CACHED_CTYPE_011F6D90, 0x6262u32);
        assert_eq!(e.call(0x00ad_38d0, &args![empty]).u32(), 0x6262);
        // Not found, none cached: made, cached, referenced and registered.
        e.set_global(CACHED_CTYPE_011F6D90, 0u32);
        e.call_log = Some(vec![]);
        let made = e.call(0x00ad_38d0, &args![empty]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.global::<u32>(CACHED_CTYPE_011F6D90), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls_to(&e, 0x00ec_3599), vec![vec![made]]);
        // The outer lock, the `_Locinfo`'s and `_Incref`'s.
        assert_eq!(calls_to(&e, LOCKIT_CTOR).len(), 3);
    }

    #[test]
    fn b046_00ad3ca0_and_00ad3db0_build_and_destroy_the_locinfo() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3ca0, &args![this, 0x8000u32]).u32(), this);
        assert_eq!(calls_to(&e, LOCKIT_CTOR), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![
                vec![this + 4, 0, 0],
                vec![this + 0x20, 0, 0],
                vec![this + 0x3c, 0, 0],
                vec![this + 0x58, 0, 0]
            ]
        );
        assert_eq!(calls_to(&e, 0x00ec_3740), vec![vec![this, 0x8000]]);
        e.call_log = Some(vec![]);
        e.call(0x00ad_3db0, &args![this]);
        assert_eq!(calls_to(&e, 0x00ec_35d7), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![
                vec![this + 0x58, 1, 0],
                vec![this + 0x3c, 1, 0],
                vec![this + 0x20, 1, 0],
                vec![this + 4, 1, 0]
            ]
        );
        assert_eq!(calls_to(&e, LOCKIT_DTOR), vec![vec![this]]);
    }

    #[test]
    fn b046_00ad3e60_assigns_the_locale_name() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        let implementation = e.mem.alloc(0x40);
        let locale = e.mem.alloc(4);
        e.mem.set_u32(locale, implementation);
        let out = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3e60, &args![locale, out]).u32(), out);
        assert_eq!(calls_to(&e, STRING_TIDY_0044A730), vec![vec![out, 0, 0]]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A580),
            vec![vec![out, implementation + 0x18, 0, 0xffff_ffff]]
        );
    }

    #[test]
    fn b046_00ad4000_returns_the_low_byte() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00ad_4000, &args![0u32, 0x1234u32]).u32() & 0xff,
            0x34
        );
    }

    #[test]
    fn b046_00ad4110_and_00ad41c0_free_the_table_by_its_flag() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE, 0x00ec_d291]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x10, 0x9000);
        for (flag, freed, deleted) in [
            (1u32, true, false),
            (0xffff_ffff, false, true),
            (0, false, false),
        ] {
            e.mem.set_u32(this + 0x14, flag);
            e.call_log = Some(vec![]);
            e.call(0x00ad_41c0, &args![this]);
            assert_eq!(!calls_to(&e, 0x00ec_d291).is_empty(), freed);
            assert_eq!(!calls_to(&e, OPERATOR_DELETE).is_empty(), deleted);
        }
        e.mem.set_u32(this + 0x14, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_4110, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_FACET_010A31FC);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x00ad_4110, &args![this, 3u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }
}
