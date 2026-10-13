//! `fallout shared/run_00618390` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00618580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00618580 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the topic form (`DIAL`, form type 0x45); `ECX` is the object and the byte parameter (`RET 4`) goes to
/// `+0x24`. `CreateFormOfType` (`00465110`) has no slot for type 0x45; this constructor registers the type itself
/// (`004f15a0(this, 0x45)`). The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_d19c`)
/// is `TESTopic`, so this is the constructor of `TESTopic`. It runs the `TESForm` constructor (`00483370`) and the component
/// constructors by address, stores the two vtable pointers (`+0`, `+0x18`), clears the two bytes at `+0x24` (the `memset`
/// wrapper `00403d30`) and then sets the byte at `+0x24` to the parameter, clears `+0x3c` and copies the float at `0101b268`
/// into `+0x28`. The exception-unwinding frame is not translated.
pub fn fn_00618580(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0104_d19c);
    e.mem.set_u32(base + 0x18, 0x0104_d188);
    e.call(0x0096_a2d0, &args![base + 0x2c]);
    e.call(0x0040_37b0, &args![base + 0x34]);
    e.call(0x0040_37b0, &args![base + 0x40]);
    e.call(0x0040_3d30, &args![base + 0x24, 0u32, 2u32]);
    e.mem.set_u32(base + 0x3c, 0);
    let value = e.global::<f32>(0x0101_b268);
    e.mem.set_f32(base + 0x28, value);
    e.mem.set_u8(base + 0x24, flag as u8);
    e.call(0x004f_15a0, &args![this, 0x45u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_8580, fn_00618580(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn fn_00618580_builds_the_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xcd; 0x48]);
        e.mem.map(0x0101_b268, 4);
        e.mem.set_u32(0x0101_b268, 0x0100_0001);
        for a in [
            0x0048_3370u32,
            0x0040_2d00,
            0x0096_a2d0,
            0x0040_37b0,
            0x0040_3d30,
            0x004f_15a0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let ret = e.call(0x0061_8580, &args![this, 0x1234_5601u32]).u32();
        assert_eq!(ret, this);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (0x0048_3370, vec![this]),
                (0x0040_2d00, vec![this + 0x18]),
                (0x0096_a2d0, vec![this + 0x2c]),
                (0x0040_37b0, vec![this + 0x34]),
                (0x0040_37b0, vec![this + 0x40]),
                (0x0040_3d30, vec![this + 0x24, 0, 2]),
                (0x004f_15a0, vec![this, 0x45]),
            ]
        );
        let mut want = vec![0xcd_u8; 0x48];
        let mut put = |off: usize, v: u32| want[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 0x0104_d19c);
        put(0x18, 0x0104_d188);
        put(0x3c, 0);
        put(0x28, 0x0100_0001);
        want[0x24] = 0x01;
        assert_eq!(e.mem.bytes(this, 0x48), want);
    }
}
