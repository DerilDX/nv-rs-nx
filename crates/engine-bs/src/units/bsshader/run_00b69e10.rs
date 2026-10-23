//! `BSShader/run_00b69e10` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's allocation context (`+0x2b4` of the `_tls_index`
/// block), 0x1d while the render passes are built.
const TLS_CONTEXT: u32 = 0x2b4;
/// The byte that enables the water passes (`011ad86e`).
const WATER_PASSES_ENABLED: u32 = 0x011a_d86e;
/// The two global flag bytes the pass choice reads (`011ff104`, `011ff101`).
const GLOBAL_FLAG_011FF104: u32 = 0x011f_f104;
const GLOBAL_FLAG_011FF101: u32 = 0x011f_f101;
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map), thiscall (property, -1).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
/// `BSShaderProperty::RenderPassArray::Add` (engine map), thiscall
/// (array, geometry, pass, 1, 0, 0, 0, 0, 0).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// thiscall (property, geometry): the second pass's geometry (word +0) and pass (16 bits at +4).
const SECOND_PASS_B69F30: u32 = 0x00b6_9f30;
/// `WaterShaderProperty`'s vtable (the one its constructor sets).
const WATER_SHADER_PROPERTY_VTABLE: u32 = 0x010a_e438;
/// The list destructor run on `+0x128` (`NiTPointerListBase<..>::RemoveAll` in the engine map; folded code).
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `BSShaderProperty` destructor (`00baa0e0`, thiscall).
const SHADER_PROPERTY_DESTROY: u32 = 0x00ba_a0e0;

/// The pass number the code picks (see [`fn_00b69ff0`]): `w` is the byte at
/// `+0x7e`, `f` whether the geometry data has a word at `+0x28`, `d` and `l`
/// the bytes at `+0x60` and `+0x61`, `g` the global byte at `011ff101`, `a`
/// and `b` the bytes at `+0x80` and `+0x81` and `c` the byte at `+0x63`.
#[allow(clippy::too_many_arguments)]
fn pass_for(w: u8, f: bool, d: u8, l: u8, g: u8, a: u8, b: u8, c: u8) -> u32 {
    let z = u32::from(c == 0);
    let (a, b) = (a != 0, b != 0);
    if w != 0 {
        return 0x2bd;
    }
    if d != 0 {
        // Four passes apart per `c`, chosen by the global flag, `a` and `b`.
        let start = match (f, g != 0, a, b) {
            (true, true, false, false) => 0x2a6,
            (true, true, false, true) => 0x2a4,
            (true, true, true, false) => 0x2a5,
            (true, true, true, true) => 0x2a3,
            (true, false, false, false) => 0x29e,
            (true, false, false, true) => 0x29c,
            (true, false, true, false) => 0x29d,
            (true, false, true, true) => 0x29b,
            (false, true, false, false) => 0x293,
            (false, true, false, true) => 0x295,
            (false, true, true, false) => 0x294,
            (false, true, true, true) => 0x292,
            (false, false, false, false) => 0x28b,
            (false, false, false, true) => 0x28d,
            (false, false, true, false) => 0x28c,
            (false, false, true, true) => 0x28a,
        };
        return z * 4 + start;
    }
    if l != 0 {
        return u32::from(!a) + 0x23f;
    }
    // Eight passes apart per `c`; without the global flag and `a`, the code
    // computes the pair as `x - 10` or `x - 6` when `c` is set, else `x`.
    let minus_ten = if c != 0 { 0xffff_fff6u32 } else { 0 };
    let minus_six = if c != 0 { 0xffff_fffau32 } else { 0 };
    match (f, g != 0, a, b) {
        (true, true, false, false) => z * 8 + 0x2c3,
        (true, true, false, true) => z * 8 + 0x2c5,
        (true, true, true, false) => z * 8 + 0x2c4,
        (true, true, true, true) => z * 8 + 0x2c2,
        (true, false, false, false) => minus_ten.wrapping_add(0x2c9),
        (true, false, false, true) => minus_six.wrapping_add(0x2c7),
        (true, false, true, false) => z * 8 + 0x2c0,
        (true, false, true, true) => z * 8 + 0x2be,
        (false, true, false, false) => z * 8 + 0x2b1,
        (false, true, false, true) => z * 8 + 0x2b3,
        (false, true, true, false) => z * 8 + 0x2b2,
        (false, true, true, true) => z * 8 + 0x2b0,
        (false, false, false, false) => minus_ten.wrapping_add(0x2b7),
        (false, false, false, true) => minus_six.wrapping_add(0x2b5),
        (false, false, true, false) => z * 8 + 0x2ae,
        (false, false, true, true) => z * 8 + 0x2ac,
    }
}

