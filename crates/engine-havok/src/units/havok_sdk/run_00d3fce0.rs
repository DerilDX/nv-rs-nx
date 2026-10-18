//! `Havok SDK/run_00d3fce0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d3fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `record`): fills four function words: `+0` = `0x00d3f620`, `+4` =
/// `0x00d3f6a0`, `+8` = `0x00d3fb20`, `+0xc` = `0x00daf8e0`.
pub fn fn_00d3fce0(e: &mut Engine, record: Ptr) {
    let r = record.addr();
    e.mem.set_u32(r, 0x00d3_f620);
    e.mem.set_u32(r + 4, 0x00d3_f6a0);
    e.mem.set_u32(r + 8, 0x00d3_fb20);
    e.mem.set_u32(r + 0xc, 0x00da_f8e0);
}

// Translated from 00d3fd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): builds a 0x14-byte record on
/// the stack (the four function words of `00d3fce0`, the bytes at `+0x10` and `+0x11` zero) and
/// registers it with `00cfbbe0` on `queue` with `(record, 1, 1)`. The two padding bytes at `+0x12`
/// are uninitialised in the exe (zero here).
pub fn fn_00d3fd00(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x10, 0);
        e.mem.set_u8(r + 0x11, 0);
        fn_00d3fce0(e, record);
        e.call(0x00cf_bbe0, &args![queue, record, 1u32, 1u32]);
    });
}

// Translated from 00d3fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words, the first never read: `_unused_1`, `object`, `value`;
/// descriptions, not confirmed): searches the first `n` of the 4-byte slots whose 16-bit word at
/// `object + 2 + 4 * i` is `0xffff` (`n` is the byte at `object + 0x21`) and stores the low 16
/// bits of `value` in it. Answers the slot index reached: the free one, `n` when none is free,
/// 0 when `n` is 0.
pub fn fn_00d3fda0(e: &mut Engine, _unused_1: u32, object: Ptr, value: u32) -> u32 {
    let base = object.addr();
    let count = u32::from(e.mem.u8(base + 0x21));
    let mut index = 0u32;
    if count == 0 {
        return 0;
    }
    let mut slot = base + 2;
    loop {
        if e.mem.u16(slot) == 0xffff {
            e.mem.set_u16(base + index * 4 + 2, value as u16);
            return index;
        }
        index += 1;
        slot += 4;
        if index >= count {
            return index;
        }
    }
}

// Translated from 00d3fdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words, only the third read: `_unused_1`, `_unused_2`, `object`;
/// descriptions, not confirmed): when `object` is not null calls `00db0920` on it (thiscall, no
/// stack words). Answers `object + 0x40`.
pub fn fn_00d3fdf0(e: &mut Engine, _unused_1: u32, _unused_2: u32, object: u32) -> u32 {
    if object != 0 {
        e.call(0x00db_0920, &args![object]);
    }
    object.wrapping_add(0x40)
}

