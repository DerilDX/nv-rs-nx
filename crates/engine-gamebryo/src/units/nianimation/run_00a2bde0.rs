//! `NiAnimation/run_00a2bde0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a2c4c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3696` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2c4c0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3696) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d5c, 0x0);
    e.mem.set_u32(0x011f_3c3c, 0x0);
    e.mem.set_u32(0x011f_3bac, 0x0);
    e.mem.set_u8(0x011f_3696, 0x1);
    e.mem.set_u32(0x011f_3dec, 0xa2c420);
    e.mem.set_u32(0x011f_3a8c, 0xa2c490);
    e.mem.set_u32(0x011f_39fc, 0xa2bbf0);
    e.mem.set_u32(0x011f_396c, 0xa2be60);
    e.mem.set_u32(0x011f_38dc, 0xa2a600);
    e.mem.set_u8(0x011f_3767, 0x40);
    e.mem.set_u32(0x011f_3ccc, 0xa2bc60);
    e.mem.set_u32(0x011f_3b1c, 0xa2bde0);
    e.mem.set_u32(0x011f_37bc, 0xa2c130);
    e.mem.set_u32(0x011f_384c, 0xa2c1b0);
    1
}

// Translated from 00a2cb80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3697` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2cb80(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3697) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d2c, 0x0);
    e.mem.set_u32(0x011f_3c0c, 0x0);
    e.mem.set_u32(0x011f_3b7c, 0x0);
    e.mem.set_u8(0x011f_3697, 0x1);
    e.mem.set_u32(0x011f_3dbc, 0xa2caf0);
    e.mem.set_u32(0x011f_3a5c, 0xa2cb50);
    e.mem.set_u32(0x011f_39cc, 0xa2c5d0);
    e.mem.set_u32(0x011f_393c, 0xa2c660);
    e.mem.set_u32(0x011f_38ac, 0xa2a600);
    e.mem.set_u8(0x011f_375b, 0x1c);
    e.mem.set_u32(0x011f_3c9c, 0xa2c610);
    e.mem.set_u32(0x011f_3aec, 0xa2c550);
    e.mem.set_u32(0x011f_378c, 0xa2c7a0);
    e.mem.set_u32(0x011f_381c, 0xa2c890);
    1
}

// Translated from 00a2ce20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3698` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2ce20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3698) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3dac, 0x0);
    e.mem.set_u32(0x011f_3c8c, 0x0);
    e.mem.set_u32(0x011f_3bfc, 0x0);
    e.mem.set_u8(0x011f_3698, 0x1);
    e.mem.set_u32(0x011f_3e3c, 0xa2cdc0);
    e.mem.set_u32(0x011f_3adc, 0xa2cc60);
    e.mem.set_u32(0x011f_3a4c, 0xa2cc10);
    e.mem.set_u32(0x011f_39bc, 0xa2cc90);
    e.mem.set_u32(0x011f_392c, 0xa2a600);
    e.mem.set_u8(0x011f_377b, 0x8);
    e.mem.set_u32(0x011f_3d1c, 0xa2cc40);
    e.mem.set_u32(0x011f_3b6c, 0xa2cc30);
    e.mem.set_u32(0x011f_380c, 0xa29680);
    e.mem.set_u32(0x011f_389c, 0xa2ccd0);
    1
}

// Translated from 00a2d140 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3699` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d140(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3699) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d7c, 0x0);
    e.mem.set_u32(0x011f_3c5c, 0x0);
    e.mem.set_u32(0x011f_3bcc, 0x0);
    e.mem.set_u32(0x011f_37dc, 0x0);
    e.mem.set_u8(0x011f_3699, 0x1);
    e.mem.set_u32(0x011f_3e0c, 0xa2cf60);
    e.mem.set_u32(0x011f_3aac, 0xa2cfd0);
    e.mem.set_u32(0x011f_3a1c, 0xa2ab10);
    e.mem.set_u32(0x011f_398c, 0xa2ced0);
    e.mem.set_u32(0x011f_38fc, 0xa2a600);
    e.mem.set_u8(0x011f_376f, 0x14);
    e.mem.set_u32(0x011f_3cec, 0xa2cf20);
    e.mem.set_u32(0x011f_3b3c, 0xa2a1a0);
    e.mem.set_u32(0x011f_386c, 0xa2d000);
    1
}

// Translated from 00a2d360 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369a` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d360(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369a) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d34, 0x0);
    e.mem.set_u32(0x011f_3c14, 0x0);
    e.mem.set_u32(0x011f_3b84, 0x0);
    e.mem.set_u8(0x011f_369a, 0x1);
    e.mem.set_u32(0x011f_3dc4, 0xa2a5a0);
    e.mem.set_u32(0x011f_3a64, 0xa2d230);
    e.mem.set_u32(0x011f_39d4, 0xa2d1e0);
    e.mem.set_u32(0x011f_3944, 0xa2cc90);
    e.mem.set_u32(0x011f_38b4, 0xa2a600);
    e.mem.set_u8(0x011f_375d, 0x8);
    e.mem.set_u32(0x011f_3ca4, 0xa2d200);
    e.mem.set_u32(0x011f_3af4, 0xa2d1d0);
    e.mem.set_u32(0x011f_3794, 0xa29680);
    e.mem.set_u32(0x011f_3824, 0xa2d260);
    1
}

// Translated from 00a2d610 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369b` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d610(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369b) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_369b, 0x1);
    e.mem.set_u32(0x011f_3ddc, 0xa2a8e0);
    e.mem.set_u32(0x011f_3a7c, 0xa2d470);
    e.mem.set_u32(0x011f_39ec, 0xa2a7b0);
    e.mem.set_u32(0x011f_395c, 0xa27040);
    e.mem.set_u32(0x011f_38cc, 0xa2a600);
    e.mem.set_u8(0x011f_3763, 0x10);
    e.call(0x00a2_d5b0, &args![0x1u32, 0x5u32]);
    1
}

// Translated from 00a2d810 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369c` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d810(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369c) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d64, 0x0);
    e.mem.set_u32(0x011f_3c44, 0x0);
    e.mem.set_u32(0x011f_3bb4, 0x0);
    e.mem.set_u8(0x011f_369c, 0x1);
    e.mem.set_u32(0x011f_3df4, 0xa2abc0);
    e.mem.set_u32(0x011f_3a94, 0xa2d690);
    e.mem.set_u32(0x011f_3a04, 0xa2ab10);
    e.mem.set_u32(0x011f_3974, 0xa2ab40);
    e.mem.set_u32(0x011f_38e4, 0xa2a600);
    e.mem.set_u8(0x011f_3769, 0x14);
    e.mem.set_u32(0x011f_3cd4, 0xa2cf20);
    e.mem.set_u32(0x011f_3b24, 0xa2d680);
    e.mem.set_u32(0x011f_37c4, 0xa2d670);
    e.mem.set_u32(0x011f_3854, 0xa2d6c0);
    1
}

// Translated from 00a2d5b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2d5b0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2d3f0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2d420);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2a7e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2d4a0);
}

