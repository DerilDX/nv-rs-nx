//! `BSMain/bsmasterparticlesystem.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

// Translated from 00c5eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c5db90` and frees 0xe0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_master_particle_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c5_db90, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xe0u32]);
    }
    this
}

// Translated from 00c5ec90 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0120436c without reading the receiver. The map names this body `BSMasterParticleSystem::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c5ec90(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_436c
}

layout! {
    /// `BSMasterParticleSystem` (Xbox PDB; every field 0x24 lower on PC, the
    /// `NiNode` base being smaller): the fields its functions use. 0xE0
    /// bytes, the size its `CreateObject` allocates.
    pub struct BSMasterParticleSystem: 0xE0 {
        /// `lEmitterObjList` (Xbox PDB): `NiTPointerList<NiPointer<NiAVObject>>`;
        /// its head node (nodes: next at +0, item at +8), tail +4, count +8.
        0xAC lEmitterObjList: u32,
        /// `usActiveEmitterObjCount` (Xbox PDB).
        0xB8 usActiveEmitterObjCount: u16,
        /// `usMaxEmitterObj` (Xbox PDB).
        0xBA usMaxEmitterObj: u16,
        /// `FirstEmitterObjIter` (Xbox PDB): a list node.
        0xC0 FirstEmitterObjIter: u32,
        /// `fMasterPSysTime` (Xbox PDB).
        0xDC fMasterPSysTime: f32,
    }
}

/// `ChildParticles` (Xbox PDB): the `NiTPrimitiveArray<NiParticles*>` at
/// +0xCC (`NiTArray` layout).
const CHILD_PARTICLES: u32 = 0xcc;
/// The count word of `lEmitterObjList` (+0xB4).
const EMITTER_LIST_COUNT: u32 = 0xb4;

