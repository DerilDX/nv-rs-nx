//! `fallout shared/actorvalue.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066fe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066fe40 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x59 (0xc4 bytes); `ECX` is the object,
/// `id` the actor value index and `name` the name string pointer (`RET 8`). The engine map gives it no name; the RTTI name
/// of the vtable it stores at `+0` (`0x0106_7a2c`) is `ActorValueInfo`, so this is the constructor of `ActorValueInfo`.
/// It runs the base and component constructors by address, stores the four vtable pointers and registers the form type
/// (`004f15a0`). When `id` is not -1 it gives the form the form ID `0066fdf0(id)` (`00485c10(this, formID, 0)`). It keeps
/// `name` at `+0x38`; when `name` is not null it formats `"AV%s"` (`010679d0`) into a 0x64-byte stack buffer
/// (`00406d00`) and passes the buffer to `00401290`. It assigns the empty string at `01011584` to the string at `+0x3c`
/// (`00438390`), clears `+0x44`, `+0x4c..+0x58` and `+0x98`, fills the fifteen words from `+0x5c` with 0x4d and zeroes the
/// ten words from `+0x9c`. The exception-unwinding frame and the security-cookie check are not translated.
pub fn fn_0066fe40(e: &mut Engine, this: Ptr, id: u32, name: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0040_2c20, &args![base + 0x2c]);
    e.mem.set_u32(base, 0x0106_7a2c);
    e.mem.set_u32(base + 0x18, 0x0106_7a14);
    e.mem.set_u32(base + 0x24, 0x0106_79fc);
    e.mem.set_u32(base + 0x2c, 0x0106_79dc);
    e.call(0x0040_37b0, &args![base + 0x3c]);
    e.call(0x004f_15a0, &args![this, 0x59u32]);
    if id != 0xffff_ffff {
        let form_id = e.call(0x0066_fdf0, &args![id]).u32();
        e.call(0x0048_5c10, &args![this, form_id, 0u32]);
    }
    e.mem.set_u32(base + 0x38, name);
    if name != 0 {
        e.with_stack(0x64, |e, buffer| {
            e.call(0x0040_6d00, &args![buffer, 0x64u32, 0x0106_79d0u32, name]);
            e.call(0x0040_1290, &args![this, buffer]);
        });
    }
    e.call(0x0043_8390, &args![base + 0x3c, 0x0101_1584u32]);
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x4c, 0);
    e.mem.set_u32(base + 0x50, 0);
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x58, 0);
    for i in 0..15 {
        e.mem.set_u32(base + 0x5c + i * 4, 0x4d);
    }
    e.mem.set_u32(base + 0x98, 0);
    for i in 0..10 {
        e.mem.set_u32(base + 0x9c + i * 4, 0);
    }
    this
}

// Translated from 0066eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name; `tesobjectweap.rs` calls it as a
/// resistance validity check (unconfirmed here). `cdecl`, one `uint` (an
/// actor value index): true for 0x12, 0x13, 0x14, 0x34 and 0x3a to 0x3d (a
/// jump table over `index - 0x12`, 44 entries), false for every other value.
pub fn fn_0066eba0(_e: &mut Engine, index: u32) -> bool {
    matches!(index, 0x12..=0x14 | 0x34 | 0x3a..=0x3d)
}

