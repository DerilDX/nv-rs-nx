//! `NiAnimation/nifloatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiFloatInterpolator` (0x18 bytes, the `CreateClone` allocation; PC
//! offsets as the code uses them): `NiKeyBasedInterpolator` (the last
//! update time at +0x08), then the current value `m_fValue` +0x0c, the
//! `NiFloatData` pointer `m_spFloatData` +0x10 and the last key index
//! `m_uiLastIdx` +0x14 (Xbox PDB names).
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;

/// The vtable of `NiFloatInterpolator` (RTTI `.?AVNiFloatInterpolator@@`).
const NI_FLOAT_INTERPOLATOR_VTABLE: u32 = 0x0109_6f34;
/// `NiFloatInterpolator` size (the `CreateClone` allocation).
const NI_FLOAT_INTERPOLATOR_SIZE: u32 = 0x18;

/// The last update time (`NiInterpolator`), `m_fValue`, `m_spFloatData`,
/// `m_uiLastIdx` (see the module comment).
const LAST_TIME: u32 = 0x08;
const VALUE: u32 = 0x0c;
const FLOAT_DATA: u32 = 0x10;
const LAST_INDEX: u32 = 0x14;

/// The "no value" float (`-FLT_MAX` in the exe, read from `.rdata`).
const INVALID_FLOAT: u32 = 0x0109_a918;

/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`).
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
/// `00aa0ab0` (unnamed in the engine map; `thiscall`, `RET 8`): the
/// `NiKeyBasedInterpolator` member copy `(this, destination, cloning)`.
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`).
const KEY_BASED_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// `NiFloatKey::GenInterp` (`00a26b40`, `cdecl`, six words: time, keys,
/// key type, key count, `&last index`, key size); returns the value in ST0.
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiStream::ResolveLinkID` (`00a64620`, `thiscall` on the stream).
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis` (virtual slot 1).
const SLOT_DELETE_THIS: u32 = 0x04;

/// `NiStream::m_pkIstr` +0x24c (Xbox PDB; PC offset), and the input
/// stream's position (+4) and read function (+8, `cdecl` `(stream, buffer,
/// size, &component size, component count)`, returns the bytes read).
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer at `slot`: when `new`
/// differs, the old one (if set) is released, `new` stored and (if set)
/// counted.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
}

// Translated from 00a3c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_INTERPOLATOR_RTTI)
}

// Translated from 00a3c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::NiFloatInterpolator` (Xbox PDB), the constructor
/// taking a pose value: the `NiKeyBasedInterpolator` constructor, then the
/// value (an x87 load and store), the vtable, no float data and last index
/// 0. Returns `this`.
pub fn ni_float_interpolator_construct(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    let object = this.addr();
    e.call(KEY_BASED_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(object.wrapping_add(VALUE), x87_quiet_bits(value.to_bits()));
    e.mem.set_u32(object, NI_FLOAT_INTERPOLATOR_VTABLE);
    e.mem.set_u32(object.wrapping_add(FLOAT_DATA), 0);
    e.mem.set_u32(object.wrapping_add(LAST_INDEX), 0);
    this
}

// Translated from 00a3c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::Update` (Xbox PDB): at the time of the last update
/// the stored value goes to `out` and the result is whether it is valid
/// (not the "no value" float at `0109a918`). At another time, with float
/// data that has keys, `NiFloatKey::GenInterp` computes the value (stored
/// as a float); an invalid value returns false and leaves `out` and the
/// time alone, a valid one goes to `out` and the time is kept. The second
/// argument word is not read.
///
/// The key size word pushed for `GenInterp` is the stack word that first
/// held `this` (`PUSH ECX`) with its low byte replaced by the key size.
pub fn ni_float_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _unused_2: u32,
    out: Ptr,
) -> bool {
    let this = this.addr();
    let time_bits = x87_quiet_bits(time.to_bits());
    let last_time = x87_load32(e.mem.u32(this.wrapping_add(LAST_TIME)));
    if x87_load32(time_bits) == last_time {
        let value = e.mem.u32(this.wrapping_add(VALUE));
        e.mem.set_u32(out.addr(), x87_quiet_bits(value));
        let value = x87_load32(e.mem.u32(this.wrapping_add(VALUE)));
        let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
        return invalid != value;
    }
    let data = e.mem.u32(this.wrapping_add(FLOAT_DATA));
    if data != 0 {
        let num_keys = e.mem.u32(data.wrapping_add(8));
        let key_size = e.mem.u8(data.wrapping_add(0x14));
        let key_type = e.mem.u32(data.wrapping_add(0x10));
        let keys = e.mem.u32(data.wrapping_add(0xc));
        if num_keys != 0 {
            let key_size_word = (this & 0xffff_ff00) | u32::from(key_size);
            let value = e
                .call(
                    FLOAT_KEY_GEN_INTERP,
                    &args![
                        time_bits,
                        keys,
                        key_type,
                        num_keys,
                        this.wrapping_add(LAST_INDEX),
                        key_size_word
                    ],
                )
                .f64();
            e.mem.set_u32(this.wrapping_add(VALUE), x87_store32(value));
        }
    }
    let value = x87_load32(e.mem.u32(this.wrapping_add(VALUE)));
    let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
    if invalid == value {
        return false;
    }
    let value = e.mem.u32(this.wrapping_add(VALUE));
    e.mem.set_u32(out.addr(), x87_quiet_bits(value));
    e.mem.set_u32(this.wrapping_add(LAST_TIME), time_bits);
    true
}

