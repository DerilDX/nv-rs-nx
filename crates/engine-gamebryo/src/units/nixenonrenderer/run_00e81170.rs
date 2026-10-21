//! `NiXenonRenderer/run_00e81170` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00a8f440` (unnamed; `thiscall`, no stack words) and
/// `NiD3DShaderInterface`'s vtable (`010ef39c`, RTTI `.?AVNiD3DShaderInterface@@`).
const BASE_CONSTRUCT_00A8F440: u32 = 0x00a8_f440;
const SHADER_INTERFACE_VTABLE_010EF39C: u32 = 0x010e_f39c;

// Translated from 00e818a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): the `NiD3DShaderInterface` constructor (the
/// vtable it stores has that class's RTTI), `thiscall`, no stack words: the base constructor
/// `00a8f440`, the words +0x14 to +0x1c and the byte +0x20 cleared, the vtable `010ef39c`.
/// Answers `this`.
pub fn fn_00e818a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00A8F440, &args![base]);
    for offset in [0x14u32, 0x18, 0x1c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x20, 0);
    e.mem.set_u32(base, SHADER_INTERFACE_VTABLE_010EF39C);
    this
}

/// What `00e812f0` calls: `00e6d760` (unnamed; `thiscall` on the renderer, one word: a
/// declaration's word, answers the vertex declaration), the renderer's packers
/// `NiXenonRenderer::PackGeometryBuffer` (`00e71fe0`, engine map; `thiscall`, five words) and
/// `00e6d780` (unnamed; `thiscall`, six words, the skinned form).
const DECLARATION_00E6D760: u32 = 0x00e6_d760;
const PACK_GEOMETRY_BUFFER_00E71FE0: u32 = 0x00e7_1fe0;
const PACK_SKINNED_00E6D780: u32 = 0x00e6_d780;

