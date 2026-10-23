//! `BSShader/run_00bc2cb0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00bc2bd0` (unit `run_00bc2bd0`, `ECX`): the constructor of the 0xC0-byte
/// node.
const NODE_CONSTRUCT: u32 = 0x00bc_2bd0;

// Translated from 00bc2cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc2cb0 - the FS:[0] exception frame (state -1, then 0 around the constructor) and its unwind handler 00f2c0f2 are not translated
/// A creation function (the `NiStream` loader table in `bsshadermanager.rs` lists
/// it for `BSOrderedNode`; `cdecl`, no words): 0xC0 bytes constructed by
/// `00bc2bd0`, or null when the allocation fails. The frame's cookie word
/// (`011c16bc ^ ESP`) is only stored for the handler, never checked.
pub fn fn_00bc2cb0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(NODE_CONSTRUCT, &args![block]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_2cb0, fn_00bc2cb0() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc2cb0_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(NODE_CONSTRUCT, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00bc_2cb0, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00bc_2cb0, &args![]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0xc0]));
        assert_eq!(log[2], (NODE_CONSTRUCT, vec![made - 1]));
    }
}
