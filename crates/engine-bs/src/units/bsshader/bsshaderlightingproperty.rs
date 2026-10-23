//! `BSShader/bsshaderlightingproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The property keeps its lights in an `NiTPointerList` of `ShadowSceneLight *`
//! (`lLightList`, Xbox PDB, at `+0x60` on PC): head, tail, count; each node
//! is next (`+0`), previous (`+4`), the light (`+8`). A light counts as
//! active when its half-word at `+0x110` is not 0xFF and bit 0 of its
//! `NiLight`'s (`spLight`, `+0xF8`) byte `+0x30` is clear; a shadow light has
//! `bCastShadow` (`+0xEC`) set.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ShadowSceneLight::GetLuminanceToBound` (`00b9dbe0`, `thiscall` on the
/// light, one word: the bound; result in ST0).
const LIGHT_LUMINANCE_TO_BOUND: u32 = 0x00b9_dbe0;
/// `ShadowSceneLight::RemoveTriGeom` (`00ba01a0`, `thiscall`, two words).
const LIGHT_REMOVE_TRI_GEOM: u32 = 0x00ba_01a0;
/// `00b702e0` (unit `run_00b70260`, `thiscall` on the list, two words: the
/// node, the node it goes before): moves a node.
const LIST_MOVE_BEFORE: u32 = 0x00b7_02e0;
/// `00b58570` (unnamed, `thiscall` on the list, one word: a pointer to the
/// light): adds the light.
const LIST_ADD: u32 = 0x00b5_8570;
/// `00e7db70` (unnamed, `thiscall` on the list, one word: a pointer to the
/// light): removes the light.
const LIST_REMOVE: u32 = 0x00e7_db70;
/// `00e7d5b0` (unnamed, `thiscall` on the list, no words): removes the tail.
const LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;
/// `00e7d7f0` (unnamed, `thiscall` on the list, one word: a pointer to the
/// node pointer): removes that node.
const LIST_REMOVE_NODE: u32 = 0x00e7_d7f0;
/// `NiTPointerListBase<..>::AddHead` (engine map; `0043a010`, `thiscall` on
/// the list's count word, no words): hands out a new node.
const LIST_NEW_NODE: u32 = 0x0043_a010;
/// `NiTPointerListBase<..>::RemoveAll` (`00b71300`, `thiscall` on the list).
const LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// `BSShaderProperty::BSShaderProperty` (`00ba8e50`, `thiscall`, no words).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
/// The spin lock of the list nodes' pool (`011c5f80`: owner, then the count
/// at `011c5f84`), its enter function (`0040fbf0`, `thiscall`, one word),
/// and the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const SPIN_LOCK_ENTER: u32 = 0x0040_fbf0;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
/// Import slot of `DebugBreak`.
const DEBUG_BREAK: u32 = 0x00fd_f0c8;
/// The marker light the fence node holds (`011fa310`).
const FENCE_MARKER: u32 = 0x011f_a310;
/// The property vtable the constructor stores.
const LIGHTING_PROPERTY_VTABLE: u32 = 0x010a_e7b0;

layout! {
    /// `BSShaderLightingProperty` (Xbox PDB), the fields used here at their
    /// PC offsets (8 below the Xbox ones from the light list on).
    pub struct BSShaderLightingProperty: 0x7C {
        /// `iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: u32,
        /// `iLastRenderPassState` (Xbox PDB).
        0x38 iLastRenderPassState: u32,
        /// `lLightList` (Xbox PDB): head, tail and count.
        0x60 lLightList: u32,
        0x64 lLightList_tail: u32,
        0x68 lLightList_count: u32,
        /// `fForcedDarkness` (Xbox PDB), as bits.
        0x6C fForcedDarkness: u32,
        /// `uiReferenceID` (Xbox PDB).
        0x70 uiReferenceID: u32,
        /// `bLightListChanged` (Xbox PDB).
        0x74 bLightListChanged: u8,
        /// PC only: the fence node of the light list (`FenceAllLights`).
        0x78 fence: u32,
    }
}

type Lit = BSShaderLightingProperty;

/// Whether `light` counts as active (see the unit comment).
fn is_active(e: &Engine, light: u32) -> bool {
    e.mem.u16(light + 0x110) != 0xff && e.mem.u8(e.mem.u32(light + 0xf8) + 0x30) & 1 == 0
}

/// Active and not casting shadows.
fn is_active_non_shadow(e: &Engine, light: u32) -> bool {
    is_active(e, light) && e.mem.u8(light + 0xec) != 1
}