// Translated from 00e812f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): a shader's geometry packing and stream set-up,
/// `thiscall` with four words: the geometry, the skin partition, the buffer data and a word
/// never read. With a geometry and no word at +0x38: its data (+0xb8; the words below read
/// through it) and second word (+0xbc); the vertex declaration is set on the object at +0x1c
/// (its slot 0xac, `thiscall`: the declaration and 0) from the data's shader declaration (+0x30)
/// when that one's slot 0x8c answers set (`00e6d760` on the renderer +0x18 with its +0x2c),
/// else from the shader's own declaration (+0x28, its slot 0xb4) when there is one (nothing set
/// otherwise). Then the buffer data comes from the packer: with a skin partition its +0x28
/// (`00e6d780` with it, the data, the second word, the partition, +0x28 and 0), else the data's
/// +0x34 (`00e71fe0` with it, the data, the second word, +0x28 and 0). Without a geometry, or
/// with +0x38 set, the third word is the buffer data. When there is buffer data, each of its
/// streams (count +0x20, strides +0x24, chips +0x28; the chip's buffer +8) is set on the device
/// (+0x14; `SetStreamSource`, slot 0x190: stream, buffer, 0, stride) and its declaration (+0x34)
/// too (slot 0x1a0). Answers the buffer data.
pub fn fn_00e812f0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    partition: u32,
    buffer_data: u32,
    _unused_4: u32,
) -> u32 {
    let base = this.addr();
    let mut result = buffer_data;
    if geometry != 0 && e.mem.u32(base.wrapping_add(0x38)) == 0 {
        let data = e.mem.u32(geometry.wrapping_add(0xb8));
        let second = e.mem.u32(geometry.wrapping_add(0xbc));
        let mut declaration = None;
        if data != 0 {
            let shader_declaration = e.mem.u32(data.wrapping_add(0x30));
            if shader_declaration != 0 && e.vcall(shader_declaration, 0x8c, &args![]).bool() {
                let word = e.mem.u32(shader_declaration.wrapping_add(0x2c));
                let renderer = e.mem.u32(base.wrapping_add(0x18));
                declaration = Some(e.call(DECLARATION_00E6D760, &args![renderer, word]).u32());
            }
        }
        if declaration.is_none() {
            let own = e.mem.u32(base.wrapping_add(0x28));
            if own != 0 {
                declaration = Some(e.vcall(own, 0xb4, &args![]).u32());
            }
        }
        if let Some(declaration) = declaration {
            let target = e.mem.u32(base.wrapping_add(0x1c));
            e.vcall(target, 0xac, &args![declaration, 0u32]);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        let own = e.mem.u32(base.wrapping_add(0x28));
        if partition != 0 {
            result = e.mem.u32(partition.wrapping_add(0x28));
            e.call(
                PACK_SKINNED_00E6D780,
                &args![renderer, result, data, second, partition, own, 0u32],
            );
        } else {
            result = e.mem.u32(data.wrapping_add(0x34));
            e.call(
                PACK_GEOMETRY_BUFFER_00E71FE0,
                &args![renderer, result, data, second, own, 0u32],
            );
        }
    }
    if result == 0 {
        return 0;
    }
    let mut stream = 0u32;
    let mut count = e.mem.u32(result.wrapping_add(0x20));
    while stream < count {
        let strides = e.mem.u32(result.wrapping_add(0x24));
        let stride = e.mem.u32(strides.wrapping_add(stream.wrapping_mul(4)));
        let chips = e.mem.u32(result.wrapping_add(0x28));
        let chip = e.mem.u32(chips.wrapping_add(stream.wrapping_mul(4)));
        let buffer = e.mem.u32(chip.wrapping_add(8));
        let device = e.mem.u32(base.wrapping_add(0x14));
        e.vcall(device, 0x190, &args![stream, buffer, 0u32, stride]);
        count = e.mem.u32(result.wrapping_add(0x20));
        stream += 1;
    }
    let device = e.mem.u32(base.wrapping_add(0x14));
    let declaration = e.mem.u32(result.wrapping_add(0x34));
    e.vcall(device, 0x1a0, &args![declaration]);
    result
}

/// The interlocked imports.
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e81420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): the smart-pointer assignment of the shader's
/// object at +0x28, `thiscall` with the new object. Nothing when it is already there;
/// otherwise the old one's count (+4) goes down (`InterlockedDecrement`; at 0 it is deleted
/// through its slot 4), the new one is stored and its count goes up when not null.
pub fn fn_00e81420(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    let old = e.mem.u32(base.wrapping_add(0x28));
    if old == object {
        return;
    }
    if old != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(0x28), object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// The current shader global (`0126f74c`).
const CURRENT_0126F74C: u32 = 0x0126_f74c;

// Translated from 00e81470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`), `thiscall`, no stack words. With a current
/// shader at `0126f74c`: its object at +0x44 gets its slot 0x84 and its object at +0x5c its slot
/// 0x9c (both `thiscall` without words) when set. Then `this`'s byte +0x10 is cleared.
pub fn fn_00e81470(e: &mut Engine, this: Ptr) {
    let current = e.mem.u32(CURRENT_0126F74C);
    if current != 0 {
        let first = e.mem.u32(current.wrapping_add(0x44));
        if first != 0 {
            e.vcall(first, 0x84, &args![]);
        }
        let second = e.mem.u32(current.wrapping_add(0x5c));
        if second != 0 {
            e.vcall(second, 0x9c, &args![]);
        }
    }
    e.mem.set_u8(this.addr().wrapping_add(0x10), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_18a0, fn_00e818a0(Ptr) -> Ptr),
        entry!(0x00e8_12f0, fn_00e812f0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_1420, fn_00e81420(Ptr, u32)),
        entry!(0x00e8_1470, fn_00e81470(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e818a0_builds_the_shader_interface() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_00A8F440, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xcc; 0x24]);
        assert_eq!(e.call(0x00e8_18a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_INTERFACE_VTABLE_010EF39C);
        assert_eq!(e.mem.bytes(this + 0x14, 0xd), vec![0; 0xd]);
        assert_eq!(e.mem.u8(this + 0x21), 0xcc);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCT_00A8F440, vec![this]));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const SET_DECLARATION: u32 = 0x0e70_00ac;
    const OWN_DECLARATION: u32 = 0x0e70_00b4;
    const IS_READY: u32 = 0x0e70_008c;
    const SET_STREAM: u32 = 0x0e70_0190;
    const SET_VERTEX_DECLARATION: u32 = 0x0e70_01a0;
    const TABLE: u32 = 0x0e71_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// One table for every object the shader touches (slots 0x8c, 0xac, 0xb4, 0x190, 0x1a0).
    fn engine(ready: bool) -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x69];
        slots[0x8c / 4] = IS_READY;
        slots[0xac / 4] = SET_DECLARATION;
        slots[0xb4 / 4] = OWN_DECLARATION;
        slots[0x190 / 4] = SET_STREAM;
        slots[0x1a0 / 4] = SET_VERTEX_DECLARATION;
        e.put_vtable(TABLE, &slots);
        for no_op in [
            SET_DECLARATION,
            SET_STREAM,
            SET_VERTEX_DECLARATION,
            PACK_GEOMETRY_BUFFER_00E71FE0,
            PACK_SKINNED_00E6D780,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(OWN_DECLARATION, |_, _| 0xdd01u32.into_ret());
        e.register(DECLARATION_00E6D760, |_, _| 0xdd02u32.into_ret());
        if ready {
            e.register(IS_READY, |_, _| 1u32.into_ret());
        } else {
            e.register(IS_READY, |_, _| 0u32.into_ret());
        }
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, TABLE);
        object
    }

    /// A shader (+0x14 device, +0x18 renderer, +0x1c target, +0x28 own object), a geometry
    /// whose data has a shader declaration and buffer data with two streams.
    fn rig(e: &mut Engine) -> (u32, u32, u32, u32) {
        let this = object(e, 0x40);
        let device = object(e, 4);
        let target = object(e, 4);
        let own = object(e, 8);
        e.mem.set_u32(this + 0x14, device);
        e.mem.set_u32(this + 0x18, 0x7100);
        e.mem.set_u32(this + 0x1c, target);
        e.mem.set_u32(this + 0x28, own);
        let declaration = object(e, 0x30);
        e.mem.set_u32(declaration + 0x2c, 0x2c2c);
        let buffer_data = e.mem.alloc(0x38);
        let strides = e.mem.alloc(8);
        e.mem.set_u32(strides, 0x10);
        e.mem.set_u32(strides + 4, 0x20);
        let chips = e.mem.alloc(8);
        for index in 0..2u32 {
            let chip = e.mem.alloc(0xc);
            e.mem.set_u32(chip + 8, 0xb0 + index);
            e.mem.set_u32(chips + 4 * index, chip);
        }
        e.mem.set_u32(buffer_data + 0x20, 2);
        e.mem.set_u32(buffer_data + 0x24, strides);
        e.mem.set_u32(buffer_data + 0x28, chips);
        e.mem.set_u32(buffer_data + 0x34, 0xdec1);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data + 0x30, declaration);
        e.mem.set_u32(data + 0x34, buffer_data);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xbc, 0x5ec0);
        (this, geometry, data, buffer_data)
    }

    #[test]
    fn fn_00e812f0_packs_and_sets_the_streams() {
        let mut e = engine(true);
        let (this, geometry, data, buffer_data) = rig(&mut e);
        let device = e.mem.u32(this + 0x14);
        let target = e.mem.u32(this + 0x1c);
        let own = e.mem.u32(this + 0x28);
        e.call_log = Some(vec![]);
        let answered = e
            .call(0x00e8_12f0, &args![this, geometry, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(answered, buffer_data);
        assert_eq!(calls(&e, DECLARATION_00E6D760), vec![vec![0x7100, 0x2c2c]]);
        assert_eq!(calls(&e, SET_DECLARATION), vec![vec![target, 0xdd02, 0]]);
        assert_eq!(
            calls(&e, PACK_GEOMETRY_BUFFER_00E71FE0),
            vec![vec![0x7100, buffer_data, data, 0x5ec0, own, 0]]
        );
        assert_eq!(
            calls(&e, SET_STREAM),
            vec![
                vec![device, 0, 0xb0, 0, 0x10],
                vec![device, 1, 0xb1, 0, 0x20]
            ]
        );
        assert_eq!(
            calls(&e, SET_VERTEX_DECLARATION),
            vec![vec![device, 0xdec1]]
        );
    }

    #[test]
    fn fn_00e812f0_falls_back_to_its_own_declaration_and_the_skinned_packer() {
        let mut e = engine(false);
        let (this, geometry, data, _) = rig(&mut e);
        let target = e.mem.u32(this + 0x1c);
        let own = e.mem.u32(this + 0x28);
        let partition_data = e.mem.alloc(0x38);
        e.mem.set_u32(partition_data + 0x20, 0);
        let partition = e.mem.alloc(0x2c);
        e.mem.set_u32(partition + 0x28, partition_data);
        e.call_log = Some(vec![]);
        let answered = e
            .call(0x00e8_12f0, &args![this, geometry, partition, 0u32, 0u32])
            .u32();
        assert_eq!(answered, partition_data);
        assert_eq!(calls(&e, OWN_DECLARATION), vec![vec![own]]);
        assert_eq!(calls(&e, SET_DECLARATION), vec![vec![target, 0xdd01, 0]]);
        assert_eq!(
            calls(&e, PACK_SKINNED_00E6D780),
            vec![vec![
                0x7100,
                partition_data,
                data,
                0x5ec0,
                partition,
                own,
                0
            ]]
        );
        assert!(calls(&e, SET_STREAM).is_empty());
        assert_eq!(calls(&e, SET_VERTEX_DECLARATION).len(), 1);
    }

    #[test]
    fn fn_00e812f0_uses_the_given_buffer_data_when_locked() {
        let mut e = engine(true);
        let (this, geometry, _, buffer_data) = rig(&mut e);
        e.mem.set_u32(this + 0x38, 1);
        e.call_log = Some(vec![]);
        let answered = e
            .call(0x00e8_12f0, &args![this, geometry, 0u32, buffer_data, 0u32])
            .u32();
        assert_eq!(answered, buffer_data);
        assert!(calls(&e, SET_DECLARATION).is_empty());
        assert_eq!(calls(&e, SET_STREAM).len(), 2);
        // No geometry and no buffer data: nothing.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e8_12f0, &args![this, 0u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e81420_assigns_the_counted_object() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(0x0e72_0004, |_, _| Ret::default());
        e.put_vtable(0x0e73_0000, &[0, 0x0e72_0004]);
        let (old, new) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(old, 0x0e73_0000);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, old);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1420, &args![this, new]);
        assert_eq!(e.mem.u32(this + 0x28), new);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT), vec![vec![old + 4]]);
        assert_eq!(calls(&e, 0x0e72_0004), vec![vec![old]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_INCREMENT), vec![vec![new + 4]]);
        // The same object: nothing; a count left above 0: not deleted.
        e.call_log = Some(vec![]);
        e.call(0x00e8_1420, &args![this, new]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.mem.set_u32(new, 0x0e73_0000);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1420, &args![this, 0u32]);
        assert!(calls(&e, 0x0e72_0004).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn fn_00e81470_tells_the_current_shader() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let mut slots = vec![0u32; 0x28];
        slots[0x84 / 4] = 0x0e74_0084;
        slots[0x9c / 4] = 0x0e74_009c;
        e.put_vtable(0x0e75_0000, &slots);
        e.register(0x0e74_0084, |_, _| Ret::default());
        e.register(0x0e74_009c, |_, _| Ret::default());
        let (first, second) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(first, 0x0e75_0000);
        e.mem.set_u32(second, 0x0e75_0000);
        let current = e.mem.alloc(0x60);
        e.mem.set_u32(current + 0x44, first);
        e.mem.set_u32(current + 0x5c, second);
        e.mem.set_u32(CURRENT_0126F74C, current);
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1470, &args![this]);
        assert_eq!(calls(&e, 0x0e74_0084), vec![vec![first]]);
        assert_eq!(calls(&e, 0x0e74_009c), vec![vec![second]]);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        // No current shader: only the byte.
        e.mem.set_u32(CURRENT_0126F74C, 0);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1470, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }
}
