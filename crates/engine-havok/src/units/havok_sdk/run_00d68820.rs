//! `Havok SDK/run_00d68820` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d68830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d68910`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68830(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8910, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The base constructor of the contact point viewers (`00de29b0`, thiscall, two arguments: the
/// process contexts and a colour word) and the process id the inactive contact point viewer
/// answers (`01269008`).
const CONTACT_POINT_VIEWER_CONSTRUCT: u32 = 0x00de_29b0;
const INACTIVE_PROCESS_ID: u32 = 0x0126_9008;
/// The vtable pointers of the active contact point viewer (RTTI: `hkpActiveContactPointViewer`):
/// `+0`, `+8`, `+0x20` and `+0x28`.
const ACTIVE_VTABLE_VIEWER: u32 = 0x010d_203c;
const ACTIVE_VTABLE_PROCESS: u32 = 0x010d_201c;
const ACTIVE_VTABLE_FILTER: u32 = 0x010d_200c;
const ACTIVE_VTABLE_LISTENER: u32 = 0x010d_1ffc;

// Translated from 00d68820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01269008`: slot 1 of the vtable `010d1f90` (RTTI:
/// `hkpInactiveContactPointViewer`); `ECX` unused.
pub fn fn_00d68820(e: &mut Engine) -> u32 {
    e.global(INACTIVE_PROCESS_ID)
}

// Translated from 00d68840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the active contact point viewer (RTTI: `hkpActiveContactPointViewer`):
/// runs the contact point viewers' base constructor (`00de29b0`) with the process `contexts` and
/// the colour `0xff0000ff`, then stores the four vtable pointers (`+0`, `+8`, `+0x20`, `+0x28`).
/// Answers `this`.
pub fn fn_00d68840(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(
        CONTACT_POINT_VIEWER_CONSTRUCT,
        &args![this, contexts, 0xff00_00ffu32],
    );
    e.mem.set_u32(at, ACTIVE_VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), ACTIVE_VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), ACTIVE_VTABLE_FILTER);
    e.mem.set_u32(at.wrapping_add(0x28), ACTIVE_VTABLE_LISTENER);
    this
}

// Translated from 00d68880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers its one stack word plus `0x28` (`ECX` is the object, ignored; a virtual method of the
/// active contact point viewer's vtable at `010d2048`).
pub fn fn_00d68880(_e: &mut Engine, _this: Ptr, base: u32) -> u32 {
    base.wrapping_add(0x28)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_8820, fn_00d68820() -> u32),
        entry!(0x00d6_8830, fn_00d68830(Ptr, u32) -> u32),
        entry!(0x00d6_8840, fn_00d68840(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_8880, fn_00d68880(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d68830() {
        assert_eq!(check(0x00d6_8830, 0x00d6_8910, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod active_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn fn_00d68820_answers_the_registered_process_id() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.set_global(INACTIVE_PROCESS_ID, 0x4242u32);
        assert_eq!(e.call(0x00d6_8820, &args![0x99u32]).u32(), 0x4242);
    }

    #[test]
    fn fn_00d68840_runs_the_base_constructor_and_stores_the_vtables() {
        let mut e = Engine::new();
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(CONTACT_POINT_VIEWER_CONSTRUCT, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], 0xdead);
            e.mem.set_u32(a[0] + 0x28, 0xbeef);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00d6_8840, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 0x1234, 0xff00_00ff]]);
        assert_eq!(e.mem.u32(this), ACTIVE_VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), ACTIVE_VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), ACTIVE_VTABLE_FILTER);
        assert_eq!(e.mem.u32(this + 0x28), ACTIVE_VTABLE_LISTENER);
    }

    #[test]
    fn fn_00d68880_adds_0x28_to_its_argument() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d6_8880, &args![0x99u32, 0x1000u32]).u32(),
            0x1028
        );
    }
}
