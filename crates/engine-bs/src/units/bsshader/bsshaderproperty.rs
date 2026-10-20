//! `BSShader/bsshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAVObject::GetProperty` (Xbox PDB, `00a59d30`): `thiscall`, `RET 4`; the object's property
/// of the given type, or null.
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `BSShaderProperty::SetFlag` (Xbox PDB, `00441130`): `thiscall`, `RET 8` (flag index, value
/// byte).
const BS_SHADER_PROPERTY_SET_FLAG: u32 = 0x0044_1130;
/// The virtual slot (byte offset) that answers the object as a geometry, or null (`IsGeometry`).
const SLOT_IS_GEOMETRY: u32 = 0x18;
/// The virtual slot (byte offset) that answers the object as a node, or null (`IsNode`).
const SLOT_IS_NODE: u32 = 0x0c;
/// The property type `NiAVObject::GetProperty` is asked for (the shader property's slot).
const SHADER_PROPERTY_TYPE: u32 = 3;

// Translated from 00ba91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB): `cdecl`, three stack words (an object, a flag
/// index, a value). For a non-null object: a geometry (virtual slot +0x18 non-null) gets the
/// flag set on its shader property (`GetProperty(3)` then `SetFlag(index, value)`), when it has
/// one; any other object that is a node (slot +0xc) passes the call on to each of its non-null
/// children (the `NiTArray` at node `+0x9c`: buffer at +0xa0, count at +0xa6).
pub fn bs_shader_property_set_flag_recurse(e: &mut Engine, object: Ptr, flag: u32, value: u32) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), SLOT_IS_GEOMETRY, &args![]).u32() != 0 {
        let property = e
            .call(
                NI_AV_OBJECT_GET_PROPERTY,
                &args![object, SHADER_PROPERTY_TYPE],
            )
            .u32();
        if property != 0 {
            e.call(BS_SHADER_PROPERTY_SET_FLAG, &args![property, flag, value]);
        }
        return;
    }
    let node = e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32();
    if node != 0 {
        let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
        let mut index = 0u32;
        if count != 0 {
            loop {
                let buffer = e.mem.u32(node.wrapping_add(0xa0));
                let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
                if child != 0 {
                    bs_shader_property_set_flag_recurse(e, Ptr::new(child), flag, value);
                }
                index = index.wrapping_add(1);
                if index >= count {
                    break;
                }
            }
        }
    }
}

layout! {
    /// `BSShaderProperty::RenderPass` (Xbox PDB), 0x10 bytes; the offsets
    /// match the PC constructor.
    pub struct RenderPass: 0x10 {
        /// `pGeometry` (Xbox PDB).
        0x00 pGeometry: u32,
        /// `usPassEnum` (Xbox PDB).
        0x04 usPassEnum: u16,
        /// `eAccumulationHint` (Xbox PDB).
        0x06 eAccumulationHint: u8,
        /// `bFirstPass` (Xbox PDB).
        0x07 bFirstPass: u8,
        /// `bLastPass` (Xbox PDB).
        0x08 bLastPass: u8,
        /// `ucNumLights` (Xbox PDB).
        0x09 ucNumLights: u8,
        /// `ucMaxNumLights` (Xbox PDB).
        0x0a ucMaxNumLights: u8,
        /// `ucExtraParam` (Xbox PDB).
        0x0b ucExtraParam: u8,
        /// `ppSceneLights` (Xbox PDB).
        0x0c ppSceneLights: u32,
    }
}

