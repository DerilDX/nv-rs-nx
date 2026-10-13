//! `fallout/ai/creature.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d43a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d43a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `Creature::Creature` (Xbox PDB name in the engine map; the RTTI name of the vtable it stores at `+0`, `0x010870ac`, is
/// `Creature`): the constructor with no parameter. It runs the `Actor` constructor (`0087d4a0(this, 1)`), stores the six
/// vtable pointers (`+0`, `+0x18`, `+0x88`, `+0x94`, `+0xa4`, `+0xa8`), registers the form type 0x3c (`004f15a0`), clears
/// the byte at `+0x1b4`, copies the float at `01012054` into `+0x1b8` and `+0x1bc` and clears the words at `+0x140` and
/// `+0x13c`. The exception-unwinding frame is not translated.
pub fn creature_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0087_d4a0, &args![this, 1u32]);
    e.mem.set_u32(base, 0x0108_70ac);
    e.mem.set_u32(base + 0x18, 0x0108_70a0);
    e.mem.set_u32(base + 0x88, 0x0108_704c);
    e.mem.set_u32(base + 0x94, 0x0108_7014);
    e.mem.set_u32(base + 0xa4, 0x0108_6fe4);
    e.mem.set_u32(base + 0xa8, 0x0108_6f9c);
    e.call(0x004f_15a0, &args![this, 0x3cu32]);
    e.mem.set_u8(base + 0x1b4, 0);
    let first = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1b8, first);
    let second = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1bc, second);
    e.mem.set_u32(base + 0x140, 0);
    e.mem.set_u32(base + 0x13c, 0);
    this
}

// Translated from 008d4500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d4500 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `Creature::Creature_ov2` (Xbox PDB name in the engine map): the constructor `CreateReference` (`00564480`) runs for
/// `ACRE`. Same as [`creature_constructor`] except that the byte parameter (`RET 4`) goes to the `Actor` constructor instead
/// of the constant 1. The exception-unwinding frame is not translated.
pub fn creature_constructor_ov2(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0087_d4a0, &args![this, flag & 0xff]);
    e.mem.set_u32(base, 0x0108_70ac);
    e.mem.set_u32(base + 0x18, 0x0108_70a0);
    e.mem.set_u32(base + 0x88, 0x0108_704c);
    e.mem.set_u32(base + 0x94, 0x0108_7014);
    e.mem.set_u32(base + 0xa4, 0x0108_6fe4);
    e.mem.set_u32(base + 0xa8, 0x0108_6f9c);
    e.call(0x004f_15a0, &args![this, 0x3cu32]);
    e.mem.set_u8(base + 0x1b4, 0);
    let first = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1b8, first);
    let second = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x1bc, second);
    e.mem.set_u32(base + 0x140, 0);
    e.mem.set_u32(base + 0x13c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_43a0, creature_constructor(Ptr) -> Ptr),
        entry!(0x008d_4500, creature_constructor_ov2(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    /// Call log, object bytes and return value of one constructor run.
    type CtorRun = (Vec<(u32, Vec<u32>)>, Vec<u8>, u32);

    fn run(addr: u32, args: &[u32]) -> CtorRun {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c0);
        e.mem.write(this, &[0xcd; 0x1c0]);
        e.mem.map(0x0101_2054, 4);
        e.mem.set_u32(0x0101_2054, 0x0100_0001);
        for a in [0x0087_d4a0u32, 0x004f_15a0] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let mut words = vec![this];
        words.extend_from_slice(args);
        let ret = e.call(addr, &words).u32();
        assert_eq!(ret, this);
        let log = e.call_log.take().unwrap();
        (
            log.into_iter().skip(1).collect(),
            e.mem.bytes(this, 0x1c0),
            this,
        )
    }

    fn expected() -> Vec<u8> {
        let mut b = vec![0xcd_u8; 0x1c0];
        let mut put = |off: usize, v: u32| b[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 0x0108_70ac);
        put(0x18, 0x0108_70a0);
        put(0x88, 0x0108_704c);
        put(0x94, 0x0108_7014);
        put(0xa4, 0x0108_6fe4);
        put(0xa8, 0x0108_6f9c);
        put(0x1b8, 0x0100_0001);
        put(0x1bc, 0x0100_0001);
        put(0x140, 0);
        put(0x13c, 0);
        b[0x1b4] = 0;
        b
    }

    #[test]
    fn creature_constructor_builds_the_object() {
        let (calls, bytes, this) = run(0x008d_43a0, &[]);
        assert_eq!(
            calls,
            vec![
                (0x0087_d4a0, vec![this, 1]),
                (0x004f_15a0, vec![this, 0x3c])
            ]
        );
        assert_eq!(bytes, expected());
    }

    #[test]
    fn creature_constructor_ov2_passes_the_flag_byte_on() {
        let (calls, bytes, this) = run(0x008d_4500, &[0x1234_5600]);
        assert_eq!(
            calls,
            vec![
                (0x0087_d4a0, vec![this, 0]),
                (0x004f_15a0, vec![this, 0x3c])
            ]
        );
        assert_eq!(bytes, expected());
        let (calls, _, this) = run(0x008d_4500, &[0xab]);
        assert_eq!(calls[0], (0x0087_d4a0, vec![this, 0xab]));
    }
}
