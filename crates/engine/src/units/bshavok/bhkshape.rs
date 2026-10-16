//! `BSHavok/bhkshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c84f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkshape.obj`). Thiscall, one word
/// (`RET 4`). Unless `index` is `-1`, calls virtual slot `+0xcc` of `this`
/// and, when that returns an object, returns that object's virtual slot
/// `+0xe0` called with `index`. Otherwise returns `[this + 0x10] & 0x1f`.
pub fn fn_00c84f10(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    if index != -1 {
        let object = e.vcall(this.addr(), 0xcc, &args![]).u32();
        if object != 0 {
            return e.vcall(object, 0xe0, &args![index]).u32();
        }
    }
    e.mem.u32(this.addr() + 0x10) & 0x1f
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c84f10, fn_00c84f10(Ptr, i32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c84f10_asks_the_object_unless_the_index_is_minus_one() {
        let mut e = Engine::new();
        // Vtable of the shape: slot 0xcc returns the object.
        let mut slots = vec![0u32; 0x34];
        slots[0xcc / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        let mut object_slots = vec![0u32; 0x39];
        object_slots[0xe0 / 4] = 0x0090_5100;
        e.put_vtable(0x0090_3000, &object_slots);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0090_3000);
        e.register_double(0x0090_5000, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        e.register(0x0090_5100, |_, a| Ret {
            eax: a[1] + 100,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x0090_2000);
        e.mem.set_u32(this + 0x10, 0xffff_ffe7);
        assert_eq!(e.call(0x00c8_4f10, &args![this, 5i32]).u32(), 105);
        assert_eq!(e.call(0x00c8_4f10, &args![this, -1i32]).u32(), 0x07);
    }

    #[test]
    fn fn_00c84f10_falls_back_to_the_low_five_bits_without_an_object() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x34];
        slots[0xcc / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        e.register(0x0090_5000, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x0090_2000);
        e.mem.set_u32(this + 0x10, 0x1234_5678);
        assert_eq!(e.call(0x00c8_4f10, &args![this, 3i32]).u32(), 0x18);
    }
}