/// `NiTPointerListBase<..>::RemoveAll` (`00e74d40`, engine map; `thiscall`).
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// A block free (`00aa10f0`, `cdecl`, one word; no Xbox PDB name).
const NI_FREE: u32 = 0x00aa_10f0;
/// A block allocation (`00aa1070`, `cdecl`, the size; no Xbox PDB name).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiNew` allocation (`00aa13e0`, `cdecl`, the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The base destructor `00a5e930` (`thiscall`; no Xbox PDB name).
const NODE_DESTRUCTOR_00A5E930: u32 = 0x00a5_e930;
/// The vtable of `BSMasterParticleSystem` (RTTI
/// `.?AVBSMasterParticleSystem@@`).
const VTABLE: u32 = 0x010c_2fb4;
/// The vtable of `NiTArray<NiParticles*, NiTMallocInterface<..>>` (RTTI
/// `.?AV?$NiTArray@PAVNiParticles@@...`).
const PARTICLES_ARRAY_VTABLE: u32 = 0x010c_30b8;
/// `00c5bfb0` (`thiscall` on the list, two words: out pointer, the address
/// of the cursor): takes the cursor's node out of the list.
const LIST_REMOVE_AT_00C5BFB0: u32 = 0x00c5_bfb0;
/// `InterlockedDecrement` (import slot `00fdf1c4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiNode::UpdateDownwardPass` (`00a5dd70`, engine map; `thiscall`, two
/// words).
const NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
/// The constructor `00c5da70` (unit `BSMain/run_00c5da70`).
const CONSTRUCTOR_00C5DA70: u32 = 0x00c5_da70;
/// `NiNode::LoadBinary` (`00a5dd40`, engine map; `thiscall`, one word).
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `00a30370` (`cdecl`: stream, address): loads a 16-bit value.
const STREAM_LOAD_SHORT_00A30370: u32 = 0x00a3_0370;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`, engine map; `thiscall`).
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// `NiNode::LinkObject` (`00a5e9d0`, engine map; `thiscall`, one word).
const NODE_LINK_OBJECT: u32 = 0x00a5_e9d0;
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`, engine map; `thiscall`).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
/// `00c4c920` (`thiscall` on the array, one word: the size; no Xbox PDB
/// name).
const ARRAY_SET_SIZE_00C4C920: u32 = 0x00c4_c920;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, engine map; `thiscall`).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `0096ae90` (`thiscall` on the array, two words: index, the address of
/// the element; no Xbox PDB name).
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// `NiObjectNET::PostLinkObject` (`00a5c6a0`, engine map; `thiscall`, one
/// word).
const OBJECT_NET_POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// `BSMasterParticleSystem::UpdateParticleSystemList` (`00c5de70`, engine
/// map; `thiscall`).
const UPDATE_PARTICLE_SYSTEM_LIST: u32 = 0x00c5_de70;

/// The stream's file version word (+4).
fn stream_version(e: &Engine, stream: Ptr) -> u32 {
    e.mem.u32(stream.addr() + 4)
}

// Translated from 00c5db90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5db90 - the C++ exception unwinding frame (FS:[0], handler 00f2fdcb) is not translated
/// The `BSMasterParticleSystem` destructor (`thiscall`; no Xbox PDB name;
/// the scalar deleting destructor `00c5eb20` runs it): stores the class
/// vtable, clears the cursor, empties the emitter list (`RemoveAll`),
/// clears and frees the child array at +0xCC (its elements nulled, its
/// size and used count cleared, the array's vtable stored, the elements
/// freed), empties the list again as its destructor does, and runs the base
/// destructor `00a5e930`. The exception-unwinding frame is not translated.
pub fn fn_00c5db90(e: &mut Engine, this: Ptr<BSMasterParticleSystem>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0);
    e.call(LIST_REMOVE_ALL, &args![base + 0xac]);
    let array = base + CHILD_PARTICLES;
    let mut index: u16 = 0;
    while index < e.mem.u16(array + 0xa) {
        let elements = e.mem.u32(array + 4);
        e.mem
            .set_u32(elements.wrapping_add(u32::from(index) * 4), 0);
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    e.mem.set_u32(array, PARTICLES_ARRAY_VTABLE);
    let elements = e.mem.u32(array + 4);
    e.call(NI_FREE, &args![elements]);
    e.call(LIST_REMOVE_ALL, &args![base + 0xac]);
    e.call(NODE_DESTRUCTOR_00A5E930, &args![this]);
}

// Translated from 00c5e080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::ClearUnreferencedEmitters` (Xbox PDB, matched by
/// the engine map; `thiscall`): walks the emitter list from the cursor (the
/// head when unset, wrapping to the head at the end) for at most the list's
/// count of steps. An emitter with a reference count (+4) of 1 or bit 0 of
/// its +0x30 flags set is taken out (`00c5bfb0`) and released (reference
/// count decremented, deleted through its vtable +4 at zero); the others are
/// counted, the cursor saved where the count reaches the maximum. The saved
/// cursor is restored; the count becomes the smaller of the count and the
/// maximum, and a zero count clears the cursor.
pub fn bs_master_particle_system_clear_unreferenced_emitters(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
) {
    let base = this.addr();
    e.set(this, BSMasterParticleSystem::usActiveEmitterObjCount, 0);
    if e.get(this, BSMasterParticleSystem::FirstEmitterObjIter) == 0 {
        let head = e.get(this, BSMasterParticleSystem::lEmitterObjList);
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, head);
    }
    let mut saved = 0u32;
    let mut steps = e.mem.u32(base + EMITTER_LIST_COUNT);
    e.with_stack(4, |e, removed| {
        while e.get(this, BSMasterParticleSystem::FirstEmitterObjIter) != 0 {
            let left = steps;
            steps = steps.wrapping_sub(1);
            if left == 0 {
                break;
            }
            let node = e.get(this, BSMasterParticleSystem::FirstEmitterObjIter);
            let emitter = e.mem.u32(node + 8);
            let references = e.mem.u32(emitter + 4);
            if references == 1 || e.mem.u32(emitter + 0x30) & 1 != 0 {
                e.call(
                    LIST_REMOVE_AT_00C5BFB0,
                    &args![base + 0xac, removed, base + 0xc0],
                );
                let object = e.mem.u32(removed.addr());
                if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
                    e.vcall(object, 4, &args![]);
                }
            } else {
                if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount)
                    == e.get(this, BSMasterParticleSystem::usMaxEmitterObj)
                {
                    saved = e.get(this, BSMasterParticleSystem::FirstEmitterObjIter);
                }
                let count = e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount);
                e.set(
                    this,
                    BSMasterParticleSystem::usActiveEmitterObjCount,
                    count.wrapping_add(1),
                );
                let node = e.get(this, BSMasterParticleSystem::FirstEmitterObjIter);
                let next = if node != 0 { e.mem.u32(node) } else { 0 };
                e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, next);
            }
            if e.get(this, BSMasterParticleSystem::FirstEmitterObjIter) == 0 {
                let head = e.get(this, BSMasterParticleSystem::lEmitterObjList);
                e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, head);
            }
        }
    });
    if saved != 0 {
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, saved);
    }
    let maximum = e.get(this, BSMasterParticleSystem::usMaxEmitterObj);
    let count = e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount);
    let kept = if count < maximum { count } else { maximum };
    e.set(this, BSMasterParticleSystem::usActiveEmitterObjCount, kept);
    if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount) == 0 {
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0);
    }
}

