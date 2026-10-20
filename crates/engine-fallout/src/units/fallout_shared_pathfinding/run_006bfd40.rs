//! `fallout shared/pathfinding/run_006bfd40` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::BSSimpleArray;

/// The vtables of `BSSimpleArray<NavMeshObstacleVertex *, 1024>` and
/// `BSSimpleArray<NavMeshObstacleEdge *, 1024>` (their classes confirmed by the RTTI before
/// them), the array's `006b3eb0` (`thiscall`, two words; the destructors call it with 0, 0)
/// and its element fill `006bfdd0` (`thiscall`, `(buffer, count)`).
const OBSTACLE_VERTEX_ARRAY_VTABLE: u32 = 0x0106_c324;
const OBSTACLE_EDGE_ARRAY_VTABLE: u32 = 0x0106_c34c;
const ARRAY_RESET_006B3EB0: u32 = 0x006b_3eb0;
const ARRAY_FILL_006BFDD0: u32 = 0x006b_fdd0;

// Translated from 006bfe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` initialisation (unnamed in the map; `thiscall`, `(reserve, size)`):
/// buffer, size and reserve cleared; with a non-zero reserve (raised to `size` when that is
/// larger) the buffer comes from the array's virtual slot +4 (`(reserve)`) and the reserve
/// is kept; with a non-zero size the elements are filled (`006bfdd0`) and the size kept.
pub fn fn_006bfe70(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = if size > reserve { size } else { reserve };
    if reserve != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(ARRAY_FILL_006BFDD0, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 006bff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshObstacleVertex *, 1024>`'s destructor body (unnamed in the map):
/// the class's vtable, then `006b3eb0(0, 0)`. Answers `this`.
pub fn fn_006bff00(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), OBSTACLE_VERTEX_ARRAY_VTABLE);
    e.call(ARRAY_RESET_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006bff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshObstacleEdge *, 1024>`'s destructor body (unnamed in the map): the
/// class's vtable, then `006b3eb0(0, 0)`. Answers `this`.
pub fn fn_006bff30(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), OBSTACLE_EDGE_ARRAY_VTABLE);
    e.call(ARRAY_RESET_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006bfe70, fn_006bfe70(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x006bff00,
            fn_006bff00(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006bff30,
            fn_006bff30(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_006bfe70_reserves_and_fills() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0090_0004, |_, a| Ret {
            eax: 0x7000 + a[1],
            ..Ret::default()
        });
        e.register(ARRAY_FILL_006BFDD0, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0x0090_0004]);
        let this = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(this.addr(), 0x0090_1000);
        e.set(this, BSSimpleArray::iSize, 9);
        e.call(0x006bfe70, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert!(calls_at(&e, 0x0090_0004).is_empty());
        // The reserve is raised to the size.
        e.call(0x006bfe70, &args![this, 2u32, 5u32]);
        assert_eq!(calls_at(&e, 0x0090_0004), vec![vec![this.addr(), 5]]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x7005);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 5);
        assert_eq!(
            calls_at(&e, ARRAY_FILL_006BFDD0),
            vec![vec![this.addr(), 0x7005, 5]]
        );
        // Reserve only.
        e.call(0x006bfe70, &args![this, 8u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(calls_at(&e, ARRAY_FILL_006BFDD0).len(), 1);
    }

    #[test]
    fn obstacle_array_destructor_bodies_reset_the_array() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_RESET_006B3EB0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x006bff00, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OBSTACLE_VERTEX_ARRAY_VTABLE);
        assert_eq!(e.call(0x006bff30, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OBSTACLE_EDGE_ARRAY_VTABLE);
        assert_eq!(
            calls_at(&e, ARRAY_RESET_006B3EB0),
            vec![vec![this, 0, 0]; 2]
        );
    }
}