fn add_pass(e: &mut Engine, list: u32, geometry: u32, pass: u32) {
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

// Translated from 00b69ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b69ff0 - the C++ exception-unwinding frame (FS:[0]) that restores the allocation context when a call throws is not translated
/// The water shader property's render passes (unnamed in the engine map;
/// the fields are those its destructor `00b6a520` releases), thiscall
/// `(geometry, five more words never read)`: with the allocation context
/// at 0x1d (restored at the end), answers 0 unless the byte at `011ad86e`
/// is set. When the global byte at `011ff104` differs from the one kept at
/// `+0x7e`, clears the pass list's count (`+0x10` of the list at `+0x3c`)
/// and keeps the new byte. Makes sure the pass list exists; when it is
/// empty, adds the pass [`pass_for`] picks, the second pass from
/// `00b69f30` when the bytes at `+0x60` and `+0x61` are 0, and, when the
/// byte at `+0x83` and the word at `+0x130` are set, a third pass (0x2ab,
/// 0x29a, 0x2ce or 0x2bc by `+0x60` and the geometry flag). Answers the
/// pass list.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b69ff0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, 0x1d);
    if e.global::<u8>(WATER_PASSES_ENABLED) == 0 {
        e.mem.set_u32(tls + TLS_CONTEXT, saved);
        return 0;
    }
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    let flag = e.mem.u32(data.wrapping_add(0x28)) != 0;
    if e.global::<u8>(GLOBAL_FLAG_011FF104) != e.mem.u8(base + 0x7e) {
        let list = e.mem.u32(base + 0x3c);
        if list != 0 {
            e.mem.set_u32(list + 0x10, 0);
        }
        let value: u8 = e.global(GLOBAL_FLAG_011FF104);
        e.mem.set_u8(base + 0x7e, value);
    }
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![base, 0xffff_ffffu32]);
    let list = e.mem.u32(base + 0x3c);
    if e.mem.u32(list.wrapping_add(0x10)) == 0 {
        let pass = pass_for(
            e.mem.u8(base + 0x7e),
            flag,
            e.mem.u8(base + 0x60),
            e.mem.u8(base + 0x61),
            e.global(GLOBAL_FLAG_011FF101),
            e.mem.u8(base + 0x80),
            e.mem.u8(base + 0x81),
            e.mem.u8(base + 0x63),
        );
        add_pass(e, list, geometry, pass);
        if e.mem.u8(base + 0x60) == 0 && e.mem.u8(base + 0x61) == 0 {
            let second = e.call(SECOND_PASS_B69F30, &args![base, geometry]).u32();
            let pass = u32::from(e.mem.u16(second + 4));
            let second_geometry = e.mem.u32(second);
            let list = e.mem.u32(base + 0x3c);
            add_pass(e, list, second_geometry, pass);
        }
        if e.mem.u8(base + 0x83) != 0 && e.mem.u32(base + 0x130) != 0 {
            let list = e.mem.u32(base + 0x3c);
            let pass = match (e.mem.u8(base + 0x60) != 0, flag) {
                (true, true) => 0x2ab,
                (true, false) => 0x29a,
                (false, true) => 0x2ce,
                (false, false) => 0x2bc,
            };
            add_pass(e, list, geometry, pass);
        }
    }
    let list = e.mem.u32(base + 0x3c);
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    list
}

