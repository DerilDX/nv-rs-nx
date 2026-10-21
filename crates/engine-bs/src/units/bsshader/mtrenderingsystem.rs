//! `BSShader/mtrenderingsystem.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte that enables the second rendering task thread (`011f94b4`).
const SECOND_THREAD_ENABLED_011F94B4: u32 = 0x011f_94b4;
/// The word passed as `AddThread`'s second argument (`01200080`).
const THREAD_ARGUMENT_01200080: u32 = 0x0120_0080;
/// `MTRenderingSystem::AddThread` (engine map, `00ba2e00`): five stack words, cleaned by the
/// callee.
const ADD_THREAD: u32 = 0x00ba_2e00;
/// The thread's code (`00ba3190`), passed as an address.
const THREAD_CODE_00BA3190: u32 = 0x00ba_3190;
/// `"Rendering Task Thread 2"` (`010b7f84`).
const THREAD_NAME: u32 = 0x010b_7f84;

// Translated from 00ba3720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MTRenderingSystem::CreateThreads` (Xbox PDB). No arguments: when the byte at `011f94b4`
/// is not 0, calls `MTRenderingSystem::AddThread` (`00ba2e00`) with 1, the word at
/// `01200080`, the code address `00ba3190`, the name `"Rendering Task Thread 2"`
/// (`010b7f84`) and 0x20000.
pub fn mt_rendering_system_create_threads(e: &mut Engine) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) != 0 {
        let word = e.global::<u32>(THREAD_ARGUMENT_01200080);
        e.call(
            ADD_THREAD,
            &args![1u32, word, THREAD_CODE_00BA3190, THREAD_NAME, 0x20000u32],
        );
    }
}

/// `ReleaseSemaphore` (import slot): handle, count, previous count.
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `WaitForSingleObject` (import slot): handle, milliseconds.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// The compiler's array constructor helper, stdcall (array, element size,
/// count, element constructor, element destructor).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// The compiler's array destructor helper (engine map: `eh vector
/// destructor iterator`), stdcall (array, element size, count, element
/// destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The semaphore holder's constructor and destructor (`run_00ba2d40.rs`),
/// passed as addresses.
const SEMAPHORE_CONSTRUCT: u32 = 0x00ba_2d40;
const SEMAPHORE_DESTRUCT: u32 = 0x00ba_2d70;
/// The task's constructor and destructor (this unit), passed as addresses.
const TASK_CONSTRUCT: u32 = 0x00ba_30e0;
const TASK_DESTRUCT: u32 = 0x00ba_3030;
/// The allocator object the task's frustum comes from (`011f6238`).
const FRUSTUM_ALLOCATOR: u32 = 0x011f_6238;
/// Thiscall on the allocator (size), `RET 4`: the block.
const ALLOCATOR_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSCompoundFrustum::BSCompoundFrustum` (engine map), thiscall.
const COMPOUND_FRUSTUM_CONSTRUCT: u32 = 0x00c4_7660;
/// `BSCompoundFrustum::CopyMembers` (engine map), thiscall (destination),
/// `RET 4`.
const COMPOUND_FRUSTUM_COPY_MEMBERS: u32 = 0x00c4_77f0;
/// The `NiCullingProcess` constructor (`nicullingprocess.rs`), thiscall
/// (word), `RET 4`.
const CULLING_PROCESS_CONSTRUCT: u32 = 0x00a6_9400;
/// `BSCullingProcess`'s vtable (RTTI `.?AVBSCullingProcess@@`).
const BS_CULLING_PROCESS_VTABLE: u32 = 0x0101_e2ec;
/// `BSShaderUtil::AccumulateScene` (engine map), cdecl (camera, scene,
/// culling process).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// `BSShaderUtil::AccumulateSceneList` (engine map), cdecl (camera, list,
/// culling process).
const ACCUMULATE_SCENE_LIST: u32 = 0x00b6_bfc0;
/// `BSCullingProcess::~BSCullingProcess` (engine map), thiscall.
const BS_CULLING_PROCESS_DESTRUCT: u32 = 0x004a_0f60;

/// Releases the reference-counted object at `object` (`InterlockedDecrement`
/// on its count at +4; at zero its vtable slot +4, `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointer` assignment at `slot`: when the new object differs, the old
/// one is released, the new one stored and its count incremented.
fn assign_reference(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    release_reference(e, old);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00ba2f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the threads are enabled (`011f94b4`),
/// clears the two words at +0xC and +0x10, then for each of the two thread
/// objects (`this[0]`, `this[1]`) releases its semaphore (+0x34) once and
/// counts it (+0x30).
pub fn fn_00ba2f30(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 {
        return;
    }
    for index in 0..2u32 {
        e.mem.set_u32(this.addr() + 0xc + 4 * index, 0);
    }
    for index in 0..2u32 {
        let thread = e.mem.u32(this.addr() + 4 * index);
        if thread != 0 {
            let semaphore = e.mem.u32(thread + 0x34);
            e.call(RELEASE_SEMAPHORE, &args![semaphore, 1u32, 0u32]);
            e.call(INTERLOCKED_INCREMENT, &args![thread + 0x30]);
        }
    }
}

// Translated from 00ba2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the threads are enabled, waits for each
/// of the two thread objects' handle (+0x40) without a time-out and counts
/// it down (+0x3C).
pub fn fn_00ba2fa0(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 {
        return;
    }
    for index in 0..2u32 {
        let thread = e.mem.u32(this.addr() + 4 * index);
        if thread != 0 {
            let handle = e.mem.u32(thread + 0x40);
            e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
            e.call(INTERLOCKED_DECREMENT, &args![thread + 0x3c]);
        }
    }
}

// Translated from 00ba3030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3030 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 between the releases) is not translated.
/// Unnamed in the engine map: the 0x20-byte accumulation task's destructor:
/// releases its three reference-counted objects, at +0x14, +8 and +0 in
/// that order.
pub fn fn_00ba3030(e: &mut Engine, this: Ptr) {
    for offset in [0x14, 0x8, 0x0] {
        let object = e.mem.u32(this.addr() + offset);
        release_reference(e, object);
    }
}

// Translated from 00ba30e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here): the 0x20-byte accumulation
/// task's constructor: clears its three object pointers (+0, +8, +0x14).
/// Returns `this`.
pub fn fn_00ba30e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    this
}

// Translated from 00ba30f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MTRenderingSystem::SetThreadStage` (engine map): when the threads are
/// enabled and `thread` is below 2, releases that thread's stage semaphore
/// (the holder at `this + 0x18 + 0xC * thread`: its handle +4) once and
/// counts it (holder +0). The second word is not read.
pub fn mt_rendering_system_set_thread_stage(
    e: &mut Engine,
    this: Ptr,
    thread: i32,
    _unused_2: u32,
) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 || thread >= 2 {
        return;
    }
    let holder = this
        .addr()
        .wrapping_add((thread.wrapping_mul(3).wrapping_add(6) as u32).wrapping_mul(4));
    let semaphore = e.mem.u32(holder + 4);
    e.call(RELEASE_SEMAPHORE, &args![semaphore, 1u32, 0u32]);
    e.call(INTERLOCKED_INCREMENT, &args![holder]);
}

