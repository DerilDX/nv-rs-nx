//! `NiAnimation/run_00a298f0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a299f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367f` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a299f0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367f) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d68, 0x0);
    e.mem.set_u32(0x011f_3c48, 0x0);
    e.mem.set_u32(0x011f_3bb8, 0x0);
    e.mem.set_u32(0x011f_37c8, 0x0);
    e.mem.set_u32(0x011f_3858, 0x0);
    e.mem.set_u8(0x011f_367f, 0x1);
    e.mem.set_u32(0x011f_3df8, 0xe7cd40);
    e.mem.set_u32(0x011f_3a98, 0xa29680);
    e.mem.set_u32(0x011f_3a08, 0xa29680);
    e.mem.set_u32(0x011f_3978, 0xe7cd40);
    e.mem.set_u32(0x011f_38e8, 0xa29680);
    e.mem.set_u8(0x011f_376a, 0x14);
    e.mem.set_u32(0x011f_3cd8, 0xa29690);
    e.mem.set_u32(0x011f_3b28, 0xa299c0);
    1
}

// Translated from 00a2a110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3690` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2a110(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3690) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d60, 0x0);
    e.mem.set_u32(0x011f_3c40, 0x0);
    e.mem.set_u32(0x011f_3bb0, 0x0);
    e.mem.set_u8(0x011f_3690, 0x1);
    e.mem.set_u32(0x011f_3df0, 0xa2a070);
    e.mem.set_u32(0x011f_3a90, 0xa2a0e0);
    e.mem.set_u32(0x011f_3a00, 0xa29cf0);
    e.mem.set_u32(0x011f_3970, 0xa29db0);
    e.mem.set_u32(0x011f_38e0, 0xa2a600);
    e.mem.set_u8(0x011f_3768, 0x48);
    e.mem.set_u32(0x011f_3cd0, 0xa29b10);
    e.mem.set_u32(0x011f_3b20, 0xa29c20);
    e.mem.set_u32(0x011f_37c0, 0xa29e00);
    e.mem.set_u32(0x011f_3850, 0xa29e40);
    1
}

// Translated from 00a2a4d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3691` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2a4d0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3691) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d6c, 0x0);
    e.mem.set_u32(0x011f_3c4c, 0x0);
    e.mem.set_u32(0x011f_3bbc, 0x0);
    e.mem.set_u32(0x011f_37cc, 0x0);
    e.mem.set_u8(0x011f_3691, 0x1);
    e.mem.set_u32(0x011f_3dfc, 0xa2a320);
    e.mem.set_u32(0x011f_3a9c, 0xa2cfd0);
    e.mem.set_u32(0x011f_3a0c, 0xa2ab10);
    e.mem.set_u32(0x011f_397c, 0xa2a1b0);
    e.mem.set_u32(0x011f_38ec, 0xa2a600);
    e.mem.set_u8(0x011f_376b, 0x14);
    e.mem.set_u32(0x011f_3cdc, 0xa2a200);
    e.mem.set_u32(0x011f_3b2c, 0xa2a1a0);
    e.mem.set_u32(0x011f_385c, 0xa2a390);
    1
}

// Translated from 00a2a720 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3692` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2a720(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3692) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d24, 0x0);
    e.mem.set_u32(0x011f_3c04, 0x0);
    e.mem.set_u32(0x011f_3b74, 0x0);
    e.mem.set_u8(0x011f_3692, 0x1);
    e.mem.set_u32(0x011f_3db4, 0xa2a5a0);
    e.mem.set_u32(0x011f_3a54, 0xa2d230);
    e.mem.set_u32(0x011f_39c4, 0xa2d1e0);
    e.mem.set_u32(0x011f_3934, 0xa2cc90);
    e.mem.set_u32(0x011f_38a4, 0xa2a600);
    e.mem.set_u8(0x011f_3759, 0x8);
    e.mem.set_u32(0x011f_3c94, 0xa2a560);
    e.mem.set_u32(0x011f_3ae4, 0xa2d1d0);
    e.mem.set_u32(0x011f_3784, 0xa29680);
    e.mem.set_u32(0x011f_3814, 0xa2a620);
    1
}

// Translated from 00a2aab0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3693` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2aab0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3693) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3693, 0x1);
    e.mem.set_u32(0x011f_3dcc, 0xa2a8e0);
    e.mem.set_u32(0x011f_3a6c, 0xa2d470);
    e.mem.set_u32(0x011f_39dc, 0xa2a7b0);
    e.mem.set_u32(0x011f_394c, 0xa27040);
    e.mem.set_u32(0x011f_38bc, 0xa2a600);
    e.mem.set_u8(0x011f_375f, 0x10);
    e.call(0x00a2_aa50, &args![0x1u32, 0x1u32]);
    1
}

// Translated from 00a2ad80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3694` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2ad80(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3694) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d54, 0x0);
    e.mem.set_u32(0x011f_3c34, 0x0);
    e.mem.set_u32(0x011f_3ba4, 0x0);
    e.mem.set_u8(0x011f_3694, 0x1);
    e.mem.set_u32(0x011f_3de4, 0xa2abc0);
    e.mem.set_u32(0x011f_3a84, 0xa2d690);
    e.mem.set_u32(0x011f_39f4, 0xa2ab10);
    e.mem.set_u32(0x011f_3964, 0xa2ab40);
    e.mem.set_u32(0x011f_38d4, 0xa2a600);
    e.mem.set_u8(0x011f_3765, 0x14);
    e.mem.set_u32(0x011f_3cc4, 0xa2ab90);
    e.mem.set_u32(0x011f_3b14, 0xa2d680);
    e.mem.set_u32(0x011f_37b4, 0xa2d670);
    e.mem.set_u32(0x011f_3844, 0xa2ac30);
    1
}

// Translated from 00a2bb90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3695` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2bb90(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3695) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3695, 0x1);
    e.mem.set_u32(0x011f_3dd4, 0xa2bb00);
    e.mem.set_u32(0x011f_3a74, 0xa2bb60);
    e.mem.set_u32(0x011f_39e4, 0xa2ae10);
    e.mem.set_u32(0x011f_3954, 0xa2af30);
    e.mem.set_u32(0x011f_38c4, 0xa2a600);
    e.mem.set_u8(0x011f_3761, 0x4c);
    e.call(0x00a2_baa0, &args![0x1u32, 0x3u32]);
    1
}

// Translated from 00a2aa50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2aa50(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2a7f0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2a890);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2a7e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2a940);
}

// Translated from 00a2baa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2baa0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa2b6d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2af70);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2b050);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa2b130);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2ae90);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa2b410);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2b850);
}

// The animation key functions below work on the key arrays the key tables (filled above)
// point at: arrays allocated from the memory manager with their key count in the word before
// them. The key layouts are described by offset (the code's own); the Euler rotation key
// (0x48 bytes) holds, after its time and quaternion, three axis lists: counts at +0x14,
// key types at +0x20, key size bytes at +0x2c, keys at +0x30 and last indices at +0x3c.