// Translated from 00a3c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiFloatInterpolator`'s member copy into
/// `destination` for a clone made by `cloning` (the function `CreateClone`
/// runs). The `NiKeyBasedInterpolator` copy (`00aa0ab0`), then the value
/// (an x87 load and store), the float data pointer (counted; the old one
/// released) and the last index.
pub fn ni_float_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr,
    destination: Ptr,
    cloning: u32,
) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, destination, cloning]);
    let this = this.addr();
    let destination = destination.addr();
    let value = e.mem.u32(this.wrapping_add(VALUE));
    e.mem
        .set_u32(destination.wrapping_add(VALUE), x87_quiet_bits(value));
    let old = e.mem.u32(destination.wrapping_add(FLOAT_DATA));
    if old != e.mem.u32(this.wrapping_add(FLOAT_DATA)) {
        if old != 0 {
            release(e, old);
        }
        let new = e.mem.u32(this.wrapping_add(FLOAT_DATA));
        e.mem.set_u32(destination.wrapping_add(FLOAT_DATA), new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
    let last_index = e.mem.u32(this.wrapping_add(LAST_INDEX));
    e.mem
        .set_u32(destination.wrapping_add(LAST_INDEX), last_index);
}

// Translated from 00a3c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::LoadBinary` (Xbox PDB): the
/// `NiKeyBasedInterpolator` load, the value (4 bytes, the component size in
/// the argument's own stack word), then the float data resolved from its
/// link ID (`NiStream::ResolveLinkID`) and stored counted.
pub fn ni_float_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(KEY_BASED_LOAD_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream);
        stream_read(e, stream, this.wrapping_add(VALUE), 4, 4, slot.addr());
    });
    let data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.wrapping_add(FLOAT_DATA), data);
}

