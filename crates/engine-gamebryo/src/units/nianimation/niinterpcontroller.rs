//! `NiAnimation/niinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTERP_CONTROLLER_RTTI: u32 = 0x011f_36b4;

// Translated from 00a2f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTERP_CONTROLLER_RTTI)
}

/// `NiTimeController::LoadBinary` and `NiTimeController::LinkObject` (`thiscall`, the stream).
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;

// Translated from 00a2f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::LoadBinary` (Xbox PDB): `NiTimeController::LoadBinary`; files of
/// versions `0x0a010068` up to (not including) `0x0a01006d` (`NiStream` +0xd8) follow with one
/// byte (read into the stream argument's stack word, the byte-count word a local set to 1):
/// not 0 sets flag bit 5 (`0x20`) of the controller's flags (+8), 0 clears it.
pub fn ni_interp_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let version = e.get(stream, NiStream::m_uiNifFileVersion);
    if !(0x0a01_0068..0x0a01_006d).contains(&version) {
        return;
    }
    let set = e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let count = slots.addr() + 4;
        let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        e.mem.set_u32(count, 1);
        let done = e
            .call(function, &args![input, argument, 1u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
        e.mem.u8(argument) != 0
    });
    // NiTimeController::m_uFlags (Xbox PDB) +0x08
    let flags = e.mem.u16(this.addr() + 8);
    let flags = if set { flags | 0x20 } else { flags & 0xffdf };
    e.mem.set_u16(this.addr() + 8, flags);
}

// Translated from 00c9a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::LinkObject` (Xbox PDB): a jump to `NiTimeController::LinkObject`
/// (`00a6d500`), passing on `this` and the stream.
pub fn ni_interp_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2f8e0, ni_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a2_f910,
            ni_interp_controller_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00c9_a6b0,
            ni_interp_controller_link_object(Ptr, Ptr<NiStream>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2f8e0, &args![]).u32(),
            NI_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn load_binary_reads_the_flag_byte_of_middle_versions() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0000);
        let mut answers = vec![0u8, 1u8];
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], answers.pop().unwrap());
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 8, 0x0101);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_0068);
        e.call(0x00a2_f910, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 8), 0x0121);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_006c);
        e.call(0x00a2_f910, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 8), 0x0101);
        // Other versions read nothing more.
        for version in [0x0a01_0067u32, 0x0a01_006d] {
            e.set(stream, NiStream::m_uiNifFileVersion, version);
            e.call(0x00a2_f910, &args![this, stream]);
        }
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 2);
    }

    #[test]
    fn link_object_jumps_to_the_time_controller() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c9_a6b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == TIME_CONTROLLER_LINK_OBJECT)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000, 0x2000]);
    }
}
