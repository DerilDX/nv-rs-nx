//! `(unplaced)/run_00e74dc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer's fields these functions use: the Direct3D device (+0x288),
/// the device-lost state (+0x6FC), the resource array object (+0x888, with
/// its size at vtable +0xA4 and element at +0xC8; +0x88C cleared with it),
/// the list of resources to recreate (+0x900 head, +0x904 tail, +0x908
/// count; nodes are next, previous, element) and the object at +0x8C8.
const DEVICE: u32 = 0x288;
const DEVICE_LOST_STATE: u32 = 0x6fc;
const RESOURCES: u32 = 0x888;
const RESOURCES_SECOND: u32 = 0x88c;
const RECREATE_LIST: u32 = 0x900;
const RECREATE_TAIL: u32 = 0x904;
const RECREATE_COUNT: u32 = 0x908;
const OBJECT_8C8: u32 = 0x8c8;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiDX92DBufferData`'s `NiRTTI` record (its name from the static
/// initializer at `00fc7320`).
const BUFFER_DATA_RTTI: u32 = 0x0126_f59c;
/// The list's add (`00a5a310`, `thiscall (list, &element)`) and node free
/// (`00e6e830`, `thiscall (list, node)`).
const LIST_ADD: u32 = 0x00a5_a310;
const LIST_FREE_NODE: u32 = 0x00e6_e830;
/// The device check (`00e74c00`), the renderer step `00e74120` and the call
/// `00e8b560` on the object at +0x8C8 (all `thiscall`).
const DEVICE_CHECK: u32 = 0x00e7_4c00;
const RENDERER_00E74120: u32 = 0x00e7_4120;
const OBJECT_00E8B560: u32 = 0x00e8_b560;
/// The node pool these lists free into: its lock (`0040fbf0`, `thiscall
/// (lock, 0)` on `011c5f80`), its lock count (`011c5f84`, the lock's owner
/// word `011c5f80` cleared when it reaches 0) and its free list (`011c5f58`).
const POOL_LOCK: u32 = 0x011c_5f80;
const POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK: u32 = 0x0040_fbf0;

fn field(this: Ptr, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

fn increment(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// Releases one reference (deleting through vtable +4 at 0).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// Unlinks the head node of the recreate list (the new head's previous
/// link, or the tail, cleared) and answers it.
fn pop_head(e: &mut Engine, this: Ptr) -> u32 {
    let node = e.mem.u32(field(this, RECREATE_LIST));
    let next = e.mem.u32(node);
    e.mem.set_u32(field(this, RECREATE_LIST), next);
    if next != 0 {
        e.mem.set_u32(next + 4, 0);
    } else {
        e.mem.set_u32(field(this, RECREATE_TAIL), 0);
    }
    node
}

// Translated from 00e74dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless the device is lost (+0x6FC), walks the
/// resource array (+0x888): each element is cast to `NiDX92DBufferData`
/// (its RTTI, vtable +0xC, or a base at +4; a null element is used as it
/// is); when its vtable +0x64 answers non-zero the device's slot +0x88 runs
/// with its words at +0x18 and +0x14; when its vtable +0x4C answers true
/// and it is not yet in the recreate list (+0x900), it is added
/// (`00a5a310`, counted). Then the array and the word after it are
/// cleared. Answers true.
pub fn fn_00e74dc0(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(field(this, DEVICE_LOST_STATE)) != 0 {
        return true;
    }
    let mut index = 0u32;
    let array = e.mem.u32(field(this, RESOURCES));
    if e.vcall(array, 0xa4, &args![]).u32() != 0 {
        loop {
            let array = e.mem.u32(field(this, RESOURCES));
            let element = e.vcall(array, 0xc8, &args![index]).u32();
            let resource = if element == 0 {
                0
            } else {
                let mut rtti = e.vcall(element, 0xc, &args![]).u32();
                let mut kind = false;
                while rtti != 0 {
                    if rtti == BUFFER_DATA_RTTI {
                        kind = true;
                        break;
                    }
                    rtti = e.mem.u32(rtti + 4);
                }
                if kind {
                    element
                } else {
                    0
                }
            };
            if e.vcall(resource, 0x64, &args![]).u32() != 0 {
                let device = e.mem.u32(field(this, DEVICE));
                let a = e.mem.u32(resource + 0x18);
                let b = e.mem.u32(resource + 0x14);
                e.vcall(device, 0x88, &args![a, 0u32, b, 0u32, 0u32]);
            }
            if e.vcall(resource, 0x4c, &args![]).bool() {
                increment(e, resource);
                let mut node = e.mem.u32(field(this, RECREATE_LIST));
                let mut found = 0u32;
                while node != 0 {
                    let next = e.mem.u32(node);
                    if resource == e.mem.u32(node + 8) {
                        found = node;
                        break;
                    }
                    node = next;
                }
                release(e, resource);
                if found == 0 {
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), resource);
                        increment(e, resource);
                        e.call(LIST_ADD, &args![field(this, RECREATE_LIST), slot]);
                        release(e, resource);
                    });
                }
            }
            index = index.wrapping_add(1);
            let array = e.mem.u32(field(this, RESOURCES));
            if index >= e.vcall(array, 0xa4, &args![]).u32() {
                break;
            }
        }
    }
    e.mem.set_u32(field(this, RESOURCES), 0);
    e.mem.set_u32(field(this, RESOURCES_SECOND), 0);
    true
}

