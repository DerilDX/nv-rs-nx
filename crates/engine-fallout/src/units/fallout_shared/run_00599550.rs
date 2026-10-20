//! `fallout shared/run_00599550` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00599c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599c40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x72 (0x30 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_449c`) is `TESAmmoEffect`, so this is the constructor of `TESAmmoEffect`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00599c40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_449c);
    e.mem.set_u32(base + 0x18, 0x0103_4484);
    e.call(0x0040_3d30, &args![this, base + 0x24, 0x0u32, 0xcu32]);
    e.call(0x004f_15a0, &args![this, 0x72u32]);
    this
}

// Translated from 00599cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00599d00` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00599cd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_9d00, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The media set's track reset (`00597850`, on a track).
const TRACK_RESET: u32 = 0x0059_7850;
/// The media set's six tracks.
const TRACKS: [u32; 6] = [0x48, 0x58, 0x68, 0x78, 0x88, 0x98];
/// The list helpers on a node: its item's slot (`006815c0`) and the next
/// node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 00599b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `MediaSet` (the engine map has no name): its six tracks
/// (`00597850`), the four floats at +0xAC .. +0xB8 to 0.0 and the two
/// sound words at +0xBC and +0xC0 to 0.
pub fn fn_00599b30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for track in TRACKS {
        e.call(TRACK_RESET, &args![base + track]);
    }
    for offset in [0xacu32, 0xb0, 0xb4, 0xb8] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
}

// Translated from 00599bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `MediaSet`'s six tracks (`00597850`) and its two sound words
/// at +0xBC and +0xC0 (the engine map has no name).
pub fn fn_00599bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for track in TRACKS {
        e.call(TRACK_RESET, &args![base + track]);
    }
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
}