/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00ba8ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::RenderPass` (Xbox PDB): `cdecl` with a
/// variable argument list: the pass, its geometry, its pass enum (low half
/// of the word), its first-pass flag (low byte), the light count (low byte)
/// and then that many light words. Clears the hint, last-pass flag, maximum
/// light count, extra parameter and light array; with lights allocates
/// `count * 4` bytes, sets the maximum to the count and copies the light
/// words in. Returns the pass. The exe reads the light words from its
/// caller's stack; here they are the words after the count (a missing word
/// stops loudly).
pub fn bs_shader_property_render_pass_render_pass(
    e: &mut Engine,
    this: Ptr<RenderPass>,
    geometry: u32,
    pass_enum: u16,
    first_pass: u8,
    light_count: u8,
    lights: &[u32],
) -> Ptr<RenderPass> {
    e.set(this, RenderPass::pGeometry, geometry);
    e.set(this, RenderPass::usPassEnum, pass_enum);
    e.set(this, RenderPass::ucNumLights, light_count);
    e.set(this, RenderPass::ucMaxNumLights, 0);
    e.set(this, RenderPass::bFirstPass, first_pass);
    e.set(this, RenderPass::bLastPass, 0);
    e.set(this, RenderPass::ppSceneLights, 0);
    e.set(this, RenderPass::eAccumulationHint, 0);
    if light_count == 0 {
        e.set(this, RenderPass::ucExtraParam, 0);
        return this;
    }
    let count = u32::from(light_count);
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, RenderPass::ppSceneLights, array);
    e.set(this, RenderPass::ucMaxNumLights, light_count);
    // `TEST EDI,EDI; JLE`: never taken (the count is at least 1 here); the
    // branch stores the zero in AL as the extra parameter.
    if (count as i32) <= 0 {
        e.set(this, RenderPass::ucExtraParam, 0);
        return this;
    }
    let mut index = 0u32;
    loop {
        let light = lights[index as usize];
        let array = e.get(this, RenderPass::ppSceneLights);
        e.mem.set_u32(array + index * 4, light);
        index += 1;
        if (index as i32) >= (count as i32) {
            break;
        }
    }
    e.set(this, RenderPass::ucExtraParam, 0);
    this
}

/// The registry entry of [`bs_shader_property_render_pass_render_pass`]:
/// the five fixed words, then the light words.
fn render_pass_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 5,
        "RenderPass::RenderPass needs 5 argument words, got {}",
        words.len()
    );
    bs_shader_property_render_pass_render_pass(
        e,
        Ptr::new(words[0]),
        words[1],
        words[2] as u16,
        words[3] as u8,
        words[4] as u8,
        &words[5..],
    )
    .addr()
    .into_ret()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_91a0,
            bs_shader_property_set_flag_recurse(Ptr, u32, u32)
        ),
        (0x00ba_8ec0, render_pass_entry as AbiFn),
    ]
}

#[cfg(test)]
mod render_pass_tests {
    use super::*;

    #[test]
    fn render_pass_without_lights_clears_the_pass() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for off in (0..0x10).step_by(4) {
            e.mem.set_u32(this + off, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        // The accumulator's call: geometry 0, pass 0x2f5, first pass, no
        // lights, and one more pushed word.
        let result = e.call(0x00ba_8ec0, &[this, 0, 0x0001_02f5, 0x0101, 0, 0]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u16(this + 4), 0x2f5);
        assert_eq!(e.mem.u8(this + 6), 0);
        assert_eq!(e.mem.u8(this + 7), 1);
        assert_eq!(e.mem.bytes(this + 8, 4), vec![0, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let allocations = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_ALLOCATE)
            .count();
        assert_eq!(allocations, 0);
    }

    #[test]
    fn render_pass_with_lights_copies_them_into_a_new_array() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ba_8ec0, &[this, 0x1234, 7, 0, 3, 0xa, 0xb, 0xc]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.mem.u16(this + 4), 7);
        assert_eq!(e.mem.u8(this + 7), 0);
        assert_eq!(e.mem.u8(this + 9), 3);
        assert_eq!(e.mem.u8(this + 0xa), 3);
        assert_eq!(e.mem.u8(this + 0xb), 0);
        let array = e.mem.u32(this + 0xc);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(array + i * 4)).collect::<Vec<_>>(),
            vec![0xa, 0xb, 0xc]
        );
        let sizes: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_ALLOCATE)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(sizes, vec![12]);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Objects share one vtable: slot +0x18 answers the dword at +0x10 (non-zero: geometry),
    /// slot +0xc answers the dword at +0x14 (the node view, non-null: a node).
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_GEOMETRY as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NI_AV_OBJECT_GET_PROPERTY, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(BS_SHADER_PROPERTY_SET_FLAG, |_, _| Ret::default());
        e
    }

    fn geometry(e: &mut Engine, property: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x10, 1);
        e.mem.set_u32(object + 0x18, property);
        object
    }

    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let object = e.mem.alloc(0xb0);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x14, object);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xa0, buffer);
        e.mem.set_u16(object + 0xa6, children.len() as u16);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *child);
        }
        object
    }

    #[test]
    fn at_00ba91a0_sets_the_flag_on_a_geometrys_shader_property() {
        let mut e = rig();
        let geometry = geometry(&mut e, 0x7000);
        e.call(0x00ba_91a0, &args![geometry, 0x30u32, 1u32]);
        assert_eq!(
            calls(&e, NI_AV_OBJECT_GET_PROPERTY),
            vec![vec![geometry, 3]]
        );
        assert_eq!(
            calls(&e, BS_SHADER_PROPERTY_SET_FLAG),
            vec![vec![0x7000, 0x30, 1]]
        );
    }

    #[test]
    fn at_00ba91a0_skips_a_geometry_without_the_property_and_a_null_object() {
        let mut e = rig();
        let geometry = geometry(&mut e, 0);
        e.call(0x00ba_91a0, &args![geometry, 0x30u32, 1u32]);
        e.call(0x00ba_91a0, &args![0u32, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 1);
    }

    #[test]
    fn at_00ba91a0_walks_the_children_of_nodes_skipping_null_slots() {
        let mut e = rig();
        let first = geometry(&mut e, 0x7001);
        let second = geometry(&mut e, 0x7002);
        let inner = node(&mut e, &[second]);
        let outer = node(&mut e, &[first, 0, inner]);
        e.call(0x00ba_91a0, &args![outer, 0x30u32, 0u32]);
        assert_eq!(
            calls(&e, BS_SHADER_PROPERTY_SET_FLAG),
            vec![vec![0x7001, 0x30, 0], vec![0x7002, 0x30, 0]]
        );
    }

    #[test]
    fn at_00ba91a0_does_nothing_for_an_object_that_is_neither() {
        let mut e = rig();
        let plain = e.mem.alloc(0x20);
        e.mem.set_u32(plain, 0x0f00_e100);
        e.call(0x00ba_91a0, &args![plain, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
        assert!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).is_empty());
    }

    #[test]
    fn at_00ba91a0_a_node_without_children_has_nothing_to_visit() {
        let mut e = rig();
        let empty = node(&mut e, &[]);
        e.call(0x00ba_91a0, &args![empty, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
    }
}
