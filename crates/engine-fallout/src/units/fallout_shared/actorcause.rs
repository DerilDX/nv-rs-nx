//! `fallout shared/actorcause.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ActorCause` (Xbox PDB), 0x1c bytes.
    pub struct ActorCause: 0x1c {
        /// `pActor` (Xbox PDB): `Actor*`.
        0x00 pActor: u32,
        /// `pWitnesses` (Xbox PDB): `BSSimpleList<DetectionState *>*`.
        0x04 pWitnesses: u32,
        /// `kOrigin` (Xbox PDB): an `NiPoint3`.
        0x08 kOrigin: u32,
        /// `iActorCauseID` (Xbox PDB).
        0x14 iActorCauseID: i32,
        /// `iRefCount` (Xbox PDB).
        0x18 iRefCount: i32,
    }
}

/// The word saved before the cause list (the Xbox PDB has the static
/// `ActorCause::iActorCauseIDCounter`; not tied to this address).
const CAUSE_ID_COUNTER: u32 = 0x011d_60cc;
/// The list of causes `0066e6d0` saves and `0066df40` adds to (the Xbox
/// PDB has the static `ActorCause::kCauseList`; not tied to this address).
const CAUSE_LIST: u32 = 0x011d_6124;
/// The global whose `0042ce10` test decides whether a cause without an id
/// is kept.
const GAME_STATE: u32 = 0x011d_df38;
/// The zero point (`NiPoint3`, three words).
const ZERO_POINT: u32 = 0x011f_426c;
/// `FLT_MAX` (`float`); the constructor stores its negation.
const FLOAT_MAX: u32 = 0x0106_73a0;

/// `008256d0` (`thiscall`): whether a list is empty (`AL`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`): a list's first item address (also run on a
/// point before it is set).
const LIST_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0` (`thiscall`): removes a list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `00726070` (`thiscall`): the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `004702f0` (`thiscall`, `RET 4`): the list's deleting destructor.
const LIST_DELETE: u32 = 0x0047_02f0;
/// `0042ce10` (`thiscall`, no stack words): a test on the global at
/// `011ddf38` (`AL`).
const GAME_STATE_TEST: u32 = 0x0042_ce10;
/// `00905330` (`thiscall`, `RET 4`): adds the item whose word is at the
/// given address to the list.
const LIST_ADD: u32 = 0x0090_5330;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `004019a0` (`cdecl`, the engine map's name `WaitForSingleObjectEx` is a
/// folded body): run on the reference count; 0 means the last reference
/// went.
const RELEASE_REFERENCE: u32 = 0x0040_19a0;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (engine map), `thiscall`: form, 0.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), `thiscall`.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map),
/// `thiscall`: count, handle.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// `DetectionState::SaveGame` (engine map), `thiscall`, `RET 4`.
const DETECTION_STATE_SAVE: u32 = 0x008d_7070;
/// `004300a0` (unit `run_00430000`): sets a float to 0.
const FLOAT_ZERO: u32 = 0x0043_00a0;
/// `00435de0` (`thiscall`, `RET 4`): stores the float argument at `this`.
const FLOAT_SET: u32 = 0x0043_5de0;

// Translated from 0066df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorCause` destructor body (no name in the engine map): clears the
/// actor, frees every witness (`00401030` on each item while the list is not
/// empty, removing the head with `0063f7b0`), deletes the list
/// (`004702f0` with 1) and clears it; then, unless `0042ce10` holds for the
/// global at `011ddf38` and the cause has no id (-1), hands the cause to
/// `00905330` on the cause list.
pub fn fn_0066df40(e: &mut Engine, this: Ptr<ActorCause>) {
    e.set(this, ActorCause::pActor, 0);
    loop {
        let witnesses = e.get(this, ActorCause::pWitnesses);
        if witnesses == 0 || e.call(LIST_IS_EMPTY, &args![witnesses]).bool() {
            break;
        }
        let witnesses = e.get(this, ActorCause::pWitnesses);
        let item_address = e.call(LIST_ITEM, &args![witnesses]).u32();
        let item = e.mem.u32(item_address);
        e.call(OPERATOR_DELETE, &args![item]);
        let witnesses = e.get(this, ActorCause::pWitnesses);
        e.call(LIST_REMOVE_HEAD, &args![witnesses]);
    }
    let witnesses = e.get(this, ActorCause::pWitnesses);
    if witnesses != 0 {
        e.call(LIST_DELETE, &args![witnesses, 1u32]);
    }
    e.set(this, ActorCause::pWitnesses, 0);
    let mut keep = true;
    let state = e.global::<u32>(GAME_STATE);
    if e.call(GAME_STATE_TEST, &args![state]).bool() && e.get(this, ActorCause::iActorCauseID) == -1
    {
        keep = false;
    }
    if keep {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), this.addr());
            e.call(LIST_ADD, &args![CAUSE_LIST, slot]);
        });
    }
}

