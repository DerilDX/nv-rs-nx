//! `NiParticle/nipsysdragmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c23cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_drag_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

// Translated from 00c2c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2768.
pub fn ni_p_sys_drag_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2768
}

use crate::units::niparticle::nipsysmodifier::{stream_read, READ_LINK_ID};

layout! {
    /// `NiPSysDragModifier` (Xbox PDB), 0x34 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysDragModifier: 0x34 {
        /// `m_pkDragObj` (Xbox PDB): `NiAVObject*`.
        0x18 m_pkDragObj: u32,
        /// `m_kDragAxis` (Xbox PDB): an `NiPoint3`.
        0x1C m_kDragAxis: u32,
        /// `m_fPercentage` (Xbox PDB).
        0x28 m_fPercentage: u32,
        /// `m_fRange` (Xbox PDB).
        0x2C m_fRange: u32,
        /// `m_fFalloff` (Xbox PDB).
        0x30 m_fFalloff: u32,
    }
}

/// The vtable of `NiPSysDragModifier` (RTTI `.?AVNiPSysDragModifier@@`,
/// `010bfe44`).
const VTABLE: u32 = 0x010b_fe44;
/// The constructor's defaults: the axis (three `float`s at `011a946c`,
/// copied by `MOV`), the percentage (the `float` at `010181bc`, 0.05) and
/// the range and falloff (the `float` at `010bfe38`, `FLT_MAX`), the two
/// floats through the x87 stack.
const DEFAULT_AXIS: u32 = 0x011a_946c;
const DEFAULT_PERCENTAGE: u32 = 0x0101_81bc;
const DEFAULT_RANGE: u32 = 0x010b_fe38;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`00c31720`) and `LoadBinary`
/// (`00c318b0`); `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the
/// colour, the stream) reads the axis.
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;

// Translated from 00c2c030 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project: the `NiPSysDragModifier`
/// constructor (`thiscall`; the stream factory `00c2c080` jumps to it). The
/// `NiPSysModifier` constructor, the vtable `010bfe44`, no drag object, the
/// axis from `011a946c`, the percentage from `010181bc`, the range and the
/// falloff from `010bfe38`. Returns `this`.
pub fn fn_00c2c030(e: &mut Engine, this: Ptr<NiPSysDragModifier>) -> Ptr<NiPSysDragModifier> {
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    let percentage = x87_quiet_bits(e.global::<u32>(DEFAULT_PERCENTAGE));
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, NiPSysDragModifier::m_pkDragObj, 0);
    for word in 0..3 {
        let value = e.global::<u32>(DEFAULT_AXIS + 4 * word);
        e.mem.set_u32(
            this.addr() + NiPSysDragModifier::m_kDragAxis.off + 4 * word,
            value,
        );
    }
    e.set(this, NiPSysDragModifier::m_fPercentage, percentage);
    let range = x87_quiet_bits(e.global::<u32>(DEFAULT_RANGE));
    e.set(this, NiPSysDragModifier::m_fRange, range);
    e.set(this, NiPSysDragModifier::m_fFalloff, range);
    this
}

// Translated from 00c2c080 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project (registered through `push
/// 00c1ab0a`): the `NiPSysDragModifier` stream factory (`cdecl`, no
/// arguments): a 0x34-byte object from `00aa13e0` given to the constructor
/// `00c2c030` (a tail jump); 0 when the allocation fails.
pub fn fn_00c2c080(e: &mut Engine) -> Ptr<NiPSysDragModifier> {
    let object = Ptr::<NiPSysDragModifier>::new(e.call(NI_NEW, &args![0x34u32]).u32());
    if object.is_null() {
        return Ptr::NULL;
    }
    fn_00c2c030(e, object)
}

// Translated from 00c2c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, a link id (`00a645b0`, the drag object), the axis
/// (`NiColor::LoadBinary`), then the percentage, range and falloff (4 bytes
/// each; the component-size word is the stream argument's stack word).
pub fn ni_p_sys_drag_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.call(READ_LINK_ID, &args![s]);
    e.call(
        COLOR_LOAD_BINARY,
        &args![this.addr() + NiPSysDragModifier::m_kDragAxis.off, s],
    );
    e.with_stack(4, |e, slot| {
        for field in [
            NiPSysDragModifier::m_fPercentage,
            NiPSysDragModifier::m_fRange,
            NiPSysDragModifier::m_fFalloff,
        ] {
            stream_read(e, s, this.addr() + field.off, 4, 4, slot.addr());
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_c030,
            fn_00c2c030(Ptr<NiPSysDragModifier>) -> Ptr<NiPSysDragModifier>
        ),
        entry!(0x00c2_c080, fn_00c2c080() -> Ptr<NiPSysDragModifier>),
        entry!(
            0x00c2_c450,
            ni_p_sys_drag_modifier_load_binary(Ptr<NiPSysDragModifier>, Ptr)
        ),
        entry!(0x00c2c0a0, ni_p_sys_drag_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c23cf0, ni_p_sys_drag_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_drag_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00c23cf0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c23cf0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_drag_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_c0a0, &args![]).u32(), 0x0120_2768);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    #[test]
    fn factory_constructs_the_defaults_or_answers_null() {
        let mut e = Engine::new();
        e.map(DEFAULT_AXIS & !0xfff, 0x1000);
        e.set_global(DEFAULT_AXIS, 1.0f32);
        e.map(DEFAULT_PERCENTAGE & !0xfff, 0x1000);
        e.set_global(DEFAULT_PERCENTAGE, 0.05f32);
        e.map(DEFAULT_RANGE & !0xfff, 0x1000);
        e.set_global(DEFAULT_RANGE, f32::MAX);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        let made = e.call(0x00c2_c080, &args![]).ptr::<NiPSysDragModifier>();
        assert_eq!(e.mem.u32(made.addr()), VTABLE);
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_kDragAxis),
            1.0f32.to_bits()
        );
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_fPercentage),
            0.05f32.to_bits()
        );
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_fRange),
            f32::MAX.to_bits()
        );
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_fFalloff),
            f32::MAX.to_bits()
        );
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_c080, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_link_axis_and_three_floats() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        let bytes: Vec<u8> = [0.5f32, 2.0, 3.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<NiPSysDragModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_c450, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == COLOR_LOAD_BINARY && c.1 == vec![this.addr() + 0x1c, stream]));
        assert!(log
            .iter()
            .any(|c| c.0 == READ_LINK_ID && c.1 == vec![stream]));
        assert_eq!(
            e.get(this, NiPSysDragModifier::m_fPercentage),
            0.5f32.to_bits()
        );
        assert_eq!(
            e.get(this, NiPSysDragModifier::m_fFalloff),
            3.0f32.to_bits()
        );
    }
}