// Translated from 00a2e180 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference-counted start-up hook of the animation unit (no Xbox PDB name; `this`, in `ECX`,
/// is returned unchanged): increments the counter at `011f36a0` and, when it was zero, registers
/// the unit's two functions (`00a2d8a0` and `00a2de70`) with the runtime's registration
/// (`00aa2080`, `cdecl`).
pub fn fn_00a2e180(e: &mut Engine, this: u32) -> u32 {
    let count = e.mem.u32(0x011f_36a0);
    e.mem.set_u32(0x011f_36a0, count.wrapping_add(1));
    if count == 0 {
        e.call(0x00aa_2080, &args![0x00a2_d8a0u32, 0x00a2_de70u32]);
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// `00a64920` (`cdecl`, one word): registers a post-processing function.
const REGISTER_POST_PROCESS: u32 = 0x00a6_4920;
/// `00a421c0` (no stack words): the step `fn_00a2d8a0` ends with, by a jump.
const NEXT_STEP: u32 = 0x00a4_21c0;
/// Byte `011f369d`: set to 1 when `fn_00a2d8a0` has started.
const LOADERS_REGISTERED: u32 = 0x011f_369d;

/// The class registrations `fn_00a2d8a0` makes, in order: (class name string, creation
/// function).
const ANIMATION_LOADERS: [(u32, u32); 71] = [
    (0x0109_6060, 0x00a5_6b30),
    (0x0109_603c, 0x00a3_9700),
    (0x0109_6024, 0x00a5_65c0),
    (0x0109_6008, 0x00a5_5ff0),
    (0x0109_5fec, 0x00a4_0580),
    (0x0109_5fd0, 0x00a5_5a80),
    (0x0109_5fb0, 0x00a5_5470),
    (0x0109_5f90, 0x00a4_09d0),
    (0x0109_5f7c, 0x00a5_5300),
    (0x0109_5f70, 0x00a5_37f0),
    (0x0109_5f5c, 0x00a5_3500),
    (0x0109_5f40, 0x00a5_2fe0),
    (0x0109_5f2c, 0x00a5_2b60),
    (0x0109_5f1c, 0x00a5_1e10),
    (0x0109_5f00, 0x00a5_1c60),
    (0x0109_5ee0, 0x00a5_18f0),
    (0x0109_5ec0, 0x00a5_1620),
    (0x0109_5ea0, 0x00a5_13c0),
    (0x0109_5e7c, 0x00a5_0f00),
    (0x0109_5e60, 0x00a5_0ad0),
    (0x0109_5e44, 0x00a5_0840),
    (0x0109_5e24, 0x00a5_0180),
    (0x0109_5e18, 0x00a3_d150),
    (0x0109_5dfc, 0x00a4_fac0),
    (0x0109_5de8, 0x00a3_c130),
    (0x0109_5dd4, 0x00a2_f6c0),
    (0x0109_5dbc, 0x00a3_26c0),
    (0x0109_5da8, 0x00a4_f970),
    (0x0109_5d9c, 0x00a3_cd00),
    (0x0109_5d80, 0x00a4_11f0),
    (0x0109_5d6c, 0x00a3_c570),
    (0x0109_5d50, 0x00a4_f000),
    (0x0109_5d2c, 0x00a4_ecd0),
    (0x0109_5d14, 0x00a3_b550),
    (0x0109_5cfc, 0x00a3_6810),
    (0x0109_5cec, 0x00a4_e040),
    (0x0109_5cd8, 0x00a4_db60),
    (0x0109_5cc0, 0x00a4_d500),
    (0x0109_5ca8, 0x00a4_d0d0),
    (0x0109_5c94, 0x00a4_cde0),
    (0x0109_5c7c, 0x00a4_c5a0),
    (0x0109_5c60, 0x00a4_b790),
    (0x0109_5c54, 0x00a4_15a0),
    (0x0109_5c30, 0x00a3_0330),
    (0x0109_5c1c, 0x00a4_b270),
    (0x0109_5c08, 0x00a3_f4b0),
    (0x0109_5bf0, 0x00a4_9a30),
    (0x0109_5be4, 0x00a4_9020),
    (0x0109_5bc8, 0x00a4_8d70),
    (0x0109_5bb4, 0x00a4_8750),
    (0x0109_5ba8, 0x00a4_8110),
    (0x0109_5b9c, 0x00a4_7f20),
    (0x0109_5b84, 0x00a4_7380),
    (0x0109_5b74, 0x00a4_7150),
    (0x0109_5b60, 0x00a4_6b70),
    (0x0109_5b40, 0x00a4_6950),
    (0x0109_5b28, 0x00a3_6810),
    (0x0109_5b18, 0x00a4_e040),
    (0x0109_5b00, 0x00a4_03f0),
    (0x0109_5af0, 0x00a4_5f50),
    (0x0109_5ae4, 0x00a4_4ff0),
    (0x0109_5ad4, 0x00a4_4f10),
    (0x0109_5ac8, 0x00a4_4940),
    (0x0109_5ab8, 0x00a4_4380),
    (0x0109_5aa8, 0x00a4_3fb0),
    (0x0109_5a88, 0x00a4_3e40),
    (0x0109_5a6c, 0x00a4_3370),
    (0x0109_5a4c, 0x00a4_2f90),
    (0x0109_5a28, 0x00a3_59d0),
    (0x0109_5a10, 0x00a4_2ee0),
    (0x0109_5a04, 0x00a5_37f0),
];

// Translated from 00a2d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00a2e180` registers; no
/// arguments, NiAnimation): once only (byte `011f369d`), registers the 71 class names of
/// `ANIMATION_LOADERS` with their creation functions through `00a64900`, registers the
/// post-processing function `00a42c80` through `00a64920`, and ends with a jump to
/// `00a421c0`.
pub fn fn_00a2d8a0(e: &mut Engine) {
    if e.mem.u8(LOADERS_REGISTERED) != 0 {
        return;
    }
    e.mem.set_u8(LOADERS_REGISTERED, 1);
    for (name, creator) in ANIMATION_LOADERS {
        e.call(REGISTER_LOADER, &args![name, creator]);
    }
    e.call(REGISTER_POST_PROCESS, &args![0x00a4_2c80u32]);
    e.call(NEXT_STEP, &args![]);
}

/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The compiler's array constructor iterator (`stdcall`, (array, element size, count,
/// constructor)).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `00a6db50` (`thiscall`, four floats, `RET 0x10`): sets a quaternion, by its use here.
const QUATERNION_SET: u32 = 0x00a6_db50;
/// The quaternion helpers `fn_00a2beb0` calls: `NiQuaternion::UnitInverse` (`00a6dc00`,
/// `cdecl` (result, quaternion)), the product (`00a2bcf0`, `thiscall` (result, right),
/// the engine map's `NiQuaternion::operator*`), `00a6dc90` (`cdecl` (result, quaternion);
/// a logarithm by its use), `00a2bc90` (`cdecl` (result, float, quaternion); a scaling by
/// its use), `00a6db70` and `00a6dba0` (`thiscall` (result, right); a sum and a difference
/// by their use) and `00a88970` (`cdecl` (result, quaternion); an exponential by its use).
const QUATERNION_UNIT_INVERSE: u32 = 0x00a6_dc00;
const QUATERNION_PRODUCT: u32 = 0x00a2_bcf0;
const QUATERNION_LOG: u32 = 0x00a6_dc90;
const QUATERNION_SCALE: u32 = 0x00a2_bc90;
const QUATERNION_SUM: u32 = 0x00a6_db70;
const QUATERNION_DIFFERENCE: u32 = 0x00a6_dba0;
const QUATERNION_EXP: u32 = 0x00a8_8970;
/// The float 0.5 (`.rdata`) the tangents are halved by.
const HALF_FLOAT: u32 = 0x0101_6248;
/// The doubles 0.5 (`01011588`) and 2.0 (`01011590`) of the float key tangents.
const HALF_DOUBLE: u32 = 0x0101_1588;
const TWO_DOUBLE: u32 = 0x0101_1590;
/// The key loaders the array builders and loaders run first (`thiscall`, the stream):
/// `00a29640` for the rotation keys, `00a26eb0` for the float keys, `00a29580` for the
/// 8-byte keys.
const ROTATION_KEY_LOAD_BASE: u32 = 0x00a2_9640;
const FLOAT_KEY_LOAD_BASE: u32 = 0x00a2_6eb0;
const SMALL_KEY_LOAD: u32 = 0x00a2_9580;
/// `00a28ab0` (`cdecl`, (keys, count, stride)): run on the keys before their tangents.
const KEYS_PREPARE: u32 = 0x00a2_8ab0;
/// `00a351d0` (`thiscall`, five words, `RET 0x14`; unit `NiAnimation/nicontrollersequence.cpp`).
const SEQUENCE_MORPH: u32 = 0x00a3_51d0;
/// The calls of the unit's exit function `fn_00a2de70`: `00a63520` (`cdecl`, a class name;
/// the counterpart of `NiStream::RegisterLoader`, by its use), `00a63540` (`cdecl`, the
/// post-process function), `00a42230` and `00a56df0` (no words).
const UNREGISTER_LOADER: u32 = 0x00a6_3520;
const UNREGISTER_POST_PROCESS: u32 = 0x00a6_3540;
const EXIT_STEP_00A42230: u32 = 0x00a4_2230;
const EXIT_STEP_00A56DF0: u32 = 0x00a5_6df0;
/// The list `fn_00a2de40` empties (head `011f3e7c`, a word cleared with it at `011f3e80`) and
/// the destructor run on each node (`00a2dd70`, `thiscall`, the flags 3).
const LIST_HEAD: u32 = 0x011f_3e7c;
const LIST_COUNT: u32 = 0x011f_3e80;
const NODE_DESTRUCT: u32 = 0x00a2_dd70;

/// `count * size + 4`, the size of a counted array, all ones when either step overflows
/// (the compiler's `SETO`/`SETC`, `NEG`, `OR`).
fn counted_array_bytes(count: u32, size: u32) -> u32 {
    count
        .checked_mul(size)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX)
}

/// Allocates a counted array of `count` elements of `size` bytes from the memory manager;
/// answers the first element (0 when the allocation failed, which the callers do not check
/// before constructing).
fn allocate_counted(e: &mut Engine, count: u32, size: u32) -> u32 {
    let block = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, counted_array_bytes(count, size)],
        )
        .u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    block + 4
}