// Translated from 00b70340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: the light, an unused
/// word, the result slot; `bool` in `AL`): clears the slot; with lights
/// (count `+0x68` at least 1) walks the list from the head (which it does
/// not check for null) and answers true with the node holding `light` in
/// the slot, false at the first null light or the end.
pub fn fn_00b70340(e: &mut Engine, this: Ptr<Lit>, light: u32, _unused_2: u32, node: Ptr) -> bool {
    e.mem.set_u32(node.addr(), 0);
    if e.get(this, Lit::lLightList_count) < 1 {
        return false;
    }
    let mut current = e.get(this, Lit::lLightList);
    let mut element = e.mem.u32(current + 8);
    let mut next = e.mem.u32(current);
    while element != 0 {
        if element == light {
            e.mem.set_u32(node.addr(), current);
            return true;
        }
        if next == 0 {
            return false;
        }
        element = e.mem.u32(next + 8);
        current = next;
        next = e.mem.u32(next);
    }
    false
}

/// The list move `00b70390` inlines: `node` goes just before `before`.
fn move_before(e: &mut Engine, this: Ptr<Lit>, node: u32, before: u32) {
    let list = this.addr() + 0x60;
    if e.mem.u32(list) == node {
        let next = e.mem.u32(node);
        e.mem.set_u32(list, next);
    }
    if e.mem.u32(list) == before {
        e.mem.set_u32(list, node);
    }
    if e.mem.u32(list + 4) == node {
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(list + 4, previous);
    }
    let next = e.mem.u32(node);
    if next != 0 {
        let previous = e.mem.u32(node + 4);
        e.mem.set_u32(next + 4, previous);
    }
    let previous = e.mem.u32(node + 4);
    if previous != 0 {
        let next = e.mem.u32(node);
        e.mem.set_u32(previous, next);
    }
    let previous = e.mem.u32(before + 4);
    e.mem.set_u32(node + 4, previous);
    e.mem.set_u32(node, before);
    if previous != 0 {
        e.mem.set_u32(previous, node);
    }
    e.mem.set_u32(before + 4, node);
}

// Translated from 00b70390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::ResortLights` (Xbox PDB; one word: the bound):
/// for each node (its next taken first), its light's luminance to the bound
/// (stored as a float) is compared with each node from the head up to it;
/// the first one with a greater luminance (an ordered `>` on the x87 result
/// against the float widened to a double) gets the node moved before it and
/// ends that scan. When anything moved, `iLastRenderPassState` is cleared.
pub fn bs_shader_lighting_property_resort_lights(e: &mut Engine, this: Ptr<Lit>, bound: u32) {
    let mut changed = false;
    let mut node = e.get(this, Lit::lLightList);
    if node == 0 {
        return;
    }
    loop {
        let mut other = e.get(this, Lit::lLightList);
        let next = e.mem.u32(node);
        let light = e.mem.u32(node + 8);
        let luminance = x87_store32(e.call(LIGHT_LUMINANCE_TO_BOUND, &args![light, bound]).f64());
        while other != node && other != 0 {
            let reference = x87_load32(luminance);
            let candidate = other;
            other = e.mem.u32(other);
            let light = e.mem.u32(candidate + 8);
            let value = e.call(LIGHT_LUMINANCE_TO_BOUND, &args![light, bound]).f64();
            if value > reference {
                if node != candidate {
                    move_before(e, this, node, candidate);
                }
                other = 0;
                changed = true;
            }
        }
        if next == 0 {
            break;
        }
        node = next;
    }
    if changed {
        e.set(this, Lit::iLastRenderPassState, 0);
    }
}

// Translated from 00b70490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetFirstLight` (Xbox PDB; one word: the
/// iterator): the iterator gets the head; with one, it moves to the next
/// node and the head's light is answered; else 0 (the null head).
pub fn bs_shader_lighting_property_get_first_light(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
) -> u32 {
    let head = e.get(this, Lit::lLightList);
    e.mem.set_u32(iterator.addr(), head);
    if head == 0 {
        return head;
    }
    let next = e.mem.u32(head);
    e.mem.set_u32(iterator.addr(), next);
    e.mem.u32(head + 8)
}

/// The shared body of `00b70590` and `00b70600`: the first light passing
/// `test`, the iterator left at the node after it.
fn first_matching(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
    test: fn(&Engine, u32) -> bool,
) -> u32 {
    let head = e.get(this, Lit::lLightList);
    e.mem.set_u32(iterator.addr(), head);
    if head == 0 {
        return 0;
    }
    let mut next = e.mem.u32(head);
    e.mem.set_u32(iterator.addr(), next);
    let mut light = e.mem.u32(head + 8);
    while next != 0 {
        if light == 0 {
            return 0;
        }
        if test(e, light) {
            break;
        }
        let node = e.mem.u32(iterator.addr());
        next = e.mem.u32(node);
        e.mem.set_u32(iterator.addr(), next);
        light = e.mem.u32(node + 8);
    }
    if light == 0 || !test(e, light) {
        return 0;
    }
    light
}

