//! `fallout/ai/run_009b88e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees: a flag test `004fd420` (`thiscall`, `(mask)`, answers a byte), the list at
/// `this + 0x88` (`008256d0` "empty", `006815c0` its first item; `thiscall`, no arguments),
/// `004181e0` (`thiscall`, no arguments, answers an object), the pointer conversion
/// `004ae750` (`thiscall` on the pointer, no arguments; the map's name for it is a folded
/// one), `00444d00` (`thiscall`, `(bit)`, answers a byte), and the others named where used.
const TEST_FLAGS_004FD420: u32 = 0x004f_d420;
const LIST_EMPTY_008256D0: u32 = 0x0082_56d0;
const LIST_FIRST_006815C0: u32 = 0x0068_15c0;
const OBJECT_004181E0: u32 = 0x0041_81e0;
const CONVERT_004AE750: u32 = 0x004a_e750;
const TEST_BIT_00444D00: u32 = 0x0044_4d00;
/// `fn_009b8900`'s callees: `0043b540` and `0043b4d0` (`thiscall`, no arguments), `008c71b0`
/// (`thiscall` on a local, one word, answers the local), the test `009c2c80` (`cdecl`, four
/// words) and `Projectile::AddImpact` (`009bf5a0`, `thiscall`, six words).
const KIND_SOURCE_0043B540: u32 = 0x0043_b540;
const KIND_LOCAL_008C71B0: u32 = 0x008c_71b0;
const KIND_OF_0043B4D0: u32 = 0x0043_b4d0;
const IMPACT_TEST_009C2C80: u32 = 0x009c_2c80;
const PROJECTILE_ADD_IMPACT: u32 = 0x009b_f5a0;
/// `fn_009b8a20`'s callees: `009b7c90` (`thiscall`, no arguments, answers a byte) and
/// `009d9f00` (`thiscall`, no arguments, answers a word).
const OBJECT_TEST_009B7C90: u32 = 0x009b_7c90;
const TEST_009D9F00: u32 = 0x009d_9f00;
/// `fn_009b8b10`'s callees: the floats (ST0) `00979260`, `009669c0`, `00644660` and
/// `00508070` (`thiscall`, no arguments), `00408840` (`cdecl`, `(float)`, ST0),
/// `Projectile::ProcessImpacts` (`009c1b70`, answers a byte), the object at `011f2250` and
/// its `00441110` / `007af430` getters and `009c8b90` / `009c8c40` setters (`(0)`), and the
/// four per-state updates `009bc8f0`, `009b8c90`, `009b90b0`, `009ba660` (`thiscall`, no
/// arguments). The `double` constants: 1/30 as a float (`0108fdd0`) and 0 (`01012060`).
const VALUE_00979260: u32 = 0x0097_9260;
const VALUE_009669C0: u32 = 0x0096_69c0;
const VALUE_00644660: u32 = 0x0064_4660;
const VALUE_00508070: u32 = 0x0050_8070;
const FLOAT_OF_00408840: u32 = 0x0040_8840;
const PROJECTILE_PROCESS_IMPACTS: u32 = 0x009c_1b70;
const HOLDER_011F2250: u32 = 0x011f_2250;
const HOLDER_GET_00441110: u32 = 0x0044_1110;
const HOLDER_GET_007AF430: u32 = 0x007a_f430;
const HOLDER_SET_009C8B90: u32 = 0x009c_8b90;
const HOLDER_SET_009C8C40: u32 = 0x009c_8c40;
const STATE_1_UPDATE_009BC8F0: u32 = 0x009b_c8f0;
const STATE_2_UPDATE_009B8C90: u32 = 0x009b_8c90;
const STATE_3_UPDATE_009B90B0: u32 = 0x009b_90b0;
const STATE_4_UPDATE_009BA660: u32 = 0x009b_a660;
const ONE_THIRTIETH: u32 = 0x0108_fdd0;
const ZERO: u32 = 0x0101_2060;
/// The state word at +0x150.
const STATE: u32 = 0x150;

