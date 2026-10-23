//! `BSShader/run_00bb7d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` the depth is multiplied by (`010b9e30`, 0.83 in the exe).
const DEPTH_FACTOR: u32 = 0x010b_9e30;
/// The `double` the result is scaled by (`0103a480`, 163840.0 in the exe).
const DISTANCE_SCALE: u32 = 0x0103_a480;
/// The `float` copied to `+0x30` (`010317c4`, 163840.0 in the exe).
const END_DISTANCE: u32 = 0x0103_17c4;

// Translated from 00bb8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader`). `thiscall`, one `float` (`RET 4`): stores
/// `(1 - value * [010b9e30]) * [0103a480]` (x87, the two `double`s) as the `float` at `+0x2c`
/// and copies the `float` at `010317c4` to `+0x30` (`FLD`/`FSTP`: a signalling NaN comes out
/// quiet). On a `BSFogProperty` these are `fStartDistance` and `fEndDistance` (Xbox PDB);
/// `BSFogProperty`'s constructor inlines the same code on its depth.
pub fn fn_00bb8140(e: &mut Engine, this: Ptr, value: f32) {
    let factor: f64 = e.global(DEPTH_FACTOR);
    let scale: f64 = e.global(DISTANCE_SCALE);
    let product = x87_mul(x87_load32(value.to_bits()), factor);
    let start = x87_mul(x87_sub(1.0, product), scale);
    e.mem
        .set_u32(this.addr().wrapping_add(0x2c), x87_store32(start));
    let end = x87_quiet_bits(e.global::<u32>(END_DISTANCE));
    e.mem.set_u32(this.addr().wrapping_add(0x30), end);
}

/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager object and its deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The vtable at `010b9d28` (`TileShaderProperty`'s, installed by `00bb7c30`).
const TILE_SHADER_PROPERTY_VTABLE: u32 = 0x010b_9d28;
/// The shader property's destructor `00baa0e0` (`thiscall`).
const SHADER_PROPERTY_DESTRUCTOR_00BAA0E0: u32 = 0x00ba_a0e0;
/// The allocation `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `TileShaderProperty::TileShaderProperty` (`00bb7c30`, `thiscall`, one word).
const TILE_SHADER_PROPERTY_CONSTRUCTOR: u32 = 0x00bb_7c30;
/// The sized free `00aa1460` (`cdecl`, two words: the block and its size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

/// Releases a reference (`InterlockedDecrement` on `+4`; at zero the object's slot +4,
/// `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00bb7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile shader property's destructor (unnamed in the engine map; [`fn_00bb7f60`], slot 0
/// of the vtable at `010b9d28`, runs it). Sets that vtable, releases and clears the
/// texture at `+0x60`, frees the file name's text at `+0x94` (when set) and clears the
/// string (`+0x94`, `+0x98`, `+0x9a`), releases the alpha map at `+0x64`, releases the
/// `+0x60` texture again when set (the members' own destructors), then runs the shader
/// property's destructor (`00baa0e0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7d70 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bb7d70(e: &mut Engine, this: Ptr) {
    let at = |off: u32| this.addr().wrapping_add(off);
    e.mem.set_u32(this.addr(), TILE_SHADER_PROPERTY_VTABLE);
    let texture = e.mem.u32(at(0x60));
    if texture != 0 {
        release_reference(e, texture);
        e.mem.set_u32(at(0x60), 0);
    }
    let text = e.mem.u32(at(0x94));
    if text != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, text]);
    }
    e.mem.set_u32(at(0x94), 0);
    e.mem.set_u16(at(0x9a), 0);
    e.mem.set_u16(at(0x98), 0);
    let alpha_map = e.mem.u32(at(0x64));
    if alpha_map != 0 {
        release_reference(e, alpha_map);
    }
    let texture = e.mem.u32(at(0x60));
    if texture != 0 {
        release_reference(e, texture);
    }
    e.call(SHADER_PROPERTY_DESTRUCTOR_00BAA0E0, &args![this]);
}

// Translated from 00bb7ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stream's creation function for `"TileShaderProperty"` (the shader manager's
/// registration table pairs the name at `010a922c` with this address): allocates 0xb0 bytes
/// (`00aa13e0`) and constructs a tile shader property in them (`00bb7c30` with 0), answering
/// what the constructor answers; 0 when the allocation fails.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7ef0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn tile_shader_property_create_object(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0xb0u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
        .u32()
}