// Translated from 00d3fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, four words `owner`, `object`, `receiver`, `argument`; descriptions, not
/// confirmed): for each of the `n` 4-byte slots whose 16-bit word at `object + 2 + 4 * i` is not
/// `0xffff` (`n` is the byte at `object + 0x21`, read again after each slot; nothing when it is 0)
/// calls slot `0x14` of `receiver` with `(word, argument)`; then clears the byte at
/// `object + 0x21` and the byte at `owner + 2`. Answers `object + 0x40`.
pub fn fn_00d3fe10(e: &mut Engine, owner: u32, object: Ptr, receiver: u32, argument: u32) -> u32 {
    let base = object.addr();
    if e.mem.u8(base + 0x21) != 0 {
        let mut index = 0i32;
        let mut slot = base + 2;
        loop {
            let value = e.mem.u16(slot);
            if value != 0xffff {
                e.vcall(receiver, 0x14, &args![u32::from(value), argument]);
            }
            let count = i32::from(e.mem.u8(base + 0x21));
            index += 1;
            slot += 4;
            if index >= count {
                break;
            }
        }
    }
    e.mem.set_u8(base + 0x21, 0);
    e.mem.set_u8(owner + 2, 0);
    base + 0x40
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_fce0, fn_00d3fce0(Ptr)),
        entry!(0x00d3_fd00, fn_00d3fd00(Ptr)),
        entry!(0x00d3_fda0, fn_00d3fda0(u32, Ptr, u32) -> u32),
        entry!(0x00d3_fdf0, fn_00d3fdf0(u32, u32, u32) -> u32),
        entry!(0x00d3_fe10, fn_00d3fe10(u32, Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d3fce0_fills_the_function_words() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x20);
        e.call(0x00d3_fce0, &args![record]);
        let words: Vec<u32> = (0..5).map(|word| e.mem.u32(record + word * 4)).collect();
        assert_eq!(
            words,
            vec![0x00d3_f620, 0x00d3_f6a0, 0x00d3_fb20, 0x00da_f8e0, 0]
        );
    }

    #[test]
    fn fn_00d3fd00_registers_the_record() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..4 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d3_fd00, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen[0], queue);
        assert_eq!(seen[2..4], [1, 1]);
        assert_eq!(
            seen[4..],
            [0x00d3_f620, 0x00d3_f6a0, 0x00d3_fb20, 0x00da_f8e0, 0, 0]
        );
    }

    #[test]
    fn fn_00d3fda0_stores_into_the_first_free_slot() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        for slot in 0..4 {
            e.mem.set_u16(object + 2 + slot * 4, 7);
        }
        e.mem.set_u8(object + 0x21, 4);
        e.mem.set_u16(object + 2 + 2 * 4, 0xffff);
        e.mem.set_u16(object + 2 + 3 * 4, 0xffff);
        assert_eq!(
            e.call(0x00d3_fda0, &args![0u32, object, 0x1_0042u32]).u32(),
            2
        );
        assert_eq!(e.mem.u16(object + 2 + 2 * 4), 0x0042);
        assert_eq!(e.mem.u16(object + 2 + 3 * 4), 0xffff);
        // The next free one is the fourth; with none free the answer is the count.
        assert_eq!(e.call(0x00d3_fda0, &args![0u32, object, 9u32]).u32(), 3);
        assert_eq!(e.call(0x00d3_fda0, &args![0u32, object, 9u32]).u32(), 4);
        e.mem.set_u8(object + 0x21, 0);
        assert_eq!(e.call(0x00d3_fda0, &args![0u32, object, 9u32]).u32(), 0);
    }

    #[test]
    fn fn_00d3fdf0_calls_00db0920_on_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00db_0920, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_fdf0, &args![0u32, 0u32, 0x1000u32]).u32(),
            0x1040
        );
        assert_eq!(e.call(0x00d3_fdf0, &args![0u32, 0u32, 0u32]).u32(), 0x40);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00db_0920).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x1000]);
    }

    #[test]
    fn fn_00d3fe10_notifies_and_clears() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, 0x0900_00d1);
        let receiver = e.mem.alloc(0x10);
        e.mem.set_u32(receiver, vtable);
        e.register(0x0900_00d1, |_, _| Ret::default());
        let owner = e.mem.alloc(0x10);
        e.mem.set_u8(owner + 2, 5);
        let object = e.mem.alloc(0x60);
        e.mem.set_u8(object + 0x21, 3);
        e.mem.set_u16(object + 2, 11);
        e.mem.set_u16(object + 6, 0xffff);
        e.mem.set_u16(object + 10, 13);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00d3_fe10, &args![owner, object, receiver, 0x99u32])
            .u32();
        assert_eq!(result, object + 0x40);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0900_00d1, vec![receiver, 11, 0x99]),
                (0x0900_00d1, vec![receiver, 13, 0x99])
            ]
        );
        assert_eq!(e.mem.u8(object + 0x21), 0);
        assert_eq!(e.mem.u8(owner + 2), 0);
        // With no slots nothing is called but the bytes are still cleared.
        e.mem.set_u8(owner + 2, 5);
        e.call_log = Some(vec![]);
        e.call(0x00d3_fe10, &args![owner, object, receiver, 0x99u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u8(owner + 2), 0);
    }
}
