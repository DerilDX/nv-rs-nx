//! `fallout/misc/saveload/bgsloadformbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSLoadFormBuffer` (Xbox PDB), PC constructor confirms size 0x30.
    pub struct BGSLoadFormBuffer: 0x30 {
        0x10 iFormID: u32,
        0x20 iSize: u32,
        0x24 pForm: Ptr,
        0x28 iFlags: u32,
        0x2c iOldChangeFlags: u32,
    }
}

// Translated from 008643b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the base, derived header and flags, retaining header padding.
// Translation incomplete: 008643b0 - compiler exception-unwind cleanup remains omitted.
pub fn construct(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> Ptr<BGSLoadFormBuffer> {
    e.call(0x008646b0, &args![this]);
    e.mem.set_u32(this.addr(), 0x01082028);
    e.set(this, BGSLoadFormBuffer::iFormID, 0);
    construct_header(e, Ptr::new(this.addr().wrapping_add(0x14)));
    e.set(this, BGSLoadFormBuffer::iSize, 0);
    e.set(this, BGSLoadFormBuffer::pForm, Ptr::NULL);
    e.set(this, BGSLoadFormBuffer::iFlags, 0);
    e.call(0x0044dee0, &args![this.addr().wrapping_add(0x2c)]);
    this
}

// Translated from 00864450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `BGSSaveLoadFormHeader` (Xbox PDB), a nine-byte packed header:
/// three-byte FormIDIndex, iChangeFlags at +3, FormInfo at +7, cVersion at +8.
pub fn construct_header(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0084eaf0, &args![this]);
    e.call(0x0044dee0, &args![this.addr().wrapping_add(3)]);
    construct_form_info(e, Ptr::new(this.addr().wrapping_add(7)));
    e.mem.set_u8(this.addr().wrapping_add(8), 0);
    this
}

// Translated from 00864490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears only the header FormInfo byte (Xbox PDB) and returns the receiver.
pub fn construct_form_info(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    this
}

