//! `Havok SDK/run_00d39840` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d39840 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers a group of six
/// functions with `00cfbd60` on `queue`, with `(record, 5, 3)`. The record, 0x40 bytes on the stack,
/// holds the function words `+0` = `0x00d390a0`, `+4` = `0x00d39130`, `+8` = `0x00d390e0`, `+0xc` =
/// `0x00d39030`, `+0x10` = `0x00d39060`, `+0x14` = `0x00d39090`, `+0x28` = `0x00d39150`, `+0x2c` =
/// `0x00d39390`, zero words at `+0x18` .. `+0x24` and zero bytes at `+0x30` .. `+0x32`; the rest is
/// uninitialised in the exe (zero here).
pub fn fn_00d39840(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x31, 0);
        e.mem.set_u8(r + 0x32, 0);
        for offset in [0x18u32, 0x1c, 0x20, 0x24] {
            e.mem.set_u32(r + offset, 0);
        }
        e.mem.set_u8(r + 0x30, 0);
        for (offset, function) in [
            (0u32, 0x00d3_90a0u32),
            (0x2c, 0x00d3_9390),
            (0x28, 0x00d3_9150),
            (0x08, 0x00d3_90e0),
            (0x0c, 0x00d3_9030),
            (0x10, 0x00d3_9060),
            (0x14, 0x00d3_9090),
            (0x04, 0x00d3_9130),
        ] {
            e.mem.set_u32(r + offset, function);
        }
        e.call(0x00cf_bd60, &args![queue, record, 5u32, 3u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d3_9840, fn_00d39840(Ptr))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d39840_registers_the_group() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bd60, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..0x0c {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            for offset in [0x30, 0x31, 0x32] {
                words.push(u32::from(e.mem.u8(a[1] + offset)));
            }
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d3_9840, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen[0], queue);
        assert_eq!(seen[2..4], [5, 3]);
        assert_eq!(
            seen[4..],
            [
                0x00d3_90a0,
                0x00d3_9130,
                0x00d3_90e0,
                0x00d3_9030,
                0x00d3_9060,
                0x00d3_9090,
                0,
                0,
                0,
                0,
                0x00d3_9150,
                0x00d3_9390,
                0,
                0,
                0
            ]
        );
    }
}
