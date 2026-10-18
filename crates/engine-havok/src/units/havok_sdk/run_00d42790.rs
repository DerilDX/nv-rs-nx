//! `Havok SDK/run_00d42790` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The last node of the chain that starts at `start` and follows the word at `+0xc` until it is 0.
fn last_in_chain_00d42790(e: &Engine, start: u32) -> u32 {
    let mut node = start;
    loop {
        let next = e.mem.u32(node + 0xc);
        if next == 0 {
            return node;
        }
        node = next;
    }
}

// Translated from 00d42790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, `RET 0xc`: three stack words `first`, `second`, `third`; no map name,
/// class unconfirmed): `+6` (`u16`) = 1, `+8` = `third`, the vtable word `+0` = `0x010c_f238`,
/// `+0xc` = the last node of the chain from `first` (following the word at `+0xc`), `+0x10` = the
/// last node of the chain from `second`, `+0x1c` = the word at `+0xc` of `*first`, `+0x20` = the
/// word at `+0xc` of `*second`. Returns `this`.
pub fn fn_00d42790(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, third);
    e.mem.set_u32(me, 0x010c_f238);
    let first_last = last_in_chain_00d42790(e, first);
    e.mem.set_u32(me + 0xc, first_last);
    let second_last = last_in_chain_00d42790(e, second);
    e.mem.set_u32(me + 0x10, second_last);
    let first_head = e.mem.u32(first);
    let word = e.mem.u32(first_head + 0xc);
    e.mem.set_u32(me + 0x1c, word);
    let second_head = e.mem.u32(second);
    let word = e.mem.u32(second_head + 0xc);
    e.mem.set_u32(me + 0x20, word);
    this
}

// Translated from 00d42960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers one 0x14-byte
/// record (function words `+0` = `0x00d42800`, `+4` = `0x00d42730`, `+8` = `0x00d42760`, `+0xc` =
/// `0x00d42780`, bytes `+0x10` = 0 and `+0x11` = 1) with `00cfbbe0` on `queue` twice: with
/// `(record, 0x1d, 0xffffffff)` and with `(record, 0xffffffff, 0x1d)`. The padding bytes at `+0x12`
/// are uninitialised in the exe (zero here).
pub fn fn_00d42960(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x10, 0);
        e.mem.set_u32(r, 0x00d4_2800);
        e.mem.set_u32(r + 4, 0x00d4_2730);
        e.mem.set_u32(r + 8, 0x00d4_2760);
        e.mem.set_u32(r + 0xc, 0x00d4_2780);
        e.mem.set_u8(r + 0x11, 1);
        e.call(0x00cf_bbe0, &args![queue, record, 0x1du32, 0xffff_ffffu32]);
        e.call(0x00cf_bbe0, &args![queue, record, 0xffff_ffffu32, 0x1du32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_2790, fn_00d42790(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d4_2960, fn_00d42960(Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    /// A chain of `length` nodes (`+0xc` links, the last 0) and the first node.
    fn chain(e: &mut Engine, length: u32) -> Vec<u32> {
        let nodes: Vec<u32> = (0..length).map(|_| e.mem.alloc(0x20)).collect();
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 0xc, pair[1]);
        }
        nodes
    }

    #[test]
    fn fn_00d42790_finds_the_ends_of_both_chains() {
        let mut e = Engine::new();
        let first_chain = chain(&mut e, 3);
        let second_chain = chain(&mut e, 1);
        // `*first` and `*second` are objects whose `+0xc` words are copied.
        let first_head = e.mem.alloc(0x20);
        e.mem.set_u32(first_head + 0xc, 0x1111);
        let second_head = e.mem.alloc(0x20);
        e.mem.set_u32(second_head + 0xc, 0x2222);
        e.mem.set_u32(first_chain[0], first_head);
        e.mem.set_u32(second_chain[0], second_head);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x5555);
        let result = e
            .call(
                0x00d4_2790,
                &args![this, first_chain[0], second_chain[0], 0x9999u32],
            )
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x9999);
        assert_eq!(e.mem.u32(this), 0x010c_f238);
        assert_eq!(e.mem.u32(this + 0xc), first_chain[2]);
        assert_eq!(e.mem.u32(this + 0x10), second_chain[0]);
        assert_eq!(e.mem.u32(this + 0x14), 0x5555);
        assert_eq!(e.mem.u32(this + 0x1c), 0x1111);
        assert_eq!(e.mem.u32(this + 0x20), 0x2222);
    }

    #[test]
    fn fn_00d42960_registers_the_record_twice() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<Vec<u32>>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..5 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            log.borrow_mut().push(words);
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_2960, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        let record = [
            0x00d4_2800,
            0x00d4_2730,
            0x00d4_2760,
            0x00d4_2780,
            0x0000_0100,
        ];
        assert_eq!(seen[0][2..4], [0x1d, 0xffff_ffff]);
        assert_eq!(seen[0][4..], record);
        assert_eq!(seen[1][2..4], [0xffff_ffff, 0x1d]);
        assert_eq!(seen[1][4..], record);
    }
}