// Translated from 00bb7f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile shader property's scalar deleting destructor (slot 0 of the vtable at
/// `010b9d28`): [`fn_00bb7d70`], then frees the 0xb0-byte object (`00aa1460(this, 0xb0)`)
/// when bit 0 of the flags is set. Answers `this`.
pub fn fn_00bb7f60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb7d70(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0xb0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_8140, fn_00bb8140(Ptr, f32)),
        entry!(0x00bb_7d70, fn_00bb7d70(Ptr)),
        entry!(0x00bb_7ef0, tile_shader_property_create_object() -> u32),
        entry!(0x00bb_7f60, fn_00bb7f60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [0x0103_1000, 0x0103_a000, 0x010b_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEPTH_FACTOR, 0.83f64);
        e.set_global(DISTANCE_SCALE, 163_840.0f64);
        e.set_global(END_DISTANCE, 163_840.0f32);
        e
    }

    #[test]
    fn fn_00bb8140_sets_the_two_distances() {
        let mut e = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00bb_8140, &args![this, 0.5f32]);
        let expected = ((1.0 - 0.5 * 0.83f64) * 163_840.0) as f32;
        assert_eq!(e.mem.f32(this + 0x2c), expected);
        assert_eq!(e.mem.f32(this + 0x30), 163_840.0);
    }

    #[test]
    fn fn_00bb8140_quiets_nans() {
        let mut e = rig();
        e.set_global(END_DISTANCE, 0x7f80_0001u32);
        let this = e.mem.alloc(0x40);
        e.call(0x00bb_8140, &args![this, 0x7f80_0002u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7fc0_0002);
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod tile_tests {
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        for addr in [
            0x0f00_0004,
            MEMORY_DEALLOCATE,
            SHADER_PROPERTY_DESTRUCTOR_00BAA0E0,
            FREE_SIZED_00AA1460,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, references: u32) -> u32 {
        let o = e.mem.alloc(8);
        e.mem.set_u32(o, 0x0f00_1000);
        e.mem.set_u32(o + 4, references);
        o
    }

    #[test]
    fn destructor_releases_textures_and_the_name() {
        let mut e = rig();
        let this = e.mem.alloc(0xb0);
        let texture = object(&mut e, 1);
        let alpha_map = object(&mut e, 2);
        e.mem.set_u32(this + 0x60, texture);
        e.mem.set_u32(this + 0x64, alpha_map);
        e.mem.set_u32(this + 0x94, 0x9000);
        e.mem.set_u32(this + 0x98, 0x0010_0004);
        e.call(0x00bb_7d70, &args![this]);
        assert_eq!(e.mem.u32(this), TILE_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), alpha_map);
        assert_eq!(e.mem.u32(alpha_map + 4), 1);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![texture]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x9000]]
        );
        assert_eq!((e.mem.u32(this + 0x94), e.mem.u32(this + 0x98)), (0, 0));
        assert_eq!(
            calls(&e, SHADER_PROPERTY_DESTRUCTOR_00BAA0E0),
            vec![vec![this]]
        );
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn create_object_constructs_a_tile_property() {
        let mut e = rig();
        e.map(0x0f00_2000, 0x1000);
        e.register(ALLOCATE_00AA13E0, |e, _| e.mem.u32(0x0f00_2000).into_ret());
        e.register(TILE_SHADER_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00bb_7ef0, &[]).u32(), 0);
        e.mem.set_u32(0x0f00_2000, 0x4000);
        assert_eq!(e.call(0x00bb_7ef0, &[]).u32(), 0x4000);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0xb0], vec![0xb0]]);
        assert_eq!(
            calls(&e, TILE_SHADER_PROPERTY_CONSTRUCTOR),
            vec![vec![0x4000, 0]]
        );
    }

    #[test]
    fn deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        let this = e.mem.alloc(0xb0);
        assert_eq!(e.call(0x00bb_7f60, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x00bb_7f60, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![this, 0xb0]]);
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTRUCTOR_00BAA0E0).len(), 2);
    }
}
