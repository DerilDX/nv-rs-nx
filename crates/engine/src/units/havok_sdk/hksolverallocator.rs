//! `Havok SDK/hksolverallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkSolverAllocator`'s virtual table (`010d1110`; the class name is confirmed
/// by the RTTI record before it).
const VTABLE: u32 = 0x010d_1110;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `thiscall`, spin count).
const CRITICAL_SECTION_CONSTRUCTOR: u32 = 0x00cf_6da0;

// Translated from 00d5e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::hkSolverAllocator` (Xbox PDB), `thiscall`, no stack
/// arguments: stores the virtual table, clears the words at `+4` to `+0x14`,
/// points `+0x18` at `this + 0x24`, clears `+0x1c`, stores `0x80000020` at `+0x20`
/// and constructs the critical section at `+0x124` with spin count 0. Returns
/// `this`.
pub fn hk_solver_allocator_hk_solver_allocator(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for offset in [0x04, 0x08, 0x0c, 0x10, 0x14] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x18, base + 0x24);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0x8000_0020);
    e.call(CRITICAL_SECTION_CONSTRUCTOR, &args![base + 0x124, 0u32]);
    this
}

/// `EnterCriticalSection` and `LeaveCriticalSection` (`kernel32` import slots,
/// `stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 00d5e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::setBuffer` (Xbox PDB), `thiscall`, `RET 8`: inside the
/// critical section at `+0x124`, points `+0x18` at `this + 0x24`, stores
/// `0x80000020` at `+0x20`, `buffer` at `+4` and `+0xc`, `buffer + size` at `+8` and
/// clears `+0x10`.
pub fn hk_solver_allocator_set_buffer(e: &mut Engine, this: Ptr, buffer: u32, size: u32) {
    let base = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![base + 0x124]);
    e.mem.set_u32(base + 0x18, base + 0x24);
    e.mem.set_u32(base + 0x20, 0x8000_0020);
    e.mem.set_u32(base + 4, buffer);
    e.mem.set_u32(base + 0xc, buffer);
    e.mem.set_u32(base + 8, buffer.wrapping_add(size));
    e.mem.set_u32(base + 0x10, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + 0x124]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5e900,
            hk_solver_allocator_hk_solver_allocator(Ptr) -> Ptr
        ),
        entry!(0x00d5e960, hk_solver_allocator_set_buffer(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_sets_the_fields_and_builds_the_critical_section() {
        let mut e = Engine::new();
        e.register(CRITICAL_SECTION_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let object = e.mem.alloc(0x130);
        e.mem.write(object, &[0xcd; 0x130]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d5_e900, &args![object]).u32(), object);
        let log = e.call_log.take().unwrap();
        let sections: Vec<_> = log
            .iter()
            .filter(|call| call.0 == CRITICAL_SECTION_CONSTRUCTOR)
            .collect();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].1, vec![object + 0x124, 0]);
        assert_eq!(e.mem.u32(object), VTABLE);
        for offset in [4, 8, 0xc, 0x10, 0x14, 0x1c] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.u32(object + 0x18), object + 0x24);
        assert_eq!(e.mem.u32(object + 0x20), 0x8000_0020);
        // Untouched: the bytes between the named words and the section.
        assert_eq!(e.mem.u32(object + 0x24), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x120), 0xcdcd_cdcd);
    }

    #[test]
    fn set_buffer_describes_the_buffer_inside_the_critical_section() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        let object = e.mem.alloc(0x130);
        e.mem.write(object, &[0xcd; 0x130]);
        e.call_log = Some(vec![]);
        e.call(0x00d5_e960, &args![object, 0x7000u32, 0x300u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![0x00d5_e960, ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]
        );
        assert_eq!(log[1].1, vec![object + 0x124]);
        assert_eq!(e.mem.u32(object + 4), 0x7000);
        assert_eq!(e.mem.u32(object + 0xc), 0x7000);
        assert_eq!(e.mem.u32(object + 8), 0x7300);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        assert_eq!(e.mem.u32(object + 0x18), object + 0x24);
        assert_eq!(e.mem.u32(object + 0x20), 0x8000_0020);
    }
}
