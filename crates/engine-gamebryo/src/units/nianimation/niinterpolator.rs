//! `NiAnimation/niinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTERPOLATOR_RTTI: u32 = 0x011f_4200;

// Translated from 00a57070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTERPOLATOR_RTTI)
}

/// `00a5d610`: the jump target of `00a88a40`.
const JUMP_TARGET_00A88A40: u32 = 0x00a5_d610;

// Translated from 00a88a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a5d610` (engine map: `NiObject::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A88A40, &args![this, stream]);
}

/// The vtable of `NiInterpolator` (RTTI `.?AVNiInterpolator@@`).
const NI_INTERPOLATOR_VTABLE: u32 = 0x0109_a924;
/// The `float` the constructor gives `m_fLastTime`.
const INITIAL_LAST_TIME: u32 = 0x0101_5f5c;
/// `NiInterpolator::m_fLastTime` (Xbox PDB) +0x08.
const LAST_TIME: u32 = 0x08;
/// `NiObject::NiObject` / `~NiObject` (`00a5d3a0` / `00a5d3d0`, `NiMain/niobject.cpp`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;

// Translated from 00a56ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::NiInterpolator` (Xbox PDB): the `NiObject` constructor,
/// `m_fLastTime` from the `float` at `01015f5c` (an x87 `FLD`/`FSTP` copy),
/// then the `NiInterpolator` vtable. Returns `this`.
pub fn ni_interpolator_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    let last_time = e.global::<u32>(INITIAL_LAST_TIME);
    e.mem.set_u32(
        this.addr().wrapping_add(LAST_TIME),
        x87_quiet_bits(last_time),
    );
    e.mem.set_u32(this.addr(), NI_INTERPOLATOR_VTABLE);
    this
}

// Translated from 00a57010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::~NiInterpolator` (unnamed in the engine map): stores the
/// `NiInterpolator` vtable, then the `NiObject` destructor (`00a5d3d0`, a
/// tail jump).
pub fn ni_interpolator_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_INTERPOLATOR_VTABLE);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57070, ni_interpolator_get_rtti() -> Ptr),
        entry!(0x00a8_8a40, ni_interpolator_load_binary(Ptr, Ptr)),
        entry!(0x00a5_6ff0, ni_interpolator_construct(Ptr) -> Ptr),
        entry!(0x00a5_7010, ni_interpolator_destruct(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a57070, &args![]).u32(), NI_INTERPOLATOR_RTTI);
    }

    #[test]
    fn thunk_00a88a40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_8a40;
        const TARGET: u32 = 0x00a5_d610;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    #[test]
    fn constructor_and_destructor_set_the_vtable_and_last_time() {
        let mut e = Engine::new();
        let log = Log::default();
        for address in [NI_OBJECT_CONSTRUCT, NI_OBJECT_DESTRUCT] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.map(INITIAL_LAST_TIME, 4);
        // A signalling NaN shows the x87 copy quieting it.
        e.mem.set_u32(INITIAL_LAST_TIME, 0x7fa0_0001);
        let this = e.mem.alloc(0x0c);
        assert_eq!(e.call(0x00a5_6ff0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x7fe0_0001);
        e.mem.set_u32(INITIAL_LAST_TIME, (-f32::MAX).to_bits());
        e.call(0x00a5_6ff0, &args![this]);
        assert_eq!(e.mem.f32(this + 8), -f32::MAX);
        e.mem.set_u32(this, 0);
        e.call(0x00a5_7010, &args![this]);
        assert_eq!(e.mem.u32(this), NI_INTERPOLATOR_VTABLE);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_OBJECT_DESTRUCT, vec![this]),
            ]
        );
    }
}
