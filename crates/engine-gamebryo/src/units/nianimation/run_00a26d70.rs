//! `NiAnimation/run_00a26d70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a26df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two stack words): reads a 4-byte value from the
/// stream's input stream into `destination`. Calls the input stream's read
/// function as `function(input, destination, 4, &count, 1)` with `count` a local set
/// to 4, and adds the byte count it returns to the input stream's position.
pub fn fn_00a26df0(e: &mut Engine, stream: Ptr<NiStream>, destination: Ptr) {
    let input = e.get(stream, NiStream::m_pkIstr);
    let input = Ptr::<NiBinaryStream>::new(input);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, destination, 4u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

// Translated from 00a26e20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3654` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a26e20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3654) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d20, 0x0);
    e.mem.set_u32(0x011f_3c00, 0x0);
    e.mem.set_u32(0x011f_3b70, 0x0);
    e.mem.set_u32(0x011f_3810, 0x0);
    e.mem.set_u8(0x011f_3654, 0x1);
    e.mem.set_u32(0x011f_3db0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a50, 0xa29680);
    e.mem.set_u32(0x011f_39c0, 0xa29680);
    e.mem.set_u32(0x011f_3930, 0xe7cd40);
    e.mem.set_u32(0x011f_38a0, 0xa29680);
    e.mem.set_u8(0x011f_3758, 0x8);
    e.mem.set_u32(0x011f_3c90, 0xa29680);
    e.mem.set_u32(0x011f_3ae0, 0xa26b00);
    e.mem.set_u32(0x011f_3780, 0xa29680);
    1
}

// Translated from 00a273a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3655` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a273a0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3655) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d28, 0x0);
    e.mem.set_u32(0x011f_3c08, 0x0);
    e.mem.set_u32(0x011f_3b78, 0x0);
    e.mem.set_u8(0x011f_3655, 0x1);
    e.mem.set_u32(0x011f_3db8, 0xa27250);
    e.mem.set_u32(0x011f_3a58, 0xa27320);
    e.mem.set_u32(0x011f_39c8, 0xa26f80);
    e.mem.set_u32(0x011f_3938, 0xa27040);
    e.mem.set_u32(0x011f_38a8, 0xa2a600);
    e.mem.set_u8(0x011f_375a, 0x10);
    e.mem.set_u32(0x011f_3c98, 0xa26fe0);
    e.mem.set_u32(0x011f_3ae8, 0xa26f30);
    e.mem.set_u32(0x011f_3788, 0xa29680);
    e.mem.set_u32(0x011f_3818, 0xa27080);
    1
}

/// `00a3da20` (`thiscall` on the key, the stream): reads the key's 4-byte time.
const READ_KEY_TIME: u32 = 0x00a3_da20;
/// `NiColor::LoadBinary` (`thiscall`, the stream): three floats read in place.
const READ_THREE_FLOATS: u32 = 0x00a5_9be0;
/// `00a42070` (`cdecl`, five floats; answers in `ST0`): the interpolation of two keys'
/// values and tangents at a time.
const INTERPOLATE_VALUE: u32 = 0x00a4_2070;
/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