/// The memory manager singleton, its `Allocate` (`thiscall`, the size) and `Deallocate`
/// (`thiscall`, the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The compiler's array constructor iterator (`stdcall`, (array, element size, count,
/// constructor)).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `NiQuaternion`'s constructor from four floats (`thiscall`, (w, x, y, z)).
const QUATERNION_CONSTRUCT: u32 = 0x00a6_db50;
/// `NiQuaternion::Slerp` (`thiscall` on the result, (t, &p, &q)) and `00a6e430` (`thiscall`
/// on the result, (t, &p, &a, &b, &q)), the spherical quadrangle interpolation.
const QUATERNION_SLERP: u32 = 0x00a6_e330;
const QUATERNION_SQUAD: u32 = 0x00a6_e430;
/// `NiFloatKey::GenInterp` (`cdecl`, (time, keys, type, count, &last index, size); answers
/// in `ST0`).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `00a42050` (`cdecl`, three floats; answers in `ST0`): the linear interpolation of two
/// values at a time.
const FLOAT_LERP: u32 = 0x00a4_2050;
/// The float key, position key and TCB position key readers (`thiscall` on the key, the
/// stream): `00a26eb0` (time and value), `00a27430` (time and three floats), and `00a29640`
/// (the 20-byte keys' reader).
const READ_FLOAT_KEY: u32 = 0x00a2_6eb0;
const READ_POS_KEY: u32 = 0x00a2_7430;
const READ_KEY_00A29640: u32 = 0x00a2_9640;
/// The tables of the scale (float) key functions, one word per key type: delete
/// (`011f38a0`), create (`011f3db0`), fill (`011f3780`), and the key size bytes (`011f3758`).
const FLOAT_KEY_DELETE: u32 = 0x011f_38a0;
const FLOAT_KEY_CREATE: u32 = 0x011f_3db0;
const FLOAT_KEY_FILL: u32 = 0x011f_3780;
const FLOAT_KEY_SIZE: u32 = 0x011f_3758;
/// The doubles 0.5 (`01011588`), 2 (`01011590`) and 3 (`01021928`).
const HALF_DOUBLE: u32 = 0x0101_1588;
const TWO_DOUBLE: u32 = 0x0101_1590;
const THREE_DOUBLE: u32 = 0x0102_1928;

/// The byte count of `count` keys of `size` bytes with the count word: `count * size + 4`,
/// all ones when either step overflows.
fn key_array_bytes(count: u32, size: u32) -> u32 {
    count
        .checked_mul(size)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX)
}

/// How the keys of a create function are read.
enum KeyReader {
    /// A reader of another unit, called by address (`thiscall` on the key, the stream).
    Call(u32),
    /// A reader of this unit.
    Direct(fn(&mut Engine, Ptr, Ptr)),
}

/// The create functions' shared body: `count` keys of `size` bytes from the memory manager
/// (the count in the word before them; null when the allocation fails), built by the array
/// constructor iterator with `constructor` when there is one, then each read from the stream.
/// Answers the keys.
fn create_keys(
    e: &mut Engine,
    stream: Ptr,
    count: u32,
    size: u32,
    constructor: Option<u32>,
    reader: KeyReader,
) -> u32 {
    let block = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, key_array_bytes(count, size)],
        )
        .u32();
    let keys = if block != 0 {
        e.mem.set_u32(block, count);
        if let Some(constructor) = constructor {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block + 4, size, count, constructor],
            );
        }
        block + 4
    } else {
        0
    };
    let mut key = keys;
    for _ in 0..count {
        match reader {
            KeyReader::Call(read) => {
                e.call(read, &args![key, stream]);
            }
            KeyReader::Direct(read) => read(e, Ptr::new(key), stream),
        }
        key = key.wrapping_add(size);
    }
    keys
}

