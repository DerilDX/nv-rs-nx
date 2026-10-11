//! `BSMain/iomanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const SLEEP: u32 = 0x00fd_f104;
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;

// The Xbox PDB describes IOManager as 0x5C. The PC constructor 00c3da50
// initializes through +0x80, so its live object is at least 0x84. Keep PC
// accesses as verified raw offsets until the divergent layout is resolved.

fn word(e: &Engine, base: Ptr, offset: u32) -> u32 {
    e.mem.u32(base.addr().wrapping_add(offset))
}

fn set_word(e: &mut Engine, base: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(base.addr().wrapping_add(offset), value);
}

fn count_queued(e: &mut Engine, this: Ptr) -> u32 {
    let queue = word(e, this, 0x64);
    e.vcall(queue, 0x0c, &[]).u32()
}

fn external_count(e: &mut Engine, this: Ptr) -> u32 {
    let target = word(e, this, 0x6c);
    if target == 0 {
        0
    } else {
        e.call(target, &[]).u32()
    }
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE, &args![0x011f_6238u32, size]).u32()
}

fn array_bytes(count: u32) -> u32 {
    let bytes = count as u64 * 4;
    (bytes as u32) | if bytes > u32::MAX as u64 { u32::MAX } else { 0 }
}

// Translated from 00456520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOManager::LoadQueuedPriority` (Xbox PDB): load through priority 5.
pub fn io_manager_load_queued_priority(e: &mut Engine, this: Ptr) {
    fn_00c3dfa0(e, this, 5);
}

// Translated from 005a8040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at receiver +0x54. The engine map's `IOManager::Pause`
/// candidate (Xbox PDB) is unconfirmed for this PC body.
pub fn fn_005a8040(e: &mut Engine, this: Ptr, value: u32) {
    set_word(e, this, 0x54, value);
}

// Translated from 00c3db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys an IO manager's post-process queue and then its task manager base.
/// The game's SEH frame is omitted.
// Translation incomplete: 00c3db60 - SEH unwinding is not modeled.
pub fn fn_00c3db60(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_1604);
    let queue = word(e, this, 0x64);
    if queue != 0 {
        e.vcall(queue, 0, &args![1u32]);
    }
    fn_00c3fa70(e, this);
}

// Translated from 00c3df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOManager::RequeueTask` (Xbox PDB): replaces the priority byte in a task's 64-bit key.
pub fn io_manager_requeue_task(e: &mut Engine, this: Ptr, task: Ptr, priority: i32) {
    let low = word(e, task, 0x10);
    let high = word(e, task, 0x14);
    let key = (((high as u64) << 32) | low as u64) & 0xffff_ffff_ff00_ffff;
    let shifted = ((priority as i64) << 16) as u64;
    let result = key.wrapping_add(shifted);
    fn_00c3e690(e, this, task, result as u32, (result >> 32) as u32);
}

// Translated from 00c3dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains queued IO work up to the requested priority and reports percentage progress.
pub fn fn_00c3dfa0(e: &mut Engine, this: Ptr, priority: i32) {
    if word(e, this, 0x68) == 7 {
        return;
    }
    e.mem.set_u8(0x011a_f70c, 0);
    if word(e, this, 0x18) == 0 && count_queued(e, this) == 0 && external_count(e, this) == 0 {
        set_word(e, this, 0x68, 6);
        e.mem.set_u8(0x011a_f70c, 1);
        return;
    }
    set_word(e, this, 0x68, priority as u32);
    fn_00c3e1b0(e, this, priority, 0);
    let bucket_limit = (priority as u32).wrapping_mul(4).wrapping_add(3);
    let total = count_queued(e, this)
        .wrapping_add(external_count(e, this))
        .wrapping_add(fn_00c3e860(e, this, bucket_limit));
    let mut remaining = total;
    let mut processed = false;
    while remaining != 0 || !processed {
        let progress_scale: f64 = e.global(0x0101_7a40);
        let progress =
            ((total.wrapping_sub(remaining) as f64 / total as f64) * progress_scale) as f32;
        let progress_callback = word(e, this, 0x78);
        if progress_callback != 0 {
            e.call(progress_callback, &args![progress]);
        }
        let previous_count = word(e, this, 0x18);
        if count_queued(e, this) < previous_count {
            e.call(SLEEP, &args![50u32]);
        }
        processed = e.call(0x00c3_dbf0, &args![this]).bool();
        remaining = count_queued(e, this)
            .wrapping_add(external_count(e, this))
            .wrapping_add(fn_00c3e860(e, this, bucket_limit));
    }
    set_word(e, this, 0x68, 6);
    let progress_callback = word(e, this, 0x78);
    if progress_callback != 0 {
        e.call(progress_callback, &args![e.global::<f32>(0x0101_6410)]);
    }
    e.mem.set_u8(0x011a_f70c, 1);
}

// Translated from 00c3e1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs queued tasks from a lower priority through the requested priority.
/// The game's SEH frame and an unreachable cleanup branch are omitted.
// Translation incomplete: 00c3e1b0 - SEH unwinding is not modeled.
pub fn fn_00c3e1b0(e: &mut Engine, this: Ptr, priority: i32, lower_bound: i32) {
    if priority <= lower_bound {
        return;
    }
    fn_00c3e310(e, this);
    e.with_stack(0x50, |e, local| {
        set_word(e, local, 0, 0x010c_1664);
        set_word(e, local, 8, 0);
        set_word(e, local, 0x10, 0);
        set_word(e, local, 0x14, 0);
        e.mem.set_u8(local.addr() + 0x18, 0);
        e.call(
            0x00c4_2060,
            &args![local, (lower_bound as u32).wrapping_mul(4).wrapping_add(4)],
        );
        while e.mem.u8(local.addr() + 0x18) & 2 == 0 {
            let key = local.addr() + 0x30;
            let task_ptr = local.addr() + 0x3c;
            e.mem.set_u32(key, 0);
            e.mem.set_u32(key + 4, 0);
            e.mem.set_u32(task_ptr, 0);
            let found = e
                .call(0x00c4_0e70, &args![this, local, key, task_ptr, 1u32])
                .bool();
            if found {
                let task_priority = (e.mem.u32(key) >> 16) & 0xff;
                if task_priority as i32 > priority {
                    let task = e.mem.u32(task_ptr);
                    if task != 0 {
                        e.call(0x0044_dd60, &args![task]);
                    }
                    break;
                }
                let task = e.mem.u32(task_ptr);
                e.vcall(task, 0x1c, &args![lower_bound]);
            }
            let task = e.mem.u32(task_ptr);
            if task != 0 {
                e.call(0x0044_dd60, &args![task]);
            }
        }
    });
    fn_00c3e340(e, this);
}

// Translated from 00c3e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries each queue until one accepts an item, then notifies the owner.
pub fn fn_00c3e420(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    let mut index = 0;
    while index < word(e, this, 8) {
        let queues = word(e, this, 4);
        let queue = Ptr::new(e.mem.u32(queues.wrapping_add(index.wrapping_mul(4))));
        if fn_00c3e490(e, queue, item) {
            e.vcall(this.addr(), 8, &[]);
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00c3e490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Acquires a queue activity token, resolves the item, and attempts insertion.
pub fn fn_00c3e490(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    e.call(0x0040_fbf0, &args![this.addr().wrapping_add(0x20), 0u32]);
    let target = e.call(0x0044_9f80, &args![this]).ptr();
    let accepted = fn_00c3ea60(e, target, item);
    let count = word(e, this, 0x24).wrapping_sub(1);
    set_word(e, this, 0x24, count);
    if count == 0 {
        set_word(e, this, 0x20, 0);
    }
    accepted
}

fn assign_ref(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old != value {
        if old != 0 {
            e.call(0x0044_dd60, &args![old]);
        }
        e.mem.set_u32(holder, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 8]);
        }
    }
}

fn assign_ref_from(e: &mut Engine, holder: u32, source: u32) {
    let old = e.mem.u32(holder);
    if old != e.mem.u32(source) {
        if old != 0 {
            e.call(0x0044_dd60, &args![old]);
        }
        let new_value = e.mem.u32(source);
        e.mem.set_u32(holder, new_value);
        let stored = e.mem.u32(holder);
        if stored != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![stored + 8]);
        }
    }
}

