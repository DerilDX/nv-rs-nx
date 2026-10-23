//! `NiXenonRenderer/run_00e8c0d0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unlock that pairs with the lock `00e8bff0` (unit `run_00e8bfa0`) on the object the
//! renderer keeps at +0x8BC.

#[allow(unused_imports)]
use crate::prelude::*;

/// The object's staging fields (unnamed; read from the code, as in `run_00e8bfa0`): the
/// staging block +0x40, the locked address +0x48, the locked size +0x4C and the critical
/// section +0x80 the lock entered.
const STAGING_BLOCK: u32 = 0x40;
const LOCKED_ADDRESS: u32 = 0x48;
const LOCKED_SIZE: u32 = 0x4c;
const LOCK_SECTION: u32 = 0x80;

/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `LeaveCriticalSection` (KERNEL32 import slot).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `IDirect3DVertexBuffer9::Unlock` (slot 12).
const VERTEX_BUFFER_UNLOCK: u32 = 0x30;

// Translated from 00e8c0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the staging block (+0x40) back to the locked address
/// (+0x48, size +0x4C), clears the locked address and size, leaves the critical section
/// (+0x80) and unlocks `buffer` (`Unlock`, slot +0x30). Answers whether the unlock succeeded
/// (a result of 0 or more).
pub fn fn_00e8c0d0(e: &mut Engine, this: Ptr, buffer: u32) -> bool {
    let base = this.addr();
    let size = e.mem.u32(base + LOCKED_SIZE);
    let block = e.mem.u32(base + STAGING_BLOCK);
    let locked = e.mem.u32(base + LOCKED_ADDRESS);
    e.call(MEMCPY, &args![locked, block, size]);
    e.mem.set_u32(base + LOCKED_ADDRESS, 0);
    e.mem.set_u32(base + LOCKED_SIZE, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + LOCK_SECTION]);
    e.vcall(buffer, VERTEX_BUFFER_UNLOCK, &[]).i32() >= 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_c0d0, fn_00e8c0d0(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const UNLOCK_FN: u32 = 0x0040_4000;

    #[test]
    fn unlock_copies_back_leaves_and_unlocks() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, a| a[0].into_ret());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(UNLOCK_FN, |_, _| Ret::default());
        let mut slots = vec![0u32; 13];
        slots[12] = UNLOCK_FN;
        e.put_vtable(0x0090_6000, &slots);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x0090_6000);
        let this: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(this.addr() + STAGING_BLOCK, 0x1111);
        e.mem.set_u32(this.addr() + LOCKED_ADDRESS, 0x2222);
        e.mem.set_u32(this.addr() + LOCKED_SIZE, 0x30);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_c0d0, &args![this, buffer]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![0x2222, 0x1111, 0x30]]);
        assert_eq!(
            calls_to(&log, LEAVE_CRITICAL_SECTION),
            vec![vec![this.addr() + LOCK_SECTION]]
        );
        assert_eq!(calls_to(&log, UNLOCK_FN), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(this.addr() + LOCKED_ADDRESS), 0);
        assert_eq!(e.mem.u32(this.addr() + LOCKED_SIZE), 0);
        // A failed unlock answers false.
        e.register(UNLOCK_FN, |_, _| 0x8876_086cu32.into_ret());
        assert!(!e.call(0x00e8_c0d0, &args![this, buffer]).bool());
    }
}