/// Reads `size` bytes from the stream's input into `destination`, the byte-count word at
/// `count` set to `size` first.
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32, count: u32) {
    // NiStream::m_pkIstr (Xbox PDB) +0x24c; the input's position +4 and read function +8.
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![input, destination, size, count, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

/// `sin` and `cos` of an angle as `FSINCOS` leaves them, each stored to `float` (computed in
/// `f64`, which can differ from the extended-precision result in the last bit of a float).
fn sin_cos(angle: f64) -> (u32, u32) {
    (x87_store32(angle.sin()), x87_store32(angle.cos()))
}

// Translated from 00a29a70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the Euler rotation key's
/// constructor, the one its arrays are built with): the quaternion at +4 from four zeros
/// (`00a6db50`), then the three axis lists empty: counts, types, keys and last indices 0 and
/// the size bytes 0. Answers `this`.
pub fn fn_00a29a70(e: &mut Engine, this: Ptr) -> Ptr {
    let zero = x87_store32(0.0);
    e.call(
        QUATERNION_CONSTRUCT,
        &args![this.addr() + 4, zero, zero, zero, zero],
    );
    for offset in [
        0x14u32, 0x18, 0x1c, 0x30, 0x34, 0x38, 0x20, 0x24, 0x28, 0x3c, 0x40, 0x44,
    ] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in [0x2cu32, 0x2d, 0x2e] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    this
}

// Translated from 00a29ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an Euler rotation key): for each of the three
/// axis lists, deletes its keys through the float key delete function of its type
/// (`011f38a0`) when there are keys, then clears its keys, count and type.
pub fn fn_00a29ad0(e: &mut Engine, this: Ptr) {
    for axis in 0..3 {
        let keys_at = this.addr() + 0x30 + 4 * axis;
        let keys = e.mem.u32(keys_at);
        if keys != 0 {
            let kind = e.mem.u32(this.addr() + 0x20 + 4 * axis);
            let delete = e
                .mem
                .u32(kind.wrapping_mul(4).wrapping_add(FLOAT_KEY_DELETE));
            e.call(delete, &args![keys]);
        }
        e.mem.set_u32(keys_at, 0);
        e.mem.set_u32(this.addr() + 0x14 + 4 * axis, 0);
        e.mem.set_u32(this.addr() + 0x20 + 4 * axis, 0);
    }
}

// Translated from 00a29b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, key, second key (not read), result)): the
/// Euler rotation key's interpolation: each axis angle at `time` from its float keys
/// (`NiFloatKey::GenInterp`, from and updating the axis's last index; 0 for an axis without
/// keys), then the quaternion of the three rotations from the half angles' sines and cosines
/// (`FSINCOS`) stored at `result` (`w x y z`).
pub fn fn_00a29b10(e: &mut Engine, time: f32, key: Ptr, _unused_2: u32, result: Ptr) {
    let k = key.addr();
    let mut angles = [0u32; 3];
    for (axis, angle) in angles.iter_mut().enumerate() {
        let axis = axis as u32;
        let count = e.mem.u32(k + 0x14 + 4 * axis);
        let value = if count == 0 {
            0.0
        } else {
            let size = u32::from(e.mem.u8(k + 0x2c + axis));
            let keys = e.mem.u32(k + 0x30 + 4 * axis);
            let kind = e.mem.u32(k + 0x20 + 4 * axis);
            let time_word = x87_store32(x87_load32(time.to_bits()));
            e.call(
                FLOAT_KEY_GEN_INTERP,
                &args![time_word, keys, kind, count, k + 0x3c + 4 * axis, size],
            )
            .f64()
        };
        *angle = x87_store32(value);
    }
    let half = f64::from_bits(e.mem.u64(HALF_DOUBLE));
    let mut sines = [0u32; 3];
    let mut cosines = [0u32; 3];
    for axis in 0..3 {
        let halved = x87_store32(x87_mul(x87_load32(angles[axis]), half));
        let (sine, cosine) = sin_cos(x87_load32(halved));
        sines[axis] = sine;
        cosines[axis] = cosine;
    }
    let (s0, s1, s2) = (
        x87_load32(sines[0]),
        x87_load32(sines[1]),
        x87_load32(sines[2]),
    );
    let (c0, c2) = (x87_load32(cosines[0]), x87_load32(cosines[2]));
    let c1 = cosines[1];
    let r = result.addr();
    let s2s1 = x87_mul(s2, s1);
    let c2c1 = x87_mul(c2, x87_m32(c1));
    let w = x87_add(x87_mul(c0, c2c1), x87_mul(s0, s2s1));
    e.mem.set_u32(r, x87_store32(w));
    let c2c1s0 = x87_mul(c2c1, s0);
    let s2s1c0 = x87_mul(s2s1, c0);
    e.mem.set_u32(r + 4, x87_store32(x87_sub(c2c1s0, s2s1c0)));
    let s2c1 = x87_mul(s2, x87_load32(c1));
    let s1c2 = x87_mul(s1, c2);
    let y = x87_add(x87_mul(c0, s1c2), x87_mul(s0, s2c1));
    e.mem.set_u32(r + 8, x87_store32(y));
    let s2c1c0 = x87_mul(s2c1, c0);
    let s0s1c2 = x87_mul(s0, s1c2);
    e.mem.set_u32(r + 0xc, x87_store32(x87_sub(s2c1c0, s0s1c2)));
}

// Translated from 00a29e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the Euler rotation key): for each axis list with
/// keys, the float key fill function of its type (`011f3780`) on `(keys, count, size)`.
pub fn fn_00a29e00(e: &mut Engine, key: Ptr) {
    let k = key.addr();
    for axis in 0..3 {
        let keys = e.mem.u32(k + 0x30 + 4 * axis);
        if keys != 0 {
            let size = u32::from(e.mem.u8(k + 0x2c + axis));
            let kind = e.mem.u32(k + 0x20 + 4 * axis);
            let count = e.mem.u32(k + 0x14 + 4 * axis);
            let fill = e.mem.u32(kind.wrapping_mul(4).wrapping_add(FLOAT_KEY_FILL));
            e.call(fill, &args![keys, count, size]);
        }
    }
}

// Translated from 00a29f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an Euler rotation key, the stream): files older
/// than version `0x0a010068` (`NiStream` +0xd8) start with a 4-byte word that is read and
/// dropped. Then for each axis a 4-byte key count and, when not 0, a 4-byte key type, the
/// type's key size byte (`011f3758`) and the keys made by the type's create function
/// (`011f3db0`, `(stream, count)`). The stream argument's stack word is the byte-count word.
pub fn fn_00a29f00(e: &mut Engine, this: Ptr, stream: Ptr) {
    let k = this.addr();
    e.with_stack(8, |e, slots| {
        let count_word = slots.addr();
        let local = slots.addr() + 4;
        // NiStream::m_uiNifFileVersion (Xbox PDB) +0xd8
        if e.mem.u32(stream.addr() + 0xd8) < 0x0a01_0068 {
            stream_read(e, stream, local, 4, count_word);
        }
        for axis in 0..3 {
            let count_at = k + 0x14 + 4 * axis;
            stream_read(e, stream, count_at, 4, count_word);
            let count = e.mem.u32(count_at);
            if count != 0 {
                stream_read(e, stream, local, 4, count_word);
                let kind = e.mem.u32(local);
                e.mem.set_u32(k + 0x20 + 4 * axis, kind);
                let size = e.mem.u8(kind.wrapping_add(FLOAT_KEY_SIZE));
                e.mem.set_u8(k + 0x2c + axis, size);
                let create = e
                    .mem
                    .u32(kind.wrapping_mul(4).wrapping_add(FLOAT_KEY_CREATE));
                let count = e.mem.u32(count_at);
                let keys = e.call(create, &args![stream, count]).u32();
                e.mem.set_u32(k + 0x30 + 4 * axis, keys);
            }
        }
    });
}

// Translated from 00a2a070 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// the Euler rotation keys' create function: `count` keys of 0x48 bytes built by `00a29a70`
/// and read by `00a29f00`. Answers the keys.
pub fn fn_00a2a070(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    create_keys(
        e,
        stream,
        count,
        0x48,
        Some(0x00a2_9a70),
        KeyReader::Direct(fn_00a29f00),
    )
}

// Translated from 00a2a200 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (time, first key,
/// second key, result)): the linear interpolation of the two keys' quaternions (+4):
/// `result = first * (1 - time) + second * time`, word by word, each product and sum
/// stored as a `float`.
pub fn fn_00a2a200(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) {
    let t = x87_load32(time.to_bits());
    let rest = x87_store32(x87_sub(1.0, t));
    let rest_value = x87_load32(rest);
    let mut sums = [0u32; 4];
    let mut scaled_first = [0u32; 4];
    for word in 0..4u32 {
        let value = e.mem.u32(first.addr() + 4 + 4 * word);
        scaled_first[word as usize] = if word == 3 {
            x87_store32(x87_mul(rest_value, x87_m32(value)))
        } else {
            x87_store32(x87_mul(x87_load32(value), rest_value))
        };
    }
    for word in 0..4u32 {
        let value = e.mem.u32(second.addr() + 4 + 4 * word);
        let scaled = if word == 3 {
            x87_store32(x87_mul(t, x87_m32(value)))
        } else {
            x87_store32(x87_mul(x87_load32(value), t))
        };
        sums[word as usize] = x87_store32(x87_add(
            x87_load32(scaled),
            x87_m32(scaled_first[word as usize]),
        ));
    }
    for (word, sum) in sums.iter().enumerate() {
        e.mem.set_u32(result.addr() + 4 * word as u32, *sum);
    }
}

// Translated from 00a2a560 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (time, first key,
/// second key, result)): stores at `result` the float `00a42050` answers for `time` and the
/// two keys' values (+4), each passed through a `float` stack word.
pub fn fn_00a2a560(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) {
    let second_value = x87_store32(x87_load32(e.mem.u32(second.addr() + 4)));
    let first_value = x87_store32(x87_load32(e.mem.u32(first.addr() + 4)));
    let words = [
        x87_store32(x87_load32(time.to_bits())),
        x87_store32(x87_load32(first_value)),
        x87_store32(x87_load32(second_value)),
    ];
    let value = e.call(FLOAT_LERP, &words).f64();
    e.mem.set_u32(result.addr(), x87_store32(value));
}

// Translated from 00a2a5a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// the 8-byte float keys' create function: `count` keys read by `00a26eb0`. Answers the keys.
pub fn fn_00a2a5a0(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    create_keys(e, stream, count, 8, None, KeyReader::Call(READ_FLOAT_KEY))
}

// Translated from 00a2a600 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, the keys): the
/// shared key delete function: gives the block (the count word before the keys) back to the
/// memory manager; nothing for null.
pub fn fn_00a2a600(e: &mut Engine, keys: u32) {
    if keys != 0 {
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, keys.wrapping_sub(4)],
        );
    }
}