// Translated from 00c3ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes one node from a lock-free IO task queue into an owning pointer.
pub fn fn_00c3ea60(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    loop {
        let queue = word(e, this, 0);
        let head = e.mem.u32(queue + 4);
        let hazard_head = word(e, this, 4);
        e.mem.set_u32(hazard_head, head);
        if head != e.mem.u32(word(e, this, 0) + 4) {
            continue;
        }
        let tail = e.mem.u32(word(e, this, 0) + 8);
        let next = e.mem.u32(head);
        let hazard_next = word(e, this, 8);
        e.mem.set_u32(hazard_next, next);
        if head != e.mem.u32(word(e, this, 0) + 4) {
            continue;
        }
        if next == 0 {
            e.mem.set_u32(word(e, this, 4), 0);
            assign_ref(e, item.addr(), 0);
            return false;
        }
        if head == tail {
            e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![word(e, this, 0) + 8, next, tail],
            );
            continue;
        }
        let previous = e.mem.u32(item.addr());
        if previous != e.mem.u32(next + 4) {
            if previous != 0 {
                e.call(0x0044_dd60, &args![previous]);
            }
            let replacement = e.mem.u32(next + 4);
            e.mem.set_u32(item.addr(), replacement);
            let retained = e.mem.u32(item.addr());
            if retained != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![retained + 8]);
            }
        }
        if e.call(
            INTERLOCKED_COMPARE_EXCHANGE,
            &args![word(e, this, 0) + 4, next, head],
        )
        .u32()
            != head
        {
            continue;
        }
        e.vcall(word(e, this, 0), 0x0c, &[]);
        assign_ref(e, next + 4, 0);
        e.mem.set_u32(word(e, this, 4), 0);
        e.mem.set_u32(word(e, this, 8), 0);
        fn_00c3f090(e, this, Ptr::new(head));
        return true;
    }
}

// Translated from 00c3f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a removed queue node to the interface's deferred deletion list.
pub fn fn_00c3f090(e: &mut Engine, this: Ptr, node: Ptr) {
    assign_ref(e, node.addr() + 4, 0);
    set_word(e, node, 4, word(e, this, 0x10));
    set_word(e, this, 0x10, node.addr());
    let count = word(e, this, 0x0c).wrapping_add(1);
    set_word(e, this, 0x0c, count);
    let queue = word(e, this, 0);
    if count == e.mem.u32(queue + 0x0c) {
        e.call(0x00c4_2b80, &args![this]);
    }
}

// Translated from 00c3e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces a task key if its state is idle or can be locked from state 1/2.
pub fn fn_00c3e690(e: &mut Engine, this: Ptr, task: Ptr, low: u32, high: u32) -> bool {
    let state = word(e, task, 0x0c) as i32;
    if state == 0 {
        set_word(e, task, 0x10, low);
        set_word(e, task, 0x14, high);
        return true;
    }
    if state >= 3
        || e.call(
            INTERLOCKED_COMPARE_EXCHANGE,
            &args![task.addr() + 0x0c, 2u32, state],
        )
        .i32()
            != state
    {
        return false;
    }
    e.call(0x0044_e050, &args![this, task]);
    set_word(e, task, 0x10, low);
    set_word(e, task, 0x14, high);
    if e.call(
        INTERLOCKED_COMPARE_EXCHANGE,
        &args![task.addr() + 0x0c, 0u32, 2u32],
    )
    .u32()
        == 2
    {
        e.vcall(this.addr(), 0x48, &args![task]);
    }
    true
}

// Translated from 00c3e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the IO manager's base task manager, bucket counts, and worker threads.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3e4f0 - SEH unwinding is not modeled.
pub fn fn_00c3e4f0(
    e: &mut Engine,
    this: Ptr,
    bucket_count: u32,
    thread_count: u32,
    capacity: u32,
) -> Ptr {
    fn_00c3ec80(
        e,
        this,
        bucket_count.wrapping_add(thread_count),
        capacity,
        12,
    );
    set_word(e, this, 0, 0x010c_167c);
    e.mem.set_u8(this.addr() + 0x40, 0);
    set_word(e, this, 0x44, 0);
    set_word(e, this, 0x48, 0);
    set_word(e, this, 0x4c, thread_count);
    let counts = allocate(e, array_bytes(capacity));
    set_word(e, this, 0x54, counts);
    e.call(0x00ec_61c0, &args![counts, 0u32, capacity.wrapping_mul(4)]);
    set_word(e, this, 0x58, 0);
    let threads = allocate(e, array_bytes(thread_count));
    set_word(e, this, 0x50, threads);
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let storage = allocate(e, 0x34);
        let thread = if storage == 0 {
            0
        } else {
            bs_task_manager_thread_construct(e, Ptr::new(storage), this, index.wrapping_add(2))
                .addr()
        };
        let current_threads = word(e, this, 0x50);
        e.mem
            .set_u32(current_threads.wrapping_add(index.wrapping_mul(4)), thread);
        let current_thread = e
            .mem
            .u32(current_threads.wrapping_add(index.wrapping_mul(4)));
        e.call(0x00c4_2f50, &args![current_thread]);
        index = index.wrapping_add(1);
    }
    this
}

// Translated from 00c3e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a priority queue with one lock-free queue interface per bucket.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3e8b0 - SEH unwinding is not modeled.
pub fn fn_00c3e8b0(e: &mut Engine, this: Ptr, count: u32, node_limit: u32, queue: Ptr) -> Ptr {
    set_word(e, this, 0, 0x010c_16dc);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    set_word(e, this, 0x0c, 0);
    set_word(e, this, 8, count);
    let slots = allocate(e, array_bytes(count));
    set_word(e, this, 4, slots);
    let mut index = 0;
    while index < word(e, this, 8) {
        let storage = allocate(e, 0x40);
        let interface = if storage == 0 {
            0
        } else {
            fn_00c3eeb0(e, Ptr::new(storage), node_limit, queue).addr()
        };
        let current_slots = word(e, this, 4);
        e.mem
            .set_u32(current_slots.wrapping_add(index.wrapping_mul(4)), interface);
        index = index.wrapping_add(1);
    }
    e.mem.set_u32(tls, previous);
    this
}

// Translated from 00c3ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the base lock-free task map and its per-thread interface manager.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3ec80 - SEH unwinding is not modeled.
pub fn fn_00c3ec80(
    e: &mut Engine,
    this: Ptr,
    bucket_count: u32,
    capacity: u32,
    batch_size: u32,
) -> Ptr {
    set_word(e, this, 0, 0x010c_16f4);
    set_word(e, this, 0x20, 0);
    set_word(e, this, 0x24, 0);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    set_word(e, this, 0x18, 0);
    set_word(e, this, 8, capacity);
    let referenced = allocate(e, array_bytes(capacity));
    if referenced != 0 {
        let mut remaining = capacity;
        let mut address = referenced;
        loop {
            remaining = remaining.wrapping_sub(1);
            if (remaining as i32) < 0 {
                break;
            }
            e.mem.set_u32(address, 0);
            address = address.wrapping_add(4);
        }
    }
    set_word(e, this, 0x0c, referenced);
    let buckets = allocate(e, array_bytes(bucket_count.wrapping_mul(3)));
    set_word(e, this, 4, buckets);
    set_word(e, this, 0x10, batch_size);
    let storage = allocate(e, 0x10);
    let interfaces = if storage == 0 {
        0
    } else {
        e.call(0x00c4_2180, &args![storage, bucket_count]).u32()
    };
    set_word(e, this, 0x14, interfaces);
    e.mem.set_u32(tls, previous);
    this
}