// Translated from 00b70590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetFirstActiveLight` (Xbox PDB; one word: the
/// iterator): the first active light from the head (0 at a null light or
/// when the last one is not active), the iterator at the node after it.
pub fn bs_shader_lighting_property_get_first_active_light(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
) -> u32 {
    first_matching(e, this, iterator, is_active)
}

// Translated from 00b70600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (Xbox PDB): as
/// `GetFirstActiveLight` for active lights whose `bCastShadow` is not 1.
pub fn bs_shader_lighting_property_get_first_active_non_shadow_light(
    e: &mut Engine,
    this: Ptr<Lit>,
    iterator: Ptr,
) -> u32 {
    first_matching(e, this, iterator, is_active_non_shadow)
}

/// The shared body of `00b70680` and `00b70700`: from the iterator's node,
/// the first light passing `test`, the iterator moved past it.
fn next_matching(e: &mut Engine, iterator: Ptr, test: fn(&Engine, u32) -> bool) -> u32 {
    let node = e.mem.u32(iterator.addr());
    if node == 0 {
        return 0;
    }
    let mut next = e.mem.u32(node);
    e.mem.set_u32(iterator.addr(), next);
    let mut light = e.mem.u32(node + 8);
    if light == 0 {
        return 0;
    }
    while next != 0 {
        if test(e, light) {
            break;
        }
        let node = next;
        next = e.mem.u32(node);
        e.mem.set_u32(iterator.addr(), next);
        light = e.mem.u32(node + 8);
        if light == 0 {
            return 0;
        }
    }
    if light != 0 && test(e, light) {
        light
    } else {
        0
    }
}

// Translated from 00b70680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNextActiveLight` (Xbox PDB; `thiscall`,
/// `this` unused, one word: the iterator): the next active light.
pub fn bs_shader_lighting_property_get_next_active_light(
    e: &mut Engine,
    _this: Ptr,
    iterator: Ptr,
) -> u32 {
    next_matching(e, iterator, is_active)
}

// Translated from 00b70700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNextActiveNonShadowLight` (Xbox PDB;
/// `this` unused): the next active light that does not cast shadows.
pub fn bs_shader_lighting_property_get_next_active_non_shadow_light(
    e: &mut Engine,
    _this: Ptr,
    iterator: Ptr,
) -> u32 {
    next_matching(e, iterator, is_active_non_shadow)
}

// Translated from 00b70790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNumberofActiveLights` (Xbox PDB): the
/// number of non-null active lights.
pub fn bs_shader_lighting_property_get_numberof_active_lights(
    e: &mut Engine,
    this: Ptr<Lit>,
) -> u32 {
    let mut node = e.get(this, Lit::lLightList);
    let mut count = 0u32;
    while node != 0 {
        let light = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if light != 0 && is_active(e, light) {
            count += 1;
        }
    }
    count
}

// Translated from 00b707d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::GetNumberofActiveNonShadowLights` (Xbox PDB):
/// the number of non-null active lights whose `bCastShadow` is 0.
pub fn bs_shader_lighting_property_get_numberof_active_non_shadow_lights(
    e: &mut Engine,
    this: Ptr<Lit>,
) -> u32 {
    let mut node = e.get(this, Lit::lLightList);
    let mut count = 0u32;
    while node != 0 {
        let light = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if light != 0 && is_active(e, light) && e.mem.u8(light + 0xec) == 0 {
            count += 1;
        }
    }
    count
}

// Translated from 00b713c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the light list, two words: the
/// node to insert before, a pointer to the light): a new node (`0043a010` on
/// the count word) holding the light goes before `before` (as the head when
/// it has no previous node); the count rises.
pub fn fn_00b713c0(e: &mut Engine, this: Ptr, before: u32, light: Ptr) {
    let list = this.addr();
    let count = list + 8;
    let node = e.call(LIST_NEW_NODE, &args![count]).u32();
    let value = e.mem.u32(light.addr());
    e.mem.set_u32(node + 8, value);
    e.mem.set_u32(node, before);
    let previous = e.mem.u32(before + 4);
    e.mem.set_u32(node + 4, previous);
    let previous = e.mem.u32(before + 4);
    if previous != 0 {
        e.mem.set_u32(previous, node);
    } else {
        e.mem.set_u32(list, node);
    }
    e.mem.set_u32(before + 4, node);
    let n = e.mem.u32(count);
    e.mem.set_u32(count, n.wrapping_add(1));
}