// Translated from 00a2a7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, first key, second key, result)): the linear
/// interpolation of the two keys' positions (+4): `result = second * time + first *
/// (1 - time)`, each product and sum stored as a `float`.
pub fn fn_00a2a7f0(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) {
    let t = x87_load32(time.to_bits());
    let rest = x87_load32(x87_store32(x87_sub(1.0, t)));
    let f = first.addr();
    let a0 = x87_store32(x87_mul(x87_load32(e.mem.u32(f + 4)), rest));
    let a1 = x87_store32(x87_mul(x87_load32(e.mem.u32(f + 8)), rest));
    let a2 = x87_store32(x87_mul(rest, x87_m32(e.mem.u32(f + 0xc))));
    let s = second.addr();
    let b0 = x87_store32(x87_mul(x87_load32(e.mem.u32(s + 4)), t));
    let b1 = x87_store32(x87_mul(x87_load32(e.mem.u32(s + 8)), t));
    let b2 = x87_store32(x87_mul(t, x87_m32(e.mem.u32(s + 0xc))));
    let r = result.addr();
    e.mem
        .set_u32(r, x87_store32(x87_add(x87_load32(b0), x87_m32(a0))));
    e.mem
        .set_u32(r + 4, x87_store32(x87_add(x87_load32(a1), x87_m32(b1))));
    e.mem
        .set_u32(r + 8, x87_store32(x87_add(x87_load32(b2), x87_m32(a2))));
}

// Translated from 00a2a8e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// the 16-byte position keys' create function: `count` keys read by `00a27430`. Answers the
/// keys.
pub fn fn_00a2a8e0(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    create_keys(e, stream, count, 0x10, None, KeyReader::Call(READ_POS_KEY))
}

// Translated from 00a2ab90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (time, first key,
/// second key, result)): `NiQuaternion::Slerp` into `result` of the two keys' quaternions
/// (+4) at `time`. Answers what it answers.
pub fn fn_00a2ab90(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) -> u32 {
    let t = x87_store32(x87_load32(time.to_bits()));
    e.call(
        QUATERNION_SLERP,
        &args![result, t, first.addr() + 4, second.addr() + 4],
    )
    .u32()
}

// Translated from 00a2abc0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// the 20-byte keys' create function: `count` keys built by `00a2bdb0` and read by
/// `00a29640`. Answers the keys.
pub fn fn_00a2abc0(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    create_keys(
        e,
        stream,
        count,
        0x14,
        Some(0x00a2_bdb0),
        KeyReader::Call(READ_KEY_00A29640),
    )
}

// Translated from 00a2b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a TCB position key, (previous position, next
/// position, previous span, next span)): the key's two tangents from its tension (+0x10),
/// continuity (+0x14) and bias (+0x18): with `d = position - previous`, `e = next -
/// position`, `T = (1 - tension) / 2`, the incoming tangent (+0x1c) `T(1 - c)(1 + b) d +
/// T(1 + c)(1 - b) e` and the outgoing one (+0x28) `T(1 + c)(1 + b) d + T(1 - c)(1 - b) e`,
/// then scaled by `2 previous / (previous + next)` and `2 next / (previous + next)` of the
/// spans; every intermediate stored as a `float` as the code stores it.
pub fn fn_00a2b1d0(
    e: &mut Engine,
    this: Ptr,
    previous: Ptr,
    next: Ptr,
    previous_span: f32,
    next_span: f32,
) {
    let k = this.addr();
    let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let md = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    let p = previous.addr();
    let n = next.addr();
    let d0 = x87_store32(x87_sub(ld(e, k + 4), md(e, p)));
    let d1 = x87_store32(x87_sub(ld(e, k + 8), md(e, p + 4)));
    let d2 = x87_store32(x87_sub(ld(e, k + 0xc), md(e, p + 8)));
    let e0 = x87_store32(x87_sub(ld(e, n), md(e, k + 4)));
    let e1 = x87_store32(x87_sub(ld(e, n + 4), md(e, k + 8)));
    let e2 = x87_store32(x87_sub(ld(e, n + 8), md(e, k + 0xc)));
    let bias_plus = x87_store32(x87_add(ld(e, k + 0x18), 1.0));
    let bias_minus = x87_store32(x87_sub(1.0, ld(e, k + 0x18)));
    let half = f64::from_bits(e.mem.u64(HALF_DOUBLE));
    let tension = x87_store32(x87_mul(x87_sub(1.0, ld(e, k + 0x10)), half));
    let continuity_plus = x87_store32(x87_add(ld(e, k + 0x14), 1.0));
    let tension_value = x87_load32(tension);
    let x = x87_store32(x87_mul(x87_load32(continuity_plus), tension_value));
    let continuity_minus = x87_store32(x87_sub(1.0, ld(e, k + 0x14)));
    let y = x87_store32(x87_mul(tension_value, x87_m32(continuity_minus)));
    let x_value = x87_load32(x);
    let bm = x87_load32(bias_minus);
    let pa = x87_store32(x87_mul(x_value, bm));
    let pa_value = x87_load32(pa);
    let (e0v, e1v, e2v) = (x87_load32(e0), x87_load32(e1), x87_load32(e2));
    let q0 = x87_store32(x87_mul(e0v, pa_value));
    let q1 = x87_store32(x87_mul(e1v, pa_value));
    let q2 = x87_store32(x87_mul(pa_value, e2v));
    let qa = x87_store32(x87_mul(x87_load32(y), x87_m32(bias_plus)));
    let qa_value = x87_load32(qa);
    let r0 = x87_store32(x87_mul(qa_value, x87_m32(d0)));
    let r1 = x87_store32(x87_mul(x87_load32(d1), qa_value));
    let d2v = x87_load32(d2);
    let r2 = x87_store32(x87_mul(qa_value, d2v));
    let sum = |a: u32, b: u32| x87_store32(x87_add(x87_load32(a), x87_m32(b)));
    e.mem.set_u32(k + 0x1c, sum(r0, q0));
    e.mem.set_u32(k + 0x20, sum(r1, q1));
    e.mem.set_u32(k + 0x24, sum(r2, q2));
    let z = x87_store32(x87_mul(bm, x87_load32(y)));
    let z_value = x87_load32(z);
    let u0 = x87_store32(x87_mul(e0v, z_value));
    let u1 = x87_store32(x87_mul(e1v, z_value));
    let u2 = x87_store32(x87_mul(z_value, e2v));
    let w = x87_store32(x87_mul(x_value, x87_load32(bias_plus)));
    let w_value = x87_load32(w);
    let v0 = x87_store32(x87_mul(x87_load32(d0), w_value));
    let v1 = x87_store32(x87_mul(x87_load32(d1), w_value));
    let v2 = x87_store32(x87_mul(d2v, w_value));
    e.mem.set_u32(k + 0x28, sum(v0, u0));
    e.mem.set_u32(k + 0x2c, sum(v1, u1));
    e.mem.set_u32(k + 0x30, sum(v2, u2));
    let a = x87_load32(previous_span.to_bits());
    let b = x87_load32(next_span.to_bits());
    let two = x87_load64(e.mem.u64(TWO_DOUBLE));
    let h = x87_store32(x87_div(two, x87_add(a, b)));
    let h_value = x87_load32(h);
    let f1 = x87_load32(x87_store32(x87_mul(a, h_value)));
    let s0 = x87_store32(x87_mul(ld(e, k + 0x1c), f1));
    e.mem.set_u32(k + 0x1c, s0);
    let s1 = x87_store32(x87_mul(ld(e, k + 0x20), f1));
    e.mem.set_u32(k + 0x20, s1);
    let s2 = x87_store32(x87_mul(f1, md(e, k + 0x24)));
    e.mem.set_u32(k + 0x24, s2);
    let f2 = x87_load32(x87_store32(x87_mul(h_value, b)));
    let t0 = x87_store32(x87_mul(ld(e, k + 0x28), f2));
    e.mem.set_u32(k + 0x28, t0);
    let t1 = x87_store32(x87_mul(ld(e, k + 0x2c), f2));
    e.mem.set_u32(k + 0x2c, t1);
    let t2 = x87_store32(x87_mul(f2, md(e, k + 0x30)));
    e.mem.set_u32(k + 0x30, t2);
}