// Translated from 00c3eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs one lock-free queue interface and its sentinel and retired-node storage.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3eeb0 - SEH unwinding is not modeled.
pub fn fn_00c3eeb0(e: &mut Engine, this: Ptr, node_limit: u32, queue: Ptr) -> Ptr {
    set_word(e, this, 0, 0x010c_1760);
    set_word(e, this, 0x20, 0);
    set_word(e, this, 0x24, 0);
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    set_word(e, this, 0x18, 0);
    let sentinel = allocate(e, 8);
    if sentinel != 0 {
        e.mem.set_u32(sentinel, 0);
        e.mem.set_u32(sentinel + 4, 0);
    }
    set_word(e, this, 4, sentinel);
    set_word(e, this, 8, sentinel);
    set_word(e, this, 0x0c, queue.addr());
    let retired = allocate(e, array_bytes(node_limit.wrapping_mul(2)));
    set_word(e, this, 0x10, retired);
    let storage = allocate(e, 0x10);
    let manager = if storage == 0 {
        0
    } else {
        e.call(0x00c4_2180, &args![storage, node_limit]).u32()
    };
    set_word(e, this, 0x14, manager);
    e.mem.set_u32(tls, previous);
    this
}

// Translated from 00c3e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pauses each task thread on the first nested pause request.
pub fn fn_00c3e750(e: &mut Engine, this: Ptr) -> bool {
    e.mem.set_u8(0x0120_2df0, 0);
    if e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x48])
        .u32()
        != 1
    {
        return false;
    }
    while word(e, this, 0x44) != 0 {
        e.call(SLEEP, &args![1u32]);
    }
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let threads = word(e, this, 0x50);
        let thread = e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)));
        e.call(0x0044_6f70, &args![thread]);
        e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x44]);
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00c3e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes every task thread when the nested pause count reaches zero.
pub fn fn_00c3e7d0(e: &mut Engine, this: Ptr) -> bool {
    if word(e, this, 0x48) == 0
        || e.call(INTERLOCKED_DECREMENT, &args![this.addr() + 0x48])
            .u32()
            != 0
    {
        return false;
    }
    while word(e, this, 0x44) != word(e, this, 0x4c) {
        e.call(SLEEP, &args![1u32]);
    }
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let threads = word(e, this, 0x50);
        let thread = e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)));
        e.call(0x0044_6ff0, &args![thread]);
        e.call(INTERLOCKED_DECREMENT, &args![this.addr() + 0x44]);
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00c3e310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the task threads and fires the callback only when the pause count becomes one.
pub fn fn_00c3e310(e: &mut Engine, this: Ptr) {
    if fn_00c3e750(e, this) && word(e, this, 0x7c) != 0 {
        let target = word(e, this, 0x7c);
        e.call(target, &[]);
    }
}

// Translated from 00c3e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes the task threads and fires the callback only when the pause count becomes zero.
pub fn fn_00c3e340(e: &mut Engine, this: Ptr) {
    if fn_00c3e7d0(e, this) && word(e, this, 0x80) != 0 {
        let target = word(e, this, 0x80);
        e.call(target, &[]);
    }
}

// Translated from 00c3e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the task manager and clears its queue.
pub fn fn_00c3e380(e: &mut Engine, this: Ptr) {
    fn_00c3fce0(e, this);
    let queue = Ptr::new(word(e, this, 0x64));
    fn_00c3ea10(e, queue);
}

// Translated from 00c3e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes a queue bucket from two bytes of a 64-bit key.
pub fn fn_00c3e3a0(_e: &mut Engine, _this: Ptr, low: u32, high: u32) -> i32 {
    let bucket = (low >> 16) & 0xff;
    let category = (((high >> 24) & 0xff) + 4) % 20;
    bucket.wrapping_mul(category) as i32
}

// Translated from 00c3e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for the IO manager: destroy, optionally free, return this.
pub fn fn_00c3e3f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c3db60(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![0x011f_6238u32, this]);
    }
    this
}

// Translated from 00c3e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sum queue bucket counts from zero through the supplied inclusive index.
pub fn fn_00c3e860(e: &mut Engine, this: Ptr, last: u32) -> u32 {
    let mut sum = 0u32;
    let mut index = 0u32;
    loop {
        if index > last {
            break;
        }
        let counts = word(e, this, 0x54);
        sum = sum.wrapping_add(e.mem.u32(counts.wrapping_add(index.wrapping_mul(4))));
        index = index.wrapping_add(1);
    }
    sum
}

// Translated from 00c3ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the priority queue count and signals each owned queue.
pub fn fn_00c3ea10(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0x0c, 0);
    let mut index = 0;
    while index < word(e, this, 8) {
        let list = word(e, this, 4);
        let queue = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
        e.call(0x00c4_1330, &args![queue, 0u32]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00c3ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskManagerThread<int64_t>::BSTaskManagerThread<int64_t>` (Xbox PDB).
pub fn bs_task_manager_thread_construct(
    e: &mut Engine,
    this: Ptr,
    manager: Ptr,
    thread_id: u32,
) -> Ptr {
    e.call(0x00c4_2dd0, &args![this, thread_id, 0x010c_1748u32]);
    set_word(e, this, 0, 0x010c_1740);
    set_word(e, this, 0x30, manager.addr());
    this
}

// Translated from 00c3f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<...>::IncrementCount` (Xbox PDB).
pub fn lock_free_map_increment_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_INCREMENT,
        &args![this.addr().wrapping_add(0x18)],
    );
}

// Translated from 00c3f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreePriorityQueue<...>::IncrementCount` (Xbox PDB).
pub fn lock_free_priority_queue_increment_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_INCREMENT,
        &args![this.addr().wrapping_add(0x0c)],
    );
}

// Translated from 00c3f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreePriorityQueue<...>::DecrementCount` (Xbox PDB).
pub fn lock_free_priority_queue_decrement_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_DECREMENT,
        &args![this.addr().wrapping_add(0x0c)],
    );
}

// Translated from 00c3f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets this thread's map interface, allocating it if TLS has none.
pub fn fn_00c3f750(e: &mut Engine, this: Ptr) -> Ptr {
    let manager = word(e, this, 0x14);
    let tls_slot = e.mem.u32(manager + 4);
    let interface = e.call(0x00fd_f044, &args![tls_slot]).u32();
    if interface != 0 {
        Ptr::new(interface)
    } else {
        e.call(0x00c4_1610, &args![manager, this]).ptr()
    }
}

// Translated from 00c3f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<...>::DecrementCount` (Xbox PDB).
pub fn lock_free_map_decrement_count(e: &mut Engine, this: Ptr) {
    e.call(
        INTERLOCKED_DECREMENT,
        &args![this.addr().wrapping_add(0x18)],
    );
}

// Translated from 00c3f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<NiPointer<IOTask>>::AllocateInterface` (Xbox PDB).
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3f7a0 - SEH unwinding is not modeled.
pub fn lock_free_queue_allocate_interface(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let tls = e.tls() + 0x2b4;
    let previous = e.mem.u32(tls);
    e.mem.set_u32(tls, 6);
    let storage = allocate(e, 0x14);
    let interface = if storage == 0 {
        0
    } else {
        let nodes = word(e, this, 0x10);
        let first = nodes.wrapping_add(index.wrapping_mul(8));
        e.call(0x0044_cd00, &args![storage, this, first, first + 4])
            .u32()
    };
    e.mem.set_u32(tls, previous);
    Ptr::new(interface)
}

