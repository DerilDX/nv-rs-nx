//! `BSShader/shadowscenenode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00cb9ce0` (unnamed in the engine map): `thiscall`, `RET 8` (new capacity, element count);
/// reallocates the array's buffer.
const ARRAY_REALLOCATE: u32 = 0x00cb_9ce0;
/// The virtual slot (byte offset) that allocates a block of the requested number of elements.
const SLOT_ALLOCATE: u32 = 0x04;

// Translated from 00b5c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowscenenode.cpp`). `__fastcall`: reserves the next slot of a
/// `BSSimpleArray`-shaped array (vtable, buffer at +4, count at +8, capacity at +0xc) and answers
/// its index. When the count equals the capacity: with no capacity yet, a four-element buffer
/// comes from virtual slot +4; otherwise the capacity doubles (grows by `0x400` once it is above
/// `0x400`) through `00cb9ce0`. This is the same body as the generic slot reservation
/// `00761540`, with the capacity helper inlined.
pub fn fn_00b5c360(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let size = e.mem.u32(base.wrapping_add(8));
    let capacity = e.mem.u32(base.wrapping_add(0xc));
    if size == capacity {
        if capacity == 0 {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![4u32]).u32();
            let count = e.mem.u32(base.wrapping_add(8)).wrapping_add(1);
            e.mem.set_u32(base.wrapping_add(8), count);
            e.mem.set_u32(base.wrapping_add(4), buffer);
            e.mem.set_u32(base.wrapping_add(0xc), 4);
            return count.wrapping_sub(1);
        }
        let new_capacity = if capacity > 0x400 {
            capacity.wrapping_add(0x400)
        } else {
            capacity.wrapping_mul(2)
        };
        e.call(ARRAY_REALLOCATE, &args![this, new_capacity, size]);
        e.mem.set_u32(base.wrapping_add(0xc), new_capacity);
    }
    let count = e.mem.u32(base.wrapping_add(8)).wrapping_add(1);
    e.mem.set_u32(base.wrapping_add(8), count);
    count.wrapping_sub(1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b5_c360, fn_00b5c360(Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.put_vtable(0x0f00_d100, &[0, 0x0f00_d000]);
        e.register(0x0f00_d000, |_, a| (0x4000_0000 + a[1]).into_ret());
        e.register(ARRAY_REALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0f00_d100);
        (e, array)
    }

    fn set_counts(e: &mut Engine, array: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array + 4, 0x5000);
        e.mem.set_u32(array + 8, size);
        e.mem.set_u32(array + 0xc, capacity);
    }

    #[test]
    fn at_00b5c360_allocates_four_elements_for_an_empty_array() {
        let (mut e, array) = rig();
        assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), 0);
        assert_eq!(calls(&e, 0x0f00_d000), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 0x4000_0004);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 4);
    }

    #[test]
    fn at_00b5c360_doubles_a_full_array_up_to_0x400_then_grows_by_0x400() {
        for (capacity, grown) in [
            (4u32, 8u32),
            (0x200, 0x400),
            (0x400, 0x800),
            (0x401, 0x801),
            (0x800, 0xc00),
        ] {
            let (mut e, array) = rig();
            set_counts(&mut e, array, capacity, capacity);
            assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), capacity);
            assert_eq!(
                calls(&e, ARRAY_REALLOCATE),
                vec![vec![array, grown, capacity]]
            );
            assert_eq!(e.mem.u32(array + 0xc), grown);
            assert_eq!(e.mem.u32(array + 8), capacity + 1);
            assert_eq!(e.mem.u32(array + 4), 0x5000);
        }
    }

    #[test]
    fn at_00b5c360_just_counts_when_there_is_room() {
        let (mut e, array) = rig();
        set_counts(&mut e, array, 3, 8);
        assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), 3);
        assert_eq!(e.mem.u32(array + 8), 4);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert!(calls(&e, ARRAY_REALLOCATE).is_empty());
        assert!(calls(&e, 0x0f00_d000).is_empty());
    }
}