// Translated from 00c5e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::UpdateDownwardPass` (Xbox PDB, matched by the
/// engine map; `thiscall (this; time, flags)`, `time` the address of a
/// `float`): clears unreferenced emitters, then, with emitters left or a
/// child with active particles, adds the time to the system's own time
/// (+0xDC) and runs `NiNode::UpdateDownwardPass` with that accumulated time
/// in `*time`, restoring `*time` afterwards.
pub fn bs_master_particle_system_update_downward_pass(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    time: Ptr,
    flags: u32,
) {
    bs_master_particle_system_clear_unreferenced_emitters(e, this);
    if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount) == 0
        && !bs_master_particle_system_child_has_active_particles(e, this)
    {
        return;
    }
    let own = this.addr() + 0xdc;
    let sum = x87_add(x87_load32(e.mem.u32(own)), x87_m32(e.mem.u32(time.addr())));
    e.mem.set_u32(own, x87_store32(sum));
    let saved = x87_quiet_bits(e.mem.u32(time.addr()));
    let accumulated = x87_quiet_bits(e.mem.u32(own));
    e.mem.set_u32(time.addr(), accumulated);
    e.call(NODE_UPDATE_DOWNWARD_PASS, &args![this, time, flags]);
    e.mem.set_u32(time.addr(), saved);
}

// Translated from 00c5e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5e5a0 - the C++ exception unwinding frame (FS:[0], handler 00f2fe48) is not translated
/// The class's `CreateObject` (`cdecl`, no words; no Xbox PDB name): a
/// 0xE0-byte block (`00aa13e0`) constructed by `00c5da70`, or null when the
/// allocation fails. The exception-unwinding frame is not translated.
pub fn fn_00c5e5a0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_NEW, &args![0xe0u32]).u32();
    if block != 0 {
        e.call(CONSTRUCTOR_00C5DA70, &args![block]).ptr()
    } else {
        Ptr::new(0)
    }
}

// Translated from 00c5e620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::LoadBinary` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the stream): `NiNode::LoadBinary`, the maximum
/// emitter count (+0xBA) and, for file versions above 0xF, the link ids
/// (`NiStream::ReadMultipleLinkIDs`).
pub fn bs_master_particle_system_load_binary(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
    e.call(
        STREAM_LOAD_SHORT_00A30370,
        &args![stream, this.addr() + 0xba],
    );
    if stream_version(e, stream) > 0xf {
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    }
}

