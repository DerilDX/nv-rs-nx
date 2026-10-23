//! `NiParticle/nipsysupdatectlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nitimecontroller::NiTimeController;

// Translated from 00c27140 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a6d290`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00c27140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_d290;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

// Translated from 00c27130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2690.
pub fn ni_p_sys_update_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2690
}

/// `00aa13e0` (`cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00a6d220`: the `NiTimeController` constructor (`thiscall`).
const TIME_CONTROLLER_CONSTRUCTOR: u32 = 0x00a6_d220;
/// `00a6cfe0`: `NiTimeController`'s member copy, `thiscall`, `(clone,
/// process)`.
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// The vtable of `NiPSysUpdateCtlr` (RTTI `.?AVNiPSysUpdateCtlr@@`).
const VTABLE: u32 = 0x010b_e6b4;
/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, one `float`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `NiPSysEmitterCtlr::ms_RTTI` (the class's `NiRTTI` record).
const NI_P_SYS_EMITTER_CTLR_RTTI: u32 = 0x0120_24e8;
/// The `float` the update stores as the low key time first (`ff7fffff`,
/// `-FLT_MAX`, in the exe's data).
const LO_KEY_TIME_START: u32 = 0x0101_5f5c;
/// The `float` it stores as the high key time first (`010be6a8`).
const HI_KEY_TIME_START: u32 = 0x010b_e6a8;
/// The `double` the last time is compared with (`010241b0`, `-FLT_MAX`):
/// no time yet.
const NO_TIME_DOUBLE: u32 = 0x0102_41b0;
/// `NiObjectNET::m_spControllers` (Xbox PDB) +0x0C.
const TARGET_CONTROLLERS: u32 = 0x0c;
/// `NiTimeController::m_spNext` (Xbox PDB) +0x30.
const CONTROLLER_NEXT: u32 = 0x30;
/// `NiTimeController::m_uFlags` (Xbox PDB) +0x08 of the emitter
/// controller; `NiPSysModifierCtlr::m_pkModifier` (Xbox PDB) +0x3C, its
/// emitter; `NiPSysEmitter::m_fLifeSpan` (Xbox PDB) +0x48.
const CONTROLLER_FLAGS: u32 = 0x08;
const CONTROLLER_MODIFIER: u32 = 0x3c;
const EMITTER_LIFE_SPAN: u32 = 0x48;
/// `NiObject::GetRTTI` (vtable slot +0x08); `NiRTTI::m_pkBaseRTTI` +4.
const GET_RTTI_SLOT: u32 = 0x08;
const RTTI_BASE: u32 = 0x04;
/// The target's vtable slot +0xF0 (`NiParticleSystem::UpdateSystem` in the
/// Xbox PDB), one `float`.
const UPDATE_SYSTEM_SLOT: u32 = 0xf0;

// Translated from 00c27160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::CreateClone` (Xbox PDB): a 0x34-byte object from
/// `00aa13e0` through the `NiTimeController` constructor, its low and high
/// key times set to 0 and this class's vtable (0 when the allocation fails),
/// then `NiTimeController`'s member copy into it (which still runs for a
/// null clone). Returns the clone.
pub fn ni_p_sys_update_ctlr_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x34u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(TIME_CONTROLLER_CONSTRUCTOR, &args![block]);
        let clone: Ptr<NiTimeController> = Ptr::new(block);
        // `FLDZ; FST [+0x18]; FSTP [+0x14]`.
        e.set(clone, NiTimeController::m_fHiKeyTime, 0);
        e.set(clone, NiTimeController::m_fLoKeyTime, 0);
        e.mem.set_u32(block, VTABLE);
        Ptr::new(block)
    };
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