// Translated from 00ba3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the threads are enabled, waits on
/// `thread`'s stage semaphore (handle at `this + 0x1C + 0xC * thread`,
/// counted down at `+0x18 + 0xC * thread`) until the thread's stage
/// counter (`this + 0xC + 4 * thread`) reaches `stage`, counting it up
/// once per wait.
pub fn fn_00ba3130(e: &mut Engine, this: Ptr, thread: i32, stage: i32) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 {
        return;
    }
    let counter = this.addr().wrapping_add((thread as u32).wrapping_mul(4)) + 0xc;
    let holder = this
        .addr()
        .wrapping_add((thread.wrapping_mul(3).wrapping_add(6) as u32).wrapping_mul(4));
    while e.mem.i32(counter) < stage {
        let handle = e.mem.u32(holder + 4);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
        e.call(INTERLOCKED_DECREMENT, &args![holder]);
        let value = e.mem.i32(counter);
        e.mem.set_i32(counter, value.wrapping_add(1));
    }
}

// Translated from 00ba3390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3390 - the C++ exception frame (FS:[0] chain, unwind state 0 around the frustum's allocation and 1 around the stack culling process) is not translated.
/// `MTRenderingSystem::AddAccumTask` (engine map). With the threads
/// enabled, fills task `index` (0x20 bytes at `this + 0x3C + 0x20 *
/// index`): `camera` (+0, a reference), the frustum cleared (+4), `scene`
/// (+8, a reference), `list` and `second_list` (+0xC, +0x10), `object`
/// (+0x14, a reference), `value` (+0x18) and `last` (+0x1C); with a
/// `frustum`, a copy of it (`BSCompoundFrustum`, 0xA4 bytes from the
/// allocator at `011f6238`) becomes the task's +4. Without the threads it
/// accumulates at once: a `BSCullingProcess` on the stack (the
/// `NiCullingProcess` constructor with 0, then the vtable and its fields at
/// +0x90, +0xBC, +0xC0, +0xC4: `value`, 0, `frustum`, `object` with a
/// reference) is passed to `BSShaderUtil::AccumulateScene(camera, scene)`
/// and `AccumulateSceneList(camera, list)` and `(camera, second_list)` for
/// each non-null one, then destroyed.
pub fn mt_rendering_system_add_accum_task(
    e: &mut Engine,
    this: Ptr,
    camera: u32,
    frustum: u32,
    scene: u32,
    list: u32,
    second_list: u32,
    object: u32,
    value: u32,
    index: u32,
    last: u32,
) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) != 0 {
        let task = this.addr().wrapping_add(index << 5);
        assign_reference(e, task + 0x3c, camera);
        let frustum_slot = this.addr().wrapping_add(index.wrapping_add(2) << 5);
        e.mem.set_u32(frustum_slot, 0);
        assign_reference(e, task + 0x44, scene);
        e.mem.set_u32(task + 0x48, list);
        e.mem.set_u32(task + 0x4c, second_list);
        assign_reference(e, task + 0x50, object);
        e.mem.set_u32(task + 0x54, value);
        e.mem.set_u32(task + 0x58, last);
        if frustum != 0 {
            let block = e
                .call(ALLOCATOR_ALLOCATE, &args![FRUSTUM_ALLOCATOR, 0xa4u32])
                .u32();
            let copy = if block != 0 {
                e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(frustum_slot, copy);
            e.call(COMPOUND_FRUSTUM_COPY_MEMBERS, &args![frustum, copy]);
        }
        return;
    }
    e.with_stack(0xc8, |e, process| {
        let process = process.addr();
        e.call(CULLING_PROCESS_CONSTRUCT, &args![process, 0u32]);
        e.mem.set_u32(process, BS_CULLING_PROCESS_VTABLE);
        e.mem.set_u32(process + 0x90, 0);
        e.mem.set_u32(process + 0xbc, 0);
        e.mem.set_u32(process + 0xc0, 0);
        e.mem.set_u32(process + 0xc4, 0);
        if object != 0 {
            e.mem.set_u32(process + 0xc4, object);
            e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
        }
        e.mem.set_u32(process + 0xc0, frustum);
        e.mem.set_u32(process + 0x90, value);
        if scene != 0 {
            e.call(ACCUMULATE_SCENE, &args![camera, scene, process]);
        }
        if list != 0 {
            e.call(ACCUMULATE_SCENE_LIST, &args![camera, list, process]);
        }
        if second_list != 0 {
            e.call(ACCUMULATE_SCENE_LIST, &args![camera, second_list, process]);
        }
        e.call(BS_CULLING_PROCESS_DESTRUCT, &args![process]);
    });
}