// Translated from 00b71560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the light, a word passed
/// on to `00b70340`): adds a light. With a fence node: a light already in the
/// list is moved before the fence (`00b702e0`; its node, or the tail when the
/// lookup left none), a new one is inserted before it (`00b713c0`) and the
/// list marked changed. Without one: a new light is added (`00b58570`), the
/// list is marked changed, and a shadow light clears `iLastRenderPassState`.
/// The lookup's node slot is the second argument's slot.
pub fn fn_00b71560(e: &mut Engine, this: Ptr<Lit>, light: u32, second: u32) {
    let fence = e.get(this, Lit::fence);
    let list = this.addr() + 0x60;
    e.with_stack(8, |e, slots| {
        let light_slot = slots;
        let node_slot: Ptr = Ptr::new(slots.addr() + 4);
        e.mem.set_u32(light_slot.addr(), light);
        e.mem.set_u32(node_slot.addr(), second);
        let found = fn_00b70340(e, this, light, second, node_slot);
        if fence != 0 {
            if found {
                let mut node = e.mem.u32(node_slot.addr());
                if node == 0 {
                    node = e.get(this, Lit::lLightList_tail);
                }
                e.call(LIST_MOVE_BEFORE, &args![list, node, fence]);
                return;
            }
            fn_00b713c0(e, Ptr::new(list), fence, light_slot);
            e.set(this, Lit::bLightListChanged, 1);
            return;
        }
        if !found {
            e.call(LIST_ADD, &args![list, light_slot]);
        }
        e.set(this, Lit::bLightListChanged, 1);
        if e.mem.u8(light + 0xec) != 0 {
            e.set(this, Lit::iLastRenderPassState, 0);
        }
    });
}

/// Returns a removed node to the node pool (`0040fbf0` enters the pool's
/// spin lock; the node joins the free list; the lock count drops and the
/// owner is cleared at zero), as `00b71600` does twice inline; the
/// element was cleared before the lock.
fn free_node(e: &mut Engine, node: u32) {
    e.call(SPIN_LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(node + 4, 0);
    let free = e.global::<u32>(NODE_POOL_FREE_LIST);
    e.mem.set_u32(node, free);
    let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global(NODE_POOL_LOCK_COUNT, count);
    e.set_global(NODE_POOL_FREE_LIST, node);
    if count == 0 {
        e.set_global(NODE_POOL_LOCK, 0u32);
    }
}

// Translated from 00b71600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::RemoveFencedLights` (Xbox PDB; one word: the
/// geometry passed to `RemoveTriGeom`): when the fence is not the tail, the
/// list is marked changed and every node after the fence is removed (the
/// head and middle ones unlinked and returned to the node pool inline, the
/// tail through `00e7d5b0`), each light losing the geometry
/// (`ShadowSceneLight::RemoveTriGeom(geometry, 0)`) and a shadow light
/// clearing `iLastRenderPassState`. Then the fence node itself is removed
/// (`00e7d7f0`) and the fence cleared.
pub fn bs_shader_lighting_property_remove_fenced_lights(
    e: &mut Engine,
    this: Ptr<Lit>,
    geometry: u32,
) {
    let fence = e.get(this, Lit::fence);
    let list = this.addr() + 0x60;
    if fence != e.get(this, Lit::lLightList_tail) {
        e.set(this, Lit::bLightListChanged, 1);
        let mut next = e.mem.u32(fence);
        while next != 0 {
            let head = e.mem.u32(list);
            let node = next;
            let light = e.mem.u32(node + 8);
            next = e.mem.u32(node);
            if node == head {
                let after = e.mem.u32(head);
                e.mem.set_u32(list, after);
                if after != 0 {
                    e.mem.set_u32(after + 4, 0);
                } else {
                    e.mem.set_u32(list + 4, 0);
                }
                e.mem.set_u32(head + 8, 0);
                free_node(e, head);
                let count = e.mem.u32(list + 8);
                e.mem.set_u32(list + 8, count.wrapping_sub(1));
            } else if node == e.mem.u32(list + 4) {
                e.call(LIST_REMOVE_TAIL, &args![list]);
            } else {
                let previous = e.mem.u32(node + 4);
                let after = e.mem.u32(node);
                if previous != 0 {
                    e.mem.set_u32(previous, after);
                }
                if after != 0 {
                    e.mem.set_u32(after + 4, previous);
                }
                e.mem.set_u32(node + 8, 0);
                free_node(e, node);
                let count = e.mem.u32(list + 8);
                e.mem.set_u32(list + 8, count.wrapping_sub(1));
            }
            e.call(LIGHT_REMOVE_TRI_GEOM, &args![light, geometry, 0u32]);
            if e.mem.u8(light + 0xec) != 0 {
                e.set(this, Lit::iLastRenderPassState, 0);
            }
        }
    }
    let fence_field = this.addr() + 0x78;
    e.call(LIST_REMOVE_NODE, &args![list, fence_field]);
    e.set(this, Lit::fence, 0);
}

// Translated from 00b71720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, no words): the
/// `BSShaderLightingProperty` constructor: the shader property part, the
/// vtable, an empty light list (then `RemoveAll`), reference ID 0, forced
/// darkness 1.0 (`FLD1`), not changed, no fence, shader property type 1.
pub fn fn_00b71720(e: &mut Engine, this: Ptr<Lit>) -> Ptr<Lit> {
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), LIGHTING_PROPERTY_VTABLE);
    e.set(this, Lit::lLightList_count, 0);
    e.set(this, Lit::lLightList, 0);
    e.set(this, Lit::lLightList_tail, 0);
    e.call(LIST_REMOVE_ALL, &args![this.addr() + 0x60]);
    e.set(this, Lit::uiReferenceID, 0);
    e.set(this, Lit::fForcedDarkness, 1.0f32.to_bits());
    e.set(this, Lit::bLightListChanged, 0);
    e.set(this, Lit::fence, 0);
    e.set(this, Lit::iShaderPropertyType, 1);
    this
}

