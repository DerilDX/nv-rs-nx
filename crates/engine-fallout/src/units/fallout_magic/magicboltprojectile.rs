//! `fallout/magic/magicboltprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Constructor-like call on the local vector (`006815c0`, `this` = the local, returns `this`).
const INIT_LOCAL: u32 = 0x0068_15c0;
/// `bhkCharacterController::GetPosition_ov2` (Xbox PDB, `00c6e300`): writes the position
/// vector (16 bytes) through its one stack argument.
const GET_POSITION_OV2: u32 = 0x00c6_e300;
/// Point copy of `tes.cpp` (`00458620`, cdecl `(destination, source)`, three floats).
const COPY_POINT: u32 = 0x0045_8620;

// Translated from 00812b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00812b00 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkCharacterController::GetPosition` (Xbox PDB; the engine map files this address under
/// `magicboltprojectile.cpp`): `this` is the controller, `out` the one stack argument. Runs
/// `006815c0` on a local, has `GetPosition_ov2` (`00c6e300`) fill it, and copies its first
/// three floats to `out` with `00458620` (a vector copy that leaves the fourth word alone).
pub fn bhk_character_controller_get_position(e: &mut Engine, this: Ptr, out: Ptr) {
    e.with_stack(0x20, |e, local| {
        e.call(INIT_LOCAL, &args![local]);
        e.call(GET_POSITION_OV2, &args![this, local]);
        e.call(COPY_POINT, &args![out, local]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00812b00,
        bhk_character_controller_get_position(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_position_is_copied_through_a_local() {
        let mut e = Engine::new();
        e.register(INIT_LOCAL, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(GET_POSITION_OV2, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            Ret::default()
        });
        e.register(COPY_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        let controller = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 12, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0081_2b00, &args![controller, out]);
        let log = e.call_log.take().unwrap();
        let of = |address: u32| log.iter().filter(|c| c.0 == address).count();
        assert_eq!(of(INIT_LOCAL), 1);
        assert_eq!(of(GET_POSITION_OV2), 1);
        assert_eq!(of(COPY_POINT), 1);
        let position_call = log.iter().find(|c| c.0 == GET_POSITION_OV2).unwrap();
        assert_eq!(position_call.1[0], controller);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        assert_eq!(e.mem.u32(out + 12), 0x1234);
    }
}
