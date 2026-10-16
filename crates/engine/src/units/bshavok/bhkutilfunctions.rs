//! `BSHavok/bhkutilfunctions.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c7ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkutilfunctions.obj`). cdecl, one
/// word. Takes the object `[object + 0xc]` (or `object` itself when that is
/// null), then the object `[holder]`. When the word at `+0xc` of that
/// object is 8 or 10, its virtual slot `+0x10` is called and, unless the
/// result is 0 or 0x10, the word at `result - 8` is returned; when it is 9
/// the word at `+8` is returned. Otherwise 0.
pub fn fn_00c7ffd0(e: &mut Engine, object: Ptr) -> u32 {
    let mut holder = e.mem.u32(object.addr() + 0xc);
    if holder == 0 {
        holder = object.addr();
    }
    if holder == 0 {
        return 0;
    }
    let inner = e.mem.u32(holder);
    let kind = e.mem.u32(inner + 0xc);
    if kind == 8 || kind == 10 {
        let result = e.vcall(inner, 0x10, &args![]).u32();
        if result == 0 || result == 0x10 {
            return 0;
        }
        return e.mem.u32(result.wrapping_sub(8));
    }
    if inner != 0 && kind == 9 {
        return e.mem.u32(inner + 8);
    }
    0
}

// Translated from 00c7fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkStiffSpringConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85f8.
pub fn bhk_stiff_spring_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85f8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c7fa60, bhk_stiff_spring_constraint_get_r_t_t_i() -> u32),
        entry!(0x00c7ffd0, fn_00c7ffd0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object whose `+0xc` word is null, holding `inner` at `+0`.
    fn object_00c7ffd0(e: &mut Engine, kind: u32, vtable: u32) -> (Ptr, u32) {
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner, vtable);
        e.mem.set_u32(inner + 0xc, kind);
        e.mem.set_u32(inner + 8, 0x1111);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, inner);
        (Ptr::<()>::new(object), inner)
    }

    #[test]
    fn fn_00c7ffd0_reads_the_word_at_8_for_kind_9() {
        let mut e = Engine::new();
        let (object, _) = object_00c7ffd0(&mut e, 9, 0);
        assert_eq!(e.call(0x00c7_ffd0, &args![object]).u32(), 0x1111);
        let (other, _) = object_00c7ffd0(&mut e, 7, 0);
        assert_eq!(e.call(0x00c7_ffd0, &args![other]).u32(), 0);
    }

    #[test]
    fn fn_00c7ffd0_follows_the_holder_word_and_the_virtual_result() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_2000, &[0, 0, 0, 0, 0x0090_3000]);
        e.register(0x0090_3000, |e, _| Ret {
            eax: e.global::<u32>(0x0090_4000),
            ..Ret::default()
        });
        e.map(0x0090_4000, 0x10);
        let (outer, inner) = object_00c7ffd0(&mut e, 8, 0x0090_2000);
        // The holder is a separate object whose first word is `inner`.
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder, inner);
        e.mem.set_u32(outer.addr() + 0xc, holder);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 0x8, 0x3333);
        // A virtual result r makes the function read the word at r - 8.
        e.set_global(0x0090_4000, record + 0x10);
        assert_eq!(e.call(0x00c7_ffd0, &args![outer]).u32(), 0x3333);
        e.set_global(0x0090_4000, 0u32);
        assert_eq!(e.call(0x00c7_ffd0, &args![outer]).u32(), 0);
        e.set_global(0x0090_4000, 0x10u32);
        assert_eq!(e.call(0x00c7_ffd0, &args![outer]).u32(), 0);
    }

    #[test]
    fn bhk_stiff_spring_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c7_fa60, &args![]).u32(), 0x0126_85f8);
    }
}