/// Reads 4 bytes from the stream's input into `destination`, the byte-count word at `count`
/// set to 4 first.
fn read_word(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
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

// Translated from 00a26eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a key, the stream): the key's time (`00a3da20`),
/// then its 4-byte value at +4 (the stream argument's stack word is the byte-count word).
pub fn fn_00a26eb0(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(READ_KEY_TIME, &args![this, stream]);
    e.with_stack(4, |e, count| {
        read_word(e, stream, this.addr() + 4, count.addr());
    });
}

// Translated from 00a26fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, first key, second key, result)): stores at
/// `result` the float `00a42070` answers for `time`, the first key's floats at +4 and +0xc
/// and the second key's floats at +4 and +8 (each copied through a `float` stack word).
pub fn fn_00a26fe0(e: &mut Engine, time: f32, first: Ptr, second: Ptr, result: Ptr) {
    let copy = |e: &Engine, at: u32| x87_store32(x87_load32(e.mem.u32(at)));
    let second_8 = copy(e, second.addr() + 8);
    let second_4 = copy(e, second.addr() + 4);
    let first_c = copy(e, first.addr() + 0xc);
    let first_4 = copy(e, first.addr() + 4);
    let words = [
        x87_store32(x87_load32(time.to_bits())),
        x87_store32(x87_load32(first_4)),
        x87_store32(x87_load32(first_c)),
        x87_store32(x87_load32(second_4)),
        x87_store32(x87_load32(second_8)),
    ];
    let value = e.call(INTERPOLATE_VALUE, &words).f64();
    e.mem.set_u32(result.addr(), x87_store32(value));
}

// Translated from 00a27250 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, (stream, count)):
/// makes `count` keys of 16 bytes from the memory manager (`count * 16 + 4` bytes,
/// saturating, the count in the first word; null when the allocation fails) and reads each:
/// its time and value (`00a26eb0`), then two more 4-byte floats at +8 and +0xc. Answers the
/// keys.
pub fn fn_00a27250(e: &mut Engine, stream: Ptr<NiStream>, count: u32) -> u32 {
    let bytes = count
        .checked_mul(0x10)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    let keys = if block != 0 {
        e.mem.set_u32(block, count);
        block + 4
    } else {
        0
    };
    e.with_stack(4, |e, local| {
        let mut key = keys;
        for _ in 0..count {
            fn_00a26eb0(e, Ptr::new(key), stream);
            read_word(e, stream, key.wrapping_add(8), local.addr());
            read_word(e, stream, key.wrapping_add(0xc), local.addr());
            key = key.wrapping_add(0x10);
        }
    });
    keys
}

// Translated from 00a27430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a key, the stream): the key's time (`00a3da20`),
/// then three floats at +4 (`00a59be0`).
pub fn fn_00a27430(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(READ_KEY_TIME, &args![this, stream]);
    e.call(READ_THREE_FLOATS, &args![this.addr() + 4, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_6eb0, fn_00a26eb0(Ptr, Ptr<NiStream>)),
        entry!(0x00a2_6fe0, fn_00a26fe0(f32, Ptr, Ptr, Ptr)),
        entry!(0x00a2_7250, fn_00a27250(Ptr<NiStream>, u32) -> u32),
        entry!(0x00a2_7430, fn_00a27430(Ptr, Ptr<NiStream>)),
        entry!(0x00a2_6df0, fn_00a26df0(Ptr<NiStream>, Ptr)),
        entry!(0x00a26e20, fn_00a26e20() -> u32),
        entry!(0x00a273a0, fn_00a273a0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 0x00aa_2e60;

    #[test]
    fn fn_00a26df0_reads_four_bytes_through_the_input_stream() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 4, 10);
        e.mem.set_u32(input.addr() + 8, READ);
        let destination = e.mem.alloc(8);
        e.register(READ, |e, a| {
            // (input, destination, 4, &count, 1) with count set to 4
            assert_eq!(a[2], 4);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x1234_5678);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2_6df0, &args![stream, destination]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1[..3], [input.addr(), destination, 4]);
        assert_eq!(e.mem.u32(destination), 0x1234_5678);
        assert_eq!(e.mem.u32(input.addr() + 4), 14);
    }

    #[test]
    fn fn_00a26df0_adds_a_short_read_to_the_position() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 8, READ);
        e.register(READ, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let destination = e.mem.alloc(8);
        e.call(0x00a2_6df0, &args![stream, destination]);
        assert_eq!(e.mem.u32(input.addr() + 4), 2);
    }
}

#[cfg(test)]
mod key_tests {
    use super::*;

    const READ: u32 = 0x0300_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A stream whose input answers 4-byte words counting up from `first`.
    fn stream(e: &mut Engine, first: u32) -> Ptr<NiStream> {
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, READ);
        let mut next = first;
        e.register_double(READ, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        stream
    }

    #[test]
    fn fn_00a26eb0_reads_the_time_then_the_value() {
        let mut e = Engine::new();
        e.register(READ_KEY_TIME, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            Ret::default()
        });
        let stream = stream(&mut e, 0x10);
        let key = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00a2_6eb0, &args![key, stream]);
        assert_eq!(calls_to(&e, READ_KEY_TIME), vec![vec![key, stream.addr()]]);
        assert_eq!((e.mem.u32(key), e.mem.u32(key + 4)), (0x77, 0x10));
    }

    #[test]
    fn fn_00a26fe0_passes_the_key_floats_and_stores_the_result() {
        let mut e = Engine::new();
        e.register(INTERPOLATE_VALUE, |_, a| {
            let f = |i: usize| f64::from(f32::from_bits(a[i]));
            Ret {
                st0: f(0) + 10.0 * f(1) + 100.0 * f(2) + 1000.0 * f(3) + 10000.0 * f(4),
                ..Ret::default()
            }
        });
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for (key, values) in [
            (first, [9.0f32, 1.0, 9.0, 2.0]),
            (second, [9.0, 3.0, 4.0, 9.0]),
        ] {
            for (i, v) in values.iter().enumerate() {
                e.mem.set_f32(key + 4 * i as u32, *v);
            }
        }
        let result = e.mem.alloc(4);
        e.call(0x00a2_6fe0, &args![0.5f32, first, second, result]);
        assert_eq!(e.mem.f32(result), 43210.5);
    }

    #[test]
    fn fn_00a27250_makes_and_reads_the_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(READ_KEY_TIME, |e, a| {
            e.mem.set_u32(a[0], 0xaa);
            Ret::default()
        });
        let stream = stream(&mut e, 1);
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_7250, &args![stream, 2u32]).u32();
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x24]]
        );
        assert_eq!(e.mem.u32(keys - 4), 2);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(keys + 4 * i)).collect();
        assert_eq!(words, vec![0xaa, 1, 2, 3, 0xaa, 4, 5, 6]);
        // A failed allocation with no keys answers null.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a2_7250, &args![stream, 0u32]).u32(), 0);
    }

    #[test]
    fn fn_00a27430_reads_the_time_then_three_floats() {
        let mut e = Engine::new();
        e.register(READ_KEY_TIME, |_, _| Ret::default());
        e.register(READ_THREE_FLOATS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_7430, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls_to(&e, READ_KEY_TIME), vec![vec![0x5000, 0x6000]]);
        assert_eq!(calls_to(&e, READ_THREE_FLOATS), vec![vec![0x5004, 0x6000]]);
    }
}