// Translated from 00c5e660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::LinkObject` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the stream): `NiNode::LinkObject`, then for
/// file versions above 0xF sizes the child array (+0xCC) to the stream's
/// link id count and stores each linked object at its index.
pub fn bs_master_particle_system_link_object(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(NODE_LINK_OBJECT, &args![this, stream]);
    if stream_version(e, stream) <= 0xf {
        return;
    }
    let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    if count == 0 {
        return;
    }
    let array = this.addr() + CHILD_PARTICLES;
    e.call(ARRAY_SET_SIZE_00C4C920, &args![array, count]);
    e.with_stack(4, |e, element| {
        let mut index = 0u32;
        while index < count {
            let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(element.addr(), object);
            e.call(ARRAY_SET_AT_0096AE90, &args![array, index, element]);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c5e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::PostLinkObject` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream):
/// `NiObjectNET::PostLinkObject`, then for file versions below 0x10
/// `BSMasterParticleSystem::UpdateParticleSystemList`.
pub fn bs_master_particle_system_post_link_object(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(OBJECT_NET_POST_LINK_OBJECT, &args![this, stream]);
    if stream_version(e, stream) < 0x10 {
        e.call(UPDATE_PARTICLE_SYSTEM_LIST, &args![this]);
    }
}

// Translated from 00c5eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::ChildHasActiveParticles` (Xbox PDB, matched by
/// the engine map; `thiscall`): whether any child of the array at +0xCC has
/// a +0xB8 object whose vtable +0x90 answers a positive 16-bit count.
pub fn bs_master_particle_system_child_has_active_particles(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
) -> bool {
    let children = Ptr::<NiTArray>::new(this.addr() + CHILD_PARTICLES);
    let size = u32::from(e.get(children, NiTArray::m_usSize));
    let mut index = 0u32;
    while index < size {
        let elements = e.get(children, NiTArray::m_pBase);
        let child = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            let data = e.mem.u32(child + 0xb8);
            if e.vcall(data, 0x90, &args![]).u16() > 0 {
                return true;
            }
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00c5ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<NiParticles*, NiTMallocInterface<..>>` constructor
/// (`thiscall (this; max_size, grow_by)`, 16-bit words; no Xbox PDB name):
/// stores the vtable, the sizes, clears the size and used count and
/// allocates `max_size` elements (`00aa1070`), or none. Returns `this`.
pub fn fn_00c5ec10(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), PARTICLES_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if e.get(this, NiTArray::m_usMaxSize) > 0 {
        let count = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let block = e.call(NI_ALLOC, &args![count << 2]).u32();
        e.set(this, NiTArray::m_pBase, block);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_ec90, fn_00c5ec90(Ptr) -> u32),
        entry!(0x00c5_eb20, bs_master_particle_system_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c5_db90, fn_00c5db90(Ptr<BSMasterParticleSystem>)),
        entry!(
            0x00c5_e080,
            bs_master_particle_system_clear_unreferenced_emitters(Ptr<BSMasterParticleSystem>)
        ),
        entry!(
            0x00c5_e350,
            bs_master_particle_system_update_downward_pass(Ptr<BSMasterParticleSystem>, Ptr, u32)
        ),
        entry!(0x00c5_e5a0, fn_00c5e5a0() -> Ptr),
        entry!(
            0x00c5_e620,
            bs_master_particle_system_load_binary(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_e660,
            bs_master_particle_system_link_object(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_e6f0,
            bs_master_particle_system_post_link_object(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_eb90,
            bs_master_particle_system_child_has_active_particles(
                Ptr<BSMasterParticleSystem>,
            ) -> bool
        ),
        entry!(
            0x00c5_ec10,
            fn_00c5ec10(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_master_particle_system_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c5_eb20, 0x00c5_db90, 0xe0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c5ec90_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_ec90, &args![0x1000u32]).u32(), 0x0120_436c);
        assert_eq!(e.call(0x00c5_ec90, &args![0u32]).u32(), 0x0120_436c);
    }
}

#[cfg(test)]
mod master_tests {
    use super::*;

    const SLOT_A: u32 = 0x0001_0010;
    const SLOT_B: u32 = 0x0001_0020;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine) -> CallLog {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    fn system(e: &mut Engine) -> Ptr<BSMasterParticleSystem> {
        e.new_object::<BSMasterParticleSystem>()
    }

    /// Nodes (next, -, item) for `items`, returned head first.
    fn nodes(e: &mut Engine, items: &[u32]) -> Vec<u32> {
        let mut list = vec![];
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *item);
            next = node;
            list.push(node);
        }
        list.reverse();
        list
    }

    /// An emitter object (reference count +4, flags +0x30) whose vtable +4
    /// is `SLOT_A`.
    fn emitter(e: &mut Engine, references: u32, flags: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.put_vtable(0x0002_0000, &[0, SLOT_A]);
        e.mem.set_u32(object, 0x0002_0000);
        e.mem.set_u32(object + 4, references);
        e.mem.set_u32(object + 0x30, flags);
        object
    }

    fn list_doubles(e: &mut Engine) {
        // Takes the iterator's node out: hands its item over and moves the
        // iterator on.
        e.register(LIST_REMOVE_AT_00C5BFB0, |e, a| {
            let node = e.mem.u32(a[2]);
            e.mem.set_u32(a[1], e.mem.u32(node + 8));
            e.mem.set_u32(a[2], e.mem.u32(node));
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(SLOT_A, |_, _| Ret::default());
    }

    #[test]
    fn destructor_empties_the_members_in_order() {
        let mut e = Engine::new();
        let this = system(&mut e);
        let elements = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(elements + 4 * i, 0x100 + i);
        }
        let array = Ptr::<NiTArray>::new(this.addr() + CHILD_PARTICLES);
        e.set(array, NiTArray::m_pBase, elements);
        e.set(array, NiTArray::m_usSize, 3);
        e.set(array, NiTArray::m_usESize, 2);
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0x55);
        for callee in [LIST_REMOVE_ALL, NI_FREE, NODE_DESTRUCTOR_00A5E930] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00c5_db90, &args![this]);
        assert_eq!(
            calls(&mut e),
            vec![
                (LIST_REMOVE_ALL, vec![this.addr() + 0xac]),
                (NI_FREE, vec![elements]),
                (LIST_REMOVE_ALL, vec![this.addr() + 0xac]),
                (NODE_DESTRUCTOR_00A5E930, vec![this.addr()]),
            ]
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, BSMasterParticleSystem::FirstEmitterObjIter), 0);
        assert_eq!(e.mem.bytes(elements, 0xc), vec![0; 0xc]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        assert_eq!(e.mem.u32(array.addr()), PARTICLES_ARRAY_VTABLE);
    }

    #[test]
    fn clear_unreferenced_emitters_drops_single_and_flagged_references() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = system(&mut e);
        let kept = emitter(&mut e, 2, 0);
        let last = emitter(&mut e, 1, 0);
        let flagged = emitter(&mut e, 3, 1);
        let list = nodes(&mut e, &[kept, last, flagged]);
        e.set(this, BSMasterParticleSystem::lEmitterObjList, list[0]);
        e.mem.set_u32(this.addr() + EMITTER_LIST_COUNT, 3);
        e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e080, &args![this]);
        let log = calls(&mut e);
        assert_eq!(
            e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount),
            1
        );
        assert_eq!(
            e.get(this, BSMasterParticleSystem::FirstEmitterObjIter),
            list[0]
        );
        assert_eq!(e.mem.u32(last + 4), 0);
        assert_eq!(e.mem.u32(flagged + 4), 2);
        let deleted: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SLOT_A)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(deleted, vec![last]);
    }

    #[test]
    fn clear_unreferenced_emitters_keeps_the_iterator_at_the_maximum() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = system(&mut e);
        let first = emitter(&mut e, 2, 0);
        let second = emitter(&mut e, 2, 0);
        let list = nodes(&mut e, &[first, second]);
        e.set(this, BSMasterParticleSystem::lEmitterObjList, list[0]);
        e.mem.set_u32(this.addr() + EMITTER_LIST_COUNT, 2);
        e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 1);
        e.call(0x00c5_e080, &args![this]);
        assert_eq!(
            e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount),
            1
        );
        assert_eq!(
            e.get(this, BSMasterParticleSystem::FirstEmitterObjIter),
            list[1]
        );

        // No emitters: the iterator is cleared.
        let empty = system(&mut e);
        e.set(empty, BSMasterParticleSystem::usMaxEmitterObj, 0x14);
        e.call(0x00c5_e080, &args![empty]);
        assert_eq!(
            e.get(empty, BSMasterParticleSystem::usActiveEmitterObjCount),
            0
        );
        assert_eq!(e.get(empty, BSMasterParticleSystem::FirstEmitterObjIter), 0);
    }

    /// One child whose +0xB8 object answers `count` from vtable +0x90.
    fn child(e: &mut Engine, this: Ptr<BSMasterParticleSystem>, count: u32) {
        let data = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x94 / 4];
        slots[0x90 / 4] = SLOT_B;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(data, 0x0002_1000);
        e.register_double(SLOT_B, move |_, _| word(count));
        let particles = e.mem.alloc(0xc0);
        e.mem.set_u32(particles + 0xb8, data);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(elements + 4, particles);
        let array = Ptr::<NiTArray>::new(this.addr() + CHILD_PARTICLES);
        e.set(array, NiTArray::m_pBase, elements);
        e.set(array, NiTArray::m_usSize, 2);
    }

    #[test]
    fn child_has_active_particles_checks_each_child() {
        let mut e = Engine::new();
        let this = system(&mut e);
        child(&mut e, this, 0xffff_0003);
        assert!(e.call(0x00c5_eb90, &args![this]).bool());
        child(&mut e, this, 0xffff_0000);
        assert!(!e.call(0x00c5_eb90, &args![this]).bool());
    }

    #[test]
    fn update_downward_pass_runs_with_the_accumulated_time() {
        let mut e = Engine::new();
        let this = system(&mut e);
        child(&mut e, this, 1);
        e.set(this, BSMasterParticleSystem::fMasterPSysTime, 1.0);
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 0.5);
        e.register(NODE_UPDATE_DOWNWARD_PASS, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 1.5);
            assert_eq!(a[2], 7);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_e350, &args![this, time, 7u32]);
        assert_eq!(e.get(this, BSMasterParticleSystem::fMasterPSysTime), 1.5);
        assert_eq!(e.mem.f32(time), 0.5);
        assert_eq!(calls(&mut e).len(), 2);

        // Nothing active: no update.
        child(&mut e, this, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e350, &args![this, time, 7u32]);
        assert!(calls(&mut e)
            .iter()
            .all(|c| c.0 != NODE_UPDATE_DOWNWARD_PASS));
        assert_eq!(e.get(this, BSMasterParticleSystem::fMasterPSysTime), 1.5);
    }

    #[test]
    fn create_object_constructs_a_block() {
        let mut e = Engine::new();
        e.register(NI_NEW, |_, a| {
            assert_eq!(a[0], 0xe0);
            word(0x7000)
        });
        e.register(CONSTRUCTOR_00C5DA70, |_, a| word(a[0]));
        assert_eq!(e.call(0x00c5_e5a0, &args![]).u32(), 0x7000);
        e.register(NI_NEW, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_e5a0, &args![]).u32(), 0);
        assert!(calls(&mut e).iter().all(|c| c.0 != CONSTRUCTOR_00C5DA70));
    }

    fn stream(e: &mut Engine, version: u32) -> u32 {
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream + 4, version);
        stream
    }

    #[test]
    fn load_binary_reads_link_ids_from_version_0x10() {
        let mut e = Engine::new();
        let this = system(&mut e);
        for callee in [
            NODE_LOAD_BINARY,
            STREAM_LOAD_SHORT_00A30370,
            STREAM_READ_MULTIPLE_LINK_IDS,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let new = stream(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e620, &args![this, new]);
        assert_eq!(
            calls(&mut e),
            vec![
                (NODE_LOAD_BINARY, vec![this.addr(), new]),
                (STREAM_LOAD_SHORT_00A30370, vec![new, this.addr() + 0xba]),
                (STREAM_READ_MULTIPLE_LINK_IDS, vec![new]),
            ]
        );
        let old = stream(&mut e, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e620, &args![this, old]);
        assert_eq!(calls(&mut e).len(), 2);
    }

    #[test]
    fn link_object_stores_each_linked_child() {
        let mut e = Engine::new();
        let this = system(&mut e);
        e.register(NODE_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(2));
        e.register(ARRAY_SET_SIZE_00C4C920, |_, _| Ret::default());
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut next = 0x110;
            move |_, _| {
                next += 0x110;
                word(next)
            }
        });
        e.register(ARRAY_SET_AT_0096AE90, |e, a| word(e.mem.u32(a[2])));
        let new = stream(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e660, &args![this, new]);
        let log = calls(&mut e);
        let array = this.addr() + CHILD_PARTICLES;
        assert_eq!(log[2], (ARRAY_SET_SIZE_00C4C920, vec![array, 2]));
        let set: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ARRAY_SET_AT_0096AE90)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(set, vec![0, 1]);

        let old = stream(&mut e, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e660, &args![this, old]);
        assert_eq!(calls(&mut e).len(), 1);
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x00c5_e660, &args![this, new]);
        assert_eq!(calls(&mut e).len(), 2);
    }

    #[test]
    fn post_link_object_updates_old_files_only() {
        let mut e = Engine::new();
        let this = system(&mut e);
        e.register(OBJECT_NET_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(UPDATE_PARTICLE_SYSTEM_LIST, |_, _| Ret::default());
        let old = stream(&mut e, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e6f0, &args![this, old]);
        assert_eq!(
            calls(&mut e),
            vec![
                (OBJECT_NET_POST_LINK_OBJECT, vec![this.addr(), old]),
                (UPDATE_PARTICLE_SYSTEM_LIST, vec![this.addr()]),
            ]
        );
        let new = stream(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e6f0, &args![this, new]);
        assert_eq!(calls(&mut e).len(), 1);
    }

    #[test]
    fn particles_array_constructor_allocates_the_maximum() {
        let mut e = Engine::new();
        let array = e.new_object::<NiTArray>();
        e.register(NI_ALLOC, |_, a| word(0x8000 + a[0]));
        assert_eq!(
            e.call(0x00c5_ec10, &args![array, 3u32, 2u32]).u32(),
            array.addr()
        );
        assert_eq!(e.mem.u32(array.addr()), PARTICLES_ARRAY_VTABLE);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0x800c);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usGrowBy), 2);
        e.call(0x00c5_ec10, &args![array, 0u32, 1u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
    }
}
