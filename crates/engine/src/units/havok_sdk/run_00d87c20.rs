//! `Havok SDK/run_00d87c20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d87c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, two words (`RET 8`): calls virtual slot `+8` (when
/// `flag` is non-zero) or `+4` (when it is zero) of the object embedded at
/// `this + 0x40`, on that object, with `argument`. Returns the callee's EAX.
pub fn fn_00d87c20(e: &mut Engine, this: Ptr, argument: u32, flag: u8) -> u32 {
    let object = this.addr() + 0x40;
    let slot = if flag != 0 { 8 } else { 4 };
    e.vcall(object, slot, &args![argument]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d87c20, fn_00d87c20(Ptr, u32, u8) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d87c20_picks_slot_8_for_a_set_flag_and_slot_4_otherwise() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_2000, &[0, 0x0090_5000, 0x0090_5100]);
        e.register(0x0090_5000, |_, a| Ret {
            eax: 1000 + a[1],
            ..Ret::default()
        });
        e.register(0x0090_5100, |_, a| Ret {
            eax: 2000 + a[1],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x40, 0x0090_2000);
        assert_eq!(
            e.call(0x00d8_7c20, &args![Ptr::<()>::new(this), 7u32, 1u8])
                .u32(),
            2007
        );
        assert_eq!(
            e.call(0x00d8_7c20, &args![Ptr::<()>::new(this), 7u32, 0u8])
                .u32(),
            1007
        );
    }
}