// Translated from 008645b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadFormBuffer::GetForm` (Xbox PDB): lazily resolves the form ID
/// when flag mask 2 allows lookup, then returns the current cached form.
pub fn get_form(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> Ptr {
    if e.get(this, BGSLoadFormBuffer::pForm).is_null() && allows_lookup(e, this) != 0 {
        let id = e.get(this, BGSLoadFormBuffer::iFormID);
        let form = e.call(0x004839c0, &args![id]).ptr();
        e.set(this, BGSLoadFormBuffer::pForm, form);
    }
    e.get(this, BGSLoadFormBuffer::pForm)
}

// Translated from 008645f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests mask 2 in the flags word.
pub fn allows_lookup(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> u8 {
    u8::from(e.get(this, BGSLoadFormBuffer::iFlags) & 2 != 0)
}

// Translated from 00864610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual GetForm, followed by the form's reference predicate (low byte).
pub fn get_reference(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> Ptr {
    let form = e.vcall(this.addr(), 4, &args![]).u32();
    if form != 0 && e.vcall(form, 0xf0, &args![]).u8() != 0 {
        Ptr::new(form)
    } else {
        Ptr::NULL
    }
}

// Translated from 00864660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual GetReference, followed by the reference's actor predicate.
pub fn get_actor(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> Ptr {
    let reference = e.vcall(this.addr(), 8, &args![]).u32();
    if reference != 0 && e.vcall(reference, 0x100, &args![]).u8() != 0 {
        Ptr::new(reference)
    } else {
        Ptr::NULL
    }
}

// Translated from 008a81c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadFormBuffer::GetVersion` (Xbox PDB): reads the packed header's
/// version byte. The shared PC body returns AL and has no stack arguments.
pub fn get_version(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x1c))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008643b0,
            construct(Ptr<BGSLoadFormBuffer>) -> Ptr<BGSLoadFormBuffer>
        ),
        entry!(0x00864450, construct_header(Ptr) -> Ptr),
        entry!(0x00864490, construct_form_info(Ptr) -> Ptr),
        entry!(0x008645b0, get_form(Ptr<BGSLoadFormBuffer>) -> Ptr),
        entry!(0x008645f0, allows_lookup(Ptr<BGSLoadFormBuffer>) -> u8),
        entry!(0x00864610, get_reference(Ptr<BGSLoadFormBuffer>) -> Ptr),
        entry!(0x00864660, get_actor(Ptr<BGSLoadFormBuffer>) -> Ptr),
        entry!(0x008a81c0, get_version(Ptr<BGSLoadFormBuffer>) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_getter_returns_the_header_byte_without_mutation() {
        let mut e = Engine::new();
        let guard = e.mem.alloc(0x38);
        let this = Ptr::<BGSLoadFormBuffer>::new(guard + 4);
        for version in [0u8, 1, 3, 0x7f, 0x80, 0xff] {
            e.mem.write(guard, &[0xa5; 0x38]);
            e.mem.set_u8(this.addr() + 0x1c, version);
            let before = e.mem.bytes(guard, 0x38);
            assert_eq!(e.call(0x008a81c0, &args![this]).u8(), version);
            assert_eq!(e.mem.bytes(guard, 0x38), before);
        }
    }

    #[test]
    fn constructor_clears_fields_but_preserves_packed_header_padding() {
        let mut e = Engine::new();
        let address = e.mem.alloc(0x34);
        e.mem.write(address, &[0xcc; 0x34]);
        let this = Ptr::<BGSLoadFormBuffer>::new(address);
        assert_eq!(e.call(0x008643b0, &args![this]).u32(), address);
        assert_eq!(e.mem.u32(address), 0x01082028);
        assert_eq!(e.mem.bytes(address + 4, 0x19), vec![0; 0x19]);
        assert_eq!(e.mem.bytes(address + 0x1d, 3), [0xcc; 3]);
        assert_eq!(e.mem.bytes(address + 0x20, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(address + 0x30), 0xcccccccc);
    }

    #[test]
    fn form_lookup_is_flag_gated_and_cached_after_callback() {
        let mut e = Engine::new();
        let this = e.new_object::<BGSLoadFormBuffer>();
        e.call(0x008643b0, &args![this]);
        e.set(this, BGSLoadFormBuffer::iFormID, 0x12345678);
        e.register_double(0x004839c0, move |e, a| {
            assert_eq!(a, &[0x12345678]);
            e.set(this, BGSLoadFormBuffer::pForm, Ptr::new(0x9999));
            0x1234u32.into_ret()
        });
        assert_eq!(e.call(0x008645b0, &args![this]).u32(), 0);
        e.set(this, BGSLoadFormBuffer::iFlags, 0x80000002);
        assert_eq!(e.call(0x008645f0, &args![this]).u8(), 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008645b0, &args![this]).u32(), 0x1234);
        e.set(this, BGSLoadFormBuffer::iFlags, 0);
        assert_eq!(e.call(0x008645b0, &args![this]).u32(), 0x1234);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x004839c0)
                .count(),
            1
        );
    }

    #[test]
    fn virtual_type_queries_use_low_byte_and_preserve_selected_pointer() {
        let mut e = Engine::new();
        let this = e.new_object::<BGSLoadFormBuffer>();
        let form = e.mem.alloc(4);
        let buffer_vtable = e.mem.alloc(16);
        let form_vtable = e.mem.alloc(0x104);
        e.mem.set_u32(this.addr(), buffer_vtable);
        e.mem.set_u32(form, form_vtable);
        e.mem.set_u32(buffer_vtable + 4, 0x008645b0);
        e.mem.set_u32(buffer_vtable + 8, 0x00864610);
        e.mem.set_u32(form_vtable + 0xf0, 0x71000001);
        e.mem.set_u32(form_vtable + 0x100, 0x71000002);
        e.set(this, BGSLoadFormBuffer::pForm, Ptr::new(form));
        e.register_double(0x71000001, move |e, a| {
            assert_eq!(a, &[form]);
            e.set(this, BGSLoadFormBuffer::pForm, Ptr::NULL);
            0x101u32.into_ret()
        });
        e.register_double(0x71000002, move |_, a| {
            assert_eq!(a, &[form]);
            0x100u32.into_ret()
        });
        assert_eq!(e.call(0x00864610, &args![this]).u32(), form);
        assert_eq!(e.call(0x00864610, &args![this]).u32(), 0);
        e.set(this, BGSLoadFormBuffer::pForm, Ptr::new(form));
        assert_eq!(e.call(0x00864660, &args![this]).u32(), 0);
        e.set(this, BGSLoadFormBuffer::pForm, Ptr::new(form));
        e.register_double(0x71000002, |_, _| 0xffffff01u32.into_ret());
        assert_eq!(e.call(0x00864660, &args![this]).u32(), form);
    }
}
