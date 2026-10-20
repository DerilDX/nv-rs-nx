//! `BSShader/imagespacetexture.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ImageSpaceTexture` (Xbox PDB), 0x10 bytes; the PC code reads the same offsets.
    pub struct ImageSpaceTexture: 0x10 {
        /// `bForceAniso` (Xbox PDB).
        0x00 bForceAniso: u8,
        /// `bIsRenderedTexture` (Xbox PDB).
        0x01 bIsRenderedTexture: u8,
        /// `bIsTextureBorrowed` (Xbox PDB).
        0x02 bIsTextureBorrowed: u8,
        /// `pTexture` (Xbox PDB): a `NiPointer<NiObject>`.
        0x04 pTexture: u32,
        /// `iFilterMode` (Xbox PDB).
        0x08 iFilterMode: u32,
        /// `iClampMode` (Xbox PDB).
        0x0C iClampMode: u32,
    }
}

/// `ImageSpaceTexture::ReturnRenderedTexture` (engine map, `00ba39a0`, `thiscall`, no stack
/// words).
const RETURN_RENDERED_TEXTURE: u32 = 0x00ba_39a0;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Releases one reference of the `NiRefObject` at `object` (count at +4): its vtable slot +4
/// when the count reaches 0.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00ba3a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3a00 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2a7fb, state variable) is not translated
/// `ImageSpaceTexture::~ImageSpaceTexture` (Xbox PDB), `thiscall`: a borrowed texture
/// (`bIsTextureBorrowed`) is returned first (`ReturnRenderedTexture`); then the texture
/// pointer is released and cleared, and the `NiPointer` member's own destructor releases
/// what the member holds after that (it re-reads the member, which the code just cleared).
pub fn image_space_texture_destructor(e: &mut Engine, this: Ptr<ImageSpaceTexture>) {
    if e.get(this, ImageSpaceTexture::bIsTextureBorrowed) != 0 {
        e.call(RETURN_RENDERED_TEXTURE, &args![this]);
    }
    let texture = e.get(this, ImageSpaceTexture::pTexture);
    if texture != 0 {
        release(e, texture);
        e.set(this, ImageSpaceTexture::pTexture, 0);
    }
    let texture = e.get(this, ImageSpaceTexture::pTexture);
    if texture != 0 {
        release(e, texture);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ba_3a00,
        image_space_texture_destructor(Ptr<ImageSpaceTexture>)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_2000;
    const DELETE_THIS: u32 = 0x0f00_2004;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig(borrowed: u8, refs: u32) -> (Engine, Ptr<ImageSpaceTexture>, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(RETURN_RENDERED_TEXTURE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE_THIS]);
        e.register(DELETE_THIS, |_, _| Ret::default());
        let texture = e.mem.alloc(8);
        e.mem.set_u32(texture, VTABLE);
        e.mem.set_u32(texture + 4, refs);
        let this = e.new_object::<ImageSpaceTexture>();
        e.set(this, ImageSpaceTexture::bIsTextureBorrowed, borrowed);
        e.set(this, ImageSpaceTexture::pTexture, texture);
        e.call_log = Some(vec![]);
        (e, this, texture)
    }

    #[test]
    fn fn_00ba3a00_returns_a_borrowed_texture_and_releases_the_pointer() {
        let (mut e, this, texture) = rig(1, 1);
        e.call(0x00ba_3a00, &args![this]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![texture]]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![texture + 4]]);
    }

    #[test]
    fn fn_00ba3a00_keeps_a_held_texture_and_skips_an_empty_pointer() {
        let (mut e, this, texture) = rig(0, 2);
        e.call(0x00ba_3a00, &args![this]);
        assert!(calls(&e, RETURN_RENDERED_TEXTURE).is_empty());
        assert!(calls(&e, DELETE_THIS).is_empty());
        assert_eq!(e.mem.u32(texture + 4), 1);
        e.call(0x00ba_3a00, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fn_00ba3a00_clears_what_the_release_put_back() {
        let (mut e, this, texture) = rig(0, 1);
        let other = e.mem.alloc(8);
        // The object's last release stores another texture in the member; the clear after
        // it wins, so nothing more is released.
        e.register_double(DELETE_THIS, move |e, _| {
            e.mem.set_u32(this.addr() + 4, other);
            Ret::default()
        });
        e.call(0x00ba_3a00, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![texture + 4]]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
    }
}