// Translated from 00e74f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: true when the device check (`00e74c00`)
/// fails. Otherwise `00e74120`, `00e8b560` on the object at +0x8C8, and the
/// device's slot +0xA4: false when it answers a negative result; else the
/// recreate list is emptied (each node unlinked and freed, `00e6e830`, its
/// element held over the free) and the answer is true.
pub fn fn_00e74f40(e: &mut Engine, this: Ptr) -> bool {
    if !e.call(DEVICE_CHECK, &args![this]).bool() {
        return true;
    }
    e.call(RENDERER_00E74120, &args![this]);
    let object = e.mem.u32(field(this, OBJECT_8C8));
    e.call(OBJECT_00E8B560, &args![object]);
    let device = e.mem.u32(field(this, DEVICE));
    if (e.vcall(device, 0xa4, &args![]).u32() as i32) < 0 {
        return false;
    }
    while e.mem.u32(field(this, RECREATE_COUNT)) != 0 {
        let node = pop_head(e, this);
        let element = e.mem.u32(node + 8);
        if element != 0 {
            increment(e, element);
        }
        e.call(LIST_FREE_NODE, &args![field(this, RECREATE_LIST), node]);
        let count = e.mem.u32(field(this, RECREATE_COUNT)).wrapping_sub(1);
        e.mem.set_u32(field(this, RECREATE_COUNT), count);
        if element != 0 {
            release(e, element);
        }
    }
    true
}