/// Writes three floats to a stack block and answers its address (a `float[3]` local).
fn put3(e: &mut Engine, at: u32, words: [u32; 3]) -> u32 {
    for (i, w) in words.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *w);
    }
    at
}

// Translated from 00a2b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (keys, count, size word, which it does not read);
/// the TCB position keys' fill function): with two keys or more, each key's tangents
/// (`00a2b1d0`): the first key against the reflection `2 first - second` with spans 1, the
/// middle keys against their neighbours with the time spans, the last against the
/// reflection `2 last - previous` with spans 1. Then for each key but the last, the cubic
/// coefficients from it and the next key: +0x34 `3 (next - key) - (2 outgoing + next's
/// incoming)` and +0x40 `(outgoing + next's incoming) - 2 (next - key)`. With no keys the
/// count less one wraps and the second loop runs on (as in the exe; its callers pass keys).
pub fn fn_00a2b410(e: &mut Engine, keys: Ptr, count: u32, _unused_2: u32) {
    let base = keys.addr();
    let two = x87_load64(e.mem.u64(TWO_DOUBLE));
    let last = count.wrapping_sub(1);
    let one = x87_store32(1.0);
    let one_value = f32::from_bits(one);
    e.with_stack(0x10, |e, local| {
        let local = local.addr();
        if count >= 2 {
            let k0 = base;
            let w0 = x87_store32(x87_mul(x87_load32(e.mem.u32(k0 + 4)), two));
            let w1 = x87_store32(x87_mul(x87_load32(e.mem.u32(k0 + 8)), two));
            let w2 = x87_store32(x87_mul(two, x87_m32(e.mem.u32(k0 + 0xc))));
            let k1 = base + 0x4c;
            let z = [
                x87_store32(x87_sub(x87_load32(w0), x87_m32(e.mem.u32(k1 + 4)))),
                x87_store32(x87_sub(x87_load32(w1), x87_m32(e.mem.u32(k1 + 8)))),
                x87_store32(x87_sub(x87_load32(w2), x87_m32(e.mem.u32(k1 + 0xc)))),
            ];
            let reflected = put3(e, local, z);
            fn_00a2b1d0(
                e,
                Ptr::new(k0),
                Ptr::new(reflected),
                Ptr::new(k1 + 4),
                one_value,
                one_value,
            );
            if last > 1 {
                let mut value = base + 4;
                for _ in 0..last - 1 {
                    let key = value + 0x48;
                    let span_next = x87_store32(x87_sub(
                        x87_load32(e.mem.u32(value + 0x94)),
                        x87_m32(e.mem.u32(key)),
                    ));
                    let span_previous = x87_store32(x87_sub(
                        x87_load32(e.mem.u32(key)),
                        x87_m32(e.mem.u32(value.wrapping_sub(4))),
                    ));
                    fn_00a2b1d0(
                        e,
                        Ptr::new(key),
                        Ptr::new(value),
                        Ptr::new(value + 0x98),
                        f32::from_bits(x87_store32(x87_load32(span_previous))),
                        f32::from_bits(x87_store32(x87_load32(span_next))),
                    );
                    value += 0x4c;
                }
            }
            let kl = base.wrapping_add(last.wrapping_mul(0x4c));
            let m0 = x87_store32(x87_mul(x87_load32(e.mem.u32(kl + 4)), two));
            let m1 = x87_store32(x87_mul(x87_load32(e.mem.u32(kl + 8)), two));
            let m2 = x87_store32(x87_mul(two, x87_m32(e.mem.u32(kl + 0xc))));
            let prior = base
                .wrapping_add(count.wrapping_sub(2).wrapping_mul(0x4c))
                .wrapping_add(4);
            let y = [
                x87_store32(x87_sub(x87_load32(m0), x87_m32(e.mem.u32(prior)))),
                x87_store32(x87_sub(x87_load32(m1), x87_m32(e.mem.u32(prior + 4)))),
                x87_store32(x87_sub(x87_load32(m2), x87_m32(e.mem.u32(prior + 8)))),
            ];
            let reflected = put3(e, local, y);
            fn_00a2b1d0(
                e,
                Ptr::new(kl),
                Ptr::new(prior),
                Ptr::new(reflected),
                one_value,
                one_value,
            );
        }
    });
    let two = x87_load64(e.mem.u64(TWO_DOUBLE));
    let three = x87_load64(e.mem.u64(THREE_DOUBLE));
    let mut remaining = last;
    let mut at = base + 0x58;
    while remaining != 0 {
        let ld = |e: &Engine, offset: i32| x87_load32(e.mem.u32(at.wrapping_add(offset as u32)));
        let md = |e: &Engine, offset: i32| x87_m32(e.mem.u32(at.wrapping_add(offset as u32)));
        let a = [
            x87_store32(x87_mul(ld(e, -0x30), two)),
            x87_store32(x87_mul(ld(e, -0x2c), two)),
            x87_store32(x87_mul(ld(e, -0x28), two)),
        ];
        let b = [
            x87_store32(x87_add(x87_load32(a[0]), md(e, 0x10))),
            x87_store32(x87_add(x87_load32(a[1]), md(e, 0x14))),
            x87_store32(x87_add(x87_load32(a[2]), md(e, 0x18))),
        ];
        let c = [
            x87_store32(x87_sub(ld(e, -8), md(e, -0x54))),
            x87_store32(x87_sub(ld(e, -4), md(e, -0x50))),
            x87_store32(x87_sub(ld(e, 0), md(e, -0x4c))),
        ];
        for i in 0..3 {
            let d = x87_store32(x87_mul(x87_load32(c[i]), three));
            let value = x87_store32(x87_sub(x87_load32(d), x87_m32(b[i])));
            e.mem
                .set_u32(at.wrapping_sub(0x24).wrapping_add(4 * i as u32), value);
        }
        let g = [
            x87_store32(x87_mul(
                x87_load32(x87_store32(x87_sub(ld(e, -8), md(e, -0x54)))),
                two,
            )),
            x87_store32(x87_mul(
                x87_load32(x87_store32(x87_sub(ld(e, -4), md(e, -0x50)))),
                two,
            )),
            x87_store32(x87_mul(
                x87_load32(x87_store32(x87_sub(ld(e, 0), md(e, -0x4c)))),
                two,
            )),
        ];
        let h = [
            x87_store32(x87_add(ld(e, -0x30), md(e, 0x10))),
            x87_store32(x87_add(ld(e, 0x14), md(e, -0x2c))),
            x87_store32(x87_add(ld(e, 0x18), md(e, -0x28))),
        ];
        let b0 = x87_store32(x87_sub(x87_load32(h[0]), x87_m32(g[0])));
        e.mem.set_u32(at.wrapping_sub(0x18), b0);
        at = at.wrapping_add(0x4c);
        remaining = remaining.wrapping_sub(1);
        let b1 = x87_store32(x87_sub(x87_load32(h[1]), x87_m32(g[1])));
        e.mem.set_u32(at.wrapping_sub(0x60), b1);
        let b2 = x87_store32(x87_sub(x87_load32(h[2]), x87_m32(g[2])));
        e.mem.set_u32(at.wrapping_sub(0x5c), b2);
    }
}

