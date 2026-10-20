//! `NiMain/nifloatextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiFloatExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_EXTRA_DATA_RTTI: u32 = 0x011f_5f40;

// Translated from 00a9a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_EXTRA_DATA_RTTI)
}

/// `NiFloatExtraData`'s vtable.
const NI_FLOAT_EXTRA_DATA_VTABLE: u32 = 0x010a_0ec4;
/// `operator new` (`cdecl`, the size) and the sized delete (`cdecl`, (block, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiExtraData`'s constructor and destructor body (`thiscall`, no stack words), its member
/// copy (`00a7b330`, (clone, cloning process)) and `NiExtraData::LoadBinary` (the stream).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;

/// The extra data's float (`m_fValue`, Xbox PDB) at +0xc.
const VALUE: u32 = 0xc;

// Translated from 00a99d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary`, then the 4-byte
/// value (+0xc) from the stream's input (the stream argument's stack word is the byte-count
/// word).
pub fn ni_float_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(
            function,
            &args![input, this.addr() + VALUE, 4u32, count, 1u32],
        )
        .u32()
    });
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// Allocates and builds an empty float extra data (0x10 bytes, `operator new`, the
/// `NiExtraData` constructor, value 0, the vtable `010a0ec4`); null when the allocation
/// fails.
fn new_float_extra_data(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block + VALUE, x87_store32(0.0));
    e.mem.set_u32(block, NI_FLOAT_EXTRA_DATA_VTABLE);
    block
}

// Translated from 00a9a480 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiFloatExtraData`): a new empty float extra data (value 0), or null.
pub fn fn_00a9a480(e: &mut Engine) -> Ptr {
    Ptr::new(new_float_extra_data(e))
}

// Translated from 00a9a430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::CreateClone` (Xbox PDB): a new empty float extra data (null when the
/// allocation fails), `NiExtraData`'s member copy into it (`00a7b330`), then this value
/// copied to its +0xc (written at 0xc for a null clone, as the exe does). Answers the clone.
pub fn ni_float_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_float_extra_data(e);
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    let value = x87_store32(x87_load32(e.mem.u32(this.addr() + VALUE)));
    e.mem.set_u32(clone.wrapping_add(VALUE), value);
    Ptr::new(clone)
}

// Translated from 00a9a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `010a0ec4`, runs the `NiExtraData` destructor body (`00a7b300`) and, when bit 0 of `flags`
/// is set, the sized delete of 0x10 bytes. Answers `this`.
pub fn ni_float_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_FLOAT_EXTRA_DATA_VTABLE);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9a420, ni_float_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00a9_9d30,
            ni_float_extra_data_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(0x00a9_a480, fn_00a9a480() -> Ptr),
        entry!(0x00a9_a430, ni_float_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a9_a4b0,
            ni_float_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_float_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9a420, &args![]).u32(), NI_FLOAT_EXTRA_DATA_RTTI);
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x10]);
            block.into_ret()
        });
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_LOAD_BINARY,
            SIZED_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn load_binary_reads_the_value() {
        let mut e = engine();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_f32(a[1], 2.5);
            4u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call(0x00a9_9d30, &args![this, stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LOAD_BINARY),
            vec![vec![this, stream.addr()]]
        );
        assert_eq!(e.mem.f32(this + 0xc), 2.5);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 4);
    }

    #[test]
    fn create_object_builds_an_empty_value() {
        let mut e = engine();
        let object = e.call(0x00a9_a480, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_FLOAT_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 4), 0xeeee_eeee);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_a480, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0xc, -1.5);
        let clone = e.call(0x00a9_a430, &args![this, 0x77u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_FLOAT_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.f32(clone + 0xc), -1.5);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a9_a4b0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_FLOAT_EXTRA_DATA_VTABLE);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x10]]);
        e.call(0x00a9_a4b0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT).len(), 2);
    }
}