// Translated from 00e75000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless the device is lost (+0x6FC), empties
/// the recreate list: each node is unlinked, its element held and the
/// node's reference dropped (cleared), the node returned to the pool's free
/// list under the pool lock (`0040fbf0`; the lock word cleared when its
/// count reaches 0), and the element's vtable +0x50 run before it is
/// released. Answers true.
pub fn fn_00e75000(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(field(this, DEVICE_LOST_STATE)) != 0 {
        return true;
    }
    while e.mem.u32(field(this, RECREATE_COUNT)) != 0 {
        let node = pop_head(e, this);
        let element = e.mem.u32(node + 8);
        if element != 0 {
            increment(e, element);
        }
        let held = e.mem.u32(node + 8);
        if held != 0 {
            release(e, held);
            e.mem.set_u32(node + 8, 0);
        }
        e.call(LOCK, &args![POOL_LOCK, 0u32]);
        e.mem.set_u32(node + 4, 0);
        let free = e.mem.u32(POOL_FREE_LIST);
        e.mem.set_u32(node, free);
        let count = e.mem.u32(POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(POOL_LOCK_COUNT, count);
        e.mem.set_u32(POOL_FREE_LIST, node);
        if count == 0 {
            e.mem.set_u32(POOL_LOCK, 0);
        }
        let remaining = e.mem.u32(field(this, RECREATE_COUNT)).wrapping_sub(1);
        e.mem.set_u32(field(this, RECREATE_COUNT), remaining);
        e.vcall(element, 0x50, &args![]);
        release(e, element);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e74dc0, fn_00e74dc0(Ptr) -> bool),
        entry!(0x00e74f40, fn_00e74f40(Ptr) -> bool),
        entry!(0x00e75000, fn_00e75000(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const VTABLE: u32 = 0x7000_0000;
    const DELETE: u32 = 0x7200_0004;
    const RTTI_SLOT: u32 = 0x7200_000c;
    const SLOT_4C: u32 = 0x7200_004c;
    const SLOT_50: u32 = 0x7200_0050;
    const SLOT_64: u32 = 0x7200_0064;
    const SLOT_88: u32 = 0x7200_0088;
    const SLOT_A4: u32 = 0x7200_00a4;
    const SLOT_C8: u32 = 0x7200_00c8;

    /// Objects share one vtable; each keeps its answers in its own words:
    /// +0x20 the RTTI, +0x24 the +0x64 answer, +0x28 the +0x4C answer;
    /// the array object keeps its count at +0x30 and its elements from
    /// +0x34; the device answers its +0x2C word for slot +0xA4.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0xcc / 4];
        slots[1] = DELETE;
        slots[3] = RTTI_SLOT;
        slots[0x4c / 4] = SLOT_4C;
        slots[0x50 / 4] = SLOT_50;
        slots[0x64 / 4] = SLOT_64;
        slots[0x88 / 4] = SLOT_88;
        slots[0xa4 / 4] = SLOT_A4;
        slots[0xc8 / 4] = SLOT_C8;
        e.put_vtable(VTABLE, &slots);
        e.register(DELETE, |_, _| Ret::default());
        e.register(RTTI_SLOT, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(SLOT_64, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(SLOT_4C, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(SLOT_50, |_, _| Ret::default());
        e.register(SLOT_88, |_, _| Ret::default());
        e.register(SLOT_A4, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(SLOT_C8, |e, a| e.mem.u32(a[0] + 0x34 + 4 * a[1]).into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.map(0x011c_5000, 0x1000);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0x60);
        e.mem.set_u32(o, VTABLE);
        e.mem.set_u32(o + 4, 1);
        o
    }

    #[test]
    fn collects_the_buffers_to_recreate() {
        let mut e = engine();
        e.register(LIST_ADD, |e, a| {
            // Push a node (next, previous, element) at the head.
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, e.mem.u32(a[0]));
            e.mem.set_u32(node + 8, e.mem.u32(a[1]));
            e.mem.set_u32(a[0], node);
            Ret::default()
        });
        let base = e.mem.alloc(8);
        e.mem.set_u32(base + 4, BUFFER_DATA_RTTI);
        let buffer = object(&mut e);
        e.mem.set_u32(buffer + 0x20, base);
        e.mem.set_u32(buffer + 0x24, 1);
        e.mem.set_u32(buffer + 0x28, 1);
        e.mem.set_u32(buffer + 0x14, 0x14);
        e.mem.set_u32(buffer + 0x18, 0x18);
        let array = object(&mut e);
        e.mem.set_u32(array + 0x2c, 2);
        // The same buffer twice: added once.
        e.mem.set_u32(array + 0x34, buffer);
        e.mem.set_u32(array + 0x38, buffer);
        let device = object(&mut e);
        let this = e.mem.alloc(0x910);
        e.mem.set_u32(this + 0x288, device);
        e.mem.set_u32(this + 0x888, array);
        e.mem.set_u32(this + 0x88c, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74dc0, &args![this]).bool());
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
        assert_eq!(
            calls(&e, SLOT_88),
            vec![vec![device, 0x18, 0, 0x14, 0, 0]; 2]
        );
        let node = e.mem.u32(this + 0x900);
        assert_eq!(e.mem.u32(node + 8), buffer);
        assert_eq!(e.mem.u32(buffer + 4), 1);
        assert_eq!((e.mem.u32(this + 0x888), e.mem.u32(this + 0x88c)), (0, 0));
        // A lost device: nothing.
        e.mem.set_u8(this + 0x6fc, 1);
        e.mem.set_u32(this + 0x888, array);
        assert!(e.call(0x00e74dc0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x888), array);
    }

    fn with_list(e: &mut Engine, this: u32, elements: &[u32]) {
        let mut next = 0u32;
        for element in elements.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *element);
            if next != 0 {
                e.mem.set_u32(next + 4, node);
            }
            next = node;
        }
        e.mem.set_u32(this + 0x900, next);
        e.mem.set_u32(this + 0x908, elements.len() as u32);
    }

    #[test]
    fn device_reset_empties_the_list() {
        let mut e = engine();
        e.register(RENDERER_00E74120, |_, _| Ret::default());
        e.register(OBJECT_00E8B560, |_, _| Ret::default());
        e.register(LIST_FREE_NODE, |_, _| Ret::default());
        e.map(0x7300_0000, 4);
        e.register(DEVICE_CHECK, |e, _| e.mem.u32(0x7300_0000).into_ret());
        let device = object(&mut e);
        let this = e.mem.alloc(0x910);
        e.mem.set_u32(this + 0x288, device);
        let a = object(&mut e);
        with_list(&mut e, this, &[a, 0]);
        // The check fails: true, nothing else.
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74f40, &args![this]).bool());
        assert!(calls(&e, RENDERER_00E74120).is_empty());
        // A negative device answer: false.
        e.mem.set_u32(0x7300_0000, 1);
        e.mem.set_u32(device + 0x2c, 0x8000_0000);
        assert!(!e.call(0x00e74f40, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x908), 2);
        e.mem.set_u32(device + 0x2c, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74f40, &args![this]).bool());
        assert_eq!(calls(&e, LIST_FREE_NODE).len(), 2);
        assert_eq!(e.mem.u32(this + 0x908), 0);
        assert_eq!(e.mem.u32(a + 4), 1);
    }

    #[test]
    fn recreate_runs_each_element_and_frees_the_nodes() {
        let mut e = engine();
        e.register(LOCK, |e, _| {
            let n = e.mem.u32(POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(POOL_LOCK_COUNT, n);
            e.mem.set_u32(POOL_LOCK, 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x910);
        let a = object(&mut e);
        let b = object(&mut e);
        e.mem.set_u32(a + 4, 2);
        with_list(&mut e, this, &[a, b]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e75000, &args![this]).bool());
        assert_eq!(calls(&e, SLOT_50), vec![vec![a], vec![b]]);
        // `a` loses the list's reference, `b` its only one (deleted).
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(calls(&e, DELETE), vec![vec![b]]);
        assert_eq!(e.mem.u32(this + 0x908), 0);
        assert_eq!(e.mem.u32(this + 0x900), 0);
        assert_eq!(e.mem.u32(POOL_LOCK), 0);
        assert_ne!(e.mem.u32(POOL_FREE_LIST), 0);
    }
}
