//! `Havok SDK/run_00cf2180` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Mask applied to the first vector (a 16-byte constant in `.rdata`).
const MASK_010C4C10: u32 = 0x010c_4c10;

/// Four words at `addr`.
fn read_words_00cf2280(e: &Engine, addr: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(addr + offset))
}

fn write_words_00cf2280(e: &mut Engine, addr: u32, words: [u32; 4]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(addr + 4 * i as u32, *word);
    }
}

// Translated from 00cf2280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, two words (`RET 8`), 16-byte vectors: fills the
/// 0x50 bytes at `this` with `first & mask` (twice, at `+0` and `+0x10`,
/// `mask` being the 16 bytes at `010c4c10`), `second` (twice, at `+0x20`
/// and `+0x30`) and zeros (`+0x40`). Each source is read again for its
/// second copy, as the code does.
pub fn fn_00cf2280(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let mask = read_words_00cf2280(e, MASK_010C4C10);
    let value = read_words_00cf2280(e, first.addr());
    write_words_00cf2280(e, this.addr(), [0, 1, 2, 3].map(|i| value[i] & mask[i]));
    let value = read_words_00cf2280(e, first.addr());
    let mask = read_words_00cf2280(e, MASK_010C4C10);
    write_words_00cf2280(
        e,
        this.addr() + 0x10,
        [0, 1, 2, 3].map(|i| value[i] & mask[i]),
    );
    let value = read_words_00cf2280(e, second.addr());
    write_words_00cf2280(e, this.addr() + 0x20, value);
    let value = read_words_00cf2280(e, second.addr());
    write_words_00cf2280(e, this.addr() + 0x30, value);
    write_words_00cf2280(e, this.addr() + 0x40, [0; 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf2280, fn_00cf2280(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cf2280_masks_the_first_vector_and_copies_the_second() {
        let mut e = Engine::new();
        e.map(0x010c_4000, 0x1000);
        for (i, word) in [0xffff_ffffu32, 0xffff_ffff, 0xffff_ffff, 0]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(MASK_010C4C10 + 4 * i as u32, *word);
        }
        let first = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(first + 4 * i, 0x1111_1111 * (i + 1));
            e.mem.set_u32(second + 4 * i, 0xa000_0000 + i);
        }
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x44, 0x1234);
        e.call(
            0x00cf_2280,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(first),
                Ptr::<()>::new(second)
            ],
        );
        let words = |e: &Engine, offset: u32| [0, 4, 8, 12].map(|o| e.mem.u32(this + offset + o));
        let masked = [0x1111_1111, 0x2222_2222, 0x3333_3333, 0];
        assert_eq!(words(&e, 0), masked);
        assert_eq!(words(&e, 0x10), masked);
        let copied = [0xa000_0000, 0xa000_0001, 0xa000_0002, 0xa000_0003];
        assert_eq!(words(&e, 0x20), copied);
        assert_eq!(words(&e, 0x30), copied);
        assert_eq!(words(&e, 0x40), [0; 4]);
    }
}
