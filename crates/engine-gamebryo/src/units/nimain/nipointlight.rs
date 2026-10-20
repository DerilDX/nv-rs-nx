//! `NiMain/nipointlight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_POINT_LIGHT_RTTI: u32 = 0x011f_4a98;

// Translated from 00a7d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_point_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POINT_LIGHT_RTTI)
}

/// `NiAVObject::UpdateWorldData` (`00a68c60`, thiscall, update data).
const NI_AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
/// `NiLight::LoadBinary` (`00a72080`, thiscall, stream).
const NI_LIGHT_LOAD_BINARY: u32 = 0x00a7_2080;
/// `NiStream`'s input file (+0x24C): read function at +8, position at +4.
const STREAM_INPUT: u32 = 0x24c;

// Translated from 00a7d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::UpdateWorldData` (Xbox PDB name in the engine map):
/// `NiAVObject::UpdateWorldData`, then increments the word at +0xA8.
pub fn ni_point_light_update_world_data(e: &mut Engine, this: Ptr, update: Ptr) {
    e.call(NI_AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update]);
    let at = this.addr().wrapping_add(0xa8);
    let count = e.mem.u32(at).wrapping_add(1);
    e.mem.set_u32(at, count);
}

// Translated from 00a7d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight::LoadBinary` (Xbox PDB name in the engine map):
/// `NiLight::LoadBinary`, then reads the three 4-byte attenuations
/// (+0xF0, +0xF4, +0xF8) through the stream input's read function (cdecl:
/// file, destination, 4, byte-count word, 1), each adding the bytes read to
/// the input's position. The stream argument's stack word is the
/// byte-count word.
pub fn ni_point_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_LIGHT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0xf0u32, 0xf4, 0xf8] {
            let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
            let read = e.mem.u32(input.wrapping_add(8));
            e.mem.set_u32(count.addr(), 4);
            let destination = this.addr().wrapping_add(offset);
            let done = e
                .call(read, &args![input, destination, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(input.wrapping_add(4)).wrapping_add(done);
            e.mem.set_u32(input.wrapping_add(4), position);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7d790, ni_point_light_get_rtti() -> Ptr),
        entry!(0x00a7_d7a0, ni_point_light_update_world_data(Ptr, Ptr)),
        entry!(0x00a7_d840, ni_point_light_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_point_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7d790, &args![]).u32(), NI_POINT_LIGHT_RTTI);
    }

    #[test]
    fn update_world_data_bumps_the_counter_at_a8() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0xfc);
        e.mem.set_u32(this + 0xa8, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00a7_d7a0, &args![this, 0x4444u32]);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (NI_AV_OBJECT_UPDATE_WORLD_DATA, vec![this, 0x4444])
        );
    }

    #[test]
    fn load_binary_reads_the_three_attenuations() {
        let mut e = Engine::new();
        e.register(NI_LIGHT_LOAD_BINARY, |_, _| Ret::default());
        // The read function fills the destination with 4 bytes of 0x11 * n
        // and answers 4.
        let mut n = 0u8;
        e.register_double(0x7000_0008, move |e, a| {
            n += 1;
            assert_eq!(e.mem.u32(a[3]), 4);
            assert_eq!(a[4], 1);
            e.mem.write(a[1], &[0x11 * n; 4]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        e.mem.set_u32(input + 4, 0x100);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0xfc);
        e.call(0x00a7_d840, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0xf0), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0xf4), 0x2222_2222);
        assert_eq!(e.mem.u32(this + 0xf8), 0x3333_3333);
        assert_eq!(e.mem.u32(input + 4), 0x10c);
    }
}