/// Reads 4 bytes from the stream's input into `destination`, the byte-count word at `count`
/// set to 4 first (the input's read function, then its position advanced by the answer).
fn read_word(e: &mut Engine, stream: u32, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(Ptr::<NiStream>::new(stream), NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00a2bdb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the element constructor of the 0x40-byte rotation keys,
/// `thiscall`, no stack words): sets the quaternion at +4 to (0, 0, 0, 0) (`00a6db50`) and
/// answers `this`.
pub fn fn_00a2bdb0(e: &mut Engine, this: u32) -> u32 {
    e.call(QUATERNION_SET, &args![this + 4, 0u32, 0u32, 0u32, 0u32]);
    this
}

// Translated from 00a2c420 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a function-table entry of `fn_00a2c4c0`; `cdecl`, (stream,
/// count)): a counted array of `count` 0x40-byte rotation keys from the memory manager,
/// built by `fn_00a2bdb0` (the array constructor iterator, when the allocation succeeded),
/// then each loaded from `stream` (`fn_00a2c300`). Answers the first key (0 when the
/// allocation failed).
pub fn fn_00a2c420(e: &mut Engine, stream: u32, count: u32) -> u32 {
    let keys = allocate_counted(e, count, 0x40);
    if keys != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![keys, 0x40u32, count, 0x00a2_bdb0u32],
        );
    }
    let mut key = keys;
    for _ in 0..count {
        fn_00a2c300(e, key, stream);
        key = key.wrapping_add(0x40);
    }
    keys
}

// Translated from 00a2c300 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the stream, `RET 4`): loads a rotation key: the
/// base part (`00a29640`), then three 4-byte values into +0x14, +0x18 and +0x1C (the
/// stream argument's stack word is the byte-count word).
pub fn fn_00a2c300(e: &mut Engine, this: u32, stream: u32) {
    e.call(ROTATION_KEY_LOAD_BASE, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0x14, 0x18, 0x1c] {
            read_word(e, stream, this + offset, count.addr());
        }
    });
}

// Translated from 00a2c9d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the stream, `RET 4`): loads a 0x1C-byte float key:
/// the base part (`00a26eb0`), then three 4-byte values into +0x8, +0xC and +0x10.
pub fn fn_00a2c9d0(e: &mut Engine, this: u32, stream: u32) {
    e.call(FLOAT_KEY_LOAD_BASE, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0x8, 0xc, 0x10] {
            read_word(e, stream, this + offset, count.addr());
        }
    });
}

// Translated from 00a2caf0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a function-table entry of `fn_00a2cb80`; `cdecl`, (stream,
/// count)): a counted array of `count` 0x1C-byte keys, each loaded by `fn_00a2c9d0`
/// (no constructor). Answers the first key (0 when the allocation failed).
pub fn fn_00a2caf0(e: &mut Engine, stream: u32, count: u32) -> u32 {
    let keys = allocate_counted(e, count, 0x1c);
    let mut key = keys;
    for _ in 0..count {
        fn_00a2c9d0(e, key, stream);
        key = key.wrapping_add(0x1c);
    }
    keys
}

// Translated from 00a2cdc0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a function-table entry of `fn_00a2ce20`; `cdecl`, (stream,
/// count)): a counted array of `count` 8-byte keys, each loaded by `00a29580`. Answers the
/// first key (0 when the allocation failed).
pub fn fn_00a2cdc0(e: &mut Engine, stream: u32, count: u32) -> u32 {
    let keys = allocate_counted(e, count, 0x8);
    let mut key = keys;
    for _ in 0..count {
        e.call(SMALL_KEY_LOAD, &args![key, stream]);
        key = key.wrapping_add(0x8);
    }
    keys
}

/// The shared choice of `fn_00a2cc40`, `fn_00a2cf20` and `fn_00a2d3f0`: the first key while
/// 1.0 is above `time` (`FLD1; FCOMP`, `TEST AH,0x41`), the second otherwise (also for a NaN).
fn choose_key(time: u32, first: u32, second: u32) -> u32 {
    if 1.0 > x87_m32(time) {
        first
    } else {
        second
    }
}