/// Releases an `NiPointer`'s object: decrements its reference count (+4)
/// and deletes it (vtable +4) when the count reaches 0.
fn release(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b6a520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b6a520 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a release throws is not translated
/// `WaterShaderProperty::~WaterShaderProperty` (by the vtable it sets;
/// unnamed in the engine map): sets the vtable, empties the list at
/// `+0x128`, releases the six `NiPointer`s from `+0x148` down to `+0x134`,
/// empties the list again (its destructor) and destroys the
/// `BSShaderProperty` base.
pub fn fn_00b6a520(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, WATER_SHADER_PROPERTY_VTABLE);
    e.call(LIST_REMOVE_ALL, &args![base + 0x128]);
    for offset in [0x148u32, 0x144, 0x140, 0x13c, 0x138, 0x134] {
        let object = e.mem.u32(base + offset);
        release(e, object);
    }
    e.call(LIST_REMOVE_ALL, &args![base + 0x128]);
    e.call(SHADER_PROPERTY_DESTROY, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b69ff0,
            fn_00b69ff0(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b6a520, fn_00b6a520(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE_DOUBLE: u32 = 0xdead_8000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn pass_choice_follows_the_flags() {
        assert_eq!(pass_for(1, true, 1, 1, 1, 1, 1, 1), 0x2bd);
        assert_eq!(pass_for(0, true, 1, 0, 1, 0, 0, 0), 0x2aa);
        assert_eq!(pass_for(0, true, 1, 0, 0, 1, 1, 1), 0x29b);
        assert_eq!(pass_for(0, false, 1, 0, 1, 1, 0, 0), 0x298);
        assert_eq!(pass_for(0, false, 1, 0, 0, 0, 1, 1), 0x28d);
        assert_eq!(pass_for(0, true, 0, 1, 0, 0, 0, 0), 0x240);
        assert_eq!(pass_for(0, false, 0, 1, 0, 1, 0, 0), 0x23f);
        assert_eq!(pass_for(0, true, 0, 0, 1, 0, 1, 0), 0x2cd);
        assert_eq!(pass_for(0, true, 0, 0, 0, 0, 0, 1), 0x2bf);
        assert_eq!(pass_for(0, true, 0, 0, 0, 0, 0, 0), 0x2c9);
        assert_eq!(pass_for(0, true, 0, 0, 0, 0, 1, 1), 0x2c1);
        assert_eq!(pass_for(0, true, 0, 0, 0, 1, 1, 0), 0x2c6);
        assert_eq!(pass_for(0, false, 0, 0, 1, 1, 1, 1), 0x2b0);
        assert_eq!(pass_for(0, false, 0, 0, 0, 0, 0, 1), 0x2ad);
        assert_eq!(pass_for(0, false, 0, 0, 0, 0, 1, 0), 0x2b5);
        assert_eq!(pass_for(0, false, 0, 0, 0, 1, 0, 0), 0x2b6);
    }

    #[test]
    fn builds_the_passes_with_the_allocation_context() {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 7);
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register_double(RENDER_PASS_ARRAY_ADD, move |e, _| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0x1d);
            Ret::default()
        });
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x4444);
        e.mem.set_u16(second + 4, 0x123);
        e.register_double(SECOND_PASS_B69F30, move |_, _| Ret {
            eax: second,
            ..Ret::default()
        });
        let list = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x2c);
        e.mem.set_u32(data + 0x28, 1);
        let geometry = e.mem.alloc(0xbc);
        e.mem.set_u32(geometry + 0xb8, data);
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this + 0x3c, list);
        e.mem.set_u8(this + 0x7e, 1);
        e.mem.set_u8(this + 0x83, 1);
        e.mem.set_u32(this + 0x130, 1);
        let words = args![this, geometry, 0u32, 0u32, 0u32, 0u32, 0u32];
        // Disabled: answers 0 and restores the context.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_9ff0, &words).u32(), 0);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
        assert!(calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY).is_empty());
        e.set_global(WATER_PASSES_ENABLED, 1u8);
        e.mem.set_u32(list + 0x10, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_9ff0, &words).u32(), list);
        // The global byte differs: the list is cleared and the byte kept.
        assert_eq!(e.mem.u8(this + 0x7e), 0);
        assert_eq!(
            calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY),
            vec![vec![this, 0xffff_ffff]]
        );
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![
                vec![list, geometry, 0x2c9, 1, 0, 0, 0, 0, 0],
                vec![list, 0x4444, 0x123, 1, 0, 0, 0, 0, 0],
                vec![list, geometry, 0x2ce, 1, 0, 0, 0, 0, 0],
            ]
        );
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
        // A list with passes is left alone.
        e.mem.set_u32(list + 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b6_9ff0, &words);
        assert!(calls(&e, RENDER_PASS_ARRAY_ADD).is_empty());
    }

    #[test]
    fn destructor_releases_the_pointers() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(SHADER_PROPERTY_DESTROY, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_DOUBLE);
        let last = e.mem.alloc(8);
        e.mem.set_u32(last, vtable);
        e.mem.set_u32(last + 4, 1);
        let shared = e.mem.alloc(8);
        e.mem.set_u32(shared, vtable);
        e.mem.set_u32(shared + 4, 2);
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this + 0x148, last);
        e.mem.set_u32(this + 0x134, shared);
        e.call_log = Some(vec![]);
        e.call(0x00b6_a520, &args![this]);
        assert_eq!(e.mem.u32(this), WATER_SHADER_PROPERTY_VTABLE);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![last]]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL),
            vec![vec![this + 0x128], vec![this + 0x128]]
        );
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTROY), vec![vec![this]]);
    }
}