// Translated from 00c3f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a lock-free task map's interface manager and allocated arrays.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3f8c0 - SEH unwinding is not modeled.
pub fn fn_00c3f8c0(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_16f4);
    e.call(0x00c4_0b70, &args![this, 1u32]);
    let referenced = word(e, this, 0x0c);
    e.call(DEALLOCATE, &args![0x011f_6238u32, referenced]);
    let buckets = word(e, this, 4);
    e.call(DEALLOCATE, &args![0x011f_6238u32, buckets]);
    set_word(e, this, 0, 0x0101_7458);
}

// Translated from 00c3f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries a task in each priority queue until one accepts it.
/// The compiler passes an owning NiPointer by value to the virtual call.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3f960 - SEH unwinding is not modeled.
pub fn fn_00c3f960(e: &mut Engine, this: Ptr, task: Ptr) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let mut accepted = false;
    let count = word(e, this, 8);
    for index in 0..count {
        if !task.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![task.addr() + 8]);
        }
        accepted = e.vcall(this.addr(), 0x18, &args![index, task]).bool();
        if accepted {
            break;
        }
    }
    let activity = word(e, this, 0x24).wrapping_sub(1);
    set_word(e, this, 0x24, activity);
    if activity == 0 {
        set_word(e, this, 0x20, 0);
    }
    if !task.is_null() {
        e.call(0x0044_dd60, &args![task]);
    }
    accepted
}

// Translated from 00c3fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys each task thread and the task manager's two arrays.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3fa70 - SEH unwinding is not modeled.
pub fn fn_00c3fa70(e: &mut Engine, this: Ptr) {
    set_word(e, this, 0, 0x010c_167c);
    let mut index = 0;
    while index < word(e, this, 0x4c) {
        let threads = word(e, this, 0x50);
        let thread = e.mem.u32(threads.wrapping_add(index.wrapping_mul(4)));
        if thread != 0 {
            e.vcall(thread, 0, &args![1u32]);
        }
        index = index.wrapping_add(1);
    }
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 0x50)]);
    e.call(DEALLOCATE, &args![0x011f_6238u32, word(e, this, 0x54)]);
    fn_00c3f8c0(e, this);
}

// Translated from 00c3fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a task through the manager's virtual key, thread, and insertion methods.
/// The game's SEH unwind path is omitted.
// Translation incomplete: 00c3fb50 - SEH unwinding is not modeled.
pub fn fn_00c3fb50(e: &mut Engine, this: Ptr, task: Ptr) -> bool {
    set_word(e, task, 0x0c, 1);
    let low = word(e, task, 0x10);
    let high = word(e, task, 0x14);
    let bucket = e.vcall(this.addr(), 0x24, &args![low, high]).u32();
    let thread = e.vcall(this.addr(), 0x58, &args![task]).u32();
    let retained = e.with_stack(4, |e, retained| {
        e.mem.set_u32(retained.addr(), task.addr());
        if !task.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![task.addr() + 8]);
        }
        let updated_low = word(e, task, 0x10);
        let updated_high = word(e, task, 0x14);
        let result = e
            .vcall(
                this.addr(),
                0x10,
                &args![updated_low, updated_high, retained, 0u32],
            )
            .bool();
        let remaining = e.mem.u32(retained.addr());
        if remaining != 0 {
            e.call(0x0044_dd60, &args![remaining]);
        }
        result
    });
    if retained {
        let counts = word(e, this, 0x54);
        e.call(
            INTERLOCKED_INCREMENT,
            &args![counts.wrapping_add(bucket.wrapping_mul(4))],
        );
        e.call(INTERLOCKED_INCREMENT, &args![this.addr() + 0x58]);
        let threads = word(e, this, 0x50);
        let target = e.mem.u32(threads.wrapping_add(thread.wrapping_mul(4)));
        e.call(0x0044_2600, &args![target]);
    }
    retained
}

// Translated from 00c3fc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls a task's virtual method at slot +4.
pub fn fn_00c3fc80(e: &mut Engine, _this: Ptr, task: Ptr) {
    e.vcall(task.addr(), 4, &[]);
}

// Translated from 00c3fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a task between state transitions 4 and 5, then enqueues it.
pub fn fn_00c3fca0(e: &mut Engine, this: Ptr, task: Ptr) {
    set_word(e, task, 0x0c, 4);
    e.vcall(task.addr(), 8, &[]);
    set_word(e, task, 0x0c, 5);
    e.call(0x0044_e050, &args![this, task]);
}

// Translated from 00c3fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cancels queued tasks until a full pass finds none left.
/// The game's SEH frame and unreachable cleanup branch are omitted.
// Translation incomplete: 00c3fce0 - SEH unwinding is not modeled.
pub fn fn_00c3fce0(e: &mut Engine, this: Ptr) {
    let mut found_any = true;
    while found_any {
        found_any = false;
        e.with_stack(0x50, |e, cursor| {
            set_word(e, cursor, 0, 0x010c_1664);
            set_word(e, cursor, 8, 0);
            set_word(e, cursor, 0x10, 0);
            set_word(e, cursor, 0x14, 0);
            e.mem.set_u8(cursor.addr() + 0x18, 0);
            while e.mem.u8(cursor.addr() + 0x18) & 2 == 0 {
                let key = cursor.addr() + 0x30;
                let task_out = cursor.addr() + 0x3c;
                e.mem.set_u32(key, 0);
                e.mem.set_u32(key + 4, 0);
                e.mem.set_u32(task_out, 0);
                let found = e
                    .call(0x00c4_0e70, &args![this, cursor, key, task_out, 1u32])
                    .bool();
                let task = e.mem.u32(task_out);
                if found {
                    e.call(0x0044_ac40, &args![this, task, 0u32]);
                    found_any = true;
                }
                if task != 0 {
                    e.call(0x0044_dd60, &args![task]);
                }
            }
        });
    }
}

// Translated from 00c3fe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a queue iterator or request record's nine words.
pub fn fn_00c3fe00(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    for (index, value) in [first, second, third, fourth, 0, 0, 0, 0, 0]
        .iter()
        .copied()
        .enumerate()
    {
        set_word(e, this, index as u32 * 4, value);
    }
    this
}