// Translated from 00a2cc40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a function-table entry of `fn_00a2ce20`; `cdecl`, (time,
/// first key, second key, result)): the byte at +4 of the chosen key (the first while
/// `time` is below 1.0) into `result`.
pub fn fn_00a2cc40(e: &mut Engine, time: u32, first: u32, second: u32, result: u32) {
    let key = choose_key(time, first, second);
    let value = e.mem.u8(key + 4);
    e.mem.set_u8(result, value);
}

// Translated from 00a2cf20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a function-table entry of `fn_00a2d140`; `cdecl`, (time,
/// first key, second key, result)): the four words at +4 of the chosen key into `result`.
pub fn fn_00a2cf20(e: &mut Engine, time: u32, first: u32, second: u32, result: u32) {
    let key = choose_key(time, first, second);
    for i in 0..4 {
        let word = e.mem.u32(key + 4 + i * 4);
        e.mem.set_u32(result + i * 4, word);
    }
}

// Translated from 00a2d3f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a function-table entry `fn_00a2d5b0` stores; `cdecl`, (time,
/// first key, second key, result)): the three words at +4 of the chosen key into `result`.
pub fn fn_00a2d3f0(e: &mut Engine, time: u32, first: u32, second: u32, result: u32) {
    let key = choose_key(time, first, second);
    for i in 0..3 {
        let word = e.mem.u32(key + 4 + i * 4);
        e.mem.set_u32(result + i * 4, word);
    }
}

// Translated from 00a2d670 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, three words): a jump thunk to `00a28ab0`.
pub fn fn_00a2d670(e: &mut Engine, keys: u32, count: u32, stride: u32) {
    e.call(KEYS_PREPARE, &args![keys, count, stride]);
}

// Translated from 00a2c6a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (key, previous, next, interval before, interval
/// after), the four floats as words): the two tangents of a 0x1C-byte tension / continuity /
/// bias float key (value +4, tension +8, continuity +0xC, bias +0x10), stored at +0x14 and
/// +0x18. Each x87 instruction is one helper call in the exe's order; the function's own
/// stack words (the arguments and the key pointer's slot) are reused for the intermediate
/// floats exactly as the exe stores and reloads them.
pub fn fn_00a2c6a0(
    e: &mut Engine,
    key: u32,
    previous: u32,
    next: u32,
    interval_before: u32,
    interval_after: u32,
) {
    let m32 = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let value = load(e, key + 4);
    // L0 = value - previous; L8 = next - value.
    let before = x87_store32(x87_sub(value, x87_m32(previous)));
    let after = x87_store32(x87_sub(x87_load32(next), m32(e, key + 4)));
    // The bias: 1 + b (in the first argument word) and 1 - b.
    let bias = load(e, key + 0x10);
    let bias_plus = x87_store32(x87_add(bias, 1.0));
    let bias = load(e, key + 0x10);
    let bias_minus = x87_store32(x87_sub(1.0, bias));
    // (1 - tension) * 0.5 (in the second argument word).
    let tension = load(e, key + 0x8);
    let half_tension = x87_store32(x87_mul(x87_sub(1.0, tension), e.mem.f64(HALF_DOUBLE)));
    // 1 + continuity (in the key pointer's slot).
    let continuity = load(e, key + 0xc);
    let continuity_plus = x87_store32(x87_add(continuity, 1.0));
    // (1 + c) * h into the second word; the stack keeps h.
    let h = x87_load32(half_tension);
    let product = x87_store32(x87_mul(x87_load32(continuity_plus), h));
    // (1 - c) into the slot, then h * (1 - c).
    let continuity = load(e, key + 0xc);
    let continuity_minus = x87_store32(x87_sub(1.0, continuity));
    let r = x87_store32(x87_mul(h, x87_m32(continuity_minus)));
    // 2 / (before + after) into the third word; the stack keeps after and before.
    let w3 = x87_load32(interval_before);
    let w4 = x87_load32(interval_after);
    let sum = x87_add(w3, w4);
    let scale = x87_store32(x87_div(e.mem.f64(TWO_DOUBLE), sum));
    // p * q, and X = p * q * after + r * (1 + b) * before (into the second word).
    let p = x87_load32(product);
    let q = x87_load32(bias_minus);
    let pq = x87_mul(p, q);
    let d1 = x87_load32(after);
    let pq_d1 = x87_mul(pq, d1);
    let r_term = x87_mul(x87_mul(x87_load32(r), x87_m32(bias_plus)), x87_m32(before));
    let x = x87_add(pq_d1, r_term);
    let x_word = x87_store32(x);
    // The first tangent: interval before * scale * X.
    let x = x87_load32(x_word);
    let m = x87_load32(scale);
    let first = x87_mul(x87_mul(w3, m), x);
    e.mem.set_u32(key + 0x14, x87_store32(first));
    // Z = p * (1 + b) * before + q * r * after, then interval after * scale * Z.
    let qr = x87_mul(q, x87_m32(r));
    let d1_qr = x87_mul(d1, qr);
    let pa = x87_mul(p, x87_load32(bias_plus));
    let pa_d0 = x87_mul(pa, x87_load32(before));
    let z = x87_store32(x87_add(pa_d0, d1_qr));
    let w4_m = x87_mul(w4, m);
    let second = x87_mul(x87_load32(z), w4_m);
    e.mem.set_u32(key + 0x18, x87_store32(second));
}

// Translated from 00a2c7a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function-table entry of `fn_00a2cb80`; `cdecl`, (keys,
/// count)): with at least two 0x1C-byte keys, the tangents (`fn_00a2c6a0`) of each: the
/// first with (2 * v0 - v1, v1, 1.0, 1.0), the inner ones with their neighbours' values and
/// the time intervals to them, the last with (v[n-2], 2 * v[n-1] - v[n-2], 1.0, 1.0). The
/// mirrored values are rounded to floats (`FSTP` into a stack word) before they are passed.
pub fn fn_00a2c7a0(e: &mut Engine, keys: u32, count: u32) {
    if count < 2 {
        return;
    }
    let value = |e: &Engine, key: u32| e.mem.u32(key + 4);
    let one = 1.0f32.to_bits();
    let mirrored = x87_store32(x87_sub(
        x87_add(x87_load32(value(e, keys)), x87_load32(value(e, keys))),
        x87_m32(value(e, keys + 0x1c)),
    ));
    let next = x87_quiet_bits(value(e, keys + 0x1c));
    fn_00a2c6a0(e, keys, x87_quiet_bits(mirrored), next, one, one);
    let last_index = count - 1;
    let mut previous = keys;
    for _ in 1..last_index {
        let key = previous + 0x1c;
        let after = x87_store32(x87_sub(
            x87_load32(e.mem.u32(previous + 0x38)),
            x87_m32(e.mem.u32(key)),
        ));
        let before = x87_store32(x87_sub(
            x87_load32(e.mem.u32(key)),
            x87_m32(e.mem.u32(previous)),
        ));
        let next = x87_quiet_bits(e.mem.u32(previous + 0x3c));
        let prior = x87_quiet_bits(e.mem.u32(previous + 4));
        fn_00a2c6a0(
            e,
            key,
            prior,
            next,
            x87_quiet_bits(before),
            x87_quiet_bits(after),
        );
        previous = key;
    }
    let last = keys.wrapping_add(last_index.wrapping_mul(0x1c));
    let before_last = keys.wrapping_add((count - 2).wrapping_mul(0x1c)) + 4;
    let mirrored = x87_store32(x87_sub(
        x87_add(x87_load32(value(e, last)), x87_load32(value(e, last))),
        x87_m32(e.mem.u32(before_last)),
    ));
    let prior = x87_quiet_bits(e.mem.u32(before_last));
    fn_00a2c6a0(e, last, prior, x87_quiet_bits(mirrored), one, one);
}

