//! `fallout shared/tescreature.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f7230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f7230 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESCreature::TESCreature` (Xbox PDB): the form constructor
/// `CreateFormOfType` (`00465110`) runs for form type 0x2b (0x160 bytes).
/// `ECX` is the object. It constructs the `TESActorBase` base (`005f75e0`)
/// and the two components at `+0x10c` and `+0x114`, stores the vtable
/// pointers of the object and its components, registers the form type
/// (`004f15a0`), runs `005f7a40`, seeds the AI form at `+0x90` (`0x32`,
/// `1`, `2`, `2` through `0047ef50`, `0047ee00`, `0047eed0`, `0047ef00`),
/// and, when `004516b0` on the object at `011c3f2c` answers false, sets the
/// faction rank of the actor base data at `+0x30` to the object at
/// `011cb550` and `0` (`0047d800`).
pub fn tescreature_tescreature(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x005f_75e0, &args![this]);
    e.call(0x0047_fdc0, &args![base + 0x10c]);
    e.call(0x0048_9a60, &args![base + 0x114]);
    for (offset, vtable) in [
        (0x00, 0x0104_8f5c),
        (0x30, 0x0104_8eec),
        (0x64, 0x0104_8ed8),
        (0x70, 0x0104_8ec4),
        (0x7c, 0x0104_8ea4),
        (0x90, 0x0104_8e84),
        (0xb0, 0x0104_8e6c),
        (0xb8, 0x0104_8e58),
        (0xc4, 0x0104_8e44),
        (0xd0, 0x0104_8e30),
        (0xdc, 0x0104_8e0c),
        (0xf4, 0x0104_8df8),
        (0x100, 0x0104_8dc8),
        (0x104, 0x0104_8db4),
        (0x10c, 0x0104_8d9c),
        (0x114, 0x0104_8d88),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x004f_15a0, &args![this, 0x2bu32]);
    e.call(0x005f_7a40, &args![this]);
    e.call(0x0047_ef50, &args![base + 0x90, 0x32u32, 0u32]);
    e.call(0x0047_ee00, &args![base + 0x90, 1u32, 0u32]);
    e.call(0x0047_eed0, &args![base + 0x90, 2u32, 0u32]);
    e.call(0x0047_ef00, &args![base + 0x90, 2u32, 0u32]);
    let checked = e.global::<u32>(0x011c_3f2c);
    if !e.call(0x0045_16b0, &args![checked]).bool() {
        let faction = e.global::<u32>(0x011c_b550);
        e.call(0x0047_d800, &args![base + 0x30, faction, 0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_7230, tescreature_tescreature(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tescreature_tescreature_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_7230,
            size: 0x160,
            globals: &[(0x011c_3f2c, 0x1000000), (0x011c_b550, 0x1000001)],
            returns: &[],
            calls: &[
                (0x005f_75e0, &[A::T(0x0)]),
                (0x0047_fdc0, &[A::T(0x10c)]),
                (0x0048_9a60, &[A::T(0x114)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2b)]),
                (0x005f_7a40, &[A::T(0x0)]),
                (0x0047_ef50, &[A::T(0x90), A::V(0x32), A::V(0x0)]),
                (0x0047_ee00, &[A::T(0x90), A::V(0x1), A::V(0x0)]),
                (0x0047_eed0, &[A::T(0x90), A::V(0x2), A::V(0x0)]),
                (0x0047_ef00, &[A::T(0x90), A::V(0x2), A::V(0x0)]),
                (0x0045_16b0, &[A::V(0x1000000)]),
                (0x0047_d800, &[A::T(0x30), A::V(0x1000001), A::V(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_8f5c),
                (0x30, 4, 0x0104_8eec),
                (0x64, 4, 0x0104_8ed8),
                (0x70, 4, 0x0104_8ec4),
                (0x7c, 4, 0x0104_8ea4),
                (0x90, 4, 0x0104_8e84),
                (0xb0, 4, 0x0104_8e6c),
                (0xb8, 4, 0x0104_8e58),
                (0xc4, 4, 0x0104_8e44),
                (0xd0, 4, 0x0104_8e30),
                (0xdc, 4, 0x0104_8e0c),
                (0xf4, 4, 0x0104_8df8),
                (0x100, 4, 0x0104_8dc8),
                (0x104, 4, 0x0104_8db4),
                (0x10c, 4, 0x0104_8d9c),
                (0x114, 4, 0x0104_8d88),
            ],
        });
    }

    #[test]
    fn tescreature_tescreature_skips_the_faction_rank_when_the_check_holds() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_7230,
            size: 0x160,
            globals: &[(0x011c_3f2c, 0x1000000), (0x011c_b550, 0x1000001)],
            returns: &[(0x0045_16b0, 1)],
            calls: &[
                (0x005f_75e0, &[A::T(0)]),
                (0x0047_fdc0, &[A::T(0x10c)]),
                (0x0048_9a60, &[A::T(0x114)]),
                (0x004f_15a0, &[A::T(0), A::V(0x2b)]),
                (0x005f_7a40, &[A::T(0)]),
                (0x0047_ef50, &[A::T(0x90), A::V(0x32), A::V(0)]),
                (0x0047_ee00, &[A::T(0x90), A::V(1), A::V(0)]),
                (0x0047_eed0, &[A::T(0x90), A::V(2), A::V(0)]),
                (0x0047_ef00, &[A::T(0x90), A::V(2), A::V(0)]),
                (0x0045_16b0, &[A::V(0x1000000)]),
            ],
            stores: &[
                (0x00, 4, 0x0104_8f5c),
                (0x30, 4, 0x0104_8eec),
                (0x64, 4, 0x0104_8ed8),
                (0x70, 4, 0x0104_8ec4),
                (0x7c, 4, 0x0104_8ea4),
                (0x90, 4, 0x0104_8e84),
                (0xb0, 4, 0x0104_8e6c),
                (0xb8, 4, 0x0104_8e58),
                (0xc4, 4, 0x0104_8e44),
                (0xd0, 4, 0x0104_8e30),
                (0xdc, 4, 0x0104_8e0c),
                (0xf4, 4, 0x0104_8df8),
                (0x100, 4, 0x0104_8dc8),
                (0x104, 4, 0x0104_8db4),
                (0x10c, 4, 0x0104_8d9c),
                (0x114, 4, 0x0104_8d88),
            ],
        });
    }
}