// Translated from 00c271c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::Update` (Xbox PDB). The PC code takes the time by
/// address (`time` points at a `float`; the Xbox PDB declares a `float`).
/// The key times become the two start values of the exe's data and the
/// flags lose bit 1 and gain bit 2. The target's controller list is searched
/// for the first controller whose RTTI chain contains `NiPSysEmitterCtlr`'s.
/// When the last time is not `-FLT_MAX` (or is NaN), such a controller was
/// found, its flags have neither bit 1 nor bit 2, and its emitter's life
/// span is below the time elapsed since the last time, the last time jumps
/// to `time`. Then `DontDoUpdate` with the time, both key times 0, and,
/// when the target is set and the update is not skipped, the target's
/// system update (slot +0xF0) with the scaled time. The target's controller
/// list is read without a null check on the target.
pub fn ni_p_sys_update_ctlr_update(e: &mut Engine, this: Ptr<NiTimeController>, time: Ptr) {
    let lo_start = x87_quiet_bits(e.global::<u32>(LO_KEY_TIME_START));
    e.set(this, NiTimeController::m_fLoKeyTime, lo_start);
    let hi_start = x87_quiet_bits(e.global::<u32>(HI_KEY_TIME_START));
    e.set(this, NiTimeController::m_fHiKeyTime, hi_start);
    let flags = e.get(this, NiTimeController::m_uFlags);
    e.set(this, NiTimeController::m_uFlags, (flags & 0xfffd) | 4);

    let target = e.get(this, NiTimeController::m_pkTarget);
    let mut controller = e.mem.u32(target + TARGET_CONTROLLERS);
    let mut emitter_ctlr = 0u32;
    let mut emitter = 0u32;
    'search: while controller != 0 {
        let mut rtti = e.vcall(controller, GET_RTTI_SLOT, &[]).u32();
        while rtti != 0 {
            if rtti == NI_P_SYS_EMITTER_CTLR_RTTI {
                emitter = e.mem.u32(controller + CONTROLLER_MODIFIER);
                emitter_ctlr = controller;
                break 'search;
            }
            rtti = e.mem.u32(rtti + RTTI_BASE);
        }
        controller = e.mem.u32(controller + CONTROLLER_NEXT);
    }

    // `FCOMP double; TEST AH,0x44; JNP`: on when not equal or unordered.
    let last = e.get(this, NiTimeController::m_fLastTime);
    let no_time = f64::from_bits(e.global::<u64>(NO_TIME_DOUBLE));
    if !(x87_load32(last) == no_time)
        && emitter_ctlr != 0
        && e.mem.u8(emitter_ctlr + CONTROLLER_FLAGS) & 6 == 0
    {
        let now = e.mem.u32(time.addr());
        let last = e.get(this, NiTimeController::m_fLastTime);
        let elapsed = x87_sub(x87_load32(now), x87_m32(last));
        let life_span = x87_load32(e.mem.u32(emitter + EMITTER_LIFE_SPAN));
        // `FLD life; FCOMPP; TEST AH,5; JP`: on when the life span is below.
        if life_span < elapsed {
            let now = e.mem.u32(time.addr());
            e.set(this, NiTimeController::m_fLastTime, x87_quiet_bits(now));
        }
    }

    let now = x87_quiet_bits(e.mem.u32(time.addr()));
    let skip = e.call(DONT_DO_UPDATE, &args![this, now]).bool();
    e.set(this, NiTimeController::m_fHiKeyTime, 0);
    e.set(this, NiTimeController::m_fLoKeyTime, 0);
    let target = e.get(this, NiTimeController::m_pkTarget);
    if target != 0 && !skip {
        let scaled = x87_quiet_bits(e.get(this, NiTimeController::m_fScaledTime));
        e.vcall(target, UPDATE_SYSTEM_SLOT, &args![scaled]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c27130, ni_p_sys_update_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00c27140, fn_00c27140(Ptr, u32) -> Ptr),
        entry!(0x00c2_7160, ni_p_sys_update_ctlr_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00c2_71c0,
            ni_p_sys_update_ctlr_update(Ptr<NiTimeController>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c27140_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_d290, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c27140, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c27140, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00a6_d290), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_update_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_7130, &args![]).u32(), 0x0120_2690);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_clone_constructs_clears_the_key_times_and_copies() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(TIME_CONTROLLER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x7f7f_ffff);
            e.mem.set_u32(a[0] + 0x18, 0xff7f_ffff);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_7160, &args![0x5000u32, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.u32(clone + 0x14), 0);
        assert_eq!(e.mem.u32(clone + 0x18), 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_7160, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }

    /// A target with one unrelated controller and then an emitter
    /// controller (RTTI chain: own record, then `NiPSysEmitterCtlr`'s), the
    /// emitter's life span `life`; the updated controller's last time
    /// `last`. Returns (engine, this, time slot, emitter controller).
    fn update_world(life: f32, last: f32, skip: bool) -> (Engine, Ptr<NiTimeController>, u32, u32) {
        const OTHER_GET_RTTI: u32 = 0x0040_2000;
        const EMITTER_GET_RTTI: u32 = 0x0040_2010;
        const UPDATE_SYSTEM: u32 = 0x0040_2020;
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1000);
        e.map(0x010b_e000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        e.set_global(LO_KEY_TIME_START, 0xff7f_ffffu32);
        e.set_global(HI_KEY_TIME_START, 0x7f7f_ffffu32);
        e.set_global(NO_TIME_DOUBLE, -(f32::MAX as f64));
        // The emitter controller's RTTI: a record whose base is the
        // emitter controller's.
        let own_rtti = e.mem.alloc(8);
        e.mem
            .set_u32(own_rtti + RTTI_BASE, NI_P_SYS_EMITTER_CTLR_RTTI);
        e.register(OTHER_GET_RTTI, |_, _| Ret::default());
        e.register_double(EMITTER_GET_RTTI, move |_, _| Ret {
            eax: own_rtti,
            ..Ret::default()
        });
        e.register(UPDATE_SYSTEM, |_, _| Ret::default());
        let skip_word = u32::from(skip);
        e.register_double(DONT_DO_UPDATE, move |_, _| Ret {
            eax: skip_word,
            ..Ret::default()
        });
        e.put_vtable(0x0090_0000, &[0, 0, OTHER_GET_RTTI]);
        e.put_vtable(0x0090_1000, &[0, 0, EMITTER_GET_RTTI]);
        let target_vtable = 0x0090_2000;
        e.put_vtable(target_vtable, &[0; 0xf4 / 4]);
        e.mem
            .set_u32(target_vtable + UPDATE_SYSTEM_SLOT, UPDATE_SYSTEM);

        let emitter = e.mem.alloc(0x60);
        e.mem.set_f32(emitter + EMITTER_LIFE_SPAN, life);
        let emitter_ctlr = e.mem.alloc(0x40);
        e.mem.set_u32(emitter_ctlr, 0x0090_1000);
        e.mem.set_u32(emitter_ctlr + CONTROLLER_MODIFIER, emitter);
        let other = e.mem.alloc(0x34);
        e.mem.set_u32(other, 0x0090_0000);
        e.mem.set_u32(other + CONTROLLER_NEXT, emitter_ctlr);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target, target_vtable);
        e.mem.set_u32(target + TARGET_CONTROLLERS, other);

        let this: Ptr<NiTimeController> = e.new_object();
        e.set(this, NiTimeController::m_pkTarget, target);
        e.set(this, NiTimeController::m_uFlags, 0x0003);
        e.set(this, NiTimeController::m_fLastTime, last.to_bits());
        e.set(this, NiTimeController::m_fScaledTime, 2.5f32.to_bits());
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 10.0);
        (e, this, time, emitter_ctlr)
    }

    #[test]
    fn update_jumps_the_last_time_past_the_emitter_life_span_and_updates() {
        let (mut e, this, time, _) = update_world(1.0, 5.0, false);
        e.call_log = Some(vec![]);
        e.call(0x00c2_71c0, &args![this, time]);
        // 10 - 5 > 1: the last time becomes 10.
        assert_eq!(
            e.get(this, NiTimeController::m_fLastTime),
            10.0f32.to_bits()
        );
        assert_eq!(e.get(this, NiTimeController::m_uFlags), 0x0005);
        assert_eq!(e.get(this, NiTimeController::m_fLoKeyTime), 0);
        assert_eq!(e.get(this, NiTimeController::m_fHiKeyTime), 0);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this.addr(), 10.0f32.to_bits()]]
        );
        let target = e.get(this, NiTimeController::m_pkTarget);
        assert_eq!(
            calls_to(&e, 0x0040_2020),
            vec![vec![target, 2.5f32.to_bits()]]
        );
    }

    #[test]
    fn update_keeps_the_last_time_within_the_life_span_or_without_a_last_time() {
        // Within the life span (10 - 5 < 6).
        let (mut e, this, time, _) = update_world(6.0, 5.0, false);
        e.call(0x00c2_71c0, &args![this, time]);
        assert_eq!(e.get(this, NiTimeController::m_fLastTime), 5.0f32.to_bits());
        // No last time yet (-FLT_MAX).
        let (mut e, this, time, _) = update_world(1.0, -f32::MAX, false);
        e.call(0x00c2_71c0, &args![this, time]);
        assert_eq!(
            e.get(this, NiTimeController::m_fLastTime),
            (-f32::MAX).to_bits()
        );
        // The emitter controller's flags have bit 2.
        let (mut e, this, time, emitter_ctlr) = update_world(1.0, 5.0, false);
        e.mem.set_u16(emitter_ctlr + CONTROLLER_FLAGS, 4);
        e.call(0x00c2_71c0, &args![this, time]);
        assert_eq!(e.get(this, NiTimeController::m_fLastTime), 5.0f32.to_bits());
    }

    #[test]
    fn update_skipped_by_dont_do_update_does_not_update_the_system() {
        let (mut e, this, time, _) = update_world(1.0, 5.0, true);
        e.call_log = Some(vec![]);
        e.call(0x00c2_71c0, &args![this, time]);
        assert!(calls_to(&e, 0x0040_2020).is_empty());
        assert_eq!(e.get(this, NiTimeController::m_fLoKeyTime), 0);
    }
}
