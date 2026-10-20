//! `NiAnimation/nicolorinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiColorInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLOR_INTERPOLATOR_RTTI: u32 = 0x011f_3730;

// Translated from 00a3bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_color_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLOR_INTERPOLATOR_RTTI)
}

/// The vtable of `NiColorInterpolator` (RTTI `.?AVNiColorInterpolator@@`).
const NI_COLOR_INTERPOLATOR_VTABLE: u32 = 0x0109_6e14;
/// The colour the constructor starts from and `Update` compares with
/// (`011a9430`, four `float`s; the Xbox PDB's
/// `NiInterpolator::INVALID_COLORA` by its use, not confirmed).
const INVALID_COLOR: u32 = 0x011a_9430;
/// `NiInterpolator::m_fLastTime` (Xbox PDB) +0x08.
const LAST_TIME: u32 = 0x08;
/// `NiColorInterpolator` (Xbox PDB): `m_kColorValue` +0x0c (`NiColorA`,
/// 16 bytes), `m_spColorData` +0x1c (`NiPointer<NiColorData>`),
/// `m_uiLastIdx` +0x20.
const COLOR_VALUE: u32 = 0x0c;
const COLOR_DATA: u32 = 0x1c;
const LAST_INDEX: u32 = 0x20;
/// `NiColorData` (Xbox PDB): `m_uiNumKeys` +0x08, `m_pkKeys` +0x0c,
/// `m_eType` +0x10, `m_ucKeySize` +0x14.
const DATA_NUM_KEYS: u32 = 0x08;
const DATA_KEYS: u32 = 0x0c;
const DATA_KEY_TYPE: u32 = 0x10;
const DATA_KEY_SIZE: u32 = 0x14;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`).
const NI_KEY_BASED_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_ca10;
/// Import slot of `InterlockedIncrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `004a4cc0` (`thiscall` on a four-`float` value, `RET 4`): whether it
/// equals the four floats at its argument (true in `AL`).
const COLOR_EQUALS: u32 = 0x004a_4cc0;
/// `NiColorKey::GenInterp` (engine map, `00a296c0`, `cdecl`, 7 words): the
/// colour at a time (result pointer, time, keys, key type, key count,
/// address of the last index, key size); returns the result pointer.
const COLOR_KEY_GEN_INTERP: u32 = 0x00a2_96c0;