// Translated from 0066e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a reference to the cause (no name in the engine map): when
/// `004019a0` on the reference count answers 0 and `this` is not null, the
/// cause deletes itself (`0066e300` with 1).
pub fn fn_0066e2b0(e: &mut Engine, this: Ptr<ActorCause>) {
    let count = this.addr().wrapping_add(0x18);
    if e.call(RELEASE_REFERENCE, &args![count]).u32() == 0 && !this.is_null() {
        fn_0066e300(e, this, 1);
    }
}

// Translated from 0066e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorCause` scalar deleting destructor (no name in the engine map):
/// the destructor body (`0066df40`), then `00401030` when `flags & 1`.
/// Returns `this`.
pub fn fn_0066e300(e: &mut Engine, this: Ptr<ActorCause>, flags: u32) -> Ptr<ActorCause> {
    fn_0066df40(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0066e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::SaveGame` (Xbox PDB): the origin (12 bytes), the id and the
/// actor's form id, then a variable-sized value holding each witness's
/// detection state (`008d7070`) and their number.
pub fn actor_cause_save_game(e: &mut Engine, this: Ptr<ActorCause>, buffer: u32) {
    let base = this.addr();
    e.call(SAVE_DATA, &args![buffer, base + 8, 0xcu32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, base + 0x14, 4u32, 0u32]);
    let actor = e.get(this, ActorCause::pActor);
    e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = e.get(this, ActorCause::pWitnesses);
    while node != 0 {
        let item_address = e.call(LIST_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            e.call(DETECTION_STATE_SAVE, &args![item, buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, handle]);
}

// Translated from 0066e590 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor of a 0x24-byte detection record (the engine map's name
/// `BSSimpleList<DetectionState_P>::AddHead` is a folded body; the class is
/// unconfirmed): `006815c0` on the point at +0xc, `004300a0` on the float at
/// +0x18, then the words at +0 and +4 cleared, byte +0x1e 0, byte +0x1f 1,
/// -100 at +8, the zero point at +0xc, `-FLT_MAX` (built by `00435de0` in a
/// local) at +0x18, bytes +0x1c and +0x1d 0 and -100 at +0x20. Returns
/// `this`.
pub fn fn_0066e590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(LIST_ITEM, &args![base + 0xc]);
    e.call(FLOAT_ZERO, &args![base + 0x18]);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u8(base + 0x1e, 0);
    e.mem.set_u8(base + 0x1f, 1);
    e.mem.set_u32(base + 8, 0xffff_ff9c);
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(base + 0xc + 4 * i, word);
    }
    let lowest = x87_quiet_bits(e.global::<u32>(FLOAT_MAX)) ^ 0x8000_0000;
    let stored = e.with_stack(4, |e, local| {
        e.call(FLOAT_SET, &args![local, lowest]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(base + 0x18, stored);
    e.mem.set_u8(base + 0x1c, 0);
    e.mem.set_u8(base + 0x1d, 0);
    e.mem.set_u32(base + 0x20, 0xffff_ff9c);
    this
}

// Translated from 0066e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::SaveActorCausesList` (Xbox PDB; `cdecl`, the buffer): the
/// word at `011d60cc`, then a variable-sized value holding each cause of
/// the list at `011d6124` (`0066e390`) and their number.
pub fn actor_cause_save_actor_causes_list(e: &mut Engine, buffer: u32) {
    e.call(SAVE_DATA, &args![buffer, CAUSE_ID_COUNTER, 4u32, 0u32]);
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = CAUSE_LIST;
    while node != 0 {
        let item_address = e.call(LIST_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            actor_cause_save_game(e, Ptr::new(item), buffer);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, handle]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_df40, fn_0066df40(Ptr<ActorCause>)),
        entry!(0x0066_e2b0, fn_0066e2b0(Ptr<ActorCause>)),
        entry!(
            0x0066_e300,
            fn_0066e300(Ptr<ActorCause>, u32) -> Ptr<ActorCause>
        ),
        entry!(0x0066_e390, actor_cause_save_game(Ptr<ActorCause>, u32)),
        entry!(0x0066_e590, fn_0066e590(Ptr) -> Ptr),
        entry!(0x0066_e6d0, actor_cause_save_actor_causes_list(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A witness list modelled as [count, first item, second item]; the
    /// head removal drops the first.
    fn witness_list(e: &mut Engine, items: &[u32]) -> u32 {
        let list = e.mem.alloc(4 + 4 * items.len() as u32);
        e.mem.set_u32(list, items.len() as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(list + 4 + 4 * i as u32, *item);
        }
        e.register(LIST_IS_EMPTY, |e, a| word(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_ITEM, |_, a| word(a[0] + 4));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let count = e.mem.u32(a[0]);
            for i in 1..count {
                let next = e.mem.u32(a[0] + 4 + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, next);
            }
            e.mem.set_u32(a[0], count - 1);
            Ret::default()
        });
        list
    }

    fn destructor_engine(state_test: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_d000, 0x1000);
        e.set_global(GAME_STATE, 0x4444u32);
        e.register(OPERATOR_DELETE, nothing);
        e.register(LIST_DELETE, nothing);
        e.register_double(GAME_STATE_TEST, move |_, _| word(u32::from(state_test)));
        e.register(LIST_ADD, |e, a| word(e.mem.u32(a[1])));
        e
    }

    #[test]
    fn destructor_body_frees_the_witnesses_and_returns_the_cause() {
        let mut e = destructor_engine(false);
        let list = witness_list(&mut e, &[0x71, 0x72]);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_u32(cause, 0x99);
        e.mem.set_u32(cause + 4, list);
        e.mem.set_i32(cause + 0x14, -1);
        e.call_log = Some(vec![]);
        e.call(0x0066_df40, &args![cause]);
        assert_eq!(e.mem.u32(cause), 0);
        assert_eq!(e.mem.u32(cause + 4), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x71], vec![0x72]]);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(calls_to(&e, GAME_STATE_TEST), vec![vec![0x4444]]);
        let adds = calls_to(&e, LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], CAUSE_LIST);
    }

    #[test]
    fn destructor_body_drops_a_cause_without_an_id_when_the_test_holds() {
        let mut e = destructor_engine(true);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_i32(cause + 0x14, -1);
        e.call_log = Some(vec![]);
        e.call(0x0066_df40, &args![cause]);
        assert!(calls_to(&e, LIST_DELETE).is_empty());
        assert!(calls_to(&e, LIST_ADD).is_empty());
        // With an id it is handed on.
        e.mem.set_i32(cause + 0x14, 5);
        e.call(0x0066_df40, &args![cause]);
        assert_eq!(calls_to(&e, LIST_ADD).len(), 1);
    }

    #[test]
    fn release_deletes_on_the_last_reference() {
        let mut e = destructor_engine(true);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_i32(cause + 0x14, -1);
        e.register(RELEASE_REFERENCE, |e, a| word(e.mem.u32(a[0])));
        e.mem.set_u32(cause + 0x18, 1);
        e.call_log = Some(vec![]);
        e.call(0x0066_e2b0, &args![cause]);
        assert_eq!(calls_to(&e, RELEASE_REFERENCE), vec![vec![cause + 0x18]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u32(cause + 0x18, 0);
        e.call(0x0066_e2b0, &args![cause]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![cause]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = destructor_engine(true);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_i32(cause + 0x14, -1);
        e.mem.set_u32(cause, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_e300, &args![cause, 0u32]).u32(), cause);
        assert_eq!(e.mem.u32(cause), 0);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0066_e300, &args![cause, 1u32]).u32(), cause);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![cause]]);
    }

    fn save_engine() -> Engine {
        let mut e = Engine::new();
        e.register(SAVE_DATA, nothing);
        e.register(SAVE_FORM_ID, nothing);
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| word(0x55));
        e.register(SAVE_VARIABLE_SIZED_VALUE, nothing);
        e.register(DETECTION_STATE_SAVE, nothing);
        // Nodes of two words: item, next.
        e.register(LIST_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e
    }

    #[test]
    fn save_game_saves_the_witnesses_that_exist() {
        let mut e = save_engine();
        let cause = e.mem.alloc(0x1c);
        e.mem.set_u32(cause, 0x99);
        let nodes = e.mem.alloc(0x18);
        e.mem.set_u32(nodes, 0x71);
        e.mem.set_u32(nodes + 4, nodes + 8);
        e.mem.set_u32(nodes + 8, 0);
        e.mem.set_u32(nodes + 0xc, nodes + 0x10);
        e.mem.set_u32(nodes + 0x10, 0x72);
        e.mem.set_u32(nodes + 0x14, 0);
        e.mem.set_u32(cause + 4, nodes);
        e.call_log = Some(vec![]);
        e.call(0x0066_e390, &args![cause, 0x9000u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![
                vec![0x9000, cause + 8, 0xc, 0],
                vec![0x9000, cause + 0x14, 4, 0]
            ]
        );
        assert_eq!(calls_to(&e, SAVE_FORM_ID), vec![vec![0x9000, 0x99, 0]]);
        assert_eq!(
            calls_to(&e, DETECTION_STATE_SAVE),
            vec![vec![0x71, 0x9000], vec![0x72, 0x9000]]
        );
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 2, 0x55]]
        );
    }

    #[test]
    fn save_actor_causes_list_saves_every_cause() {
        let mut e = save_engine();
        e.map(0x011d_6000, 0x1000);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_u32(CAUSE_LIST, cause);
        e.mem.set_u32(CAUSE_LIST + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_e6d0, &args![0x9000u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA)[0],
            vec![0x9000, CAUSE_ID_COUNTER, 4, 0]
        );
        assert_eq!(calls_to(&e, SAVE_FORM_ID).len(), 1);
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 0, 0x55], vec![0x9000, 1, 0x55]]
        );
    }

    #[test]
    fn detection_record_constructor_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0106_7000, 0x1000);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.map(0x011f_4000, 0x1000);
        e.set_global(ZERO_POINT, 2.0f32);
        e.register(LIST_ITEM, nothing);
        e.register(FLOAT_ZERO, nothing);
        e.register(FLOAT_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_u32(this + 4 * i, 0xcccc_cccc);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_e590, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, LIST_ITEM), vec![vec![this + 0xc]]);
        assert_eq!(calls_to(&e, FLOAT_ZERO), vec![vec![this + 0x18]]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.i32(this + 8), -100);
        assert_eq!(e.mem.f32(this + 0xc), 2.0);
        assert_eq!(e.mem.f32(this + 0x18), -f32::MAX);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0100_0000);
        assert_eq!(e.mem.i32(this + 0x20), -100);
    }
}