// Translated from 00a2beb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a 0x40-byte rotation key, (previous key, next
/// key), `RET 8`): the key's two derived quaternions from its neighbours: the logarithms of
/// `inverse(previous) * q` and `inverse(q) * next`, weighted by the time ratios and the
/// tension (+0x14), continuity (+0x18) and bias (+0x1C) factors, combined (sum, difference,
/// halved), exponentiated and multiplied onto `q`; the results go to +0x20 and +0x30. The
/// stack frame's buffers are those of the exe (results passed on by the address each helper
/// answers), and the factors are stored to and reloaded from their float slots in the
/// exe's order.
pub fn fn_00a2beb0(e: &mut Engine, this: u32, previous: u32, next: u32) {
    e.with_stack(0xb0, |e, frame| {
        let s = frame.addr();
        let quaternion = this + 4;
        let m32 = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
        let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
        let store = |e: &mut Engine, at: u32, value: f64| e.mem.set_u32(at, x87_store32(value));

        // The two logarithms (into s+0x5c and s+0x4c).
        let inverse = e
            .call(QUATERNION_UNIT_INVERSE, &args![s + 0x3c, previous + 4])
            .u32();
        let product = e
            .call(QUATERNION_PRODUCT, &args![inverse, s + 0x2c, quaternion])
            .u32();
        e.call(QUATERNION_LOG, &args![s + 0x5c, product]);
        let inverse = e
            .call(QUATERNION_UNIT_INVERSE, &args![s + 0x2c, quaternion])
            .u32();
        let product = e
            .call(QUATERNION_PRODUCT, &args![inverse, s + 0x3c, next + 4])
            .u32();
        e.call(QUATERNION_LOG, &args![s + 0x4c, product]);

        // The factors.
        let span = x87_sub(load(e, next), m32(e, previous));
        store(e, s + 0x28, x87_div(1.0, span));
        let tension = load(e, this + 0x14);
        store(e, s + 0x24, x87_sub(1.0, tension));
        let continuity = load(e, this + 0x18);
        store(e, s + 0x1c, x87_sub(1.0, continuity));
        let continuity = load(e, this + 0x18);
        store(e, s + 0x20, x87_add(continuity, 1.0));
        let bias = load(e, this + 0x1c);
        store(e, s + 0x14, x87_sub(1.0, bias));
        store(e, s + 0x18, x87_add(1.0, m32(e, this + 0x1c)));

        // The first derived quaternion.
        let before = x87_sub(load(e, this), m32(e, previous));
        store(e, s + 0x10, x87_mul(before, m32(e, s + 0x28)));
        let g = x87_mul(load(e, s + 0x10), m32(e, s + 0x24));
        e.mem.set_f64(s + 0x2c, g);
        let weight = x87_mul(x87_mul(g, m32(e, s + 0x20)), m32(e, s + 0x18));
        store(e, s + 0x10, weight);
        let weight = x87_quiet_bits(e.mem.u32(s + 0x10));
        let first = e
            .call(QUATERNION_SCALE, &args![s + 0x3c, weight, s + 0x5c])
            .u32();
        let other = x87_mul(load(e, s + 0x1c), e.mem.f64(s + 0x2c));
        store(e, s + 0x10, x87_mul(other, m32(e, s + 0x14)));
        let weight = x87_quiet_bits(e.mem.u32(s + 0x10));
        let second = e
            .call(QUATERNION_SCALE, &args![s + 0x6c, weight, s + 0x4c])
            .u32();
        e.call(QUATERNION_SUM, &args![second, s + 0x9c, first]);
        let difference = e
            .call(QUATERNION_DIFFERENCE, &args![s + 0x9c, s + 0x6c, s + 0x4c])
            .u32();
        let half = x87_quiet_bits(e.mem.u32(HALF_FLOAT));
        let halved = e
            .call(QUATERNION_SCALE, &args![s + 0x3c, half, difference])
            .u32();
        let exponential = e.call(QUATERNION_EXP, &args![s + 0x2c, halved]).u32();
        let result = e
            .call(
                QUATERNION_PRODUCT,
                &args![quaternion, s + 0x7c, exponential],
            )
            .u32();
        for i in 0..4 {
            let word = e.mem.u32(result + i * 4);
            e.mem.set_u32(this + 0x20 + i * 4, word);
        }

        // The second derived quaternion.
        let after = x87_sub(load(e, next), m32(e, this));
        store(e, s + 0x10, x87_mul(after, m32(e, s + 0x28)));
        let g = x87_mul(load(e, s + 0x10), m32(e, s + 0x24));
        e.mem.set_f64(s + 0x2c, g);
        let weight = x87_mul(x87_mul(g, m32(e, s + 0x1c)), m32(e, s + 0x18));
        store(e, s + 0x18, weight);
        let weight = x87_quiet_bits(e.mem.u32(s + 0x18));
        let first = e
            .call(QUATERNION_SCALE, &args![s + 0x7c, weight, s + 0x5c])
            .u32();
        let other = x87_mul(load(e, s + 0x20), e.mem.f64(s + 0x2c));
        store(e, s + 0x14, x87_mul(other, m32(e, s + 0x14)));
        let weight = x87_quiet_bits(e.mem.u32(s + 0x14));
        let second = e
            .call(QUATERNION_SCALE, &args![s + 0x6c, weight, s + 0x4c])
            .u32();
        e.call(QUATERNION_SUM, &args![second, s + 0x8c, first]);
        let difference = e
            .call(QUATERNION_DIFFERENCE, &args![s + 0x5c, s + 0x7c, s + 0x8c])
            .u32();
        let half = x87_quiet_bits(e.mem.u32(HALF_FLOAT));
        let halved = e
            .call(QUATERNION_SCALE, &args![s + 0x6c, half, difference])
            .u32();
        let exponential = e.call(QUATERNION_EXP, &args![s + 0x3c, halved]).u32();
        let result = e
            .call(
                QUATERNION_PRODUCT,
                &args![quaternion, s + 0x2c, exponential],
            )
            .u32();
        for i in 0..4 {
            let word = e.mem.u32(result + i * 4);
            e.mem.set_u32(this + 0x30 + i * 4, word);
        }
    });
}