/// Copies the four words of a colour.
fn copy_color(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8, 12] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 00a3bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::NiColorInterpolator(NiColorData*)` (by the
/// vtable it stores; unnamed in the engine map): the
/// `NiKeyBasedInterpolator` constructor, the vtable, `m_kColorValue` copied
/// (as words) from `011a9430`, `m_spColorData = data` (its count
/// incremented when non-null) and `m_uiLastIdx = 0`. Returns `this`.
pub fn ni_color_interpolator_construct(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_COLOR_INTERPOLATOR_VTABLE);
    copy_color(e, base.wrapping_add(COLOR_VALUE), INVALID_COLOR);
    e.mem.set_u32(base.wrapping_add(COLOR_DATA), data.addr());
    if data.addr() != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data.addr().wrapping_add(4)]);
    }
    e.mem.set_u32(base.wrapping_add(LAST_INDEX), 0);
    this
}

// Translated from 00a3bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::Update` (Xbox PDB, `bool Update(float fTime,
/// NiObjectNET*, NiColorA& kValue)`). At the time of the last update
/// (`time == m_fLastTime`, an x87 `FUCOMP`: NaN is not equal) it copies
/// `m_kColorValue` into `value` and returns whether that differs from the
/// colour at `011a9430`. Otherwise, with colour data holding keys, the
/// colour at `time` (`NiColorKey::GenInterp`, the time passed as the
/// `float` an x87 `FSTP` stores) becomes `m_kColorValue`; then, when
/// `m_kColorValue` equals the colour at `011a9430` it returns false,
/// else copies it into `value`, stores `time` as `m_fLastTime` (an x87
/// copy) and returns true. The key-size word pushed for `GenInterp` holds
/// the size byte in its low byte; the exe leaves the upper three bytes of
/// that stack word as they were (not modelled: they are passed as zero).
pub fn ni_color_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: Ptr,
    value: Ptr,
) -> bool {
    let base = this.addr();
    let color = base.wrapping_add(COLOR_VALUE);
    let last_time = e.mem.f32(base.wrapping_add(LAST_TIME));
    if time == last_time {
        copy_color(e, value.addr(), color);
        return !e.call(COLOR_EQUALS, &args![color, INVALID_COLOR]).bool();
    }
    let data = e.mem.u32(base.wrapping_add(COLOR_DATA));
    if data != 0 {
        let num_keys = e.mem.u32(data.wrapping_add(DATA_NUM_KEYS));
        let key_size = e.mem.u8(data.wrapping_add(DATA_KEY_SIZE));
        let key_type = e.mem.u32(data.wrapping_add(DATA_KEY_TYPE));
        let keys = e.mem.u32(data.wrapping_add(DATA_KEYS));
        if num_keys != 0 {
            e.with_stack(16, |e, result| {
                let interpolated = e
                    .call(
                        COLOR_KEY_GEN_INTERP,
                        &args![
                            result,
                            x87_quiet_bits(time.to_bits()),
                            keys,
                            key_type,
                            num_keys,
                            base.wrapping_add(LAST_INDEX),
                            u32::from(key_size)
                        ],
                    )
                    .u32();
                copy_color(e, color, interpolated);
            });
        }
    }
    if e.call(COLOR_EQUALS, &args![color, INVALID_COLOR]).bool() {
        return false;
    }
    copy_color(e, value.addr(), color);
    e.mem
        .set_u32(base.wrapping_add(LAST_TIME), x87_quiet_bits(time.to_bits()));
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3bd70, ni_color_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a3_bd10,
            ni_color_interpolator_construct(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a3_bfb0,
            ni_color_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_color_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3bd70, &args![]).u32(),
            NI_COLOR_INTERPOLATOR_RTTI
        );
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const INVALID: [f32; 4] = [-f32::MAX, -f32::MAX, -f32::MAX, -f32::MAX];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn colors(e: &Engine, at: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.f32(at + offset))
    }

    fn set_colors(e: &mut Engine, at: u32, values: [f32; 4]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// An interpolator (last time 1.0, colour `start`) with data holding
    /// `num_keys` keys; `GenInterp` writes `interpolated`.
    fn fixture(num_keys: u32, start: [f32; 4], interpolated: [f32; 4]) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(INVALID_COLOR, 16);
        set_colors(&mut e, INVALID_COLOR, INVALID);
        e.register(COLOR_EQUALS, |e, a| {
            ret((colors(e, a[0]) == colors(e, a[1])) as u32)
        });
        let l = log.clone();
        e.register_double(COLOR_KEY_GEN_INTERP, move |e, a| {
            l.borrow_mut().push((COLOR_KEY_GEN_INTERP, a[1..].to_vec()));
            set_colors(e, a[0], interpolated);
            ret(a[0])
        });
        let this = e.mem.alloc(0x24);
        e.mem.set_f32(this + LAST_TIME, 1.0);
        set_colors(&mut e, this + COLOR_VALUE, start);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + DATA_NUM_KEYS, num_keys);
        e.mem.set_u32(data + DATA_KEYS, 0x7000);
        e.mem.set_u32(data + DATA_KEY_TYPE, 2);
        e.mem.set_u8(data + DATA_KEY_SIZE, 0x14);
        e.mem.set_u32(this + COLOR_DATA, data);
        let value = e.mem.alloc(16);
        (e, log, this, value)
    }

    #[test]
    fn constructor_starts_from_the_invalid_colour_and_holds_the_data() {
        let mut e = Engine::new();
        e.register(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.map(INVALID_COLOR, 16);
        // Word copies: a signalling NaN stays as it is.
        e.mem.set_u32(INVALID_COLOR, 0x7fa0_0001);
        e.mem.set_u32(INVALID_COLOR + 12, 0x1234_5678);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + LAST_INDEX, 3);
        let data = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00a3_bd10, &args![this, data]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_COLOR_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + COLOR_VALUE), 0x7fa0_0001);
        assert_eq!(e.mem.u32(this + COLOR_VALUE + 12), 0x1234_5678);
        assert_eq!(e.mem.u32(this + COLOR_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(this + LAST_INDEX), 0);
        e.call(0x00a3_bd10, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + COLOR_DATA), 0);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn update_at_the_last_time_copies_the_current_colour() {
        let current = [0.5, 0.25, 1.0, 1.0];
        let (mut e, log, this, value) = fixture(3, current, [0.0; 4]);
        assert!(e
            .call(0x00a3_bfb0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(colors(&e, value), current);
        assert!(log.borrow().is_empty());
        // The same time with the invalid colour answers false, still copied.
        set_colors(&mut e, this + COLOR_VALUE, INVALID);
        assert!(!e
            .call(0x00a3_bfb0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(colors(&e, value), INVALID);
    }

    #[test]
    fn update_at_a_new_time_interpolates_the_keys() {
        let new = [0.1, 0.2, 0.3, 0.4];
        let (mut e, log, this, value) = fixture(3, [0.0; 4], new);
        assert!(e
            .call(0x00a3_bfb0, &args![this, 2.5f32, 0u32, value])
            .bool());
        assert_eq!(
            *log.borrow(),
            [(
                COLOR_KEY_GEN_INTERP,
                vec![2.5f32.to_bits(), 0x7000, 2, 3, this + LAST_INDEX, 0x14]
            )]
        );
        assert_eq!(colors(&e, this + COLOR_VALUE), new);
        assert_eq!(colors(&e, value), new);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.5);
    }

    #[test]
    fn update_with_a_nan_time_interpolates_with_the_quieted_time() {
        let new = [0.1, 0.2, 0.3, 0.4];
        let (mut e, log, this, value) = fixture(3, [0.0; 4], new);
        e.mem.set_u32(this + LAST_TIME, 0x7fa0_0000);
        let time = f32::from_bits(0x7fa0_0000);
        assert!(e.call(0x00a3_bfb0, &args![this, time, 0u32, value]).bool());
        assert_eq!(log.borrow()[0].1[0], 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + LAST_TIME), 0x7fe0_0000);
    }

    #[test]
    fn update_without_keys_keeps_the_colour() {
        let current = [0.5, 0.5, 0.5, 1.0];
        let (mut e, log, this, value) = fixture(0, current, [0.0; 4]);
        assert!(e
            .call(0x00a3_bfb0, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert!(log.borrow().is_empty());
        assert_eq!(colors(&e, value), current);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        // No data at all, and the invalid colour: false, nothing stored.
        e.mem.set_u32(this + COLOR_DATA, 0);
        set_colors(&mut e, this + COLOR_VALUE, INVALID);
        set_colors(&mut e, value, [9.0; 4]);
        assert!(!e
            .call(0x00a3_bfb0, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(colors(&e, value), [9.0; 4]);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
    }
}
