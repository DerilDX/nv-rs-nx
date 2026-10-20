//! `NiMain/nisingleshadermateriallibrary.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, interlocked_increment, release_fixed_string, EMPTY_FIXED_STRING,
    GET_FIXED_STRING,
};

/// `011f4400`: a counted word `00a8fe70` increments for the object it sets up.
const LIBRARY_COUNT: u32 = 0x011f_4400;
/// `011f5e44`: the global array of pointers `00a8fe70` appends to: the data pointer, then
/// (`+4`) the capacity at `011f5e48` and (`+8`) the count at `011f5e4c`.
const LIBRARY_LIST: u32 = 0x011f_5e44;
/// `00a311e0` (`thiscall`, `RET 4`, engine map `NiTSet<...>::Realloc`): gives the array
/// the capacity given, keeping its entries.
const LIST_REALLOC: u32 = 0x00a3_11e0;

// Translated from 00a8fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterialLibrary::NiSingleShaderMaterialLibrary` (Xbox PDB; `thiscall`, no
/// stack word): makes the fixed-string handle for the literal at `0109f7d0`, runs
/// `fn_00a8fe70` with `this` and the address of that handle, releases the handle, clears
/// the words at `+0xc` and `+0x10`, sets the vtable `0109f7b4` and returns `this`.
pub fn ni_single_shader_material_library_ni_single_shader_material_library(
    e: &mut Engine,
    this: Ptr,
) -> Ptr {
    let name = e.call(GET_FIXED_STRING, &args![0x0109_f7d0u32]).u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), name);
        fn_00a8fe70(e, this, local);
        let handle = e.mem.u32(local.addr());
        release_fixed_string(e, handle);
    });
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr(), 0x0109_f7b4);
    this
}

// Translated from 00a8fe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nisingleshadermateriallibrary.cpp`; `thiscall`, `RET 4`):
/// sets the vtable `0101dce4` and the word at `+4` to 0, counts `011f4400`, sets the vtable
/// `0109f7f4`, starts the handle at `+8` as the empty string's and replaces it (counted, the
/// old one released) by the handle `name` points to when that differs, then appends `this`
/// to the global array at `011f5e44`, first growing the array through `00a311e0` to twice
/// its capacity (1 when the capacity is 0) when it is full. Returns `this`.
pub fn fn_00a8fe70(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_dce4);
    e.mem.set_u32(base + 4, 0);
    interlocked_increment(e, LIBRARY_COUNT);
    e.mem.set_u32(base, 0x0109_f7f4);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(base + 8, empty);
    let handle = e.mem.u32(name.addr());
    if empty != handle {
        acquire_fixed_string(e, handle);
        let current = e.mem.u32(base + 8);
        release_fixed_string(e, current);
        e.mem.set_u32(base + 8, handle);
    }
    let capacity = e.mem.u32(LIBRARY_LIST + 4);
    if e.mem.u32(LIBRARY_LIST + 8) == capacity {
        let grown = if capacity == 0 {
            1
        } else {
            capacity.wrapping_add(capacity)
        };
        e.call(LIST_REALLOC, &args![LIBRARY_LIST, grown]);
    }
    let data = e.mem.u32(LIBRARY_LIST);
    let count = e.mem.u32(LIBRARY_LIST + 8);
    e.mem
        .set_u32(data.wrapping_add(count.wrapping_mul(4)), base);
    e.mem.set_u32(LIBRARY_LIST + 8, count.wrapping_add(1));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_fce0, ni_single_shader_material_library_ni_single_shader_material_library(Ptr) -> Ptr),
        entry!(0x00a8_fe70, fn_00a8fe70(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    /// An engine whose array has room for `capacity` entries (`count` used); the realloc
    /// double doubles the data block and records the capacity asked for.
    fn engine(capacity: u32, count: u32) -> Engine {
        let mut e = engine_with_counting();
        let data = if capacity == 0 {
            0
        } else {
            e.mem.alloc(capacity * 4)
        };
        e.mem.set_u32(LIBRARY_LIST, data);
        e.mem.set_u32(LIBRARY_LIST + 4, capacity);
        e.mem.set_u32(LIBRARY_LIST + 8, count);
        e.register(LIST_REALLOC, |e, a| {
            assert_eq!(a[0], LIBRARY_LIST);
            let old = e.mem.u32(LIBRARY_LIST);
            let count = e.mem.u32(LIBRARY_LIST + 8);
            let new = e.mem.alloc(a[1] * 4);
            for index in 0..count {
                let entry = e.mem.u32(old + 4 * index);
                e.mem.set_u32(new + 4 * index, entry);
            }
            e.mem.set_u32(LIBRARY_LIST, new);
            e.mem.set_u32(LIBRARY_LIST + 4, a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn appends_the_library_to_the_list_with_room() {
        let mut e = engine(4, 1);
        e.mem.set_u32(LIBRARY_COUNT, 5);
        let this = e.mem.alloc(0x20);
        let name = e.mem.alloc(4);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(name, handle);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_fe70, &args![this, name]).u32(), this);
        assert!(calls_to(&e, LIST_REALLOC).is_empty());
        assert_eq!(e.mem.u32(this), 0x0109_f7f4);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), handle);
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.mem.u32(LIBRARY_COUNT), 6);
        let data = e.mem.u32(LIBRARY_LIST);
        assert_eq!(e.mem.u32(data + 4), this);
        assert_eq!(e.mem.u32(LIBRARY_LIST + 8), 2);
    }

    #[test]
    fn grows_a_full_list_to_twice_its_capacity_or_one() {
        for (capacity, grown) in [(0u32, 1u32), (2, 4)] {
            let mut e = engine(capacity, capacity);
            let this = e.mem.alloc(0x20);
            let name = e.mem.alloc(4);
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            e.mem.set_u32(name, empty);
            e.call_log = Some(vec![]);
            e.call(0x00a8_fe70, &args![this, name]);
            assert_eq!(calls_to(&e, LIST_REALLOC), vec![vec![LIBRARY_LIST, grown]]);
            // The empty handle is kept as it is.
            assert_eq!(e.mem.u32(this + 8), empty);
            let data = e.mem.u32(LIBRARY_LIST);
            assert_eq!(e.mem.u32(data + 4 * capacity), this);
            assert_eq!(e.mem.u32(LIBRARY_LIST + 8), capacity + 1);
        }
    }

    #[test]
    fn constructor_sets_up_with_the_string_handle_and_releases_it() {
        let mut e = engine(4, 0);
        let this = e.mem.alloc(0x20);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(handle - 8, 1);
        e.register_double(GET_FIXED_STRING, move |_, a| {
            assert_eq!(a, &[0x0109_f7d0]);
            handle.into_ret()
        });
        e.mem.set_u32(this + 0xc, 0x77);
        assert_eq!(e.call(0x00a8_fce0, &args![this]).u32(), this);
        // Counted once by the library at +8, once released by the constructor.
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.mem.u32(this + 8), handle);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this), 0x0109_f7b4);
        let data = e.mem.u32(LIBRARY_LIST);
        assert_eq!(e.mem.u32(data), this);
    }
}
