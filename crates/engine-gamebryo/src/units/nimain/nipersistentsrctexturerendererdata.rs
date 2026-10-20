//! `NiMain/nipersistentsrctexturerendererdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a98480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPersistentSrcTextureRendererData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a98240`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x94 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_persistent_src_texture_renderer_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_8240;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x94u32]);
    }
    this
}

/// `00a63520` (`cdecl`, one word; unit `NiMain/nistream.cpp`).
const UNREGISTER_NAME: u32 = 0x00a6_3520;
/// `00a64900` (`cdecl`, two words; engine map `NiStream::RegisterLoader`).
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// The class-name string both calls pass.
const CLASS_NAME: u32 = 0x0109_f0a0;
/// The creation function `00a98460` registers (`00a982d0`).
const CREATE_FUNCTION: u32 = 0x00a9_82d0;

// Translated from 00a98460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nipersistentsrctexturerendererdata.cpp`, no arguments):
/// passes the class-name string at `0109f0a0` to `00a63520`, then registers it with the
/// creation function `00a982d0` (`NiStream::RegisterLoader`, `00a64900`).
pub fn fn_00a98460(e: &mut Engine) {
    e.call(UNREGISTER_NAME, &args![CLASS_NAME]);
    e.call(REGISTER_LOADER, &args![CLASS_NAME, CREATE_FUNCTION]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a98480, ni_persistent_src_texture_renderer_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_8460, fn_00a98460()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_persistent_src_texture_renderer_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_8240, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a98480, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a98480, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x94]]);
        assert_eq!(calls_to(&e, 0x00a9_8240), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a98460_reregisters_the_loader() {
        let mut e = Engine::new();
        e.register(UNREGISTER_NAME, |_, _| Ret::default());
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a9_8460, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (UNREGISTER_NAME, vec![CLASS_NAME]),
                (REGISTER_LOADER, vec![CLASS_NAME, CREATE_FUNCTION]),
            ]
        );
    }
}