// Translated from 00b717a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::RemoveLight` (Xbox PDB; one word: the light):
/// only without a fence: the light is removed (`00e7db70`, through the
/// argument slot), the list marked changed, a shadow light clears
/// `iLastRenderPassState`; then (still without a fence) every node still
/// holding the light calls `DebugBreak`.
pub fn bs_shader_lighting_property_remove_light(e: &mut Engine, this: Ptr<Lit>, light: u32) {
    if e.get(this, Lit::fence) != 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), light);
        e.call(LIST_REMOVE, &args![this.addr() + 0x60, slot]);
    });
    e.set(this, Lit::bLightListChanged, 1);
    if e.mem.u8(light + 0xec) != 0 {
        e.set(this, Lit::iLastRenderPassState, 0);
    }
    if e.get(this, Lit::fence) != 0 {
        return;
    }
    let mut node = e.get(this, Lit::lLightList);
    while node != 0 {
        let element = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if element == light {
            e.call(DEBUG_BREAK, &[]);
        }
    }
}

// Translated from 00b718b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderLightingProperty::FenceAllLights` (Xbox PDB): removes the marker
/// light `011fa310` (`00e7db70`, through a local), then puts a new node
/// (`0043a010`) holding the marker at the head (also the tail of an empty
/// list), counted, and makes it the fence.
pub fn bs_shader_lighting_property_fence_all_lights(e: &mut Engine, this: Ptr<Lit>) {
    let list = this.addr() + 0x60;
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), FENCE_MARKER);
        e.call(LIST_REMOVE, &args![list, local]);
    });
    let count = list + 8;
    let node = e.call(LIST_NEW_NODE, &args![count]).u32();
    e.mem.set_u32(node + 8, FENCE_MARKER);
    e.mem.set_u32(node + 4, 0);
    let head = e.mem.u32(list);
    e.mem.set_u32(node, head);
    let head = e.mem.u32(list);
    if head != 0 {
        e.mem.set_u32(head + 4, node);
    } else {
        e.mem.set_u32(list + 4, node);
    }
    let n = e.mem.u32(count);
    e.mem.set_u32(count, n.wrapping_add(1));
    e.mem.set_u32(list, node);
    e.set(this, Lit::fence, node);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_0340, fn_00b70340(Ptr<Lit>, u32, u32, Ptr) -> bool),
        entry!(
            0x00b7_0390,
            bs_shader_lighting_property_resort_lights(Ptr<Lit>, u32)
        ),
        entry!(
            0x00b7_0490,
            bs_shader_lighting_property_get_first_light(Ptr<Lit>, Ptr) -> u32
        ),
        entry!(
            0x00b7_0590,
            bs_shader_lighting_property_get_first_active_light(Ptr<Lit>, Ptr) -> u32
        ),
        entry!(
            0x00b7_0600,
            bs_shader_lighting_property_get_first_active_non_shadow_light(Ptr<Lit>, Ptr) -> u32
        ),
        entry!(
            0x00b7_0680,
            bs_shader_lighting_property_get_next_active_light(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00b7_0700,
            bs_shader_lighting_property_get_next_active_non_shadow_light(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00b7_0790,
            bs_shader_lighting_property_get_numberof_active_lights(Ptr<Lit>) -> u32
        ),
        entry!(
            0x00b7_07d0,
            bs_shader_lighting_property_get_numberof_active_non_shadow_lights(Ptr<Lit>) -> u32
        ),
        entry!(0x00b7_13c0, fn_00b713c0(Ptr, u32, Ptr)),
        entry!(0x00b7_1560, fn_00b71560(Ptr<Lit>, u32, u32)),
        entry!(
            0x00b7_1600,
            bs_shader_lighting_property_remove_fenced_lights(Ptr<Lit>, u32)
        ),
        entry!(0x00b7_1720, fn_00b71720(Ptr<Lit>) -> Ptr<Lit>),
        entry!(
            0x00b7_17a0,
            bs_shader_lighting_property_remove_light(Ptr<Lit>, u32)
        ),
        entry!(
            0x00b7_18b0,
            bs_shader_lighting_property_fence_all_lights(Ptr<Lit>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A light: active unless `index` is 0xFF or `hidden`, casting shadows
    /// when `shadow`.
    fn light(e: &mut Engine, index: u16, hidden: bool, shadow: bool) -> u32 {
        let ni_light = e.mem.alloc(0x40);
        e.mem.set_u8(ni_light + 0x30, u8::from(hidden));
        let light = e.mem.alloc(0x120);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.mem.set_u16(light + 0x110, index);
        e.mem.set_u8(light + 0xec, u8::from(shadow));
        light
    }

    /// A property whose light list holds `lights` in order; answers it and
    /// the nodes.
    fn property(e: &mut Engine, lights: &[u32]) -> (Ptr<Lit>, Vec<u32>) {
        let this: Ptr<Lit> = e.new_object();
        let nodes: Vec<u32> = lights.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, lights[i]);
            if i + 1 < nodes.len() {
                e.mem.set_u32(*node, nodes[i + 1]);
            }
            if i > 0 {
                e.mem.set_u32(node + 4, nodes[i - 1]);
            }
        }
        e.set(this, Lit::lLightList, nodes.first().copied().unwrap_or(0));
        e.set(
            this,
            Lit::lLightList_tail,
            nodes.last().copied().unwrap_or(0),
        );
        e.set(this, Lit::lLightList_count, nodes.len() as u32);
        (this, nodes)
    }

    fn order(e: &Engine, this: Ptr<Lit>) -> Vec<u32> {
        let mut out = vec![];
        let mut node = e.get(this, Lit::lLightList);
        while node != 0 {
            out.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        out
    }

    #[test]
    fn fn_00b70340_finds_the_node_of_a_light() {
        let mut e = Engine::new();
        let (a, b) = (
            light(&mut e, 0, false, false),
            light(&mut e, 0, false, false),
        );
        let (this, nodes) = property(&mut e, &[a, b]);
        let slot = e.mem.alloc(4);
        assert!(e.call(0x00b7_0340, &args![this, b, 0u32, slot]).bool());
        assert_eq!(e.mem.u32(slot), nodes[1]);
        assert!(!e
            .call(0x00b7_0340, &args![this, 0x77u32, 0u32, slot])
            .bool());
        assert_eq!(e.mem.u32(slot), 0);
        e.set(this, Lit::lLightList_count, 0);
        assert!(!e.call(0x00b7_0340, &args![this, a, 0u32, slot]).bool());
    }

    #[test]
    fn resort_lights_moves_each_light_before_a_brighter_one() {
        let mut e = Engine::new();
        let (a, b, c) = (
            light(&mut e, 1, false, false),
            light(&mut e, 2, false, false),
            light(&mut e, 3, false, false),
        );
        // Luminance: the light's index.
        e.register(LIGHT_LUMINANCE_TO_BOUND, |e, a| {
            f64::from(e.mem.u16(a[0] + 0x110)).into_ret()
        });
        let (this, _) = property(&mut e, &[c, a, b]);
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call(0x00b7_0390, &args![this, 0x5000u32]);
        // Each node moves before the first brighter one ahead of it: a
        // before c, then b before c.
        assert_eq!(order(&e, this), vec![a, b, c]);
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        e.set(this, Lit::iLastRenderPassState, 9);
        // Reversed luminance: b before a, then c before b.
        e.register(LIGHT_LUMINANCE_TO_BOUND, |e, a| {
            (10.0 - f64::from(e.mem.u16(a[0] + 0x110))).into_ret()
        });
        e.call(0x00b7_0390, &args![this, 0x5000u32]);
        assert_eq!(order(&e, this), vec![c, b, a]);
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        // NaN luminance never moves anything.
        e.register(LIGHT_LUMINANCE_TO_BOUND, |_, _| f64::NAN.into_ret());
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call(0x00b7_0390, &args![this, 0x5000u32]);
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 9);
    }

    #[test]
    fn get_first_light_answers_the_head() {
        let mut e = Engine::new();
        let a = light(&mut e, 1, false, false);
        let (this, _) = property(&mut e, &[a]);
        let iterator = e.mem.alloc(4);
        assert_eq!(e.call(0x00b7_0490, &args![this, iterator]).u32(), a);
        assert_eq!(e.mem.u32(iterator), 0);
        let (empty, _) = property(&mut e, &[]);
        assert_eq!(e.call(0x00b7_0490, &args![empty, iterator]).u32(), 0);
    }

    #[test]
    fn first_and_next_active_lights_skip_inactive_ones() {
        let mut e = Engine::new();
        let off = light(&mut e, 0xff, false, false);
        let hidden = light(&mut e, 1, true, false);
        let on = light(&mut e, 2, false, false);
        let shadow = light(&mut e, 3, false, true);
        let last = light(&mut e, 4, false, false);
        let (this, nodes) = property(&mut e, &[off, hidden, on, shadow, last]);
        let iterator = e.mem.alloc(4);
        assert_eq!(e.call(0x00b7_0590, &args![this, iterator]).u32(), on);
        assert_eq!(e.mem.u32(iterator), nodes[3]);
        assert_eq!(e.call(0x00b7_0680, &args![this, iterator]).u32(), shadow);
        assert_eq!(e.call(0x00b7_0680, &args![this, iterator]).u32(), last);
        assert_eq!(e.call(0x00b7_0680, &args![this, iterator]).u32(), 0);
        assert_eq!(e.call(0x00b7_0600, &args![this, iterator]).u32(), on);
        assert_eq!(e.call(0x00b7_0700, &args![this, iterator]).u32(), last);
        // A last inactive light gives 0.
        let (tail_off, _) = property(&mut e, &[off]);
        assert_eq!(e.call(0x00b7_0590, &args![tail_off, iterator]).u32(), 0);
        assert_eq!(e.call(0x00b7_0600, &args![tail_off, iterator]).u32(), 0);
    }

    #[test]
    fn counts_active_and_non_shadow_lights() {
        let mut e = Engine::new();
        let lights = [
            light(&mut e, 0xff, false, false),
            light(&mut e, 1, true, false),
            light(&mut e, 2, false, false),
            light(&mut e, 3, false, true),
            0,
        ];
        let (this, _) = property(&mut e, &lights);
        assert_eq!(e.call(0x00b7_0790, &args![this]).u32(), 2);
        assert_eq!(e.call(0x00b7_07d0, &args![this]).u32(), 1);
    }

    #[test]
    fn fn_00b713c0_inserts_before_a_node() {
        let mut e = Engine::new();
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let (a, b) = (
            light(&mut e, 1, false, false),
            light(&mut e, 2, false, false),
        );
        let (this, nodes) = property(&mut e, &[a, b]);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x77);
        let list: Ptr = Ptr::new(this.addr() + 0x60);
        e.call(0x00b7_13c0, &args![list, nodes[1], value]);
        assert_eq!(order(&e, this), vec![a, 0x77, b]);
        e.call(0x00b7_13c0, &args![list, nodes[0], value]);
        assert_eq!(order(&e, this), vec![0x77, a, 0x77, b]);
        assert_eq!(e.get(this, Lit::lLightList_count), 4);
    }

    #[test]
    fn fn_00b71560_adds_or_moves_a_light() {
        let mut e = Engine::new();
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(LIST_MOVE_BEFORE, |_, _| Ret::default());
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let a = light(&mut e, 1, false, true);
        let b = light(&mut e, 2, false, false);
        let (this, nodes) = property(&mut e, &[a]);
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call_log = Some(vec![]);
        // No fence, already there: nothing added; a shadow light clears the state.
        e.call(0x00b7_1560, &args![this, a, 5u32]);
        assert!(calls_to(&e, LIST_ADD).is_empty());
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        assert_eq!(e.get(this, Lit::bLightListChanged), 1);
        e.call(0x00b7_1560, &args![this, b, 5u32]);
        assert_eq!(calls_to(&e, LIST_ADD).len(), 1);
        assert_eq!(calls_to(&e, LIST_ADD)[0][0], this.addr() + 0x60);
        // With a fence: a known light is moved before it.
        e.set(this, Lit::fence, nodes[0]);
        e.call(0x00b7_1560, &args![this, a, 5u32]);
        assert_eq!(
            calls_to(&e, LIST_MOVE_BEFORE),
            vec![vec![this.addr() + 0x60, nodes[0], nodes[0]]]
        );
        // A new one is inserted before it.
        e.call(0x00b7_1560, &args![this, b, 5u32]);
        assert_eq!(order(&e, this), vec![b, a]);
    }

    #[test]
    fn remove_fenced_lights_removes_everything_after_the_fence() {
        let mut e = Engine::new();
        e.register(SPIN_LOCK_ENTER, |e, _| {
            let n = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global(NODE_POOL_LOCK_COUNT, n);
            Ret::default()
        });
        e.register(LIGHT_REMOVE_TRI_GEOM, |_, _| Ret::default());
        e.register(LIST_REMOVE_NODE, |_, _| Ret::default());
        e.register(LIST_REMOVE_TAIL, |e, a| {
            let tail = e.mem.u32(a[0] + 4);
            let previous = e.mem.u32(tail + 4);
            e.mem.set_u32(a[0] + 4, previous);
            e.mem.set_u32(previous, 0);
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            Ret::default()
        });
        e.map(NODE_POOL_FREE_LIST, 0x40);
        e.map(NODE_POOL_LOCK, 8);
        let fence_light = light(&mut e, 1, false, false);
        let middle = light(&mut e, 2, false, true);
        let tail = light(&mut e, 3, false, false);
        let (this, nodes) = property(&mut e, &[fence_light, middle, tail]);
        e.set(this, Lit::fence, nodes[0]);
        e.set(this, Lit::iLastRenderPassState, 9);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1600, &args![this, 0x6000u32]);
        assert_eq!(order(&e, this), vec![fence_light]);
        assert_eq!(e.get(this, Lit::lLightList_count), 1);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE_LIST), nodes[1]);
        assert_eq!(
            calls_to(&e, LIGHT_REMOVE_TRI_GEOM),
            vec![vec![middle, 0x6000, 0], vec![tail, 0x6000, 0]]
        );
        assert_eq!(e.get(this, Lit::iLastRenderPassState), 0);
        assert_eq!(
            calls_to(&e, LIST_REMOVE_NODE),
            vec![vec![this.addr() + 0x60, this.addr() + 0x78]]
        );
        assert_eq!(e.get(this, Lit::fence), 0);
        // A fence outside the list whose next is the head: the head branch,
        // twice.
        let (other, other_nodes) = property(&mut e, &[fence_light, middle]);
        let outside = e.mem.alloc(0xc);
        e.mem.set_u32(outside, other_nodes[0]);
        e.set(other, Lit::fence, outside);
        e.call(0x00b7_1600, &args![other, 0x6000u32]);
        assert_eq!(e.get(other, Lit::lLightList), 0);
        assert_eq!(e.get(other, Lit::lLightList_tail), 0);
        assert_eq!(e.get(other, Lit::lLightList_count), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE_LIST), other_nodes[1]);
    }

    #[test]
    fn constructor_sets_up_an_empty_property() {
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        let this: Ptr<Lit> = e.new_object();
        e.set(this, Lit::fence, 5);
        assert_eq!(e.call(0x00b7_1720, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), LIGHTING_PROPERTY_VTABLE);
        assert_eq!(e.get(this, Lit::fForcedDarkness), 1.0f32.to_bits());
        assert_eq!(e.get(this, Lit::iShaderPropertyType), 1);
        assert_eq!(e.get(this, Lit::fence), 0);
    }

    #[test]
    fn remove_light_breaks_when_the_light_is_still_listed() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(DEBUG_BREAK, |_, _| Ret::default());
        let a = light(&mut e, 1, false, true);
        let (this, _) = property(&mut e, &[a]);
        e.call_log = Some(vec![]);
        e.call(0x00b7_17a0, &args![this, a]);
        assert_eq!(calls_to(&e, DEBUG_BREAK).len(), 1);
        assert_eq!(e.get(this, Lit::bLightListChanged), 1);
        assert_eq!(calls_to(&e, LIST_REMOVE)[0][0], this.addr() + 0x60);
        // With a fence nothing happens.
        e.set(this, Lit::fence, 1);
        e.call(0x00b7_17a0, &args![this, a]);
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 1);
    }

    #[test]
    fn fence_all_lights_puts_the_marker_at_the_head() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE, |e, a| {
            assert_eq!(e.mem.u32(a[1]), FENCE_MARKER);
            Ret::default()
        });
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let (empty, _) = property(&mut e, &[]);
        e.call(0x00b7_18b0, &args![empty]);
        let fence = e.get(empty, Lit::fence);
        assert_eq!(e.get(empty, Lit::lLightList), fence);
        assert_eq!(e.get(empty, Lit::lLightList_tail), fence);
        assert_eq!(e.get(empty, Lit::lLightList_count), 1);
        let a = light(&mut e, 1, false, false);
        let (this, nodes) = property(&mut e, &[a]);
        e.call(0x00b7_18b0, &args![this]);
        assert_eq!(order(&e, this), vec![FENCE_MARKER, a]);
        assert_eq!(e.mem.u32(nodes[0] + 4), e.get(this, Lit::fence));
        assert_eq!(e.get(this, Lit::lLightList_count), 2);
    }
}