// Translated from 00a2b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a TCB position key, the stream): the time and
/// position (`00a27430`), then the 4-byte tension (+0x10), continuity (+0x14) and bias
/// (+0x18); the stream argument's stack word is the byte-count word.
pub fn fn_00a2b980(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(READ_POS_KEY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0x10u32, 0x14, 0x18] {
            stream_read(e, stream, this.addr() + offset, 4, count.addr());
        }
    });
}

// Translated from 00a2bb00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// the TCB position keys' create function: `count` keys of 0x4c bytes read by `00a2b980`.
/// Answers the keys.
pub fn fn_00a2bb00(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    create_keys(e, stream, count, 0x4c, None, KeyReader::Direct(fn_00a2b980))
}

// Translated from 00a2bc60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (time, first key,
/// second key, result)): the spherical quadrangle interpolation (`00a6e430`) into `result`
/// at `time` of the first key's quaternion (+4) and its tangent quaternion (+0x20) and the
/// second key's tangent quaternion (+0x30) and quaternion (+4). Answers what it answers.
pub fn fn_00a2bc60(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) -> u32 {
    let t = x87_store32(x87_load32(time.to_bits()));
    let (f, s) = (first.addr(), second.addr());
    e.call(
        QUATERNION_SQUAD,
        &args![result, t, f + 4, f + 0x20, s + 0x30, s + 4],
    )
    .u32()
}