// Translated from 00a2c130 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function-table entry of `fn_00a2c4c0`; `cdecl`, (keys,
/// count, stride)): with at least two 0x40-byte rotation keys, `00a28ab0(keys, count,
/// stride)`, then the derived quaternions of each key (`fn_00a2beb0`): the first with
/// itself and the second, each inner one with its neighbours, the last with the one before
/// and itself.
pub fn fn_00a2c130(e: &mut Engine, keys: u32, count: u32, stride: u32) {
    if count < 2 {
        return;
    }
    e.call(KEYS_PREPARE, &args![keys, count, stride]);
    fn_00a2beb0(e, keys, keys, keys + 0x40);
    let last_index = count - 1;
    if last_index > 1 {
        let mut previous = keys;
        for _ in 0..last_index - 1 {
            let key = previous + 0x40;
            fn_00a2beb0(e, key, previous, previous + 0x80);
            previous = key;
        }
    }
    let last = (last_index << 6).wrapping_add(keys);
    let before_last = ((count - 2) << 6).wrapping_add(keys);
    fn_00a2beb0(e, last, before_last, last);
}

// Translated from 00a2e1b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, six words, `RET 0x18`; `this` is not read; called
/// by `StartGroup_ov2` with an `NiControllerManager` and (current sequence, sequence,
/// blend time, priority, two weights)): `00a351d0` on the first word with the other five
/// (the three floats copied through the x87 stack).
pub fn fn_00a2e1b0(
    e: &mut Engine,
    _this: u32,
    source: u32,
    destination: u32,
    duration: u32,
    priority: u32,
    source_weight: u32,
    destination_weight: u32,
) -> u32 {
    e.call(
        SEQUENCE_MORPH,
        &args![
            source,
            destination,
            x87_quiet_bits(duration),
            priority,
            x87_quiet_bits(source_weight),
            x87_quiet_bits(destination_weight)
        ],
    )
    .u32()
}

// Translated from 00a2de40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): destroys every node of the list at `011f3e7c`
/// (`00a2dd70(3)` on each, the next taken first) and clears the head and `011f3e80`.
pub fn fn_00a2de40(e: &mut Engine) {
    let mut node = e.mem.u32(LIST_HEAD);
    while node != 0 {
        let next = e.mem.u32(node);
        e.call(NODE_DESTRUCT, &args![node, 3u32]);
        node = next;
    }
    e.mem.set_u32(LIST_COUNT, 0);
    e.mem.set_u32(LIST_HEAD, 0);
}