// Translated from 0066ef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `actorvalue.cpp`): clears the first 0x4d words (0x134
/// bytes) of `this`, signed loop. Returns `this`.
pub fn fn_0066ef90(e: &mut Engine, this: Ptr) -> Ptr {
    for index in 0..0x4d_u32 {
        e.mem.set_u32(this.addr() + index * 4, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_fe40, fn_0066fe40(Ptr, u32, u32) -> Ptr),
        entry!(0x0066_eba0, fn_0066eba0(u32) -> bool),
        entry!(0x0066_ef90, fn_0066ef90(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    const CALLEES: [u32; 8] = [
        0x0048_3370,
        0x0040_2d00,
        0x0048_2c50,
        0x0040_2c20,
        0x0040_37b0,
        0x004f_15a0,
        0x0048_5c10,
        0x0043_8390,
    ];

    /// Call log, object bytes and return value of one constructor run.
    type CtorRun = (Vec<(u32, Vec<u32>)>, Vec<u8>, u32);

    fn build(id: u32, name: u32) -> CtorRun {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.mem.write(this, &[0xcd; 0xc4]);
        for a in CALLEES {
            e.register(a, |_, _| Ret::default());
        }
        e.register(0x0040_6d00, |_, _| Ret::default());
        e.register(0x0040_1290, |_, _| Ret::default());
        e.register(0x0066_fdf0, |_, a| Ret {
            eax: a[0] + 1000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let ret = e.call(0x0066_fe40, &args![this, id, name]).u32();
        assert_eq!(ret, this);
        let log = e.call_log.take().unwrap();
        (
            log.into_iter().skip(1).collect(),
            e.mem.bytes(this, 0xc4),
            this,
        )
    }

    fn expected(name: u32) -> Vec<u8> {
        let mut b = vec![0xcd_u8; 0xc4];
        let mut put = |off: usize, v: u32| b[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 0x0106_7a2c);
        put(0x18, 0x0106_7a14);
        put(0x24, 0x0106_79fc);
        put(0x2c, 0x0106_79dc);
        put(0x38, name);
        for off in [0x44, 0x4c, 0x50, 0x54, 0x58, 0x98] {
            put(off, 0);
        }
        for i in 0..15 {
            put(0x5c + i * 4, 0x4d);
        }
        for i in 0..10 {
            put(0x9c + i * 4, 0);
        }
        b
    }

    #[test]
    fn fn_0066fe40_with_an_index_and_a_name() {
        let (calls, bytes, this) = build(7, 0x2000_0000);
        assert_eq!(bytes, expected(0x2000_0000));
        let addrs: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            addrs,
            vec![
                0x0048_3370,
                0x0040_2d00,
                0x0048_2c50,
                0x0040_2c20,
                0x0040_37b0,
                0x004f_15a0,
                0x0066_fdf0,
                0x0048_5c10,
                0x0040_6d00,
                0x0040_1290,
                0x0043_8390
            ]
        );
        assert_eq!(calls[5].1, vec![this, 0x59]);
        assert_eq!(calls[6].1, vec![7]);
        assert_eq!(calls[7].1, vec![this, 1007, 0]);
        let buffer = calls[8].1[0];
        assert_eq!(calls[8].1[1..], [0x64, 0x0106_79d0, 0x2000_0000]);
        assert_eq!(calls[9].1, vec![this, buffer]);
        assert_eq!(calls[10].1, vec![this + 0x3c, 0x0101_1584]);
    }

    #[test]
    fn fn_0066fe40_without_an_index_or_a_name() {
        let (calls, bytes, this) = build(0xffff_ffff, 0);
        assert_eq!(bytes, expected(0));
        let addrs: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            addrs,
            vec![
                0x0048_3370,
                0x0040_2d00,
                0x0048_2c50,
                0x0040_2c20,
                0x0040_37b0,
                0x004f_15a0,
                0x0043_8390
            ]
        );
        assert_eq!(calls[0].1, vec![this]);
        assert_eq!(calls[1].1, vec![this + 0x18]);
        assert_eq!(calls[2].1, vec![this + 0x24]);
        assert_eq!(calls[3].1, vec![this + 0x2c]);
        assert_eq!(calls[4].1, vec![this + 0x3c]);
    }
}

#[cfg(test)]
mod table_tests {
    use crate::prelude::*;

    #[test]
    fn fn_0066eba0_is_true_for_exactly_the_tabled_indices() {
        let mut e = Engine::new();
        let yes = [0x12, 0x13, 0x14, 0x34, 0x3a, 0x3b, 0x3c, 0x3d];
        for index in 0..0x60u32 {
            let got = e.call(0x0066_eba0, &args![index]).bool();
            assert_eq!(got, yes.contains(&index), "index {index:#x}");
        }
        assert!(!e.call(0x0066_eba0, &args![0xffff_ffffu32]).bool());
        assert!(!e
            .call(0x0066_eba0, &args![0x12u32.wrapping_add(0x1_0000)])
            .bool());
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    #[test]
    fn clears_the_first_4d_words_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x140);
        for i in 0..0x50u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0066_ef90, &args![this]).u32(), this);
        for i in 0..0x4du32 {
            assert_eq!(e.mem.u32(this + i * 4), 0);
        }
        assert_eq!(e.mem.u32(this + 0x4d * 4), 0xffff_ffff);
    }
}
