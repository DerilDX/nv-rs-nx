//! `NiXenonRenderer/nixenondirect3dtexture.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slots of `EnterCriticalSection` and `LeaveCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The critical section guarding the texture list (`011f4580`).
const TEXTURE_LIST_LOCK: u32 = 0x011f_4580;
/// The texture list's two end pointers (`011f4500`, `011f4504`).
const LIST_HEAD: u32 = 0x011f_4500;
const LIST_TAIL: u32 = 0x011f_4504;

// Translated from 00a5f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::~NiTexture` (engine map; `ECX`, no words): unlinks the texture
/// from the global texture list under its critical section: a head
/// (`011f4500`) on this texture moves to its `+0x2C` link, a tail
/// (`011f4504`) to its `+0x28` link, and the neighbours (`+0x28`'s `+0x2C`,
/// `+0x2C`'s `+0x28`) are joined.
pub fn ni_texture_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![TEXTURE_LIST_LOCK]);
    if e.global::<u32>(LIST_HEAD) == t {
        let next = e.mem.u32(t + 0x2c);
        e.set_global(LIST_HEAD, next);
    }
    if e.global::<u32>(LIST_TAIL) == t {
        let previous = e.mem.u32(t + 0x28);
        e.set_global(LIST_TAIL, previous);
    }
    let previous = e.mem.u32(t + 0x28);
    if previous != 0 {
        let next = e.mem.u32(t + 0x2c);
        e.mem.set_u32(previous + 0x2c, next);
    }
    let next = e.mem.u32(t + 0x2c);
    if next != 0 {
        let previous = e.mem.u32(t + 0x28);
        e.mem.set_u32(next + 0x28, previous);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![TEXTURE_LIST_LOCK]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_f800, ni_texture_destructor(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_unlinks_the_texture_under_the_lock() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.map(LIST_HEAD, 8);
        let [a, b, c] = [e.mem.alloc(0x30), e.mem.alloc(0x30), e.mem.alloc(0x30)];
        e.mem.set_u32(a + 0x2c, b);
        e.mem.set_u32(b + 0x28, a);
        e.mem.set_u32(b + 0x2c, c);
        e.mem.set_u32(c + 0x28, b);
        e.set_global(LIST_HEAD, a);
        e.set_global(LIST_TAIL, c);
        e.call_log = Some(vec![]);
        // The middle one: its neighbours are joined.
        e.call(0x00a5_f800, &args![b]);
        assert_eq!(e.mem.u32(a + 0x2c), c);
        assert_eq!(e.mem.u32(c + 0x28), a);
        // The head and the tail move.
        e.call(0x00a5_f800, &args![a]);
        assert_eq!(e.global::<u32>(LIST_HEAD), c);
        assert_eq!(e.mem.u32(c + 0x28), 0);
        e.call(0x00a5_f800, &args![c]);
        assert_eq!(e.global::<u32>(LIST_HEAD), 0);
        assert_eq!(e.global::<u32>(LIST_TAIL), 0);
        let log = e.call_log.take().unwrap();
        let locks = log.iter().filter(|c| c.0 == ENTER_CRITICAL_SECTION).count();
        assert_eq!(locks, 3);
        assert!(log.contains(&(LEAVE_CRITICAL_SECTION, vec![TEXTURE_LIST_LOCK])));
    }
}
