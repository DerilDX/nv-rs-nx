//! `NiAnimation/run_00a42b70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObjectNET::ms_RTTI` (the `NiRTTI` record whose name is
/// `"NiObjectNET"`, `0109b2e0`).
const NI_OBJECT_NET_RTTI: u32 = 0x011f_4304;
/// `NiObject::GetRTTI` (Xbox PDB vtable slot +8).
const GET_RTTI_SLOT: u32 = 0x8;
/// `NiRTTI::m_pkBaseRTTI` (Xbox PDB) +4.
const RTTI_BASE: u32 = 0x4;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// The first file version this conversion no longer applies to.
const CONVERSION_END_VERSION: u32 = 0x0a01_0068;
/// The conversion run on the object (`00a42b70`, `cdecl (stream, object)`,
/// untranslated).
const CONVERT: u32 = 0x00a4_2b70;

// Translated from 00a42c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl (stream, object)`): for files before
/// version `0x0A010068` and a non-null object whose class derives from
/// `NiObjectNET` (its `GetRTTI` record, vtable +8, or one of its bases at
/// +4, is `NiObjectNET`'s), runs `00a42b70` on them.
pub fn fn_00a42c80(e: &mut Engine, stream: Ptr, object: Ptr) {
    if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) >= CONVERSION_END_VERSION
        || object.is_null()
    {
        return;
    }
    let mut rtti = e.vcall(object.addr(), GET_RTTI_SLOT, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_OBJECT_NET_RTTI {
            e.call(CONVERT, &args![stream, object]);
            return;
        }
        rtti = e.mem.u32(rtti.wrapping_add(RTTI_BASE));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a42c80, fn_00a42c80(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (version, object present, RTTI chain contains NiObjectNET) -> converts.
    #[test]
    fn converts_old_object_net_derived_objects_only() {
        const GET_RTTI: u32 = 0x7200_0000;
        for (version, present, derived, converts) in [
            (0x0a01_0067u32, true, true, true),
            (0x0a01_0068, true, true, false),
            (0x0a01_0067, false, true, false),
            (0x0a01_0067, true, false, false),
        ] {
            let mut e = Engine::new();
            e.put_vtable(0x7000_0000, &[0, 0, GET_RTTI]);
            e.map(0x7300_0000, 4);
            let base = e.mem.alloc(8);
            let own = e.mem.alloc(8);
            e.mem.set_u32(own + 4, base);
            if derived {
                e.mem.set_u32(base + 4, NI_OBJECT_NET_RTTI);
            }
            e.mem.set_u32(0x7300_0000, own);
            e.register(GET_RTTI, |e, _| Ret {
                eax: e.mem.u32(0x7300_0000),
                ..Ret::default()
            });
            e.register(CONVERT, |_, _| Ret::default());
            let stream = e.mem.alloc(0x5c4);
            e.mem.set_u32(stream + 0xd8, version);
            let object = if present { e.mem.alloc(0x18) } else { 0 };
            if present {
                e.mem.set_u32(object, 0x7000_0000);
            }
            e.call_log = Some(vec![]);
            e.call(0x00a42c80, &args![stream, object]);
            let converted: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == CONVERT)
                .map(|c| c.1)
                .collect();
            let expected = if converts {
                vec![vec![stream, object]]
            } else {
                vec![]
            };
            assert_eq!(converted, expected);
        }
    }
}
