//! `NiXenonRenderer/run_00e88fc0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00e895f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem; no arguments):
/// allocates 0x18 bytes from the memory manager, zeroes the first four words, sets the
/// words at `+0x10` and `+0x14` to 8 and keeps the block in `0126f9a0` (null when the
/// allocation fails).
pub fn fn_00e895f0(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f9a0, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f9a0, block);
}

/// The pool this item goes back to (`0126f9a0`) and `NiTPrimitiveSet<..>::AddUnique`
/// (`00e7ec00`, `thiscall` on the set, the address of the element; the engine map names it
/// `NiControllerSequence::SetState`).
const ITEM_POOL_0126F9A0: u32 = 0x0126_f9a0;
const POOL_ADD_UNIQUE_00E7EC00: u32 = 0x00e7_ec00;
/// The words and bytes `00e89670` clears, in its order.
const CLEARED_WORDS_00E89670: [u32; 38] = [
    0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50,
    0x54, 0x58, 0x5c, 0x60, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x90, 0x94, 0x98, 0x9c, 0xa0, 0xa4,
    0xb0, 0x34, 0x4, 0x88, 0x64, 0xac,
];
const CLEARED_BYTES_00E89670: [u32; 3] = [0x84, 0xa8, 0xb4];

// Translated from 00e89670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): an item's release to its pool, `cdecl` with one
/// word (the item; no null check). When the item's first byte is set (a pool item): clears the
/// words +0xc to +0x30, +0x3c to +0x60, +0x6c to +0x80, +0x90 to +0xa4, +0xb0, +0x34, +4, +0x88,
/// +0x64 and +0xac and the bytes +0x84, +0xa8, +0xb4, then gives the item back to the pool at
/// `0126f9a0` (`00e7ec00`, with the item in the argument's own stack word). Otherwise nothing.
pub fn fn_00e89670(e: &mut Engine, item: Ptr) {
    let item = item.addr();
    if e.mem.u8(item) == 0 {
        return;
    }
    for offset in CLEARED_WORDS_00E89670 {
        e.mem.set_u32(item + offset, 0);
    }
    for offset in CLEARED_BYTES_00E89670 {
        e.mem.set_u8(item + offset, 0);
    }
    let pool = e.global::<u32>(ITEM_POOL_0126F9A0);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
    });
}

// Translated from 00e891f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem), `thiscall` with no
/// stack words (called from `00e7ecd0`): for each of 8 steps clears the byte at `+0x2c + i`
/// and the byte at `+0x5c + i`, then the words at `+0x34` and `+4`.
pub fn fn_00e891f0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for i in 0..8u32 {
        e.mem.set_u8(this + 0x2c + i, 0);
        e.mem.set_u8(this + 0x5c + i, 0);
    }
    e.mem.set_u32(this + 0x34, 0);
    e.mem.set_u32(this + 4, 0);
}

// Translated from 00e89220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem), `thiscall` with no
/// stack words (called from `00e7ecd0`): for each of 5 steps clears the bytes at `+0x80 + i`,
/// `+0xa4 + i` and `+0xb0 + i`, then the words at `+0x88`, `+0x64` and `+0xac`.
pub fn fn_00e89220(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for i in 0..5u32 {
        e.mem.set_u8(this + 0x80 + i, 0);
        e.mem.set_u8(this + 0xa4 + i, 0);
        e.mem.set_u8(this + 0xb0 + i, 0);
    }
    e.mem.set_u32(this + 0x88, 0);
    e.mem.set_u32(this + 0x64, 0);
    e.mem.set_u32(this + 0xac, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_95f0, fn_00e895f0()),
        entry!(0x00e8_9670, fn_00e89670(Ptr)),
        entry!(0x00e8_91f0, fn_00e891f0(Ptr)),
        entry!(0x00e8_9220, fn_00e89220(Ptr)),
    ]
}

#[cfg(test)]
mod clear_tests {
    use super::*;

    /// A 0xc0-byte block of 0xaa; the call clears exactly the bytes in `cleared`.
    fn check(addr: u32, cleared: &[std::ops::Range<u32>]) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xaa; 0xc0]);
        e.call(addr, &args![this]);
        for offset in 0..0xc0u32 {
            let want = if cleared.iter().any(|r| r.contains(&offset)) {
                0
            } else {
                0xaa
            };
            assert_eq!(e.mem.u8(this + offset), want, "offset {offset:#x}");
        }
    }

    #[test]
    fn fn_00e891f0_clears_its_bytes_and_words() {
        check(0x00e8_91f0, &[0x2c..0x38, 0x5c..0x64, 4..8]);
    }

    #[test]
    fn fn_00e89220_clears_its_bytes_and_words() {
        check(
            0x00e8_9220,
            &[
                0x80..0x85,
                0xa4..0xa9,
                0xb0..0xb5,
                0x88..0x8c,
                0x64..0x68,
                0xac..0xb0,
            ],
        );
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_or_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00e8_95f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f9a0), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e8_95f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f9a0), 0);
    }
}

#[cfg(test)]
mod item_release_tests {
    use super::*;

    #[test]
    fn fn_00e89670_clears_a_pool_item_and_gives_it_back() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.set_global(ITEM_POOL_0126F9A0, 0x0e00_0000u32);
        // The double notes the element its slot holds.
        let seen = e.mem.alloc(4);
        e.register_double(POOL_ADD_UNIQUE_00E7EC00, move |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(seen, item);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let item = e.mem.alloc(0xb8);
        e.mem.write(item, &[0xcc; 0xb8]);
        e.call(0x00e8_9670, &args![item]);
        let mut expected = vec![0xccu8; 0xb8];
        for offset in CLEARED_WORDS_00E89670.iter() {
            expected[*offset as usize..*offset as usize + 4].copy_from_slice(&[0; 4]);
        }
        for offset in CLEARED_BYTES_00E89670 {
            expected[offset as usize] = 0;
        }
        assert_eq!(e.mem.bytes(item, 0xb8), expected);
        assert_eq!(e.mem.u32(seen), item);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, POOL_ADD_UNIQUE_00E7EC00);
        assert_eq!(log[1].1[0], 0x0e00_0000);
        // An item whose first byte is clear is left alone.
        let other = e.mem.alloc(0xb8);
        e.mem.write(other + 4, &[0xcc; 4]);
        e.call(0x00e8_9670, &args![other]);
        assert_eq!(e.mem.u32(other + 4), 0xcccc_cccc);
    }
}