// Translated from 00a2de70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the exit function `fn_00a2e180` registers; no arguments):
/// undoes `fn_00a2d8a0` once (byte `011f369d` set): clears the byte, hands each of the 71
/// class names of `ANIMATION_LOADERS` (the same order) to `00a63520`, the post-process
/// function `00a42c80` to `00a63540`, runs `fn_00a2de40` and `00a42230`, and ends with a
/// jump to `00a56df0`.
pub fn fn_00a2de70(e: &mut Engine) {
    if e.mem.u8(LOADERS_REGISTERED) == 0 {
        return;
    }
    e.mem.set_u8(LOADERS_REGISTERED, 0);
    for (name, _) in ANIMATION_LOADERS {
        e.call(UNREGISTER_LOADER, &args![name]);
    }
    e.call(UNREGISTER_POST_PROCESS, &args![0x00a4_2c80u32]);
    fn_00a2de40(e);
    e.call(EXIT_STEP_00A42230, &[]);
    e.call(EXIT_STEP_00A56DF0, &[]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_bdb0, fn_00a2bdb0(u32) -> u32),
        entry!(0x00a2_c420, fn_00a2c420(u32, u32) -> u32),
        entry!(0x00a2_c300, fn_00a2c300(u32, u32)),
        entry!(0x00a2_c9d0, fn_00a2c9d0(u32, u32)),
        entry!(0x00a2_caf0, fn_00a2caf0(u32, u32) -> u32),
        entry!(0x00a2_cdc0, fn_00a2cdc0(u32, u32) -> u32),
        entry!(0x00a2_cc40, fn_00a2cc40(u32, u32, u32, u32)),
        entry!(0x00a2_cf20, fn_00a2cf20(u32, u32, u32, u32)),
        entry!(0x00a2_d3f0, fn_00a2d3f0(u32, u32, u32, u32)),
        entry!(0x00a2_d670, fn_00a2d670(u32, u32, u32)),
        entry!(0x00a2_c6a0, fn_00a2c6a0(u32, u32, u32, u32, u32)),
        entry!(0x00a2_c7a0, fn_00a2c7a0(u32, u32)),
        entry!(0x00a2_beb0, fn_00a2beb0(u32, u32, u32)),
        entry!(0x00a2_c130, fn_00a2c130(u32, u32, u32)),
        entry!(
            0x00a2_e1b0,
            fn_00a2e1b0(u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00a2_de40, fn_00a2de40()),
        entry!(0x00a2_de70, fn_00a2de70()),
        entry!(0x00a2c4c0, fn_00a2c4c0() -> u32),
        entry!(0x00a2cb80, fn_00a2cb80() -> u32),
        entry!(0x00a2ce20, fn_00a2ce20() -> u32),
        entry!(0x00a2d140, fn_00a2d140() -> u32),
        entry!(0x00a2d360, fn_00a2d360() -> u32),
        entry!(0x00a2d610, fn_00a2d610() -> u32),
        entry!(0x00a2d810, fn_00a2d810() -> u32),
        entry!(0x00a2d5b0, fn_00a2d5b0(u32, u32)),
        entry!(0x00a2e180, fn_00a2e180(u32) -> u32),
        entry!(0x00a2_d8a0, fn_00a2d8a0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_call_registers_the_unit_and_every_call_returns_this() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.register(0x00aa_2080, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_e180, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.mem.u32(0x011f_36a0), 1);
        assert_eq!(e.call(0x00a2_e180, &args![0x5678u32]).u32(), 0x5678);
        assert_eq!(e.mem.u32(0x011f_36a0), 2);
        let log = e.call_log.take().unwrap();
        let registrations: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00aa_2080)
            .collect();
        assert_eq!(registrations.len(), 1);
        assert_eq!(registrations[0].1, vec![0x00a2_d8a0, 0x00a2_de70]);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn registers_every_loader_once_then_the_post_process_function() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        for address in [REGISTER_LOADER, REGISTER_POST_PROCESS, NEXT_STEP] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00a2_d8a0, &args![]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(log.len(), 73);
        assert_eq!(log[0], (REGISTER_LOADER, vec![0x0109_6060, 0x00a5_6b30]));
        assert_eq!(log[70].0, REGISTER_LOADER);
        assert_eq!(log[71], (REGISTER_POST_PROCESS, vec![0x00a4_2c80]));
        assert_eq!(log[72], (NEXT_STEP, vec![]));
        assert_eq!(e.mem.u8(LOADERS_REGISTERED), 1);
        e.call_log = Some(vec![]);
        e.call(0x00a2_d8a0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A stream whose input reads the words 1, 2, 3, ... (one per call) and answers 4 bytes.
    fn stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0099_1000);
        let counter = e.mem.alloc(4);
        e.register_double(0x0099_1000, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            let next = e.mem.u32(counter) + 1;
            e.mem.set_u32(counter, next);
            e.mem.set_u32(a[1], next);
            4u32.into_ret()
        });
        stream
    }

    fn input_position(e: &Engine, stream: u32) -> u32 {
        e.mem.u32(e.mem.u32(stream + 0x24c) + 4)
    }

    #[test]
    fn rotation_key_constructor_and_loaders() {
        let mut e = Engine::new();
        e.register(QUATERNION_SET, |_, _| Ret::default());
        e.register(ROTATION_KEY_LOAD_BASE, |_, _| Ret::default());
        e.register(FLOAT_KEY_LOAD_BASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_bdb0, &[0x4000]).u32(), 0x4000);
        assert_eq!(calls_to(&e, QUATERNION_SET), vec![vec![0x4004, 0, 0, 0, 0]]);

        let stream = stream(&mut e);
        let key = e.mem.alloc(0x40);
        e.call(0x00a2_c300, &[key, stream]);
        assert_eq!(
            calls_to(&e, ROTATION_KEY_LOAD_BASE),
            vec![vec![key, stream]]
        );
        assert_eq!(
            [
                e.mem.u32(key + 0x14),
                e.mem.u32(key + 0x18),
                e.mem.u32(key + 0x1c)
            ],
            [1, 2, 3]
        );
        assert_eq!(input_position(&e, stream), 12);
        let float_key = e.mem.alloc(0x1c);
        e.call(0x00a2_c9d0, &[float_key, stream]);
        assert_eq!(
            [
                e.mem.u32(float_key + 8),
                e.mem.u32(float_key + 0xc),
                e.mem.u32(float_key + 0x10)
            ],
            [4, 5, 6]
        );
        assert_eq!(
            calls_to(&e, FLOAT_KEY_LOAD_BASE),
            vec![vec![float_key, stream]]
        );
    }

    #[test]
    fn key_arrays_are_counted_and_loaded() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(ROTATION_KEY_LOAD_BASE, |_, _| Ret::default());
        e.register(FLOAT_KEY_LOAD_BASE, |_, _| Ret::default());
        e.register(SMALL_KEY_LOAD, |_, _| Ret::default());
        let stream = stream(&mut e);
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_c420, &[stream, 2]).u32();
        assert_eq!(e.mem.u32(keys - 4), 2);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 2 * 0x40 + 4]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![keys, 0x40, 2, 0x00a2_bdb0]]
        );
        assert_eq!(
            calls_to(&e, ROTATION_KEY_LOAD_BASE),
            vec![vec![keys, stream], vec![keys + 0x40, stream]]
        );
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_caf0, &[stream, 3]).u32();
        assert_eq!(e.mem.u32(keys - 4), 3);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 3 * 0x1c + 4]]
        );
        assert_eq!(calls_to(&e, FLOAT_KEY_LOAD_BASE).len(), 3);
        assert_eq!(
            calls_to(&e, FLOAT_KEY_LOAD_BASE)[2],
            vec![keys + 0x38, stream]
        );
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_cdc0, &[stream, 2]).u32();
        assert_eq!(
            calls_to(&e, SMALL_KEY_LOAD),
            vec![vec![keys, stream], vec![keys + 8, stream]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 2 * 8 + 4]]
        );
        // A failed allocation: no constructor; no keys to load for a count of 0.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_c420, &[stream, 0]).u32(), 0);
        assert!(calls_to(&e, VECTOR_CONSTRUCT).is_empty());
        assert_eq!(counted_array_bytes(0x0400_0000, 0x40), u32::MAX);
    }

    #[test]
    fn the_step_keys_choose_by_time() {
        let mut e = Engine::new();
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        for i in 0..4 {
            e.mem.set_u32(first + 4 + i * 4, 0x10 + i);
            e.mem.set_u32(second + 4 + i * 4, 0x20 + i);
        }
        let result = e.mem.alloc(0x10);
        let time = |t: f32| t.to_bits();
        e.call(0x00a2_cf20, &[time(0.5), first, second, result]);
        assert_eq!(e.mem.u32(result + 0xc), 0x13);
        e.call(0x00a2_cf20, &[time(1.0), first, second, result]);
        assert_eq!(e.mem.u32(result), 0x20);
        e.call(0x00a2_d3f0, &[time(0.25), first, second, result]);
        assert_eq!(e.mem.u32(result + 8), 0x12);
        e.call(0x00a2_d3f0, &[0x7fc0_0000, first, second, result]);
        assert_eq!(e.mem.u32(result), 0x20);
        e.mem.set_u32(result, 0xffff_ffff);
        e.call(0x00a2_cc40, &[time(0.0), first, second, result]);
        assert_eq!(e.mem.u32(result), 0xffff_ff10);
        e.call(0x00a2_cc40, &[time(2.0), first, second, result]);
        assert_eq!(e.mem.u32(result), 0xffff_ff20);
    }

    #[test]
    fn fn_00a2d670_forwards_its_words() {
        let mut e = Engine::new();
        e.register(KEYS_PREPARE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_d670, &[1, 2, 3]);
        assert_eq!(calls_to(&e, KEYS_PREPARE), vec![vec![1, 2, 3]]);
    }

    fn doubles_mapped(e: &mut Engine) {
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(HALF_FLOAT, 0.5);
    }

    /// A float key: time, value, tension, continuity, bias.
    fn float_key(e: &mut Engine, at: u32, fields: [f32; 5]) {
        for (i, field) in (0u32..).zip(fields) {
            e.mem.set_f32(at + i * 4, field);
        }
    }

    #[test]
    fn float_key_tangents() {
        let mut e = Engine::new();
        doubles_mapped(&mut e);
        let key = e.mem.alloc(0x1c);
        // Catmull-Rom (no tension, continuity or bias): (next - previous) / 2 both ways.
        float_key(&mut e, key, [0.0, 1.0, 0.0, 0.0, 0.0]);
        let bits = |v: f32| v.to_bits();
        e.call(
            0x00a2_c6a0,
            &[key, bits(0.0), bits(3.0), bits(1.0), bits(1.0)],
        );
        assert_eq!(e.mem.f32(key + 0x14), 1.5);
        assert_eq!(e.mem.f32(key + 0x18), 1.5);
        // Tension 0.5 and uneven intervals.
        float_key(&mut e, key, [0.0, 1.0, 0.5, 0.0, 0.0]);
        e.call(
            0x00a2_c6a0,
            &[key, bits(0.0), bits(3.0), bits(1.0), bits(3.0)],
        );
        assert_eq!(e.mem.f32(key + 0x14), 0.375);
        assert_eq!(e.mem.f32(key + 0x18), 1.125);

        // Three keys at times 0, 1, 2 with values 0, 1, 4.
        let keys = e.mem.alloc(0x1c * 3);
        float_key(&mut e, keys, [0.0, 0.0, 0.0, 0.0, 0.0]);
        float_key(&mut e, keys + 0x1c, [1.0, 1.0, 0.0, 0.0, 0.0]);
        float_key(&mut e, keys + 0x38, [2.0, 4.0, 0.0, 0.0, 0.0]);
        e.call(0x00a2_c7a0, &[keys, 3]);
        let tangents: Vec<f32> = (0..3)
            .flat_map(|i| [keys + i * 0x1c + 0x14, keys + i * 0x1c + 0x18])
            .map(|at| e.mem.f32(at))
            .collect();
        assert_eq!(tangents, vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
        // One key: nothing.
        e.mem.set_f32(keys + 0x14, 9.0);
        e.call(0x00a2_c7a0, &[keys, 1]);
        assert_eq!(e.mem.f32(keys + 0x14), 9.0);
    }

    /// Doubles for the quaternion helpers: each answers its result buffer; the product
    /// writes the words 0x100.. into its result.
    fn quaternion_doubles(e: &mut Engine) {
        for address in [
            QUATERNION_UNIT_INVERSE,
            QUATERNION_LOG,
            QUATERNION_SCALE,
            QUATERNION_EXP,
        ] {
            e.register(address, |_, a| a[0].into_ret());
        }
        e.register(QUATERNION_SUM, |_, a| a[1].into_ret());
        e.register(QUATERNION_DIFFERENCE, |_, a| a[1].into_ret());
        e.register(QUATERNION_PRODUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[1] + i * 4, 0x100 + i);
            }
            a[1].into_ret()
        });
    }

    /// A rotation key: time, the quaternion, tension, continuity, bias.
    fn rotation_key(e: &mut Engine, at: u32, time: f32, tcb: [f32; 3]) {
        e.mem.set_f32(at, time);
        for (i, value) in (0u32..).zip(tcb) {
            e.mem.set_f32(at + 0x14 + i * 4, value);
        }
    }

    #[test]
    fn rotation_key_tangents() {
        let mut e = Engine::new();
        doubles_mapped(&mut e);
        quaternion_doubles(&mut e);
        let keys = e.mem.alloc(0xc0);
        rotation_key(&mut e, keys, 0.0, [0.0; 3]);
        rotation_key(&mut e, keys + 0x40, 1.0, [0.5, 0.5, 0.0]);
        rotation_key(&mut e, keys + 0x80, 2.0, [0.0; 3]);
        let (previous, this, next) = (keys, keys + 0x40, keys + 0x80);
        e.call_log = Some(vec![]);
        e.call(0x00a2_beb0, &[this, previous, next]);
        let inverses = calls_to(&e, QUATERNION_UNIT_INVERSE);
        let s = inverses[0][0] - 0x3c;
        assert_eq!(
            inverses,
            vec![vec![s + 0x3c, previous + 4], vec![s + 0x2c, this + 4]]
        );
        let half = 0.5f32.to_bits();
        let w = |v: f32| v.to_bits();
        assert_eq!(
            calls_to(&e, QUATERNION_SCALE),
            vec![
                vec![s + 0x3c, w(0.375), s + 0x5c],
                vec![s + 0x6c, w(0.125), s + 0x4c],
                vec![s + 0x3c, half, s + 0x6c],
                vec![s + 0x7c, w(0.125), s + 0x5c],
                vec![s + 0x6c, w(0.375), s + 0x4c],
                vec![s + 0x6c, half, s + 0x7c],
            ]
        );
        assert_eq!(
            calls_to(&e, QUATERNION_SUM),
            vec![
                vec![s + 0x6c, s + 0x9c, s + 0x3c],
                vec![s + 0x6c, s + 0x8c, s + 0x7c]
            ]
        );
        assert_eq!(
            calls_to(&e, QUATERNION_DIFFERENCE),
            vec![
                vec![s + 0x9c, s + 0x6c, s + 0x4c],
                vec![s + 0x5c, s + 0x7c, s + 0x8c]
            ]
        );
        assert_eq!(
            calls_to(&e, QUATERNION_EXP),
            vec![vec![s + 0x2c, s + 0x3c], vec![s + 0x3c, s + 0x6c]]
        );
        let products = calls_to(&e, QUATERNION_PRODUCT);
        assert_eq!(products[1], vec![s + 0x2c, s + 0x3c, next + 4]);
        assert_eq!(products[2], vec![this + 4, s + 0x7c, s + 0x2c]);
        assert_eq!(products[3], vec![this + 4, s + 0x2c, s + 0x3c]);
        assert_eq!(e.mem.u32(this + 0x20), 0x100);
        assert_eq!(e.mem.u32(this + 0x3c), 0x103);
    }

    #[test]
    fn rotation_keys_get_their_tangents_from_their_neighbours() {
        let mut e = Engine::new();
        doubles_mapped(&mut e);
        quaternion_doubles(&mut e);
        e.register(KEYS_PREPARE, |_, _| Ret::default());
        let keys = e.mem.alloc(0xc0);
        for i in 0..3 {
            rotation_key(&mut e, keys + i * 0x40, i as f32, [0.0; 3]);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a2_c130, &[keys, 3, 0x40]);
        assert_eq!(calls_to(&e, KEYS_PREPARE), vec![vec![keys, 3, 0x40]]);
        let inverted: Vec<u32> = calls_to(&e, QUATERNION_UNIT_INVERSE)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(
            inverted,
            vec![
                keys + 4,
                keys + 4,
                keys + 4,
                keys + 0x44,
                keys + 0x44,
                keys + 0x84
            ]
        );
        // One key: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a2_c130, &[keys, 1, 0x40]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00a2e1b0_forwards_to_the_first_sequence() {
        let mut e = Engine::new();
        e.register(SEQUENCE_MORPH, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a2_e1b0, &[0x9, 1, 2, 0x7f80_0001, 4, 0x3f80_0000, 6])
                .u32(),
            1
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_MORPH),
            vec![vec![1, 2, 0x7fc0_0001, 4, 0x3f80_0000, 6]]
        );
    }

    #[test]
    fn fn_00a2de40_destroys_the_list() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.register(NODE_DESTRUCT, |_, _| Ret::default());
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, second);
        e.mem.set_u32(LIST_HEAD, first);
        e.mem.set_u32(LIST_COUNT, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a2_de40, &[]);
        assert_eq!(
            calls_to(&e, NODE_DESTRUCT),
            vec![vec![first, 3], vec![second, 3]]
        );
        assert_eq!(e.mem.u32(LIST_HEAD), 0);
        assert_eq!(e.mem.u32(LIST_COUNT), 0);
    }

    #[test]
    fn fn_00a2de70_unregisters_once() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        for address in [
            UNREGISTER_LOADER,
            UNREGISTER_POST_PROCESS,
            EXIT_STEP_00A42230,
            EXIT_STEP_00A56DF0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00a2_de70, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u8(LOADERS_REGISTERED, 1);
        e.call(0x00a2_de70, &[]);
        let names = calls_to(&e, UNREGISTER_LOADER);
        assert_eq!(names.len(), 71);
        assert_eq!(names[0], vec![0x0109_6060]);
        assert_eq!(names[70], vec![0x0109_5a04]);
        assert_eq!(
            calls_to(&e, UNREGISTER_POST_PROCESS),
            vec![vec![0x00a4_2c80]]
        );
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        // fn_00a2de40 is a direct call (not in the log): the list is empty here.
        assert_eq!(
            &order[order.len() - 3..],
            &[
                UNREGISTER_POST_PROCESS,
                EXIT_STEP_00A42230,
                EXIT_STEP_00A56DF0
            ]
        );
        assert_eq!(e.mem.u8(LOADERS_REGISTERED), 0);
    }
}
