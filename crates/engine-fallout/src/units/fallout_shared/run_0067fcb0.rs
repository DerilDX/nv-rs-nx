//! `fallout shared/run_0067fcb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A small constructor, `__thiscall`, that returns `this`: it sets the bytes
/// at `+0`, `+1`, `+2` and `+3` to `0xff`, `0xff`, `0x00` and `0xff` and the
/// word at `+4` to zero. `TESPackage`'s constructor (`006707c0`) runs it on the
/// member at `+0x38`. It is not itself a form constructor, and the engine map
/// gives it no name, so the type of the member is unconfirmed.
pub fn fn_0067fcb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 0xff);
    e.mem.set_u8(base + 1, 0xff);
    e.mem.set_u8(base + 2, 0x00);
    e.mem.set_u8(base + 3, 0xff);
    e.mem.set_u32(base + 4, 0);
    this
}

// Translated from 0067fcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of the 8-byte value: copies the bytes +0..+3 one by one and
/// the word at +4 from `source` (one stack word). Returns `this`.
pub fn fn_0067fcf0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    for offset in 0..4 {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    let word = e.mem.u32(from + 4);
    e.mem.set_u32(to + 4, word);
    this
}

// Translated from 0067fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills this value from `source` (one stack word; nothing when it is
/// null): +1 from `00678cc0`, then `005407b0` with
/// `PackageLocation::GetLocType` (`00678ca0`, engine map), +2 from
/// `00678ce0`, +3 from `0086a460` (both `AL`, zero-extended), then
/// `006ecd40` with `00726070` (each getter with `ECX` = `source`, each
/// setter with `ECX` = `this`).
pub fn fn_0067fd40(e: &mut Engine, this: Ptr, source: Ptr) {
    if source.is_null() {
        return;
    }
    let value = e.call(0x0067_8cc0, &args![source]).u32();
    fn_0067fdb0(e, this, value as u8);
    let location_type = e.call(0x0067_8ca0, &args![source]).u32();
    e.call(0x0054_07b0, &args![this, location_type]);
    let value = e.call(0x0067_8ce0, &args![source]).u8();
    fn_0067fdd0(e, this, value);
    let value = e.call(0x0086_a460, &args![source]).u8();
    fn_0067fdf0(e, this, value);
    let value = e.call(0x0072_6070, &args![source]).u32();
    e.call(0x006e_cd40, &args![this, value]);
}

// Translated from 0067fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` (one stack word) at `this + 1`.
pub fn fn_0067fdb0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 1, value);
}

// Translated from 0067fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` (one stack word) at `this + 2`.
pub fn fn_0067fdd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 2, value);
}

// Translated from 0067fdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` (one stack word) at `this + 3`.
pub fn fn_0067fdf0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 3, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_fcb0, fn_0067fcb0(Ptr) -> Ptr),
        entry!(0x0067_fcf0, fn_0067fcf0(Ptr, Ptr) -> Ptr),
        entry!(0x0067_fd40, fn_0067fd40(Ptr, Ptr)),
        entry!(0x0067_fdb0, fn_0067fdb0(Ptr, u8)),
        entry!(0x0067_fdd0, fn_0067fdd0(Ptr, u8)),
        entry!(0x0067_fdf0, fn_0067fdf0(Ptr, u8)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_0067fcf0_copies_the_eight_bytes() {
        let mut e = Engine::new();
        let to = e.mem.alloc(12);
        let from = e.mem.alloc(12);
        e.mem.write(from, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        e.mem.set_u8(to + 8, 0xee);
        assert_eq!(e.call(0x0067_fcf0, &args![to, from]).u32(), to);
        assert_eq!(e.mem.bytes(to, 9), vec![1, 2, 3, 4, 5, 6, 7, 8, 0xee]);
    }

    #[test]
    fn the_byte_setters_store_their_low_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x0067_fdb0, &args![this, 0x1234u32]);
        e.call(0x0067_fdd0, &args![this, 0x56u32]);
        e.call(0x0067_fdf0, &args![this, 0xffff_ff78u32]);
        assert_eq!(e.mem.bytes(this, 5), vec![0, 0x34, 0x56, 0x78, 0]);
    }

    #[test]
    fn fn_0067fd40_copies_the_getters_into_the_value() {
        let mut e = Engine::new();
        e.register(0x0067_8cc0, |_, _| 0x0000_0111u32.into_ret());
        e.register(0x0067_8ca0, |_, _| 0x22u32.into_ret());
        e.register(0x0067_8ce0, |_, _| 0xaaaa_aa33u32.into_ret());
        e.register(0x0086_a460, |_, _| 0x44u32.into_ret());
        e.register(0x0072_6070, |_, _| 0x5555u32.into_ret());
        e.register(0x0054_07b0, |_, _| Ret::default());
        e.register(0x006e_cd40, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        let source = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0067_fd40, &args![this, source]);
        assert_eq!(e.mem.bytes(this, 4), vec![0, 0x11, 0x33, 0x44]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0054_07b0, vec![this, 0x22])));
        assert!(log.contains(&(0x006e_cd40, vec![this, 0x5555])));
        assert!(log.contains(&(0x0067_8cc0, vec![source])));

        // A null source changes nothing and calls nothing.
        e.call_log = Some(vec![]);
        e.call(0x0067_fd40, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self};

    #[test]
    fn fn_0067fcb0_sets_the_start_values() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0067_fcb0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[],
            stores: &[
                (0x0, 1, 0xff),
                (0x1, 1, 0xff),
                (0x2, 1, 0x0),
                (0x3, 1, 0xff),
                (0x4, 4, 0x0),
            ],
        });
    }
}