// Translated from 00a2bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (result, scalar, quaternion); the scalar times a
/// quaternion): the quaternion `scalar * q`, each part stored through a `float` stack word,
/// built at `result` by `00a6db50`. Answers `result`.
pub fn fn_00a2bc90(e: &mut Engine, result: Ptr, scalar: f32, quaternion: Ptr) -> Ptr {
    let q = quaternion.addr();
    let s = x87_load32(scalar.to_bits());
    let z = x87_store32(x87_mul(x87_load32(e.mem.u32(q + 0xc)), s));
    let z = x87_store32(x87_load32(z));
    let y = x87_store32(x87_load32(x87_store32(x87_mul(
        x87_load32(e.mem.u32(q + 8)),
        s,
    ))));
    let x = x87_store32(x87_load32(x87_store32(x87_mul(
        x87_load32(e.mem.u32(q + 4)),
        s,
    ))));
    let w = x87_store32(x87_load32(x87_store32(x87_mul(s, x87_m32(e.mem.u32(q))))));
    e.call(QUATERNION_CONSTRUCT, &args![result, w, x, y, z]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_9a70, fn_00a29a70(Ptr) -> Ptr),
        entry!(0x00a2_9ad0, fn_00a29ad0(Ptr)),
        entry!(0x00a2_9b10, fn_00a29b10(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_9e00, fn_00a29e00(Ptr)),
        entry!(0x00a2_9f00, fn_00a29f00(Ptr, Ptr)),
        entry!(0x00a2_a070, fn_00a2a070(Ptr, u32) -> u32),
        entry!(0x00a2_a200, fn_00a2a200(f32, Ptr, Ptr, Ptr)),
        entry!(0x00a2_a560, fn_00a2a560(f32, Ptr, Ptr, Ptr)),
        entry!(0x00a2_a5a0, fn_00a2a5a0(Ptr, u32) -> u32),
        entry!(0x00a2_a600, fn_00a2a600(u32)),
        entry!(0x00a2_a7f0, fn_00a2a7f0(f32, Ptr, Ptr, Ptr)),
        entry!(0x00a2_a8e0, fn_00a2a8e0(Ptr, u32) -> u32),
        entry!(0x00a2_ab90, fn_00a2ab90(f32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00a2_abc0, fn_00a2abc0(Ptr, u32) -> u32),
        entry!(0x00a2_b1d0, fn_00a2b1d0(Ptr, Ptr, Ptr, f32, f32)),
        entry!(0x00a2_b410, fn_00a2b410(Ptr, u32, u32)),
        entry!(0x00a2_b980, fn_00a2b980(Ptr, Ptr)),
        entry!(0x00a2_bb00, fn_00a2bb00(Ptr, u32) -> u32),
        entry!(0x00a2_bc60, fn_00a2bc60(f32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00a2_bc90, fn_00a2bc90(Ptr, f32, Ptr) -> Ptr),
        entry!(0x00a299f0, fn_00a299f0() -> u32),
        entry!(0x00a2a110, fn_00a2a110() -> u32),
        entry!(0x00a2a4d0, fn_00a2a4d0() -> u32),
        entry!(0x00a2a720, fn_00a2a720() -> u32),
        entry!(0x00a2aab0, fn_00a2aab0() -> u32),
        entry!(0x00a2ad80, fn_00a2ad80() -> u32),
        entry!(0x00a2bb90, fn_00a2bb90() -> u32),
        entry!(0x00a2aa50, fn_00a2aa50(u32, u32)),
        entry!(0x00a2baa0, fn_00a2baa0(u32, u32)),
    ]
}

#[cfg(test)]
mod key_function_tests {
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

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// An engine with the double constants, a heap allocator and a call log.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_1000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.mem.set_f64(THREE_DOUBLE, 3.0);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A stream (file version `version`) whose input answers 4-byte words counting up from
    /// `first`.
    fn stream(e: &mut Engine, first: u32, version: u32) -> u32 {
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut next = first;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        stream
    }

    #[test]
    fn euler_key_constructor_and_release() {
        let mut e = engine();
        e.register(QUATERNION_CONSTRUCT, |_, a| a[0].into_ret());
        let key = e.mem.alloc(0x48);
        e.mem.write(key, &[0xee; 0x48]);
        assert_eq!(e.call(0x00a2_9a70, &args![key]).u32(), key);
        assert_eq!(
            calls_to(&e, QUATERNION_CONSTRUCT),
            vec![vec![key + 4, 0, 0, 0, 0]]
        );
        for offset in (0x14..0x48).step_by(4) {
            let expected = if offset == 0x2c { 0xee00_0000 } else { 0 };
            assert_eq!(e.mem.u32(key + offset), expected, "{offset:#x}");
        }
        // Release: the second axis has keys of type 2.
        for kind in 0..3u32 {
            e.mem
                .set_u32(FLOAT_KEY_DELETE + 4 * kind, 0x0310_0000 + kind);
            e.register(0x0310_0000 + kind, |_, _| Ret::default());
        }
        e.mem.set_u32(key + 0x34, 0x7000);
        e.mem.set_u32(key + 0x24, 2);
        e.mem.set_u32(key + 0x18, 5);
        e.call(0x00a2_9ad0, &args![key]);
        assert_eq!(calls_to(&e, 0x0310_0002), vec![vec![0x7000]]);
        assert!(calls_to(&e, 0x0310_0000).is_empty());
        assert_eq!(
            (
                e.mem.u32(key + 0x34),
                e.mem.u32(key + 0x24),
                e.mem.u32(key + 0x18)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn euler_key_interpolation_builds_the_quaternion() {
        let mut e = engine();
        // Only the z axis has keys: its angle gives a rotation about z.
        e.register(FLOAT_KEY_GEN_INTERP, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        let key = e.mem.alloc(0x48);
        e.mem.set_u32(key + 0x1c, 2);
        e.mem.set_u32(key + 0x28, 1);
        e.mem.set_u8(key + 0x2e, 8);
        e.mem.set_u32(key + 0x38, 0x7000);
        let result = e.mem.alloc(0x10);
        e.call(0x00a2_9b10, &args![0.25f32, key, 0u32, result]);
        let call = &calls_to(&e, FLOAT_KEY_GEN_INTERP)[0];
        assert_eq!(call, &vec![0.25f32.to_bits(), 0x7000, 1, 2, key + 0x44, 8]);
        let (s, c) = (1.0f64.sin() as f32, 1.0f64.cos() as f32);
        assert_eq!(floats(&e, result, 4), vec![c, 0.0, 0.0, s]);
    }

    #[test]
    fn euler_key_fill_runs_the_fill_function_per_axis() {
        let mut e = engine();
        e.mem.set_u32(FLOAT_KEY_FILL + 4, 0x0320_0000);
        e.register(0x0320_0000, |_, _| Ret::default());
        let key = e.mem.alloc(0x48);
        e.mem.set_u32(key + 0x30, 0x7000);
        e.mem.set_u32(key + 0x20, 1);
        e.mem.set_u32(key + 0x14, 3);
        e.mem.set_u8(key + 0x2c, 8);
        e.call(0x00a2_9e00, &args![key]);
        assert_eq!(calls_to(&e, 0x0320_0000), vec![vec![0x7000, 3, 8]]);
    }

    #[test]
    fn euler_key_load_reads_the_axis_lists() {
        let mut e = engine();
        e.mem.set_u8(FLOAT_KEY_SIZE + 1, 0x14);
        e.mem.set_u32(FLOAT_KEY_CREATE + 4, 0x0330_0000);
        e.register(0x0330_0000, |_, a| (0x9000 + a[1]).into_ret());
        // Old file: a dropped word 7, then counts 0, 2 (type 1), 0.
        let s = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(s + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut answers = std::collections::VecDeque::from(vec![7u32, 0, 2, 1, 0]);
        e.register_double(0x0300_0000, move |e, a| {
            e.mem.set_u32(a[1], answers.pop_front().unwrap());
            4u32.into_ret()
        });
        e.mem.set_u32(s + 0xd8, 0x0a01_0067);
        let key = e.mem.alloc(0x48);
        e.call(0x00a2_9f00, &args![key, s]);
        assert_eq!(e.mem.u32(key + 0x18), 2);
        assert_eq!(e.mem.u32(key + 0x24), 1);
        assert_eq!(e.mem.u8(key + 0x2d), 0x14);
        assert_eq!(e.mem.u32(key + 0x34), 0x9002);
        assert_eq!(calls_to(&e, 0x0330_0000), vec![vec![s, 2]]);
        assert_eq!(e.mem.u32(input + 4), 20);
    }

    #[test]
    fn create_functions_allocate_and_read_each_key() {
        let mut e = engine();
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        for read in [READ_FLOAT_KEY, READ_POS_KEY, READ_KEY_00A29640] {
            e.register(read, |_, _| Ret::default());
        }
        let s = stream(&mut e, 1, 0x0a01_0068);
        let keys = e.call(0x00a2_a5a0, &args![s, 3u32]).u32();
        assert_eq!(e.mem.u32(keys - 4), 3);
        assert_eq!(
            calls_to(&e, READ_FLOAT_KEY),
            vec![vec![keys, s], vec![keys + 8, s], vec![keys + 16, s]]
        );
        let keys = e.call(0x00a2_a8e0, &args![s, 2u32]).u32();
        assert_eq!(
            calls_to(&e, READ_POS_KEY),
            vec![vec![keys, s], vec![keys + 0x10, s]]
        );
        let keys = e.call(0x00a2_abc0, &args![s, 2u32]).u32();
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![keys, 0x14, 2, 0x00a2_bdb0]]
        );
        assert_eq!(calls_to(&e, READ_KEY_00A29640)[1], vec![keys + 0x14, s]);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![
                vec![MEMORY_MANAGER, 28],
                vec![MEMORY_MANAGER, 36],
                vec![MEMORY_MANAGER, 44]
            ]
        );
        // Euler keys: built through the array constructor with 00a29a70, read by 00a29f00
        // (one axis count each, all 0 here).
        let s = stream(&mut e, 0, 0x0a01_0068);
        e.register_double(0x0300_0000, |e, a| {
            e.mem.set_u32(a[1], 0);
            4u32.into_ret()
        });
        let keys = e.call(0x00a2_a070, &args![s, 1u32]).u32();
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT)[1],
            vec![keys, 0x48, 1, 0x00a2_9a70]
        );
        // TCB position keys: the time and position, then three floats each.
        let s = stream(&mut e, 0, 0x0a01_0068);
        let keys = e.call(0x00a2_bb00, &args![s, 1u32]).u32();
        assert_eq!(calls_to(&e, READ_POS_KEY).last().unwrap(), &vec![keys, s]);
        assert_eq!(
            (
                e.mem.u32(keys + 0x10),
                e.mem.u32(keys + 0x14),
                e.mem.u32(keys + 0x18)
            ),
            (0, 1, 2)
        );
        // A failed allocation: null keys.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a2_a5a0, &args![s, 0u32]).u32(), 0);
    }

    #[test]
    fn delete_function_frees_the_counted_block() {
        let mut e = engine();
        e.call(0x00a2_a600, &args![0x1004u32]);
        e.call(0x00a2_a600, &args![0u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1000]]
        );
    }

    #[test]
    fn linear_interpolations_of_quaternions_values_and_positions() {
        let mut e = engine();
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        put(&mut e, first + 4, &[1.0, 2.0, 4.0, 8.0]);
        put(&mut e, second + 4, &[3.0, 6.0, 0.0, -8.0]);
        let result = e.mem.alloc(0x10);
        e.call(0x00a2_a200, &args![0.25f32, first, second, result]);
        assert_eq!(floats(&e, result, 4), vec![1.5, 3.0, 3.0, 4.0]);
        e.call(0x00a2_a7f0, &args![0.5f32, first, second, result]);
        assert_eq!(floats(&e, result, 3), vec![2.0, 4.0, 2.0]);
        e.register(FLOAT_LERP, |_, a| {
            let f = |i: usize| f64::from(f32::from_bits(a[i]));
            Ret {
                st0: f(0) + 10.0 * f(1) + 100.0 * f(2),
                ..Ret::default()
            }
        });
        e.call(0x00a2_a560, &args![0.5f32, first, second, result]);
        assert_eq!(e.mem.f32(result), 310.5);
    }

    #[test]
    fn spherical_interpolations_pass_the_quaternions() {
        let mut e = engine();
        e.register(QUATERNION_SLERP, |_, a| a[0].into_ret());
        e.register(QUATERNION_SQUAD, |_, a| a[0].into_ret());
        let r = e
            .call(0x00a2_ab90, &args![0.5f32, 0x100u32, 0x200u32, 0x300u32])
            .u32();
        assert_eq!(r, 0x300);
        assert_eq!(
            calls_to(&e, QUATERNION_SLERP),
            vec![vec![0x300, 0.5f32.to_bits(), 0x104, 0x204]]
        );
        e.call(0x00a2_bc60, &args![0.5f32, 0x100u32, 0x200u32, 0x300u32]);
        assert_eq!(
            calls_to(&e, QUATERNION_SQUAD),
            vec![vec![0x300, 0.5f32.to_bits(), 0x104, 0x120, 0x230, 0x204]]
        );
    }

    #[test]
    fn tcb_tangents_follow_tension_continuity_and_bias() {
        let mut e = engine();
        let key = e.mem.alloc(0x4c);
        put(&mut e, key + 4, &[1.0, 2.0, 3.0]);
        let previous = e.mem.alloc(0xc);
        let next = e.mem.alloc(0xc);
        put(&mut e, next, &[3.0, 2.0, 3.0]);
        // Tension, continuity, bias 0: both tangents (d + e) / 2, spans 1.
        e.call(0x00a2_b1d0, &args![key, previous, next, 1.0f32, 1.0f32]);
        assert_eq!(floats(&e, key + 0x1c, 3), vec![1.5, 1.0, 1.5]);
        assert_eq!(floats(&e, key + 0x28, 3), vec![1.5, 1.0, 1.5]);
        // Bias 1: the previous difference doubled on both; spans 1 and 3.
        put(&mut e, key + 0x18, &[1.0]);
        e.call(0x00a2_b1d0, &args![key, previous, next, 1.0f32, 3.0f32]);
        assert_eq!(floats(&e, key + 0x1c, 3), vec![0.5, 1.0, 1.5]);
        assert_eq!(floats(&e, key + 0x28, 3), vec![1.5, 3.0, 4.5]);
    }

    #[test]
    fn tcb_fill_computes_tangents_and_coefficients() {
        let mut e = engine();
        // Two keys at times 0 and 1, positions 0 and (2, 4, 6), all parameters 0.
        let keys = e.mem.alloc(0x4c * 2);
        put(&mut e, keys + 0x4c, &[1.0, 2.0, 4.0, 6.0]);
        e.call(0x00a2_b410, &args![keys, 2u32, 0u32]);
        // The reflections make every tangent the difference (2, 4, 6).
        assert_eq!(floats(&e, keys + 0x1c, 3), vec![2.0, 4.0, 6.0]);
        assert_eq!(floats(&e, keys + 0x28, 3), vec![2.0, 4.0, 6.0]);
        assert_eq!(floats(&e, keys + 0x4c + 0x1c, 3), vec![2.0, 4.0, 6.0]);
        // A = 3 d - (2 outgoing + next incoming) = 0, B = (outgoing + incoming) - 2 d = 0.
        assert_eq!(floats(&e, keys + 0x34, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(floats(&e, keys + 0x40, 3), vec![0.0, 0.0, 0.0]);
        // Three keys: the middle one uses the time spans.
        let keys = e.mem.alloc(0x4c * 3);
        put(&mut e, keys + 0x4c, &[1.0, 1.0, 0.0, 0.0]);
        put(&mut e, keys + 0x98, &[3.0, 3.0, 0.0, 0.0]);
        e.call(0x00a2_b410, &args![keys, 3u32, 0u32]);
        // Middle tangents: (d + e) / 2 = 1.5, scaled by 2*1/3 and 2*2/3.
        assert_eq!(e.mem.f32(keys + 0x4c + 0x1c), 1.0);
        assert_eq!(e.mem.f32(keys + 0x4c + 0x28), 2.0);
        // One key: nothing is touched.
        let one = e.mem.alloc(0x4c);
        e.mem.write(one, &[0xee; 0x4c]);
        e.call(0x00a2_b410, &args![one, 1u32, 0u32]);
        assert_eq!(e.mem.u32(one + 0x1c), 0xeeee_eeee);
    }

    #[test]
    fn scalar_times_quaternion() {
        let mut e = engine();
        e.register(QUATERNION_CONSTRUCT, |_, a| a[0].into_ret());
        let q = e.mem.alloc(0x10);
        put(&mut e, q, &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            e.call(0x00a2_bc90, &args![0x500u32, 0.5f32, q]).u32(),
            0x500
        );
        let words: Vec<f32> = calls_to(&e, QUATERNION_CONSTRUCT)[0][1..]
            .iter()
            .map(|w| f32::from_bits(*w))
            .collect();
        assert_eq!(words, vec![0.5, 1.0, 1.5, 2.0]);
    }
}
