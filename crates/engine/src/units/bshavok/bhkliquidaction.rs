//! `BSHavok/bhkliquidaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa3e40`: thiscall on the pool object at `011f6238`, one word (the
/// size); returns the new block, or 0.
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// The pool object the allocation is requested from.
const POOL_011F6238: u32 = 0x011f_6238;

// Translated from 00cbc7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkliquidaction.obj`). Thiscall, one
/// word (`RET 4`): `this` is a head word plus a link at `+4`, and `source`
/// points at the word to put in. Nothing happens when `[source]` is 0.
/// When `[this]` is empty it just takes the word. Otherwise an 8-byte node
/// is allocated from the pool and the old head is pushed on the list
/// (`node = {old head, old [this + 4]}`, `[this + 4] = node`) before
/// `[this]` takes the new word. When the allocation fails the code writes
/// the link to address 4 (a null pointer plus 4), which faults in the game,
/// and so it does here; the rest of that path is kept after it.
pub fn fn_00cbc7b0(e: &mut Engine, this: Ptr, source: Ptr) {
    let mut word = e.mem.u32(source.addr());
    if word == 0 {
        return;
    }
    if e.mem.u32(this.addr()) != 0 {
        let node = e.call(POOL_ALLOCATE, &args![POOL_011F6238, 8u32]).u32();
        if node != 0 {
            let head = e.mem.u32(this.addr());
            e.mem.set_u32(node, head);
            e.mem.set_u32(node + 4, 0);
            let link = e.mem.u32(this.addr() + 4);
            e.mem.set_u32(node + 4, link);
            e.mem.set_u32(this.addr() + 4, node);
            let new_head = e.mem.u32(source.addr());
            e.mem.set_u32(this.addr(), new_head);
            return;
        }
        let link = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(4, link);
        e.mem.set_u32(this.addr() + 4, 0);
        word = e.mem.u32(source.addr());
    }
    e.mem.set_u32(this.addr(), word);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cbc7b0, fn_00cbc7b0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_00cbc7b0(e: &mut Engine, head: u32, link: u32, source: u32) -> (Ptr, Ptr) {
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, head);
        e.mem.set_u32(this + 4, link);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, source);
        (Ptr::<()>::new(this), Ptr::<()>::new(word))
    }

    #[test]
    fn fn_00cbc7b0_ignores_a_null_word_and_fills_an_empty_head() {
        let mut e = Engine::new();
        let (this, source) = list_00cbc7b0(&mut e, 0, 0x77, 0);
        e.call(0x00cb_c7b0, &args![this, source]);
        assert_eq!(e.mem.u32(this.addr()), 0);
        let (this, source) = list_00cbc7b0(&mut e, 0, 0x77, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c7b0, &args![this, source]);
        assert_eq!(e.mem.u32(this.addr()), 0x55);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x77);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(a, _)| *a != POOL_ALLOCATE));
    }

    #[test]
    fn fn_00cbc7b0_pushes_the_old_head_on_a_new_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(8);
        e.register_double(POOL_ALLOCATE, move |_, a| {
            assert_eq!(a, [POOL_011F6238, 8]);
            Ret {
                eax: node,
                ..Ret::default()
            }
        });
        let (this, source) = list_00cbc7b0(&mut e, 0x11, 0x77, 0x55);
        e.call(0x00cb_c7b0, &args![this, source]);
        assert_eq!(e.mem.u32(node), 0x11);
        assert_eq!(e.mem.u32(node + 4), 0x77);
        assert_eq!(e.mem.u32(this.addr()), 0x55);
        assert_eq!(e.mem.u32(this.addr() + 4), node);
    }
}