// Translated from 00c3fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets a task from a marked map node and clears the caller's three hazard pointers.
pub fn fn_00c3fe80(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    task_out: Ptr,
) -> bool {
    let mut found = e
        .call(0x00c4_16a0, &args![this, first, second, third])
        .bool();
    if found {
        let node = word(e, this, 0x14) & !1;
        assign_ref_from(e, task_out.addr(), node + 8);
        if e.mem.u32(node + 0x0c) & 1 != 0 {
            found = false;
        }
    }
    for offset in [4, 8, 0x0c] {
        let hazard = word(e, this, offset);
        e.mem.set_u32(hazard, 0);
    }
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0045_6520, io_manager_load_queued_priority(Ptr)),
        entry!(0x005a_8040, fn_005a8040(Ptr, u32)),
        entry!(0x00c3_db60, fn_00c3db60(Ptr)),
        entry!(0x00c3_df40, io_manager_requeue_task(Ptr, Ptr, i32)),
        entry!(0x00c3_dfa0, fn_00c3dfa0(Ptr, i32)),
        entry!(0x00c3_e1b0, fn_00c3e1b0(Ptr, i32, i32)),
        entry!(0x00c3_e310, fn_00c3e310(Ptr)),
        entry!(0x00c3_e340, fn_00c3e340(Ptr)),
        entry!(0x00c3_e380, fn_00c3e380(Ptr)),
        entry!(0x00c3_e3a0, fn_00c3e3a0(Ptr, u32, u32) -> i32),
        entry!(0x00c3_e3f0, fn_00c3e3f0(Ptr, u32) -> Ptr),
        entry!(0x00c3_e420, fn_00c3e420(Ptr, Ptr) -> bool),
        entry!(0x00c3_e490, fn_00c3e490(Ptr, Ptr) -> bool),
        entry!(0x00c3_e4f0, fn_00c3e4f0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00c3_e690, fn_00c3e690(Ptr, Ptr, u32, u32) -> bool),
        entry!(0x00c3_e750, fn_00c3e750(Ptr) -> bool),
        entry!(0x00c3_e7d0, fn_00c3e7d0(Ptr) -> bool),
        entry!(0x00c3_e860, fn_00c3e860(Ptr, u32) -> u32),
        entry!(0x00c3_e8b0, fn_00c3e8b0(Ptr, u32, u32, Ptr) -> Ptr),
        entry!(0x00c3_ea10, fn_00c3ea10(Ptr)),
        entry!(0x00c3_ea60, fn_00c3ea60(Ptr, Ptr) -> bool),
        entry!(0x00c3_ec80, fn_00c3ec80(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00c3_ee70, bs_task_manager_thread_construct(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00c3_eeb0, fn_00c3eeb0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00c3_f090, fn_00c3f090(Ptr, Ptr)),
        entry!(0x00c3_f6f0, lock_free_map_increment_count(Ptr)),
        entry!(0x00c3_f710, lock_free_priority_queue_increment_count(Ptr)),
        entry!(0x00c3_f730, lock_free_priority_queue_decrement_count(Ptr)),
        entry!(0x00c3_f750, fn_00c3f750(Ptr) -> Ptr),
        entry!(0x00c3_f7a0, lock_free_queue_allocate_interface(Ptr, u32) -> Ptr),
        entry!(0x00c3_f8a0, lock_free_map_decrement_count(Ptr)),
        entry!(0x00c3_f8c0, fn_00c3f8c0(Ptr)),
        entry!(0x00c3_f960, fn_00c3f960(Ptr, Ptr) -> bool),
        entry!(0x00c3_fa70, fn_00c3fa70(Ptr)),
        entry!(0x00c3_fb50, fn_00c3fb50(Ptr, Ptr) -> bool),
        entry!(0x00c3_fc80, fn_00c3fc80(Ptr, Ptr)),
        entry!(0x00c3_fca0, fn_00c3fca0(Ptr, Ptr)),
        entry!(0x00c3_fce0, fn_00c3fce0(Ptr)),
        entry!(0x00c3_fe00, fn_00c3fe00(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00c3_fe80, fn_00c3fe80(Ptr, u32, u32, u32, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn one(_: &mut Engine, _: &[u32]) -> Ret {
        1u32.into_ret()
    }

    fn increment(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], value);
        value.into_ret()
    }

    fn decrement(e: &mut Engine, a: &[u32]) -> Ret {
        let value = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], value);
        value.into_ret()
    }

    fn compare_exchange(e: &mut Engine, a: &[u32]) -> Ret {
        let old = e.mem.u32(a[0]);
        if old == a[2] {
            e.mem.set_u32(a[0], a[1]);
        }
        old.into_ret()
    }

    fn alloc(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.alloc(a[1].max(1)).into_ret()
    }

    fn setup_atomics(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, increment);
        e.register(INTERLOCKED_DECREMENT, decrement);
        e.register(INTERLOCKED_COMPARE_EXCHANGE, compare_exchange);
    }

    fn setup_allocator(e: &mut Engine) {
        e.register(ALLOCATE, alloc);
        e.register(DEALLOCATE, no_op);
        e.register(0x00c4_2180, |_, a| a[0].into_ret());
    }

    fn setup_destructor(e: &mut Engine) {
        e.register(0x00c4_0b70, no_op);
        e.register(DEALLOCATE, no_op);
    }

    #[test]
    fn load_queued_priority_uses_five() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x90);
        set_word(&mut e, manager, 0x68, 7);
        io_manager_load_queued_priority(&mut e, manager);
        assert_eq!(word(&e, manager, 0x68), 7);
    }

    #[test]
    fn store_word_at_54() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x58);
        fn_005a8040(&mut e, manager, 0x1234);
        assert_eq!(word(&e, manager, 0x54), 0x1234);
    }

    #[test]
    fn destructor_resets_vtable_and_releases_base() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x90);
        fn_00c3db60(&mut e, manager);
        assert_eq!(word(&e, manager, 0), 0x0101_7458);
    }

    #[test]
    fn requeue_sets_priority_byte() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        set_word(&mut e, task, 0x10, 0x4455_6677);
        set_word(&mut e, task, 0x14, 0x1122_3344);
        io_manager_requeue_task(&mut e, manager, task, 3);
        assert_eq!(word(&e, task, 0x10), 0x4403_6677);
        assert_eq!(word(&e, task, 0x14), 0x1122_3344);
    }

    #[test]
    fn load_loop_stops_when_already_loading() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x68, 7);
        fn_00c3dfa0(&mut e, manager, 5);
        assert_eq!(word(&e, manager, 0x68), 7);
    }

    #[test]
    fn lower_priority_range_is_empty() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        fn_00c3e1b0(&mut e, manager, 2, 2);
    }

    #[test]
    fn priority_runner_reloads_task_pointer_after_virtual_call() {
        use std::cell::Cell;
        use std::rc::Rc;

        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.map(0x0120_2df0, 1);
        e.register(0x00c4_2060, no_op);
        e.register(0x0044_dd60, no_op);
        let manager = block(&mut e, 0x84);
        let first = block(&mut e, 4);
        let replacement = block(&mut e, 4);
        e.put_vtable(0x7900_0000, &[0, 0, 0, 0, 0, 0, 0, 0x7900_001c]);
        set_word(&mut e, first, 0, 0x7900_0000);
        let out_slot = Rc::new(Cell::new(0u32));
        let saved_slot = out_slot.clone();
        let first_addr = first.addr();
        e.register_double(0x00c4_0e70, move |e, a| {
            saved_slot.set(a[3]);
            e.mem.set_u32(a[2], 0x0001_0000);
            e.mem.set_u32(a[3], first_addr);
            e.mem.set_u8(a[1] + 0x18, 2);
            1u32.into_ret()
        });
        let replacement_addr = replacement.addr();
        e.register_double(0x7900_001c, move |e, _| {
            e.mem.set_u32(out_slot.get(), replacement_addr);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c3e1b0(&mut e, manager, 1, 0);
        let releases: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(addr, _)| *addr == 0x0044_dd60)
            .collect();
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].1[0], replacement_addr);
    }

    #[test]
    fn nested_pause_skips_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.map(0x0120_2df0, 1);
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x48, 1);
        fn_00c3e310(&mut e, manager);
        assert_eq!(word(&e, manager, 0x48), 2);
    }

    #[test]
    fn nested_resume_skips_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x48, 2);
        fn_00c3e340(&mut e, manager);
        assert_eq!(word(&e, manager, 0x48), 1);
    }

    #[test]
    fn clear_manager_resets_empty_queue_count() {
        let mut e = Engine::new();
        e.register(0x00c4_0e70, |e, a| {
            e.mem.set_u8(a[1] + 0x18, 2);
            Ret::default()
        });
        let manager = block(&mut e, 0x84);
        let queue = block(&mut e, 0x10);
        set_word(&mut e, manager, 0x64, queue.addr());
        set_word(&mut e, queue, 0x0c, 17);
        fn_00c3e380(&mut e, manager);
        assert_eq!(word(&e, queue, 0x0c), 0);
    }

    #[test]
    fn bucket_formula_uses_key_bytes() {
        let mut e = Engine::new();
        assert_eq!(
            fn_00c3e3a0(&mut e, Ptr::new(0), 0x0003_0000, 0x0200_0000),
            18
        );
    }

    #[test]
    fn scalar_delete_returns_this() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x90);
        assert_eq!(fn_00c3e3f0(&mut e, manager, 0), manager);
    }

    #[test]
    fn empty_queue_scan_returns_false() {
        let mut e = Engine::new();
        let queue = block(&mut e, 0x10);
        assert!(!fn_00c3e420(&mut e, queue, Ptr::new(0)));
    }

    #[test]
    fn queue_scan_rechecks_count_after_pop_callback() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        let outer = block(&mut e, 0x10);
        let slots = block(&mut e, 8);
        let inner = block(&mut e, 0x28);
        let queue = block(&mut e, 0x10);
        let sentinel = block(&mut e, 8);
        let hazard_head = block(&mut e, 4);
        let hazard_next = block(&mut e, 4);
        let output = block(&mut e, 4);
        set_word(&mut e, outer, 4, slots.addr());
        set_word(&mut e, outer, 8, 2);
        set_word(&mut e, slots, 0, inner.addr());
        set_word(&mut e, inner, 0, queue.addr());
        set_word(&mut e, inner, 4, hazard_head.addr());
        set_word(&mut e, inner, 8, hazard_next.addr());
        set_word(&mut e, inner, 0x24, 1);
        set_word(&mut e, queue, 4, sentinel.addr());
        let outer_addr = outer.addr();
        e.register_double(0x0044_9f80, move |e, a| {
            e.mem.set_u32(outer_addr + 8, 1);
            a[0].into_ret()
        });
        assert!(!fn_00c3e420(&mut e, outer, output));
    }

    #[test]
    fn queue_pop_miss_releases_activity() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        e.register(0x0044_9f80, |_, a| a[0].into_ret());
        let queue = block(&mut e, 0x28);
        let state = block(&mut e, 0x10);
        let sentinel = block(&mut e, 8);
        let hazard_head = block(&mut e, 4);
        let hazard_next = block(&mut e, 4);
        set_word(&mut e, queue, 0, state.addr());
        set_word(&mut e, queue, 4, hazard_head.addr());
        set_word(&mut e, queue, 8, hazard_next.addr());
        set_word(&mut e, state, 4, sentinel.addr());
        let item = block(&mut e, 4);
        set_word(&mut e, queue, 0x24, 1);
        assert!(!fn_00c3e490(&mut e, queue, item));
        assert_eq!(word(&e, queue, 0x24), 0);
    }

    #[test]
    fn task_manager_constructor_sets_thread_count() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        e.register(0x00ec_61c0, no_op);
        e.register(0x00c4_2f50, no_op);
        e.register(0x00c4_2dd0, no_op);
        let manager = block(&mut e, 0x5c);
        fn_00c3e4f0(&mut e, manager, 1, 1, 1);
        assert_eq!(word(&e, manager, 0x4c), 1);
        assert_ne!(word(&e, manager, 0x50), 0);
    }

    #[test]
    fn requeue_idle_task_writes_both_key_words() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        assert!(fn_00c3e690(&mut e, manager, task, 7, 8));
        assert_eq!(word(&e, task, 0x10), 7);
        assert_eq!(word(&e, task, 0x14), 8);
    }

    #[test]
    fn first_pause_returns_true() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.map(0x0120_2df0, 1);
        let manager = block(&mut e, 0x5c);
        assert!(fn_00c3e750(&mut e, manager));
        assert_eq!(word(&e, manager, 0x48), 1);
    }

    #[test]
    fn final_resume_returns_true() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let manager = block(&mut e, 0x5c);
        set_word(&mut e, manager, 0x48, 1);
        assert!(fn_00c3e7d0(&mut e, manager));
        assert_eq!(word(&e, manager, 0x48), 0);
    }

    #[test]
    fn bucket_sum_includes_last() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x5c);
        let counts = block(&mut e, 12);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, counts, 0, 2);
        set_word(&mut e, counts, 4, 3);
        set_word(&mut e, counts, 8, 5);
        assert_eq!(fn_00c3e860(&mut e, manager, 2), 10);
    }

    #[test]
    fn priority_queue_constructor_allocates_one_interface() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let queue = block(&mut e, 0x10);
        fn_00c3e8b0(&mut e, queue, 1, 1, Ptr::new(0));
        let slots = word(&e, queue, 4);
        assert_ne!(e.mem.u32(slots), 0);
    }

    #[test]
    fn reset_queue_signals_each_entry() {
        let mut e = Engine::new();
        e.register(0x00c4_1330, no_op);
        let queue = block(&mut e, 0x10);
        let items = block(&mut e, 4);
        set_word(&mut e, queue, 4, items.addr());
        set_word(&mut e, queue, 8, 1);
        set_word(&mut e, queue, 0x0c, 3);
        e.call_log = Some(vec![]);
        fn_00c3ea10(&mut e, queue);
        assert_eq!(word(&e, queue, 0x0c), 0);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, _)| *addr == 0x00c4_1330));
    }

    #[test]
    fn empty_lock_free_pop_clears_hazards() {
        let mut e = Engine::new();
        let interface = block(&mut e, 0x20);
        let queue = block(&mut e, 0x10);
        let sentinel = block(&mut e, 8);
        let first_hazard = block(&mut e, 4);
        let second_hazard = block(&mut e, 4);
        let output = block(&mut e, 4);
        set_word(&mut e, interface, 0, queue.addr());
        set_word(&mut e, interface, 4, first_hazard.addr());
        set_word(&mut e, interface, 8, second_hazard.addr());
        set_word(&mut e, queue, 4, sentinel.addr());
        assert!(!fn_00c3ea60(&mut e, interface, output));
        assert_eq!(word(&e, first_hazard, 0), 0);
    }

    #[test]
    fn map_constructor_sets_capacity_and_arrays() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let map = block(&mut e, 0x28);
        fn_00c3ec80(&mut e, map, 2, 1, 4);
        assert_eq!(word(&e, map, 8), 1);
        assert_ne!(word(&e, map, 4), 0);
    }

    #[test]
    fn thread_constructor_sets_manager() {
        let mut e = Engine::new();
        e.register(0x00c4_2dd0, no_op);
        let thread = block(&mut e, 0x34);
        let manager = block(&mut e, 4);
        assert_eq!(
            bs_task_manager_thread_construct(&mut e, thread, manager, 7),
            thread
        );
        assert_eq!(word(&e, thread, 0x30), manager.addr());
    }

    #[test]
    fn queue_interface_constructor_sets_sentinel() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let interface = block(&mut e, 0x40);
        fn_00c3eeb0(&mut e, interface, 2, Ptr::new(0x1234));
        assert_eq!(word(&e, interface, 4), word(&e, interface, 8));
        assert_eq!(word(&e, interface, 0x0c), 0x1234);
    }

    #[test]
    fn retired_node_is_linked_and_counted() {
        let mut e = Engine::new();
        let interface = block(&mut e, 0x20);
        let queue = block(&mut e, 0x10);
        let node = block(&mut e, 8);
        set_word(&mut e, interface, 0, queue.addr());
        set_word(&mut e, queue, 0x0c, 2);
        fn_00c3f090(&mut e, interface, node);
        assert_eq!(word(&e, interface, 0x10), node.addr());
        assert_eq!(word(&e, interface, 0x0c), 1);
    }

    #[test]
    fn map_count_increments_at_offset_18() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let map = block(&mut e, 0x1c);
        lock_free_map_increment_count(&mut e, map);
        assert_eq!(word(&e, map, 0x18), 1);
    }

    #[test]
    fn priority_queue_count_increments_at_offset_c() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let queue = block(&mut e, 0x10);
        lock_free_priority_queue_increment_count(&mut e, queue);
        assert_eq!(word(&e, queue, 0x0c), 1);
    }

    #[test]
    fn priority_queue_count_decrements_at_offset_c() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let queue = block(&mut e, 0x10);
        set_word(&mut e, queue, 0x0c, 2);
        lock_free_priority_queue_decrement_count(&mut e, queue);
        assert_eq!(word(&e, queue, 0x0c), 1);
    }

    #[test]
    fn existing_tls_interface_is_returned() {
        let mut e = Engine::new();
        e.register(0x00fd_f044, |_, _| 0x4321u32.into_ret());
        let map = block(&mut e, 0x18);
        let manager = block(&mut e, 8);
        set_word(&mut e, map, 0x14, manager.addr());
        set_word(&mut e, manager, 4, 5);
        assert_eq!(fn_00c3f750(&mut e, map).addr(), 0x4321);
    }

    #[test]
    fn allocate_interface_passes_adjacent_hazards() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        e.register(0x0044_cd00, |_, a| a[0].into_ret());
        let queue = block(&mut e, 0x14);
        let hazards = block(&mut e, 16);
        set_word(&mut e, queue, 0x10, hazards.addr());
        e.call_log = Some(vec![]);
        assert!(!lock_free_queue_allocate_interface(&mut e, queue, 1).is_null());
        let call = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(addr, _)| *addr == 0x0044_cd00)
            .unwrap();
        assert_eq!(call.1[2], hazards.addr() + 8);
        assert_eq!(call.1[3], hazards.addr() + 12);
    }

    #[test]
    fn map_count_decrements_at_offset_18() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let map = block(&mut e, 0x1c);
        set_word(&mut e, map, 0x18, 2);
        lock_free_map_decrement_count(&mut e, map);
        assert_eq!(word(&e, map, 0x18), 1);
    }

    #[test]
    fn map_destructor_resets_base_vtable() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let map = block(&mut e, 0x28);
        fn_00c3f8c0(&mut e, map);
        assert_eq!(word(&e, map, 0), 0x0101_7458);
    }

    #[test]
    fn empty_priority_scan_releases_activity() {
        let mut e = Engine::new();
        e.register(0x0040_fbf0, no_op);
        let queue = block(&mut e, 0x28);
        set_word(&mut e, queue, 0x24, 1);
        assert!(!fn_00c3f960(&mut e, queue, Ptr::new(0)));
        assert_eq!(word(&e, queue, 0x24), 0);
    }

    #[test]
    fn manager_destructor_frees_arrays() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x5c);
        e.call_log = Some(vec![]);
        fn_00c3fa70(&mut e, manager);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(addr, _)| *addr == DEALLOCATE)
                .count(),
            4
        );
    }

    #[test]
    fn run_task_updates_bucket_and_thread_counts() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0044_dd60, no_op);
        e.register(0x0044_2600, no_op);
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        let counts = block(&mut e, 4);
        let threads = block(&mut e, 4);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x50, threads.addr());
        let mut slots = vec![0; 0x5c / 4];
        slots[0x10 / 4] = 0x7000_0010;
        slots[0x24 / 4] = 0x7000_0024;
        slots[0x58 / 4] = 0x7000_0058;
        e.put_vtable(0x7100_0000, &slots);
        set_word(&mut e, manager, 0, 0x7100_0000);
        e.register(0x7000_0010, one);
        e.register(0x7000_0024, no_op);
        e.register(0x7000_0058, no_op);
        assert!(fn_00c3fb50(&mut e, manager, task));
        assert_eq!(word(&e, counts, 0), 1);
        assert_eq!(word(&e, manager, 0x58), 1);
    }

    #[test]
    fn task_virtual_slot_four_is_called() {
        let mut e = Engine::new();
        let task = block(&mut e, 4);
        e.put_vtable(0x7200_0000, &[0, 0x7200_0004]);
        set_word(&mut e, task, 0, 0x7200_0000);
        e.register(0x7200_0004, no_op);
        e.call_log = Some(vec![]);
        fn_00c3fc80(&mut e, Ptr::new(0), task);
        assert_eq!(e.call_log.as_ref().unwrap()[0].0, 0x7200_0004);
    }

    #[test]
    fn task_finishes_in_state_five() {
        let mut e = Engine::new();
        let manager = block(&mut e, 4);
        let task = block(&mut e, 0x10);
        e.put_vtable(0x7300_0000, &[0, 0, 0x7300_0008]);
        set_word(&mut e, task, 0, 0x7300_0000);
        e.register(0x7300_0008, no_op);
        e.register(0x0044_e050, no_op);
        fn_00c3fca0(&mut e, manager, task);
        assert_eq!(word(&e, task, 0x0c), 5);
    }

    #[test]
    fn cancel_empty_queue_stops_after_one_pass() {
        let mut e = Engine::new();
        e.register(0x00c4_0e70, |e, a| {
            e.mem.set_u8(a[1] + 0x18, 2);
            Ret::default()
        });
        let manager = block(&mut e, 0x5c);
        fn_00c3fce0(&mut e, manager);
    }

    #[test]
    fn record_constructor_clears_remaining_words() {
        let mut e = Engine::new();
        let record = block(&mut e, 0x24);
        fn_00c3fe00(&mut e, record, 1, 2, 3, 4);
        assert_eq!(word(&e, record, 0x0c), 4);
        assert_eq!(word(&e, record, 0x20), 0);
    }

    #[test]
    fn failed_map_lookup_clears_three_hazards() {
        let mut e = Engine::new();
        e.register(0x00c4_16a0, no_op);
        let interface = block(&mut e, 0x18);
        let hazards = [block(&mut e, 4), block(&mut e, 4), block(&mut e, 4)];
        for (index, hazard) in hazards.iter().enumerate() {
            set_word(&mut e, interface, 4 + index as u32 * 4, hazard.addr());
            set_word(&mut e, *hazard, 0, 99);
        }
        let output = block(&mut e, 4);
        assert!(!fn_00c3fe80(&mut e, interface, 1, 2, 3, output));
        assert!(hazards.iter().all(|h| word(&e, *h, 0) == 0));
    }

    #[test]
    fn queued_count_reloads_queue_after_callback_swap() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        let first = block(&mut e, 4);
        let second = block(&mut e, 4);
        e.put_vtable(0x7400_0000, &[0, 0, 0, 0x7400_000c]);
        e.put_vtable(0x7500_0000, &[0, 0, 0, 0x7500_000c]);
        set_word(&mut e, first, 0, 0x7400_0000);
        set_word(&mut e, second, 0, 0x7500_0000);
        set_word(&mut e, manager, 0x64, first.addr());
        let manager_addr = manager.addr();
        let second_addr = second.addr();
        e.register_double(0x7400_000c, move |e, _| {
            e.mem.set_u32(manager_addr + 0x64, second_addr);
            1u32.into_ret()
        });
        e.register(0x7500_000c, |_, _| 2u32.into_ret());
        assert_eq!(count_queued(&mut e, manager), 1);
        assert_eq!(count_queued(&mut e, manager), 2);
    }

    #[test]
    fn external_count_callback_has_no_argument_words() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        set_word(&mut e, manager, 0x6c, 0x7d00_0000);
        e.register(0x7d00_0000, |_, a| {
            assert!(a.is_empty());
            3u32.into_ret()
        });
        assert_eq!(external_count(&mut e, manager), 3);
    }

    #[test]
    fn loading_loop_uses_replaced_queue_and_exe_progress_scale() {
        let mut e = Engine::new();
        let manager = block(&mut e, 0x84);
        let first = block(&mut e, 4);
        let second = block(&mut e, 4);
        let counts = block(&mut e, 16);
        e.put_vtable(0x7a00_0000, &[0, 0, 0, 0x7a00_000c]);
        e.put_vtable(0x7b00_0000, &[0, 0, 0, 0x7b00_000c]);
        set_word(&mut e, first, 0, 0x7a00_0000);
        set_word(&mut e, second, 0, 0x7b00_0000);
        set_word(&mut e, manager, 0x64, first.addr());
        set_word(&mut e, manager, 0x18, 1);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x78, 0x7c00_0000);
        e.map(0x0101_7a40, 8);
        e.set_global(0x0101_7a40, 37.0f64);
        e.map(0x0101_6410, 4);
        e.set_global(0x0101_6410, 100.0f32);
        e.map(0x011a_f70c, 1);
        e.register(0x7a00_000c, |_, _| 2u32.into_ret());
        let mut second_reads = 0;
        e.register_double(0x7b00_000c, move |_, _| {
            second_reads += 1;
            (if second_reads <= 2 { 1u32 } else { 0u32 }).into_ret()
        });
        e.register(SLEEP, no_op);
        let mut process_calls = 0;
        e.register_double(0x00c3_dbf0, move |_, _| {
            process_calls += 1;
            (process_calls == 2).into_ret()
        });
        let manager_addr = manager.addr();
        let second_addr = second.addr();
        e.register_double(0x7c00_0000, move |e, a| {
            if a[0] != 100.0f32.to_bits() {
                e.mem.set_u32(manager_addr + 0x64, second_addr);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c3dfa0(&mut e, manager, 0);
        assert_eq!(word(&e, manager, 0x68), 6);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(addr, _)| *addr == 0x7b00_000c));
        let progress: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(addr, _)| *addr == 0x7c00_0000)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(
            progress,
            vec![0.0f32.to_bits(), 18.5f32.to_bits(), 100.0f32.to_bits()]
        );
    }

    #[test]
    fn map_constructor_signed_decrement_skips_high_bit_capacity() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, _| {
            let p = e.mem.alloc(16);
            e.mem.set_u32(p, 0xdead_beef);
            p.into_ret()
        });
        e.register(0x00c4_2180, |_, a| a[0].into_ret());
        let map = block(&mut e, 0x28);
        fn_00c3ec80(&mut e, map, 1, 0x8000_0001, 1);
        let first = word(&e, map, 0x0c);
        assert_eq!(e.mem.u32(first), 0xdead_beef);
    }

    #[test]
    fn queue_pop_rereads_item_after_release_callback() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        let interface = block(&mut e, 0x20);
        let queue = block(&mut e, 0x10);
        let head = block(&mut e, 8);
        let next = block(&mut e, 8);
        let previous_item = block(&mut e, 0x10);
        let first_item = block(&mut e, 0x10);
        let replacement_item = block(&mut e, 0x10);
        let output = block(&mut e, 4);
        let hazard_head = block(&mut e, 4);
        let hazard_next = block(&mut e, 4);
        set_word(&mut e, interface, 0, queue.addr());
        set_word(&mut e, interface, 4, hazard_head.addr());
        set_word(&mut e, interface, 8, hazard_next.addr());
        set_word(&mut e, queue, 4, head.addr());
        set_word(&mut e, queue, 8, next.addr());
        set_word(&mut e, queue, 0x0c, 2);
        set_word(&mut e, head, 0, next.addr());
        set_word(&mut e, next, 4, first_item.addr());
        set_word(&mut e, output, 0, previous_item.addr());
        e.put_vtable(0x7600_0000, &[0, 0, 0, 0x7600_000c]);
        set_word(&mut e, queue, 0, 0x7600_0000);
        e.register(0x7600_000c, no_op);
        let old_addr = previous_item.addr();
        let next_addr = next.addr();
        let replacement_addr = replacement_item.addr();
        e.register_double(0x0044_dd60, move |e, a| {
            if a[0] == old_addr {
                e.mem.set_u32(next_addr + 4, replacement_addr);
            }
            Ret::default()
        });
        assert!(fn_00c3ea60(&mut e, interface, output));
        assert_eq!(word(&e, output, 0), replacement_addr);
    }

    #[test]
    fn run_task_rereads_key_after_virtual_callbacks() {
        let mut e = Engine::new();
        setup_atomics(&mut e);
        e.register(0x0044_dd60, no_op);
        e.register(0x0044_2600, no_op);
        let manager = block(&mut e, 0x5c);
        let task = block(&mut e, 0x18);
        let counts = block(&mut e, 4);
        let threads = block(&mut e, 4);
        set_word(&mut e, manager, 0x54, counts.addr());
        set_word(&mut e, manager, 0x50, threads.addr());
        set_word(&mut e, task, 0x10, 1);
        set_word(&mut e, task, 0x14, 2);
        let mut slots = vec![0; 0x5c / 4];
        slots[0x10 / 4] = 0x7700_0010;
        slots[0x24 / 4] = 0x7700_0024;
        slots[0x58 / 4] = 0x7700_0058;
        e.put_vtable(0x7701_0000, &slots);
        set_word(&mut e, manager, 0, 0x7701_0000);
        let task_addr = task.addr();
        e.register_double(0x7700_0024, move |e, _| {
            e.mem.set_u32(task_addr + 0x10, 7);
            e.mem.set_u32(task_addr + 0x14, 8);
            0u32.into_ret()
        });
        e.register(0x7700_0058, no_op);
        e.register(0x7700_0010, |_, a| {
            assert_eq!((a[1], a[2]), (7, 8));
            1u32.into_ret()
        });
        assert!(fn_00c3fb50(&mut e, manager, task));
    }

    #[test]
    fn queue_constructor_rechecks_count_after_nested_constructor() {
        let mut e = Engine::new();
        setup_allocator(&mut e);
        let queue = block(&mut e, 0x10);
        let queue_addr = queue.addr();
        e.register_double(0x00c4_2180, move |e, a| {
            e.mem.set_u32(queue_addr + 8, 1);
            a[0].into_ret()
        });
        fn_00c3e8b0(&mut e, queue, 2, 1, Ptr::new(0));
        let slots = word(&e, queue, 4);
        assert_ne!(e.mem.u32(slots), 0);
        assert_eq!(e.mem.u32(slots + 4), 0);
    }

    #[test]
    fn manager_destructor_reloads_thread_array_before_free() {
        let mut e = Engine::new();
        setup_destructor(&mut e);
        let manager = block(&mut e, 0x5c);
        let first_array = block(&mut e, 8);
        let second_array = block(&mut e, 8);
        let thread = block(&mut e, 4);
        set_word(&mut e, first_array, 0, thread.addr());
        set_word(&mut e, manager, 0x4c, 2);
        set_word(&mut e, manager, 0x50, first_array.addr());
        e.put_vtable(0x7800_0000, &[0x7800_0004]);
        set_word(&mut e, thread, 0, 0x7800_0000);
        let manager_addr = manager.addr();
        let replacement_addr = second_array.addr();
        e.register_double(0x7800_0004, move |e, _| {
            e.mem.set_u32(manager_addr + 0x4c, 1);
            e.mem.set_u32(manager_addr + 0x50, replacement_addr);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00c3fa70(&mut e, manager);
        let freed = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(addr, _)| *addr == DEALLOCATE)
            .unwrap();
        assert_eq!(freed.1[1], replacement_addr);
    }
}