// Translated from 009b88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): `004fd420(0x800)` on `this`; answers what
/// it leaves in EAX.
pub fn fn_009b88e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(TEST_FLAGS_004FD420, &args![this, 0x800u32]).u32()
}

// Translated from 009ba350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(pointer)`): the word at `this + 0xC` becomes
/// `004ae750` on `pointer`, or 0 for a null pointer.
pub fn fn_009ba350(e: &mut Engine, this: Ptr, pointer: u32) {
    let value = if pointer == 0 {
        0
    } else {
        e.call(CONVERT_004AE750, &args![pointer]).u32()
    };
    e.mem.set_u32(this.addr() + 0xc, value);
}

// Translated from 009ba390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(pointer)`): the word at `this + 0x10` becomes
/// `004ae750` on `pointer`, or 0 for a null pointer.
pub fn fn_009ba390(e: &mut Engine, this: Ptr, pointer: u32) {
    let value = if pointer == 0 {
        0
    } else {
        e.call(CONVERT_004AE750, &args![pointer]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, value);
}

// Translated from 009ba320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(first, second)`; the decompiler's library match
/// `CRect::MoveToXY` is wrong): `fn_009ba350(first)` and `fn_009ba390(second)`.
pub fn fn_009ba320(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    fn_009ba350(e, this, first);
    fn_009ba390(e, this, second);
}

// Translated from 009b8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): the state at +0x150 moves on: 3 becomes
/// 4; otherwise, when the list at +0x88 is not empty, it becomes 1, then 3 when
/// `009b7c90` answers true for the `004181e0` object, then 0 when bit 1 is set and bit 2 is
/// not (`00444d00`); when bit 1 is not set it becomes 2 if the object's `004fd420(0x10)`
/// answers true or `009d9f00` answers non-zero.
pub fn fn_009b8a20(e: &mut Engine, this: Ptr) {
    let state = this.addr() + STATE;
    if e.mem.u32(state) == 3 {
        e.mem.set_u32(state, 4);
        return;
    }
    if e.call(LIST_EMPTY_008256D0, &args![this.addr() + 0x88])
        .bool()
    {
        return;
    }
    e.mem.set_u32(state, 1);
    let object = e.call(OBJECT_004181E0, &args![this]).u32();
    if e.call(OBJECT_TEST_009B7C90, &args![object]).bool() {
        e.mem.set_u32(state, 3);
    }
    if e.call(TEST_BIT_00444D00, &args![this, 1u32]).bool()
        && !e.call(TEST_BIT_00444D00, &args![this, 2u32]).bool()
    {
        e.mem.set_u32(state, 0);
    }
    if !e.call(TEST_BIT_00444D00, &args![this, 1u32]).bool() {
        let object = e.call(OBJECT_004181E0, &args![this]).u32();
        if e.call(TEST_FLAGS_004FD420, &args![object, 0x10u32]).bool()
            || e.call(TEST_009D9F00, &args![this]).u32() != 0
        {
            e.mem.set_u32(state, 2);
        }
    }
}

// Translated from 009b8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, six words): the impact is allowed when the list at +0x88
/// is empty or its first item's byte at +0x28 is set. The fourth word's kind (via
/// `0043b540`, `008c71b0` on a local and `0043b4d0`; 0 without it) is read. When allowed and
/// `009c2c80(first, fourth, this, 004181e0 object)` answers true, and the state (+0x150) is
/// not 3 or the kind is 1, 2, 9, 0x11, 0xD or 0x14, `Projectile::AddImpact` with the six
/// words; when that answers true the state moves on (`fn_009b8a20`) and the answer is true.
/// Otherwise false.
pub fn fn_009b8900(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let list = this.addr() + 0x88;
    let allowed = e.call(LIST_EMPTY_008256D0, &args![list]).bool() || {
        let item = e.call(LIST_FIRST_006815C0, &args![list]).u32();
        let record = e.mem.u32(item);
        e.mem.u8(record.wrapping_add(0x28)) != 0
    };
    let kind = if fourth != 0 {
        let source = e.call(KIND_SOURCE_0043B540, &args![fourth]).u32();
        e.with_stack(4, |e, local| {
            let filled = e.call(KIND_LOCAL_008C71B0, &args![local, source]).u32();
            e.call(KIND_OF_0043B4D0, &args![filled]).u32()
        })
    } else {
        0
    };
    if !allowed {
        return false;
    }
    let object = e.call(OBJECT_004181E0, &args![this]).u32();
    if !e
        .call(IMPACT_TEST_009C2C80, &args![first, fourth, this, object])
        .bool()
    {
        return false;
    }
    let state = e.mem.u32(this.addr() + STATE);
    if state == 3 && ![1u32, 2, 9, 0x11, 0xd, 0x14].contains(&kind) {
        return false;
    }
    if !e
        .call(
            PROJECTILE_ADD_IMPACT,
            &args![this, first, second, third, fourth, fifth, sixth],
        )
        .bool()
    {
        return false;
    }
    fn_009b8a20(e, this);
    true
}

// Translated from 009b8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): when the byte at +0x90 is clear and the
/// list at +0x88 is not empty, the impacts are processed (`Projectile::ProcessImpacts`,
/// its byte the answer) unless bit 1 (`00444d00`) is set and both the float `00644660` is
/// above 0 (or NaN) and `00979260 - 00408840(009669c0 * 1/30)` (as a float) is not below
/// `00508070`. When processed: the holder at `011f2250` drops this object from its two
/// slots it holds it in (`009c8b90(0)`, `009c8c40(0)`), and the state (+0x150) 1 to 4 runs
/// its update (`009bc8f0`, `009b8c90`, `009b90b0`, `009ba660`). Answers whether processed.
pub fn fn_009b8b10(e: &mut Engine, this: Ptr) -> bool {
    let mut processed = false;
    if e.mem.u8(this.addr() + 0x90) == 0
        && !e
            .call(LIST_EMPTY_008256D0, &args![this.addr() + 0x88])
            .bool()
    {
        let mut go = true;
        if e.call(TEST_BIT_00444D00, &args![this, 1u32]).bool() {
            let total = e.call(VALUE_00979260, &args![this]).f64();
            let rate = e.call(VALUE_009669C0, &args![this]).f64();
            let scale = e.global::<f64>(ONE_THIRTIETH);
            let scaled = x87_quiet_bits(x87_store32(x87_mul(rate, scale)));
            let step = e.call(FLOAT_OF_00408840, &args![scaled]).f64();
            let left = x87_store32(x87_sub(total, step));
            let limit = e.call(VALUE_00644660, &args![this]).f64();
            go = if limit <= e.global::<f64>(ZERO) {
                true
            } else {
                let bound = e.call(VALUE_00508070, &args![this]).f64();
                x87_load32(left) < bound
            };
        }
        if go {
            processed = e.call(PROJECTILE_PROCESS_IMPACTS, &args![this]).bool();
        }
    }
    if processed {
        if e.call(HOLDER_GET_00441110, &args![HOLDER_011F2250]).u32() == this.addr() {
            e.call(HOLDER_SET_009C8B90, &args![HOLDER_011F2250, 0u32]);
        }
        if e.call(HOLDER_GET_007AF430, &args![HOLDER_011F2250]).u32() == this.addr() {
            e.call(HOLDER_SET_009C8C40, &args![HOLDER_011F2250, 0u32]);
        }
        let update = match e.mem.u32(this.addr() + STATE).wrapping_sub(1) {
            0 => Some(STATE_1_UPDATE_009BC8F0),
            1 => Some(STATE_2_UPDATE_009B8C90),
            2 => Some(STATE_3_UPDATE_009B90B0),
            3 => Some(STATE_4_UPDATE_009BA660),
            _ => None,
        };
        if let Some(update) = update {
            e.call(update, &args![this]);
        }
    }
    processed
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b88e0, fn_009b88e0(Ptr) -> u32),
        entry!(0x009ba350, fn_009ba350(Ptr, u32)),
        entry!(0x009ba390, fn_009ba390(Ptr, u32)),
        entry!(0x009ba320, fn_009ba320(Ptr, u32, u32)),
        entry!(0x009b8a20, fn_009b8a20(Ptr)),
        entry!(0x009b8900, fn_009b8900(Ptr, u32, u32, u32, u32, u32, u32) -> bool),
        entry!(0x009b8b10, fn_009b8b10(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_009b88e0_tests_flag_0x800() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TEST_FLAGS_004FD420, |_, _| answer(0x101));
        assert_eq!(e.call(0x009b88e0, &args![0x5000u32]).u32(), 0x101);
        assert_eq!(calls_at(&e, TEST_FLAGS_004FD420), vec![vec![0x5000, 0x800]]);
    }

    #[test]
    fn pointer_setters_convert_or_clear() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CONVERT_004AE750, |_, a| answer(a[0] + 1));
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 9);
        e.call(0x009ba320, &args![this, 0x100u32, 0x200u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x101);
        assert_eq!(e.mem.u32(this + 0x10), 0x201);
        e.call(0x009ba350, &args![this, 0u32]);
        e.call(0x009ba390, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(calls_at(&e, CONVERT_004AE750).len(), 2);
    }

    /// A projectile-like object with the state at +0x150 and the list helpers doubled.
    fn projectile(e: &mut Engine, state: u32) -> u32 {
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + STATE, state);
        this
    }

    #[test]
    fn fn_009b8a20_moves_the_state_on() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(0));
        e.register(OBJECT_004181E0, |_, _| answer(0x7000));
        e.register(OBJECT_TEST_009B7C90, |_, _| answer(0));
        e.register(TEST_BIT_00444D00, |_, _| answer(0));
        e.register(TEST_FLAGS_004FD420, |_, _| answer(0));
        e.register(TEST_009D9F00, |_, _| answer(0));
        let this = projectile(&mut e, 3);
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 4);
        // Not 3: 1, then 2 through 009d9f00 since bit 1 is clear.
        e.register(TEST_009D9F00, |_, _| answer(5));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 2);
        // Bit 1 set and bit 2 clear: 0.
        e.register(TEST_BIT_00444D00, |_, a| answer(u32::from(a[1] == 1)));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 0);
        // Both bits set and the object test true: 3.
        e.register(TEST_BIT_00444D00, |_, _| answer(1));
        e.register(OBJECT_TEST_009B7C90, |_, _| answer(1));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 3);
        // An empty list leaves the state.
        e.mem.set_u32(this + STATE, 7);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(1));
        e.call(0x009b8a20, &args![this]);
        assert_eq!(e.mem.u32(this + STATE), 7);
    }

    #[test]
    fn fn_009b8900_adds_allowed_impacts() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(0));
        let item = e.mem.alloc(4);
        let record = e.mem.alloc(0x2c);
        e.mem.set_u32(item, record);
        e.register_double(LIST_FIRST_006815C0, move |_, _| answer(item));
        e.register(KIND_SOURCE_0043B540, |_, _| answer(0x10));
        e.register(KIND_LOCAL_008C71B0, |_, a| answer(a[0]));
        e.register(KIND_OF_0043B4D0, |_, _| answer(0x11));
        e.register(OBJECT_004181E0, |_, _| answer(0x7000));
        e.register(IMPACT_TEST_009C2C80, |_, _| answer(1));
        e.register(PROJECTILE_ADD_IMPACT, |_, _| answer(1));
        e.register(TEST_BIT_00444D00, |_, _| answer(1));
        e.register(OBJECT_TEST_009B7C90, |_, _| answer(0));
        let this = projectile(&mut e, 3);
        let words = |fourth: u32| args![this, 1u32, 2u32, 3u32, fourth, 5u32, 6u32];
        // The first item's byte at +0x28 clear: not allowed.
        assert!(!e.call(0x009b8900, &words(0x4000)).bool());
        assert!(calls_at(&e, IMPACT_TEST_009C2C80).is_empty());
        e.mem.set_u8(record + 0x28, 1);
        // State 3 with kind 0x11: added, and the state moves on (3 -> 4).
        assert!(e.call(0x009b8900, &words(0x4000)).bool());
        assert_eq!(
            calls_at(&e, IMPACT_TEST_009C2C80),
            vec![vec![1, 0x4000, this, 0x7000]]
        );
        assert_eq!(
            calls_at(&e, PROJECTILE_ADD_IMPACT),
            vec![vec![this, 1, 2, 3, 0x4000, 5, 6]]
        );
        assert_eq!(e.mem.u32(this + STATE), 4);
        // State 3 without the fourth word (kind 0): refused.
        e.mem.set_u32(this + STATE, 3);
        assert!(!e.call(0x009b8900, &words(0)).bool());
        assert_eq!(calls_at(&e, PROJECTILE_ADD_IMPACT).len(), 1);
        // AddImpact refusing: false.
        e.mem.set_u32(this + STATE, 0);
        e.register(PROJECTILE_ADD_IMPACT, |_, _| answer(0));
        assert!(!e.call(0x009b8900, &words(0)).bool());
    }

    #[test]
    fn fn_009b8b10_processes_impacts_and_runs_the_state_update() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(ONE_THIRTIETH, 8);
        e.map(ZERO, 8);
        e.set_global::<f64>(ONE_THIRTIETH, 0.5);
        e.register(LIST_EMPTY_008256D0, |_, _| answer(0));
        e.register(TEST_BIT_00444D00, |_, _| answer(1));
        e.register(VALUE_00979260, |_, _| float(10.0));
        e.register(VALUE_009669C0, |_, _| float(4.0));
        e.register(FLOAT_OF_00408840, |_, a| {
            float(f64::from(f32::from_bits(a[0])) * 2.0)
        });
        e.register(VALUE_00644660, |_, _| float(1.0));
        e.register(VALUE_00508070, |_, _| float(6.0));
        e.register(PROJECTILE_PROCESS_IMPACTS, |_, _| answer(1));
        let this = projectile(&mut e, 2);
        e.register_double(HOLDER_GET_00441110, move |_, _| answer(this));
        e.register(HOLDER_GET_007AF430, |_, _| answer(0));
        e.register(HOLDER_SET_009C8B90, |_, _| Ret::default());
        e.register(HOLDER_SET_009C8C40, |_, _| Ret::default());
        for update in [
            STATE_1_UPDATE_009BC8F0,
            STATE_2_UPDATE_009B8C90,
            STATE_3_UPDATE_009B90B0,
            STATE_4_UPDATE_009BA660,
        ] {
            e.register(update, |_, _| Ret::default());
        }
        // 10 - (4 * 0.5) * 2 = 6, not below 6, and the limit 1 is above 0: not processed.
        assert!(!e.call(0x009b8b10, &args![this]).bool());
        assert_eq!(
            calls_at(&e, FLOAT_OF_00408840),
            vec![vec![2.0f32.to_bits()]]
        );
        assert!(calls_at(&e, PROJECTILE_PROCESS_IMPACTS).is_empty());
        // A bound above what is left: processed; state 2 runs 009b8c90.
        e.register(VALUE_00508070, |_, _| float(6.5));
        assert!(e.call(0x009b8b10, &args![this]).bool());
        assert_eq!(
            calls_at(&e, HOLDER_SET_009C8B90),
            vec![vec![HOLDER_011F2250, 0]]
        );
        assert!(calls_at(&e, HOLDER_SET_009C8C40).is_empty());
        assert_eq!(calls_at(&e, STATE_2_UPDATE_009B8C90), vec![vec![this]]);
        // A limit at 0 processes too; bit 1 clear skips the floats; state 5 runs nothing.
        e.register(VALUE_00644660, |_, _| float(0.0));
        e.register(VALUE_00508070, |_, _| float(0.0));
        assert!(e.call(0x009b8b10, &args![this]).bool());
        e.register(TEST_BIT_00444D00, |_, _| answer(0));
        e.mem.set_u32(this + STATE, 5);
        assert!(e.call(0x009b8b10, &args![this]).bool());
        assert_eq!(calls_at(&e, STATE_2_UPDATE_009B8C90).len(), 2);
        // The byte at +0x90 set: nothing.
        e.mem.set_u8(this + 0x90, 1);
        assert!(!e.call(0x009b8b10, &args![this]).bool());
    }
}