// Translated from 00ba3620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3620 - the C++ exception frame (FS:[0] chain, unwind state 0 between the two arrays) is not translated.
/// Unnamed in the engine map: the rendering system's destructor: the 0x18
/// tasks of 0x20 bytes at +0x3C (`00ba3030`), then the three semaphore
/// holders of 0xC bytes at +0x18 (`00ba2d70`), through the compiler's array
/// destructor helper.
pub fn fn_00ba3620(e: &mut Engine, this: Ptr) {
    e.call(
        VECTOR_DESTRUCT,
        &args![this.addr() + 0x3c, 0x20u32, 0x18u32, TASK_DESTRUCT],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![this.addr() + 0x18, 0xcu32, 3u32, SEMAPHORE_DESTRUCT],
    );
}

// Translated from 00ba3690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3690 - the C++ exception frame (FS:[0] chain, unwind state 0 after the semaphores are built) is not translated.
/// Unnamed in the engine map: the rendering system's constructor: three
/// semaphore holders of 0xC bytes at +0x18 (`00ba2d40`/`00ba2d70`) and
/// 0x18 tasks of 0x20 bytes at +0x3C (`00ba30e0`/`00ba3030`) through the
/// compiler's array constructor helper, then the three words at +0 and the
/// three at +0xC cleared. Returns `this`.
pub fn fn_00ba3690(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            this.addr() + 0x18,
            0xcu32,
            3u32,
            SEMAPHORE_CONSTRUCT,
            SEMAPHORE_DESTRUCT
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            this.addr() + 0x3c,
            0x20u32,
            0x18u32,
            TASK_CONSTRUCT,
            TASK_DESTRUCT
        ],
    );
    for index in 0..3u32 {
        e.mem.set_u32(this.addr() + 4 * index, 0);
        e.mem.set_u32(this.addr() + 4 * index + 0xc, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_3720, mt_rendering_system_create_threads()),
        entry!(0x00ba_2f30, fn_00ba2f30(Ptr)),
        entry!(0x00ba_2fa0, fn_00ba2fa0(Ptr)),
        entry!(0x00ba_3030, fn_00ba3030(Ptr)),
        entry!(0x00ba_30e0, fn_00ba30e0(Ptr) -> Ptr),
        entry!(
            0x00ba_30f0,
            mt_rendering_system_set_thread_stage(Ptr, i32, u32)
        ),
        entry!(0x00ba_3130, fn_00ba3130(Ptr, i32, i32)),
        entry!(
            0x00ba_3390,
            mt_rendering_system_add_accum_task(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00ba_3620, fn_00ba3620(Ptr)),
        entry!(0x00ba_3690, fn_00ba3690(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_thread_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ADD_THREAD)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_threads_adds_the_second_thread_only_when_enabled() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global(THREAD_ARGUMENT_01200080, 0x4242u32);
        e.register(ADD_THREAD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ba_3720, &args![]);
        assert!(add_thread_calls(&e).is_empty());

        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.call(0x00ba_3720, &args![]);
        assert_eq!(
            add_thread_calls(&e),
            vec![vec![1, 0x4242, 0x00ba_3190, 0x010b_7f84, 0x20000]]
        );
    }
}

#[cfg(test)]
mod rendering_system_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the enable byte mapped (set when `enabled`) and the
    /// imports doubled; `InterlockedDecrement` returns the decremented count.
    fn engine(enabled: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, u8::from(enabled));
        for addr in [RELEASE_SEMAPHORE, WAIT_FOR_SINGLE_OBJECT, 0x0000_7004] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        e
    }

    /// A reference-counted object with the test vtable and `count`.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f00_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn threads(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x400);
        let thread = e.mem.alloc(0x50);
        e.mem.set_u32(thread + 0x34, 0x51);
        e.mem.set_u32(thread + 0x40, 0x52);
        e.mem.set_u32(this, thread);
        (this, thread)
    }

    #[test]
    fn fn_00ba2f30_releases_the_thread_semaphores() {
        let mut e = engine(false);
        let (this, thread) = threads(&mut e);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ba_2f30, &args![this]);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.mem.set_u32(this + 0x10, 6);
        e.call(0x00ba_2f30, &args![this]);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        assert_eq!(calls_to(&e, RELEASE_SEMAPHORE), vec![vec![0x51, 1, 0]]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![thread + 0x30]]
        );
    }

    #[test]
    fn fn_00ba2fa0_waits_for_the_threads() {
        let mut e = engine(false);
        let (this, thread) = threads(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00ba_2fa0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.call(0x00ba_2fa0, &args![this]);
        assert_eq!(
            calls_to(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x52, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![thread + 0x3c]]
        );
    }

    #[test]
    fn fn_00ba3030_releases_the_three_references() {
        let mut e = engine(true);
        let task = e.mem.alloc(0x20);
        let last = object(&mut e, 1);
        let kept = object(&mut e, 3);
        e.mem.set_u32(task + 0x14, last);
        e.mem.set_u32(task, kept);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3030, &args![task]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![last + 4], vec![kept + 4]]
        );
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![last]]);
        assert_eq!(e.mem.u32(kept + 4), 2);
    }

    #[test]
    fn fn_00ba30e0_clears_the_task_pointers() {
        let mut e = engine(true);
        let task = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(task + offset, 9);
        }
        assert_eq!(e.call(0x00ba_30e0, &args![task]).u32(), task);
        assert_eq!(
            (e.mem.u32(task), e.mem.u32(task + 4), e.mem.u32(task + 8)),
            (0, 9, 0)
        );
        assert_eq!(e.mem.u32(task + 0x14), 0);
    }

    #[test]
    fn set_thread_stage_releases_one_stage() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x400);
        e.mem.set_u32(this + 0x18 + 0xc + 4, 0x61);
        e.call_log = Some(vec![]);
        e.call(0x00ba_30f0, &args![this, 1u32, 0x99u32]);
        assert_eq!(calls_to(&e, RELEASE_SEMAPHORE), vec![vec![0x61, 1, 0]]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![this + 0x18 + 0xc]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00ba_30f0, &args![this, 2u32, 0u32]);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 0u8);
        e.call(0x00ba_30f0, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn fn_00ba3130_waits_until_the_stage_is_reached() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x400);
        e.mem.set_u32(this + 0x18 + 4, 0x71);
        e.mem.set_u32(this + 0x18, 5);
        e.mem.set_u32(this + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3130, &args![this, 0u32, 3u32]);
        assert_eq!(
            calls_to(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x71, 0xffff_ffff], vec![0x71, 0xffff_ffff]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 0x18), 3);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3130, &args![this, 0u32, 3u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn add_accum_task_fills_the_task_when_threaded() {
        let mut e = engine(true);
        e.register(ALLOCATOR_ALLOCATE, |_, _| eax(0x8800));
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |_, a| eax(a[0]));
        e.register(COMPOUND_FRUSTUM_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        let camera = object(&mut e, 0);
        let scene = object(&mut e, 0);
        let old = object(&mut e, 1);
        let task = this + 0x20;
        e.mem.set_u32(task + 0x3c, old);
        e.mem.set_u32(task + 0x40, 0x1234);
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_3390,
            &args![this, camera, 0x4400u32, scene, 0x11u32, 0x12u32, 0u32, 0x13u32, 1u32, 0x14u32],
        );
        assert_eq!(e.mem.u32(task + 0x3c), camera);
        assert_eq!(e.mem.u32(camera + 4), 1);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![old]]);
        assert_eq!(e.mem.u32(task + 0x40), 0x8800);
        assert_eq!(e.mem.u32(task + 0x44), scene);
        assert_eq!(e.mem.u32(task + 0x48), 0x11);
        assert_eq!(e.mem.u32(task + 0x4c), 0x12);
        assert_eq!(e.mem.u32(task + 0x50), 0);
        assert_eq!(e.mem.u32(task + 0x54), 0x13);
        assert_eq!(e.mem.u32(task + 0x58), 0x14);
        assert_eq!(
            calls_to(&e, ALLOCATOR_ALLOCATE),
            vec![vec![FRUSTUM_ALLOCATOR, 0xa4]]
        );
        assert_eq!(
            calls_to(&e, COMPOUND_FRUSTUM_COPY_MEMBERS),
            vec![vec![0x4400, 0x8800]]
        );

        // The same camera again: no reference change; no frustum: cleared.
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_3390,
            &args![this, camera, 0u32, scene, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        assert_eq!(e.mem.u32(camera + 4), 1);
        assert_eq!(e.mem.u32(task + 0x40), 0);
        assert!(calls_to(&e, ALLOCATOR_ALLOCATE).is_empty());
    }

    #[test]
    fn add_accum_task_accumulates_at_once_without_threads() {
        let mut e = engine(false);
        e.register(CULLING_PROCESS_CONSTRUCT, |_, _| Ret::default());
        e.register_double(ACCUMULATE_SCENE, |e, a| {
            let process = a[2];
            assert_eq!(e.mem.u32(process), BS_CULLING_PROCESS_VTABLE);
            assert_eq!(e.mem.u32(process + 0x90), 0x13);
            assert_eq!(e.mem.u32(process + 0xc0), 0x4400);
            assert_ne!(e.mem.u32(process + 0xc4), 0);
            Ret::default()
        });
        e.register(ACCUMULATE_SCENE_LIST, |_, _| Ret::default());
        e.register(BS_CULLING_PROCESS_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        let held = object(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_3390,
            &args![this, 0x2000u32, 0x4400u32, 0x3000u32, 0u32, 0x12u32, held, 0x13u32, 1u32, 0u32],
        );
        assert_eq!(e.mem.u32(held + 4), 1);
        let scene = calls_to(&e, ACCUMULATE_SCENE);
        assert_eq!(scene.len(), 1);
        assert_eq!(scene[0][..2], [0x2000, 0x3000]);
        let lists = calls_to(&e, ACCUMULATE_SCENE_LIST);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0][..2], [0x2000, 0x12]);
        assert_eq!(calls_to(&e, CULLING_PROCESS_CONSTRUCT)[0][1], 0);
        assert_eq!(
            calls_to(&e, BS_CULLING_PROCESS_DESTRUCT),
            vec![vec![scene[0][2]]]
        );
    }

    #[test]
    fn fn_00ba3620_destroys_the_arrays() {
        let mut e = engine(true);
        e.register(VECTOR_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3620, &args![this]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x3c, 0x20, 0x18, 0x00ba_3030],
                vec![this + 0x18, 0xc, 3, 0x00ba_2d70]
            ]
        );
    }

    #[test]
    fn fn_00ba3690_builds_the_arrays_and_clears() {
        let mut e = engine(true);
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(this + offset, 7);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_3690, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![
                vec![this + 0x18, 0xc, 3, 0x00ba_2d40, 0x00ba_2d70],
                vec![this + 0x3c, 0x20, 0x18, 0x00ba_30e0, 0x00ba_3030]
            ]
        );
        for offset in (0..0x18).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }
}