// Translated from 00a3c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::CreateClone` (Xbox PDB): allocates 0x18 bytes;
/// when that succeeds runs the `NiKeyBasedInterpolator` constructor, stores
/// the vtable, the "no value" float, no float data and last index 0. Then
/// the member copy into it, and returns it. When the allocation fails the
/// member copy still runs, into address 0, and 0 is returned.
pub fn ni_float_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_FLOAT_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        ni_float_interpolator_copy_members(e, this, Ptr::new(0), cloning);
        return Ptr::new(0);
    }
    e.call(KEY_BASED_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_FLOAT_INTERPOLATOR_VTABLE);
    let invalid = e.mem.u32(INVALID_FLOAT);
    e.mem
        .set_u32(object.wrapping_add(VALUE), x87_quiet_bits(invalid));
    e.mem.set_u32(object.wrapping_add(FLOAT_DATA), 0);
    e.mem.set_u32(object.wrapping_add(LAST_INDEX), 0);
    ni_float_interpolator_copy_members(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3c380, ni_float_interpolator_get_rtti() -> Ptr),
        entry!(0x00a3_c390, ni_float_interpolator_construct(Ptr, f32) -> Ptr),
        entry!(0x00a3_c6d0, ni_float_interpolator_update(Ptr, f32, u32, Ptr) -> bool),
        entry!(
            0x00a3_c780,
            ni_float_interpolator_copy_members(Ptr, Ptr, u32)
        ),
        entry!(0x00a3_c800, ni_float_interpolator_load_binary(Ptr, Ptr)),
        entry!(0x00a3_c8d0, ni_float_interpolator_create_clone(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const READ: u32 = 0x00ee_0400;
    const DELETE_THIS: u32 = 0x00ee_0500;
    const INVALID: f32 = -f32::MAX;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    /// The `Interlocked` import slots, counting in memory.
    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        interlocked(&mut e);
        e.map(INVALID_FLOAT, 4);
        e.mem.set_f32(INVALID_FLOAT, INVALID);
        e
    }

    /// A reference-counted object with count `count` whose `DeleteThis` is
    /// `DELETE_THIS`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn ni_float_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3c380, &args![]).u32(),
            NI_FLOAT_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn construct_sets_the_value_vtable_and_clears_the_data() {
        let mut e = engine();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_CONSTRUCT, 0);
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xaa; 0x18]);
        let back = e.call(0x00a3_c390, &args![this, 2.5f32]).u32();
        assert_eq!(back, this);
        assert_eq!(*log.borrow(), [(KEY_BASED_CONSTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this), NI_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(this + VALUE), 2.5);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(e.mem.u32(this + LAST_INDEX), 0);
        // A signalling NaN pose value is quieted by the x87 load.
        e.call(0x00a3_c390, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + VALUE), 0x7fc0_0001);
    }

    #[test]
    fn update_at_the_last_time_reports_the_stored_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let out = e.mem.alloc(4);
        e.mem.set_f32(this + LAST_TIME, 1.5);
        e.mem.set_f32(this + VALUE, 7.0);
        assert!(e.call(0x00a3_c6d0, &args![this, 1.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 7.0);
        // The invalid value still goes to `out`, but the result is false.
        e.mem.set_f32(this + VALUE, INVALID);
        assert!(!e.call(0x00a3_c6d0, &args![this, 1.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), INVALID);
    }

    #[test]
    fn update_at_a_new_time_interpolates_the_keys() {
        let mut e = engine();
        let log = Log::default();
        let l = log.clone();
        e.register_double(FLOAT_KEY_GEN_INTERP, move |_, a| {
            l.borrow_mut().push((FLOAT_KEY_GEN_INTERP, a.to_vec()));
            Ret {
                st0: 3.25,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x18);
        let data = e.mem.alloc(0x18);
        let out = e.mem.alloc(4);
        e.mem.set_u32(data + 8, 4);
        e.mem.set_u32(data + 0xc, 0x6000);
        e.mem.set_u32(data + 0x10, 2);
        e.mem.set_u8(data + 0x14, 0x14);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_f32(this + LAST_TIME, 1.0);
        assert!(e.call(0x00a3_c6d0, &args![this, 2.0f32, 0u32, out]).bool());
        assert_eq!(
            *log.borrow(),
            [(
                FLOAT_KEY_GEN_INTERP,
                vec![
                    2.0f32.to_bits(),
                    0x6000,
                    2,
                    4,
                    this + LAST_INDEX,
                    (this & 0xffff_ff00) | 0x14
                ]
            )]
        );
        assert_eq!(e.mem.f32(this + VALUE), 3.25);
        assert_eq!(e.mem.f32(out), 3.25);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);

        // No keys: the stored value is reported unchanged; an invalid one
        // leaves `out` and the time alone.
        e.mem.set_u32(data + 8, 0);
        e.mem.set_f32(this + VALUE, INVALID);
        e.mem.set_f32(out, 9.0);
        assert!(!e.call(0x00a3_c6d0, &args![this, 5.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 9.0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        assert_eq!(log.borrow().len(), 1);
        // No data at all, a valid value.
        e.mem.set_u32(this + FLOAT_DATA, 0);
        e.mem.set_f32(this + VALUE, 6.0);
        assert!(e.call(0x00a3_c6d0, &args![this, 5.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 6.0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 5.0);
    }

    #[test]
    fn copy_members_copies_value_index_and_counts_the_data() {
        let mut e = engine();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        logged(&mut e, &log, DELETE_THIS, 0);
        let this = e.mem.alloc(0x18);
        let destination = e.mem.alloc(0x18);
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.mem.set_f32(this + VALUE, 4.0);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_u32(this + LAST_INDEX, 3);
        e.mem.set_u32(destination + FLOAT_DATA, old);
        e.call(0x00a3_c780, &args![this, destination, 0x9000u32]);
        assert_eq!(
            *log.borrow(),
            [
                (KEY_BASED_COPY_MEMBERS, vec![this, destination, 0x9000]),
                (DELETE_THIS, vec![old]),
            ]
        );
        assert_eq!(e.mem.f32(destination + VALUE), 4.0);
        assert_eq!(e.mem.u32(destination + FLOAT_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u32(destination + LAST_INDEX), 3);
        // Same data already: no count change.
        e.call(0x00a3_c780, &args![this, destination, 0x9000u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    #[test]
    fn load_binary_reads_the_value_and_resolves_the_data() {
        let mut e = engine();
        let log = Log::default();
        logged(&mut e, &log, KEY_BASED_LOAD_BINARY, 0);
        let data = counted(&mut e, 0);
        logged(&mut e, &log, RESOLVE_LINK_ID, data);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[0], a[1], a[2], e.mem.u32(a[3]), a[4]]));
            e.mem.set_f32(a[1], 0.75);
            ret(4)
        });
        let this = e.mem.alloc(0x18);
        e.call(0x00a3_c800, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (KEY_BASED_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, this + VALUE, 4, 4, 1]),
                (RESOLVE_LINK_ID, vec![stream]),
            ]
        );
        assert_eq!(e.mem.f32(this + VALUE), 0.75);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn create_clone_builds_a_copy_or_copies_into_null() {
        let mut e = engine();
        let log = Log::default();
        let block = e.mem.alloc(0x18);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, KEY_BASED_CONSTRUCT, 0);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + VALUE, 1.25);
        e.mem.set_u32(this + LAST_INDEX, 7);
        assert_eq!(e.call(0x00a3_c8d0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(block + VALUE), 1.25);
        assert_eq!(e.mem.u32(block + LAST_INDEX), 7);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x18]),
                (KEY_BASED_CONSTRUCT, vec![block]),
                (KEY_BASED_COPY_MEMBERS, vec![this, block, 0x9000]),
            ]
        );
        // A failed allocation copies into address 0.
        e.map(0, 0x100);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a3_c8d0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x18]),
                (KEY_BASED_COPY_MEMBERS, vec![this, 0, 0x9000]),
            ]
        );
        assert_eq!(e.mem.f32(VALUE), 1.25);
    }
}
