//! `fallout/misc/taskqueueinterface.cpp` (Xbox PDB source unit).
//! FalloutNV.exe 1.4.0.525 task-record producers. The 32-byte records have
//! a tag followed by argument slots at four-byte intervals. Byte arguments
//! write only the low byte of their slot; unused bytes are unspecified in
//! the exe (the engine's temporary-memory allocator starts them at zero).
//! Compiler security-cookie checks are omitted. QueueIfActive at 0087d160
//! is outside this batch and remains an address call until translated.

use crate::prelude::*;

layout! {
    /// `TaskQueueInterface` (Xbox PDB), verified by 0087a670's PC stores.
    pub struct TaskQueueInterface: 0x10 {
        0x00 pQueue: Ptr,
        0x04 pSecondaryQueue: Ptr,
        0x08 pActiveQueue: Ptr,
        0x0C iProcessingThread: u32,
    }
}

/// The FLD/FSTP pairs in the PC wrappers quiet signaling single-precision
/// NaNs while preserving their sign and payload.
fn x87_task_float(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

/// Build the exe's local task record, retaining only the fields the caller
/// writes. `prepare` includes its calls before the record is dispatched.
fn with_task(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    tag: u32,
    prepare: impl FnOnce(&mut Engine, Ptr),
) {
    e.with_stack(0x20, |e, task| {
        e.mem.set_u32(task.addr(), tag);
        prepare(e, task);
        e.call(0x0087_d160, &args![this, task]);
    });
}

fn with_task_if_target(e: &mut Engine, this: Ptr<TaskQueueInterface>, object: Ptr) {
    e.with_stack(0x20, |e, task| {
        e.mem.set_u32(task.addr(), 0x12);
        let target = e.call(0x0059_bb30, &args![object]).u32();
        let actor = e.call(0x0082_2b40, &args![target]).u32();
        if actor != 0 {
            let target = e.call(0x0059_bb30, &args![object]).u32();
            let actor = e.call(0x0082_2b40, &args![target]).u32();
            if e.vcall(actor, 0x1d0, &[]).u32() == 0 {
                return;
            }
        }
        e.mem.set_u32(task.addr() + 4, object.addr());
        e.call(0x0087_d160, &args![this, task]);
    });
}

// Translated from 0087a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the two queues, makes the first active and clears the thread.
pub fn fn_0087a670(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    first: Ptr,
    second: Ptr,
) -> Ptr<TaskQueueInterface> {
    e.set(this, TaskQueueInterface::pQueue, first);
    e.set(this, TaskQueueInterface::pSecondaryQueue, second);
    e.set(this, TaskQueueInterface::iProcessingThread, 0);
    let first = e.get(this, TaskQueueInterface::pQueue);
    e.set(this, TaskQueueInterface::pActiveQueue, first);
    this
}

// Translated from 0087a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the argument through the object's virtual slot +4.
pub fn fn_0087a890(e: &mut Engine, this: Ptr, argument: u32) -> u8 {
    e.vcall(this.addr(), 4, &args![argument]).u8()
}

// Translation incomplete: 0087aad0 - compiler exception unwinding for the temporary reference is not implemented.
// Translated from 0087aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the global container is enabled, constructs a temporary reference,
/// inserts it in the global collection, and destroys it on the normal path.
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0087aad0(e: &mut Engine, _unused_this: Ptr<TaskQueueInterface>, object: Ptr) {
    let enabled = e.call(0x0040_8d60, &args![0x011d_f1b8u32]).u32();
    if e.mem.u8(enabled) != 0 {
        e.with_stack(4, |e, reference| {
            e.call(0x0063_3c90, &args![reference, object]);
            e.call(0x006c_61b0, &args![0x011d_f1c4u32, reference]);
            e.call(0x0045_cec0, &args![reference]);
        });
    }
}

// Translated from 0087ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a two-word entry when the global container is enabled. Reloads
/// the indexed element before writing its second word, as the exe does.
pub fn fn_0087ab50(e: &mut Engine, _unused_this: Ptr<TaskQueueInterface>, first: u32, second: u32) {
    let enabled = e.call(0x0040_8d60, &args![0x011d_f214u32]).u32();
    if e.mem.u8(enabled) != 0 {
        let index = e.call(0x0084_5460, &args![0x011d_f244u32]).u32();
        let entry = e.call(0x0069_50d0, &args![0x011d_f244u32, index]).u32();
        e.mem.set_u32(entry, first);
        let entry = e.call(0x0069_50d0, &args![0x011d_f244u32, index]).u32();
        e.mem.set_u32(entry.wrapping_add(4), second);
    }
}

// Translated from 0087adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes the embedded member at +0x60 to 0040b460 (cdecl).
pub fn fn_0087adf0(e: &mut Engine, object: Ptr) {
    e.call(0x0040_b460, &args![object.addr().wrapping_add(0x60)]);
}

// Translation incomplete: 0087ae60 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues tag 0x0C only for a nonnull object with virtual slot +0x220
/// returning a nonzero low byte. Upper return bytes do not affect the gate.
pub fn fn_0087ae60(e: &mut Engine, this: Ptr<TaskQueueInterface>, object: Ptr) {
    if !object.is_null() && e.vcall(object.addr(), 0x220, &[]).u8() != 0 {
        with_task(e, this, 0x0c, |e, task| {
            e.mem.set_u32(task.addr() + 4, object.addr());
        });
    }
}

// Translation incomplete: 0087aec0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues tag 0x12 for a nonnull target with no actor, or with an actor
/// whose virtual slot +0x1D0 returns nonzero. The second actor lookup is
/// deliberately repeated rather than cached.
pub fn fn_0087aec0(e: &mut Engine, this: Ptr<TaskQueueInterface>, object: Ptr) {
    if object.is_null() {
        return;
    }
    with_task_if_target(e, this, object);
}

// Translation incomplete: 0087aa40 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueWeaponFire` (Xbox PDB): packs tag 0x20 and dispatches the task.
pub fn task_queue_interface_queue_weapon_fire(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
) {
    with_task(e, this, 0x20, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087aa90 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x04 and dispatches the task.
pub fn fn_0087aa90(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    with_task(e, this, 0x4, |_, _| {});
}

// Translation incomplete: 0087abb0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x06 and dispatches the task.
pub fn fn_0087abb0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    with_task(e, this, 0x6, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ac00 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueNodeAttach` (Xbox PDB): packs tag 0x07 and dispatches the task.
pub fn task_queue_interface_queue_node_attach(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: Ptr,
    argument_3: u8,
    argument_4: u8,
) {
    with_task(e, this, 0x7, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        e.mem.set_u8(task.addr() + 12, argument_3);
        e.mem.set_u8(task.addr() + 16, argument_4);
        e.call(0x0040_f6e0, &args![argument_1]);
        e.call(0x0040_f6e0, &args![argument_2]);
    });
}

// Translation incomplete: 0087ac60 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x08 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087ac60(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x8, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087acb0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2F and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087acb0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x2f, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ad00 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x09 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087ad00(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u32,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x9, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ad50 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0A and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087ad50(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xa, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ada0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0B and dispatches the task.
/// A null argument 2 suppresses all calls.
pub fn fn_0087ada0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: Ptr,
) {
    if argument_2.is_null() {
        return;
    }
    with_task(e, this, 0xb, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        fn_0087adf0(e, argument_2);
    });
}

// Translation incomplete: 0087ae10 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x11 and dispatches the task.
/// A null argument 2 suppresses all calls.
pub fn fn_0087ae10(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: Ptr,
) {
    if argument_2.is_null() {
        return;
    }
    with_task(e, this, 0x11, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        fn_0087adf0(e, argument_2);
    });
}

// Translation incomplete: 0087af50 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0D and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087af50(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: f32,
    argument_3: u8,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xd, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_f32(task.addr() + 8, x87_task_float(argument_2));
        e.mem.set_u8(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087afa0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x30 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087afa0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr, argument_2: u8) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x30, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u8(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087aff0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueActorKill` (Xbox PDB): packs tag 0x0E and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_actor_kill(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: Ptr,
    argument_3: u32,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xe, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        e.mem.set_u32(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087b040 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0F and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087b040(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u8,
    argument_3: u8,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xf, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u8(task.addr() + 8, argument_2);
        e.mem.set_u8(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087b090 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueActorKnockExplosion` (Xbox PDB): packs tag 0x10 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_actor_knock_explosion(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: f32,
    argument_3: f32,
    argument_4: f32,
    argument_5: f32,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x10, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_f32(task.addr() + 8, x87_task_float(argument_2));
        e.mem.set_f32(task.addr() + 12, x87_task_float(argument_3));
        e.mem.set_f32(task.addr() + 16, x87_task_float(argument_4));
        e.mem.set_f32(task.addr() + 20, x87_task_float(argument_5));
    });
}

// Translation incomplete: 0087b0f0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x13 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087b0f0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    e.call(0x0040_f6e0, &args![argument_1]);
    with_task(e, this, 0x13, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
    });
}

// Translation incomplete: 0087b140 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueHavokRemove` (Xbox PDB): packs tag 0x14 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_havok_remove(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
) {
    if argument_1.is_null() {
        return;
    }
    e.call(0x0040_f6e0, &args![argument_1]);
    with_task(e, this, 0x14, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
    });
}

// Translation incomplete: 0087b190 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2C and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087b190(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x2c, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
    });
}

// Translation incomplete: 0087b1e0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueHavokMotionChange` (Xbox PDB): packs tag 0x22 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_havok_motion_change(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u32,
) {
    if argument_1.is_null() {
        return;
    }
    e.call(0x0040_f6e0, &args![argument_1]);
    with_task(e, this, 0x22, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087b230 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2D and dispatches the task.
pub fn fn_0087b230(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x2d, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b270 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x29 and dispatches the task.
pub fn fn_0087b270(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x29, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b2b0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2A and dispatches the task.
pub fn fn_0087b2b0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    with_task(e, this, 0x2a, |e, task| {
        let value = e.call(0x0084_e3a0, &args![argument_1]).u32();
        e.mem.set_u32(task.addr() + 4, value);
    });
}

// Translation incomplete: 0087b300 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueHavokAdjustForceAddCount` (Xbox PDB): packs tag 0x31 and dispatches the task.
pub fn task_queue_interface_queue_havok_adjust_force_add_count(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u8,
    argument_3: u8,
    argument_4: u8,
) {
    with_task(e, this, 0x31, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u8(task.addr() + 8, argument_2);
        e.mem.set_u8(task.addr() + 12, argument_3);
        e.mem.set_u8(task.addr() + 16, argument_4);
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087b360 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueAttachWeapon` (Xbox PDB): packs tag 0x15 and dispatches the task.
pub fn task_queue_interface_queue_attach_weapon(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
) {
    with_task(e, this, 0x15, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087b3b0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x16 and dispatches the task.
pub fn fn_0087b3b0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
) {
    with_task(e, this, 0x16, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087b400 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueSet3DNull` (Xbox PDB): packs tag 0x17 and dispatches the task.
pub fn task_queue_interface_queue_set_3d_null(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
) {
    with_task(e, this, 0x17, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b440 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueDismemberLimb` (Xbox PDB): packs tag 0x18 and dispatches the task.
pub fn task_queue_interface_queue_dismember_limb(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
    argument_3: u32,
    argument_4: u8,
    argument_5: u8,
) {
    with_task(e, this, 0x18, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.mem.set_u32(task.addr() + 12, argument_3);
        e.mem.set_u8(task.addr() + 16, argument_4);
        e.mem.set_u8(task.addr() + 20, argument_5);
    });
}

// Translation incomplete: 0087b4a0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x19 and dispatches the task.
pub fn fn_0087b4a0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
    argument_3: f32,
) {
    with_task(e, this, 0x19, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.mem.set_f32(task.addr() + 12, x87_task_float(argument_3));
    });
}

// Translation incomplete: 0087b4f0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x1A and dispatches the task.
pub fn fn_0087b4f0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
    argument_3: f32,
) {
    with_task(e, this, 0x1a, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.mem.set_f32(task.addr() + 12, x87_task_float(argument_3));
    });
}

// Translation incomplete: 0087b540 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x1B and dispatches the task.
pub fn fn_0087b540(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x1b, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b580 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x1C and dispatches the task.
pub fn fn_0087b580(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x1c, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b5c0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueCharacterReset3D` (Xbox PDB): packs tag 0x1D and dispatches the task.
pub fn task_queue_interface_queue_character_reset_3d(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
) {
    with_task(e, this, 0x1d, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b600 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueSetOpenState` (Xbox PDB): packs tag 0x1E and dispatches the task.
pub fn task_queue_interface_queue_set_open_state(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u8,
    argument_3: u8,
) {
    with_task(e, this, 0x1e, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u8(task.addr() + 8, argument_2);
        e.mem.set_u8(task.addr() + 12, argument_3);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0087a670,
            fn_0087a670(Ptr<TaskQueueInterface>, Ptr, Ptr) -> Ptr<TaskQueueInterface>
        ),
        entry!(0x0087a890, fn_0087a890(Ptr, u32) -> u8),
        entry!(
            0x0087aa40,
            task_queue_interface_queue_weapon_fire(Ptr<TaskQueueInterface>, u32, u32)
        ),
        entry!(0x0087aa90, fn_0087aa90(Ptr<TaskQueueInterface>)),
        entry!(0x0087aad0, fn_0087aad0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087ab50, fn_0087ab50(Ptr<TaskQueueInterface>, u32, u32)),
        entry!(0x0087abb0, fn_0087abb0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087ac00,
            task_queue_interface_queue_node_attach(Ptr<TaskQueueInterface>, Ptr, Ptr, u8, u8)
        ),
        entry!(0x0087ac60, fn_0087ac60(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087acb0, fn_0087acb0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087ad00, fn_0087ad00(Ptr<TaskQueueInterface>, Ptr, u32)),
        entry!(0x0087ad50, fn_0087ad50(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087ada0, fn_0087ada0(Ptr<TaskQueueInterface>, u32, Ptr)),
        entry!(0x0087adf0, fn_0087adf0(Ptr)),
        entry!(0x0087ae10, fn_0087ae10(Ptr<TaskQueueInterface>, u32, Ptr)),
        entry!(0x0087ae60, fn_0087ae60(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087aec0, fn_0087aec0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087af50,
            fn_0087af50(Ptr<TaskQueueInterface>, Ptr, f32, u8)
        ),
        entry!(0x0087afa0, fn_0087afa0(Ptr<TaskQueueInterface>, Ptr, u8)),
        entry!(
            0x0087aff0,
            task_queue_interface_queue_actor_kill(Ptr<TaskQueueInterface>, Ptr, Ptr, u32)
        ),
        entry!(
            0x0087b040,
            fn_0087b040(Ptr<TaskQueueInterface>, Ptr, u8, u8)
        ),
        entry!(
            0x0087b090,
            task_queue_interface_queue_actor_knock_explosion(
                Ptr<TaskQueueInterface>,
                Ptr,
                f32,
                f32,
                f32,
                f32,
            )
        ),
        entry!(0x0087b0f0, fn_0087b0f0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087b140,
            task_queue_interface_queue_havok_remove(Ptr<TaskQueueInterface>, Ptr)
        ),
        entry!(0x0087b190, fn_0087b190(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087b1e0,
            task_queue_interface_queue_havok_motion_change(Ptr<TaskQueueInterface>, Ptr, u32)
        ),
        entry!(0x0087b230, fn_0087b230(Ptr<TaskQueueInterface>, u32)),
        entry!(0x0087b270, fn_0087b270(Ptr<TaskQueueInterface>, u32)),
        entry!(0x0087b2b0, fn_0087b2b0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087b300,
            task_queue_interface_queue_havok_adjust_force_add_count(
                Ptr<TaskQueueInterface>,
                Ptr,
                u8,
                u8,
                u8,
            )
        ),
        entry!(
            0x0087b360,
            task_queue_interface_queue_attach_weapon(Ptr<TaskQueueInterface>, u32, u32)
        ),
        entry!(0x0087b3b0, fn_0087b3b0(Ptr<TaskQueueInterface>, u32, u32)),
        entry!(
            0x0087b400,
            task_queue_interface_queue_set_3d_null(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(
            0x0087b440,
            task_queue_interface_queue_dismember_limb(
                Ptr<TaskQueueInterface>,
                u32,
                u32,
                u32,
                u8,
                u8,
            )
        ),
        entry!(
            0x0087b4a0,
            fn_0087b4a0(Ptr<TaskQueueInterface>, u32, u32, f32)
        ),
        entry!(
            0x0087b4f0,
            fn_0087b4f0(Ptr<TaskQueueInterface>, u32, u32, f32)
        ),
        entry!(0x0087b540, fn_0087b540(Ptr<TaskQueueInterface>, u32)),
        entry!(0x0087b580, fn_0087b580(Ptr<TaskQueueInterface>, u32)),
        entry!(
            0x0087b5c0,
            task_queue_interface_queue_character_reset_3d(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(
            0x0087b600,
            task_queue_interface_queue_set_open_state(Ptr<TaskQueueInterface>, u32, u8, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the task's initialized slots, exact argument order, retention
    /// before dispatch, byte truncation, float bits, and null-call suppression.
    fn check_record(
        address: u32,
        arguments: &[u32],
        tag: u32,
        payload: &[u32],
        before: &[(u32, Vec<u32>)],
        null_gate: Option<usize>,
        byte_slots: &[usize],
    ) {
        let mut e = Engine::new();
        let this: Ptr<TaskQueueInterface> = e.new_object();
        for callee in [0x0040_f6e0, 0x0040_b460] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(0x0084_e3a0, |_, a| {
            assert_eq!(a, &[0x1234_5678]);
            0x8765_4321u32.into_ret()
        });
        let expected = payload.to_vec();
        let byte_slots = byte_slots.to_vec();
        e.register_double(0x0087_d160, move |e, a| {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], this.addr());
            assert_eq!(e.mem.u32(a[1]), tag);
            for (index, value) in expected.iter().enumerate() {
                let slot = a[1] + 4 * (index as u32 + 1);
                if byte_slots.contains(&index) {
                    assert_eq!(e.mem.u8(slot), *value as u8);
                } else {
                    assert_eq!(e.mem.u32(slot), *value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![this.addr()];
        words.extend_from_slice(arguments);
        e.call(address, &words);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), before.len() + 2);
        assert_eq!(&log[1..log.len() - 1], before);
        assert_eq!(log.last().unwrap().0, 0x0087_d160);
        if let Some(index) = null_gate {
            words[index + 1] = 0;
            e.call_log = Some(vec![]);
            e.call(address, &words);
            assert_eq!(e.call_log.take().unwrap(), vec![(address, words)]);
        }
    }

    #[test]
    fn record_0087aa40() {
        check_record(
            0x0087aa40,
            &[0x12345678, 0x90abcdef],
            0x20,
            &[0x12345678, 0x90abcdef],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087aa90() {
        check_record(0x0087aa90, &[], 0x4, &[], &[], None, &[]);
    }

    #[test]
    fn record_0087abb0() {
        check_record(
            0x0087abb0,
            &[0x12345678],
            0x6,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087ac00() {
        check_record(
            0x0087ac00,
            &[0x12345678, 0x90abcdef, 0xdeadbe83, 0xdeadbe84],
            0x7,
            &[0x12345678, 0x90abcdef, 0x00000083, 0x00000084],
            &[
                (0x0040f6e0, vec![0x12345678]),
                (0x0040f6e0, vec![0x90abcdef]),
            ],
            None,
            &[2, 3],
        );
    }

    #[test]
    fn record_0087ac60() {
        check_record(
            0x0087ac60,
            &[0x12345678],
            0x8,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087acb0() {
        check_record(
            0x0087acb0,
            &[0x12345678],
            0x2f,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087ad00() {
        check_record(
            0x0087ad00,
            &[0x12345678, 0x90abcdef],
            0x9,
            &[0x12345678, 0x90abcdef],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087ad50() {
        check_record(
            0x0087ad50,
            &[0x12345678],
            0xa,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087ada0() {
        check_record(
            0x0087ada0,
            &[0x12345678, 0x90abcdef],
            0xb,
            &[0x12345678, 0x90abcdef],
            &[(0x0040b460, vec![0x90abce4f])],
            Some(1),
            &[],
        );
    }

    #[test]
    fn record_0087ae10() {
        check_record(
            0x0087ae10,
            &[0x12345678, 0x90abcdef],
            0x11,
            &[0x12345678, 0x90abcdef],
            &[(0x0040b460, vec![0x90abce4f])],
            Some(1),
            &[],
        );
    }

    #[test]
    fn record_0087af50() {
        check_record(
            0x0087af50,
            &[0x12345678, 0xc0600000, 0xdeadbe83],
            0xd,
            &[0x12345678, 0xc0600000, 0x00000083],
            &[],
            Some(0),
            &[2],
        );
    }

    #[test]
    fn record_0087afa0() {
        check_record(
            0x0087afa0,
            &[0x12345678, 0xdeadbe82],
            0x30,
            &[0x12345678, 0x00000082],
            &[],
            Some(0),
            &[1],
        );
    }

    #[test]
    fn record_0087aff0() {
        check_record(
            0x0087aff0,
            &[0x12345678, 0x90abcdef, 0x10203042],
            0xe,
            &[0x12345678, 0x90abcdef, 0x10203042],
            &[],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b040() {
        check_record(
            0x0087b040,
            &[0x12345678, 0xdeadbe82, 0xdeadbe83],
            0xf,
            &[0x12345678, 0x00000082, 0x00000083],
            &[],
            Some(0),
            &[1, 2],
        );
    }

    #[test]
    fn record_0087b090() {
        check_record(
            0x0087b090,
            &[0x12345678, 0xc0600000, 0x80000000, 0x3fa00000, 0x40980000],
            0x10,
            &[0x12345678, 0xc0600000, 0x80000000, 0x3fa00000, 0x40980000],
            &[],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b0f0() {
        check_record(
            0x0087b0f0,
            &[0x12345678],
            0x13,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b140() {
        check_record(
            0x0087b140,
            &[0x12345678],
            0x14,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b190() {
        check_record(
            0x0087b190,
            &[0x12345678],
            0x2c,
            &[0x12345678],
            &[],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b1e0() {
        check_record(
            0x0087b1e0,
            &[0x12345678, 0x90abcdef],
            0x22,
            &[0x12345678, 0x90abcdef],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b230() {
        check_record(
            0x0087b230,
            &[0x12345678],
            0x2d,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b270() {
        check_record(
            0x0087b270,
            &[0x12345678],
            0x29,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b2b0() {
        check_record(
            0x0087b2b0,
            &[0x12345678],
            0x2a,
            &[0x87654321],
            &[(0x0084e3a0, vec![0x12345678])],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b300() {
        check_record(
            0x0087b300,
            &[0x12345678, 0xdeadbe82, 0xdeadbe83, 0xdeadbe84],
            0x31,
            &[0x12345678, 0x00000082, 0x00000083, 0x00000084],
            &[(0x0040f6e0, vec![0x12345678])],
            None,
            &[1, 2, 3],
        );
    }

    #[test]
    fn record_0087b360() {
        check_record(
            0x0087b360,
            &[0x12345678, 0x90abcdef],
            0x15,
            &[0x12345678, 0x90abcdef],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b3b0() {
        check_record(
            0x0087b3b0,
            &[0x12345678, 0x90abcdef],
            0x16,
            &[0x12345678, 0x90abcdef],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b400() {
        check_record(
            0x0087b400,
            &[0x12345678],
            0x17,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b440() {
        check_record(
            0x0087b440,
            &[0x12345678, 0x90abcdef, 0x10203042, 0xdeadbe84, 0xdeadbe85],
            0x18,
            &[0x12345678, 0x90abcdef, 0x10203042, 0x00000084, 0x00000085],
            &[],
            None,
            &[3, 4],
        );
    }

    #[test]
    fn record_0087b4a0() {
        check_record(
            0x0087b4a0,
            &[0x12345678, 0x90abcdef, 0x80000000],
            0x19,
            &[0x12345678, 0x90abcdef, 0x80000000],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b4f0() {
        check_record(
            0x0087b4f0,
            &[0x12345678, 0x90abcdef, 0x80000000],
            0x1a,
            &[0x12345678, 0x90abcdef, 0x80000000],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b540() {
        check_record(
            0x0087b540,
            &[0x12345678],
            0x1b,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b580() {
        check_record(
            0x0087b580,
            &[0x12345678],
            0x1c,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b5c0() {
        check_record(
            0x0087b5c0,
            &[0x12345678],
            0x1d,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b600() {
        check_record(
            0x0087b600,
            &[0x12345678, 0xdeadbe82, 0xdeadbe83],
            0x1e,
            &[0x12345678, 0x00000082, 0x00000083],
            &[],
            None,
            &[1, 2],
        );
    }

    #[test]
    fn constructor_sets_all_fields_returns_this_and_preserves_guards() {
        // Lead native 0087a670 vectors: generated guarded receivers, each
        // fill combined with both queue pairs. No executable data is stored.
        for fill in [0xa5u8, 0xcc, 0xff] {
            for (first, second) in [(0u32, u32::MAX), (0x1234_5678, 0x90ab_cdef)] {
                let mut e = Engine::new();
                let buffer = e.mem.alloc(0x18);
                e.mem.write(buffer, &[fill; 0x18]);
                let this: Ptr<TaskQueueInterface> = Ptr::new(buffer + 4);
                let result = e.call(0x0087_a670, &args![this, first, second]).u32();
                assert_eq!(result, this.addr());
                assert_eq!(e.get(this, TaskQueueInterface::pQueue).addr(), first);
                assert_eq!(
                    e.get(this, TaskQueueInterface::pSecondaryQueue).addr(),
                    second
                );
                assert_eq!(e.get(this, TaskQueueInterface::pActiveQueue).addr(), first);
                assert_eq!(e.get(this, TaskQueueInterface::iProcessingThread), 0);
                assert_eq!(e.mem.u32(buffer), u32::from_le_bytes([fill; 4]));
                assert_eq!(e.mem.u32(buffer + 0x14), u32::from_le_bytes([fill; 4]));
            }
        }
    }

    fn virtual_object(e: &mut Engine, slot: u32, callback: u32) -> Ptr {
        let object = e.mem.alloc(4);
        let table = e.mem.alloc(slot + 4);
        e.mem.set_u32(object, table);
        e.mem.set_u32(table + slot, callback);
        Ptr::new(object)
    }

    #[test]
    fn virtual_forwarder_reloads_current_vtable_and_keeps_argument() {
        let mut e = Engine::new();
        let object = virtual_object(&mut e, 4, 0x0f00_0010);
        let second_table = e.mem.alloc(8);
        e.mem.set_u32(second_table + 4, 0x0f00_0020);
        e.register_double(0x0f00_0010, move |e, a| {
            assert_eq!(a, &[object.addr(), 0xfedc_ba98]);
            e.mem.set_u32(object.addr(), second_table);
            Ret {
                eax: 0xfeed_be7f,
                ..Ret::default()
            }
        });
        e.register_double(0x0f00_0020, move |_, a| {
            assert_eq!(a, &[object.addr(), 0x7654_3210]);
            Ret {
                eax: 0xfeed_be00,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0087_a890, &args![object, 0xfedc_ba98u32]).u8(),
            0x7f
        );
        assert_eq!(e.call(0x0087_a890, &args![object, 0x7654_3210u32]).u8(), 0);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|c| c.0)
            .collect();
        assert_eq!(calls, [0x0087_a890, 0x0f00_0010, 0x0087_a890, 0x0f00_0020]);
    }

    #[test]
    fn guarded_reference_insertion_preserves_normal_cleanup_and_disabled_path() {
        for enabled_byte in [0u8, 0x80] {
            let mut e = Engine::new();
            let enabled = e.mem.alloc(1);
            e.mem.set_u8(enabled, enabled_byte);
            e.register_double(0x0040_8d60, move |_, a| {
                assert_eq!(a, &[0x011d_f1b8]);
                enabled.into_ret()
            });
            e.register(0x0063_3c90, |e, a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[1], 0x1234_5678);
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register(0x006c_61b0, |e, a| {
                assert_eq!(a[0], 0x011d_f1c4);
                assert_eq!(e.mem.u32(a[1]), 0x1234_5678);
                e.mem.set_u32(a[1], 0x8765_4321);
                Ret::default()
            });
            e.register(0x0045_cec0, |e, a| {
                assert_eq!(a.len(), 1);
                assert_eq!(e.mem.u32(a[0]), 0x8765_4321);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_aad0, &args![0u32, 0x1234_5678u32]);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.iter().map(|c| c.0).collect();
            if enabled_byte == 0 {
                assert_eq!(calls, [0x0087_aad0, 0x0040_8d60]);
            } else {
                assert_eq!(
                    calls,
                    [
                        0x0087_aad0,
                        0x0040_8d60,
                        0x0063_3c90,
                        0x006c_61b0,
                        0x0045_cec0
                    ]
                );
                assert_eq!(log[2].1[0], log[3].1[1]);
                assert_eq!(log[2].1[0], log[4].1[0]);
            }
        }
    }

    #[test]
    fn pair_insertion_reloads_indexed_element_and_respects_disabled_container() {
        for enabled_byte in [0u8, 0xff] {
            let mut e = Engine::new();
            let enabled = e.mem.alloc(1);
            let first = e.mem.alloc(8);
            let second = e.mem.alloc(8);
            e.mem.set_u8(enabled, enabled_byte);
            e.mem.set_u32(first + 4, 0xaabb_ccdd);
            e.mem.set_u32(second, 0x1122_3344);
            e.register_double(0x0040_8d60, move |_, a| {
                assert_eq!(a, &[0x011d_f214]);
                enabled.into_ret()
            });
            e.register(0x0084_5460, |_, a| {
                assert_eq!(a, &[0x011d_f244]);
                0xffff_fffeu32.into_ret()
            });
            let mut lookups = 0;
            e.register_double(0x0069_50d0, move |e, a| {
                assert_eq!(a, &[0x011d_f244, 0xffff_fffe]);
                lookups += 1;
                if lookups == 1 {
                    first.into_ret()
                } else {
                    assert_eq!(lookups, 2);
                    assert_eq!(e.mem.u32(first), 0x5566_7788);
                    second.into_ret()
                }
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_ab50, &args![0u32, 0x5566_7788u32, 0x99aa_bbccu32]);
            let calls: Vec<_> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            if enabled_byte == 0 {
                assert_eq!(calls, [0x0087_ab50, 0x0040_8d60]);
                assert_eq!(e.mem.u32(first), 0);
                assert_eq!(e.mem.u32(second + 4), 0);
            } else {
                assert_eq!(
                    calls,
                    [
                        0x0087_ab50,
                        0x0040_8d60,
                        0x0084_5460,
                        0x0069_50d0,
                        0x0069_50d0
                    ]
                );
                assert_eq!(e.mem.u32(first), 0x5566_7788);
                assert_eq!(e.mem.u32(second + 4), 0x99aa_bbcc);
            }
            assert_eq!(e.mem.u32(first + 4), 0xaabb_ccdd);
            assert_eq!(e.mem.u32(second), 0x1122_3344);
        }
    }

    #[test]
    fn embedded_member_helper_wraps_32_bit_address() {
        for object in [0x1234_5678u32, 0xffff_ffc0] {
            let mut e = Engine::new();
            e.register_double(0x0040_b460, move |_, a| {
                assert_eq!(a, &[object.wrapping_add(0x60)]);
                Ret::default()
            });
            e.call(0x0087_adf0, &args![object]);
        }
    }

    #[test]
    fn object_gate_checks_null_and_only_low_return_byte() {
        for result in [0u32, 0x100, 0xffff_ff00, 0x101, 0x80] {
            let mut e = Engine::new();
            let this: Ptr<TaskQueueInterface> = e.new_object();
            let object = virtual_object(&mut e, 0x220, 0x0f00_0030);
            e.register_double(0x0f00_0030, move |_, a| {
                assert_eq!(a, &[object.addr()]);
                result.into_ret()
            });
            e.register_double(0x0087_d160, move |e, a| {
                assert_eq!(a[0], this.addr());
                assert_eq!(e.mem.u32(a[1]), 0x0c);
                assert_eq!(e.mem.u32(a[1] + 4), object.addr());
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_ae60, &args![this, 0u32]);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
            e.call_log = Some(vec![]);
            e.call(0x0087_ae60, &args![this, object]);
            let log = e.call_log.take().unwrap();
            assert_eq!(log.len(), if result as u8 == 0 { 2 } else { 3 });
            assert_eq!(log[1].0, 0x0f00_0030);
            if result as u8 != 0 {
                assert_eq!(log[2].0, 0x0087_d160);
            }
        }
    }

    #[test]
    fn target_actor_gate_reloads_both_getters_and_uses_full_virtual_result() {
        // None, actor without process, and nonzero process (including a
        // zero low byte) exercise every branch at 0087aef1 and 0087af17.
        for process in [None, Some(0u32), Some(0x100), Some(u32::MAX)] {
            let mut e = Engine::new();
            let this: Ptr<TaskQueueInterface> = e.new_object();
            let object = e.mem.alloc(4);
            let actor_first = e.mem.alloc(4);
            let actor_second = virtual_object(&mut e, 0x1d0, 0x0f00_0040);
            let mut target_lookups = 0;
            e.register_double(0x0059_bb30, move |_, a| {
                assert_eq!(a, &[object]);
                target_lookups += 1;
                (0x1000u32 + target_lookups).into_ret()
            });
            let mut actor_lookups = 0;
            e.register_double(0x0082_2b40, move |_, a| {
                actor_lookups += 1;
                assert_eq!(a, &[0x1000u32 + actor_lookups]);
                if process.is_none() {
                    0u32.into_ret()
                } else if actor_lookups == 1 {
                    actor_first.into_ret()
                } else {
                    assert_eq!(actor_lookups, 2);
                    actor_second.into_ret()
                }
            });
            e.register_double(0x0f00_0040, move |_, a| {
                assert_eq!(a, &[actor_second.addr()]);
                process.unwrap().into_ret()
            });
            e.register_double(0x0087_d160, move |e, a| {
                assert_eq!(a[0], this.addr());
                assert_eq!(e.mem.u32(a[1]), 0x12);
                assert_eq!(e.mem.u32(a[1] + 4), object);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_aec0, &args![this, 0u32]);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
            e.call_log = Some(vec![]);
            e.call(0x0087_aec0, &args![this, object]);
            let calls: Vec<_> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            let mut expected = vec![0x0087_aec0, 0x0059_bb30, 0x0082_2b40];
            if process.is_some() {
                expected.extend([0x0059_bb30, 0x0082_2b40, 0x0f00_0040]);
            }
            if process != Some(0) {
                expected.push(0x0087_d160);
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn float_records_quiet_signaling_nan_but_word_fields_preserve_payloads() {
        check_record(
            0x0087_af50,
            &[0x1234_5678, 0x7f80_0001, 0x81],
            13,
            &[0x1234_5678, 0x7fc0_0001, 0x81],
            &[],
            Some(0),
            &[2],
        );
        check_record(
            0x0087_b090,
            &[
                0x1234_5678,
                0x7f80_1234,
                0xff80_2345,
                0x7fc0_3456,
                0x8000_0000,
            ],
            16,
            &[
                0x1234_5678,
                0x7fc0_1234,
                0xffc0_2345,
                0x7fc0_3456,
                0x8000_0000,
            ],
            &[],
            Some(0),
            &[],
        );
        for (address, tag) in [(0x0087_b4a0, 0x19), (0x0087_b4f0, 0x1a)] {
            check_record(
                address,
                &[0x7f80_0001, 0xff80_0002, 0xff80_0003],
                tag,
                &[0x7f80_0001, 0xff80_0002, 0xffc0_0003],
                &[],
                None,
                &[],
            );
        }
    }
    #[test]
    fn unconditional_retaining_producers_forward_null_inputs() {
        check_record(
            0x0087_abb0,
            &[0],
            6,
            &[0],
            &[(0x0040_f6e0, vec![0])],
            None,
            &[],
        );
        check_record(
            0x0087_ac00,
            &[0, 0, 0xff, 0x81],
            7,
            &[0, 0, 0xff, 0x81],
            &[(0x0040_f6e0, vec![0]), (0x0040_f6e0, vec![0])],
            None,
            &[2, 3],
        );
        check_record(
            0x0087_b300,
            &[0, 0x81, 0xff, 0x80],
            0x31,
            &[0, 0x81, 0xff, 0x80],
            &[(0x0040_f6e0, vec![0])],
            None,
            &[1, 2, 3],
        );
    }
}
