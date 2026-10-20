//! `NiXenonRenderer/nid3dshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

/// The five words `fn_00e81100` stores at `0126f738`.
const SHADER_FUNCTIONS: [u32; 5] = [
    0x00e8_0860,
    0x00e8_0a50,
    0x00e8_0c50,
    0x00e8_0de0,
    0x00e8_0f70,
];

// Translated from 00e81100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShader::_SDMInit` (Xbox PDB; run, by a jump, at the end of the start-up
/// `00e91dc0`; no arguments): sets the static fixed string at `0126f750` from the literal
/// at `010ef37c` when it does not hold the literal's address already (releasing the old
/// handle), then stores five function addresses at `0126f738` to `0126f748`.
pub fn ni_d3d_shader_sdm_init(e: &mut Engine) {
    let old = e.mem.u32(0x0126_f750);
    if old != 0x010e_f37c {
        let handle = e.call(GET_FIXED_STRING, &args![0x010e_f37cu32]).u32();
        e.mem.set_u32(0x0126_f750, handle);
        release_fixed_string(e, old);
    }
    for (index, function) in SHADER_FUNCTIONS.into_iter().enumerate() {
        e.mem.set_u32(0x0126_f738 + 4 * index as u32, function);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_1100, ni_d3d_shader_sdm_init())]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_the_string_once_and_the_five_functions() {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x1000);
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(0x0126_f750, empty);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1100, &args![]);
        assert_eq!(e.mem.u32(0x0126_f750), 0x010e_f37c + 0x1000);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x010e_f37c]]);
        for (index, function) in SHADER_FUNCTIONS.into_iter().enumerate() {
            assert_eq!(e.mem.u32(0x0126_f738 + 4 * index as u32), function);
        }
        // The word already equals the literal's address: no lookup.
        e.mem.set_u32(0x0126_f750, 0x010e_f37c);
        e.call(0x00e8_1100, &args![]);
        assert_eq!(calls_to(&e, GET_FIXED_STRING).len(), 1);
    }
}