// Translated from 00599d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599d00 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESAmmoEffect` (the vtables `00599c40` stores):
/// puts back the vtables, destroys the name at +0x18 (`00591300`) and the
/// `TESForm` base (`00483630`).
pub fn fn_00599d00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_449c);
    e.mem.set_u32(base + 0x18, 0x0103_4484);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00599d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00599d70 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESAmmoEffect`'s record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the form type byte is 0x72; otherwise
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID` (vtable
/// +0x134), `OBND` (vtable +0xE0), `FULL` (+0x18) and `DATA` (0xC bytes at
/// +0x24, swapped by `005a6b10`). Returns true.
pub fn fn_00599d70(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(0x0047_2660, &args![file]).u8();
    if form_type != 0x72 {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            // DATA
            0x4154_4144 => {
                e.call(0x0048_61f0, &args![this, file, base + 0x24, 0xcu32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x005a_6b10, &args![base + 0x24]);
                }
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(0x0047_2890, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            // FULL
            0x4c4c_5546 => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full, file]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0059a030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059a030 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Applies the ammo effects of the list `effects` of type `kind` to
/// `value` (cdecl; the engine map has no name): the effects are first
/// copied into a new list (8 bytes, `0096a2d0`; those whose operation,
/// `0045cd60`, is 1 through `005ae3d0`, the others through `00905820`),
/// then each effect whose type (`0059bb30`) is `kind` adds (operation 0),
/// multiplies by (1) or subtracts (2) its amount (`00759450`) in x87
/// arithmetic stored as a `float`; the copy is deleted (`004702f0(1)`). No
/// list: `value` itself.
pub fn fn_0059a030(e: &mut Engine, kind: u32, effects: u32, value: f32) -> f32 {
    let mut result = x87_quiet_bits(value.to_bits());
    if effects == 0 {
        return f32::from_bits(result);
    }
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![block]).u32()
    };
    let mut node = effects;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        let operation = e.call(0x0045_cd60, &args![effect]).u32();
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if operation == 1 {
            e.call(0x005a_e3d0, &args![copy, slot]);
        } else {
            e.call(0x0090_5820, &args![copy, slot]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let mut node = copy;
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let effect_kind = e.call(0x0059_bb30, &args![effect]).u32();
        let operation = e.call(0x0045_cd60, &args![effect]).u32();
        let amount = x87_store32(e.call(0x0075_9450, &args![effect]).f64());
        if effect_kind != kind {
            continue;
        }
        let current = x87_load32(result);
        let operand = x87_m32(amount);
        match operation {
            0 => result = x87_store32(x87_add(current, operand)),
            1 => result = x87_store32(x87_mul(current, operand)),
            2 => result = x87_store32(x87_sub(current, operand)),
            _ => {}
        }
    }
    if copy != 0 {
        e.call(0x0047_02f0, &args![copy, 1u32]);
    }
    f32::from_bits(x87_quiet_bits(result))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_9c40, fn_00599c40(Ptr) -> Ptr),
        entry!(0x0059_9cd0, fn_00599cd0(Ptr, u32) -> Ptr),
        entry!(0x0059_9b30, fn_00599b30(Ptr)),
        entry!(0x0059_9bd0, fn_00599bd0(Ptr)),
        entry!(0x0059_9d00, fn_00599d00(Ptr)),
        entry!(0x0059_9d70, fn_00599d70(Ptr, Ptr) -> bool),
        entry!(0x0059_a030, fn_0059a030(u32, u32, f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00599c40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_9c40,
            size: 0x30,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_3d30, &[A::T(0x0), A::T(0x24), A::V(0x0), A::V(0xc)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x72)]),
            ],
            stores: &[(0x0, 4, 0x0103_449c), (0x18, 4, 0x0103_4484)],
        });
    }
    #[test]
    fn at_00599cd0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_9d00, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_9cd0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_9cd0)
                .collect();
            let mut expected = vec![(0x0059_9d00, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod media_set_and_ammo_effect_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn media_set_resets() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        for offset in (0xac..0xc4).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        quiet(&mut e, &[TRACK_RESET]);
        e.call_log = Some(vec![]);
        e.call(0x0059_9bd0, &args![this]);
        let tracks: Vec<u32> = calls(&e, TRACK_RESET).iter().map(|c| c[0] - this).collect();
        assert_eq!(tracks, TRACKS.to_vec());
        assert_eq!(e.mem.u32(this + 0xac), 0xffff_ffff);
        assert_eq!((e.mem.u32(this + 0xbc), e.mem.u32(this + 0xc0)), (0, 0));

        e.call_log = Some(vec![]);
        e.call(0x0059_9b30, &args![this]);
        assert_eq!(calls(&e, TRACK_RESET).len(), 6);
        for offset in (0xac..0xc4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn ammo_effect_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        quiet(&mut e, &[0x0059_1300, 0x0048_3630]);
        e.call_log = Some(vec![]);
        e.call(0x0059_9d00, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_449c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_4484);
        assert_eq!(calls(&e, 0x0059_1300), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
    }

    fn load(form_type: u32, swap: bool) -> (Engine, u32, u32, bool) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![0x4449_4445u32, 0x444e_424f, 0x4c4c_5546, 0x4154_4144],
            0usize,
        )));
        e.register_double(0x0047_2660, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(0x0047_26f0, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        if swap {
            e.register(0x0040_1680, |_, _| ret(1));
        } else {
            e.register(0x0040_1680, |_, _| ret(0));
        }
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                0x0047_2890,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_61f0,
                0x005a_6b10,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x0059_9d70, &args![this, file]).bool();
        (e, this, file, result)
    }

    #[test]
    fn ammo_effect_load() {
        let (e, this, file, result) = load(0x72, true);
        assert!(result);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x24, 0xc]]
        );
        assert_eq!(calls(&e, 0x005a_6b10), vec![vec![this + 0x24]]);

        let (e, _, _, result) = load(0x72, false);
        assert!(result);
        assert!(calls(&e, 0x005a_6b10).is_empty());

        let (e, _, _, result) = load(0x71, false);
        assert!(!result);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }

    /// Effects: words at `effect` (type), `effect + 4` (operation),
    /// `effect + 8` (amount); the source list's nodes are the addresses of
    /// a word array ending in 0, and the copy is a separate list.
    fn effects(e: &mut Engine, list: &[(u32, u32, f32)]) -> u32 {
        let nodes = e.mem.alloc(4 * (list.len() as u32 + 1));
        for (i, (kind, operation, amount)) in list.iter().enumerate() {
            let effect = e.mem.alloc(0xc);
            e.mem.set_u32(effect, *kind);
            e.mem.set_u32(effect + 4, *operation);
            e.mem.set_f32(effect + 8, *amount);
            e.mem.set_u32(nodes + 4 * i as u32, effect);
        }
        let copy = Rc::new(RefCell::new(Vec::<u32>::new()));
        let copy_nodes = e.mem.alloc(0x40);
        e.register(0x0040_1000, |_, _| ret(0x5000));
        e.register_double(0x0096_a2d0, move |_, _| ret(copy_nodes));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |_, a| ret(a[0] + 4));
        let c = copy.clone();
        let add = move |e: &mut Engine, a: &[u32]| {
            let mut c = c.borrow_mut();
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4 * c.len() as u32, item);
            c.push(item);
            Ret::default()
        };
        e.register_double(0x005a_e3d0, add.clone());
        e.register_double(0x0090_5820, add);
        e.register(0x0045_cd60, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0059_bb30, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0075_9450, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 8)),
            ..Ret::default()
        });
        quiet(e, &[0x0047_02f0]);
        nodes
    }

    #[test]
    fn ammo_effects_apply_in_order() {
        let mut e = Engine::new();
        let list = effects(
            &mut e,
            &[
                (1, 1, 2.0),
                (2, 0, 100.0),
                (1, 0, 3.0),
                (1, 2, 0.5),
                (1, 7, 9.0),
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x0059_a030, &args![1u32, list, 10.0f32]).f32();
        // ((10 * 2) + 3) - 0.5; the other kind and operation 7 change nothing.
        assert_eq!(result, 22.5);
        assert_eq!(calls(&e, 0x005a_e3d0).len(), 1);
        assert_eq!(calls(&e, 0x0090_5820).len(), 4);
        assert_eq!(calls(&e, 0x0047_02f0).len(), 1);

        // No list: the value itself, nothing allocated.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_a030, &args![1u32, 0u32, 4.0f32]).f32(), 4.0);
        assert!(calls(&e, 0x0040_1000).is_empty());
    }
}
