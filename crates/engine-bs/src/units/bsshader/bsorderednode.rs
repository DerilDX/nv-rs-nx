//! `BSShader/bsorderednode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNode::LoadBinary` (`00a5dd40`, `thiscall`, one word: the stream).
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `NiSphere::LoadBinary` (`00a69ac0`, `thiscall` on the sphere, one word).
const SPHERE_LOAD_BINARY: u32 = 0x00a6_9ac0;
/// The stream's `+4` word above which the flag byte is in the file (compared
/// unsigned with 0x17).
const LAST_VERSION_WITHOUT_FLAG: u32 = 0x17;

// Translated from 00bc2e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOrderedNode::LoadBinary` (Xbox PDB): the node part, the bound sphere at
/// `+0xAC`; then, when the stream's `+4` word is above 0x17, one byte read
/// into `+0xBC` from the stream's input file (`+0x24c`: its read function
/// `+8`, `cdecl (file, dest, 1, &count, 1)`, the result added to its
/// position `+4`; the count word is the argument slot), else that byte is 1.
pub fn bs_ordered_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
    e.call(SPHERE_LOAD_BINARY, &args![this.addr() + 0xac, stream]);
    if e.mem.u32(stream.addr() + 4) <= LAST_VERSION_WITHOUT_FLAG {
        e.mem.set_u8(this.addr() + 0xbc, 1);
        return;
    }
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let got = e
            .call(read, &args![file, this.addr() + 0xbc, 1u32, count, 1u32])
            .u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(got));
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_2e80, bs_ordered_node_load_binary(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_binary_reads_the_flag_only_from_newer_streams() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(NODE_LOAD_BINARY, |_, _| Ret::default());
        e.register(SPHERE_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 4, 0x17);
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2e80, &args![this, stream]);
        assert_eq!(e.mem.u8(this + 0xbc), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SPHERE_LOAD_BINARY, vec![this + 0xac, stream])));
        assert!(log.contains(&(NODE_LOAD_BINARY, vec![this, stream])));
        e.mem.set_u32(stream + 4, 0x18);
        e.call(0x00bc_2e80, &args![this, stream]);
        assert_eq!(e.mem.u8(this + 0xbc), 0);
        assert_eq!(e.mem.u32(file + 4), 1);
    }
}
