//! `fallout/ai/dialoguepackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `DialoguePackage` fields on PC (the Xbox PDB's offsets less 0x10:
/// `TESPackage` is 0x80 bytes on PC): `handle` (`BSSoundHandle`) +0x80,
/// `ptopic` +0x8C, `flistentimer` +0x90, `pTalkingActor` +0x94,
/// `bWaitingforLipFile` +0x98, `pGenericSecondLoc` +0x9C, `bUseIdleManager`
/// +0xA0, `pConversation` +0xA4, `pCurrentItem` +0xA8, `pCurrentResponse`
/// +0xAC, `pActor` +0xB0, `pTargetActor` +0xB4, `pTalkingActivator` +0xB8,
/// `bActorHeadTrack` +0xBC, `bCreatedPack` +0xBD, `bContinueMoving` +0xBE,
/// `bTargetContinueMoving` +0xBF, `bTargetMoveTowardStarter` +0xC0,
/// `bDoNotControlTargetMovement` +0xC1, `bReachedAmbushLocation` +0xC2,
/// `iEmotion` +0xC4, `iEmotionValue` +0xC8, `bSoundFile` +0xCC.
const HANDLE: u32 = 0x80;
const TOPIC: u32 = 0x8c;
const LISTEN_TIMER: u32 = 0x90;
const TALKING_ACTOR: u32 = 0x94;
const WAITING_FOR_LIP_FILE: u32 = 0x98;
const GENERIC_SECOND_LOCATION: u32 = 0x9c;
const USE_IDLE_MANAGER: u32 = 0xa0;
const CONVERSATION: u32 = 0xa4;
const CURRENT_ITEM: u32 = 0xa8;
const CURRENT_RESPONSE: u32 = 0xac;
const ACTOR: u32 = 0xb0;
const TARGET_ACTOR: u32 = 0xb4;
const TALKING_ACTIVATOR: u32 = 0xb8;
const ACTOR_HEAD_TRACK: u32 = 0xbc;
const CREATED_PACK: u32 = 0xbd;
const CONTINUE_MOVING: u32 = 0xbe;
const TARGET_CONTINUE_MOVING: u32 = 0xbf;
const TARGET_MOVE_TOWARD_STARTER: u32 = 0xc0;
const DO_NOT_CONTROL_TARGET_MOVEMENT: u32 = 0xc1;
const REACHED_AMBUSH_LOCATION: u32 = 0xc2;
const EMOTION: u32 = 0xc4;
const EMOTION_VALUE: u32 = 0xc8;
const SOUND_FILE: u32 = 0xcc;

/// `DialoguePackage`'s vtable (RTTI `.?AVDialoguePackage@@`).
const DIALOGUE_PACKAGE_VTABLE: u32 = 0x0109_2f6c;
/// `TESPackage::TESPackage` (`006707c0`, `thiscall`).
const PACKAGE_CONSTRUCTOR: u32 = 0x0067_07c0;
/// The sound handle's constructor (`0041a250`, `thiscall`; engine map unit
/// `extradatalist.cpp`, a folded body).
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
/// `Conversation::FirstItem` (`0083b9a0`, `thiscall`).
const CONVERSATION_FIRST_ITEM: u32 = 0x0083_b9a0;
/// The actor's vtable slot +0x200 the constructor calls with 0.
const ACTOR_SLOT_200: u32 = 0x200;
/// The player (`011dea3c`) and `PlayerCharacter::CheckForClosesConversation`
/// (`00967730`, `thiscall (player, actor, target, package)`).
const PLAYER: u32 = 0x011d_ea3c;
const CHECK_FOR_CLOSE_CONVERSATION: u32 = 0x0096_7730;
/// `TESPackage::SaveGame` (`006797c0`, one word),
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, `(buffer, form, 0)`),
/// the buffer's data save (`00865e50`, `(buffer, data, size, 0)`),
/// `Conversation::SaveGame` (`0083c270`, one word),
/// `Conversation::GetDialogueItemIndex` (`0083ba50`, one word) and
/// `DialogueItem::GetDialogueResponseIndex` (`0083c900`, one word), both
/// answering a 16-bit index.
const PACKAGE_SAVE_GAME: u32 = 0x0067_97c0;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_DATA: u32 = 0x0086_5e50;
const CONVERSATION_SAVE_GAME: u32 = 0x0083_c270;
const CONVERSATION_ITEM_INDEX: u32 = 0x0083_ba50;
const ITEM_RESPONSE_INDEX: u32 = 0x0083_c900;

fn field(this: Ptr, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

// Translated from 008d80e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage::GetTargetOfConversation` (Xbox PDB): `pTargetActor`.
pub fn dialogue_package_get_target_of_conversation(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(field(this, TARGET_ACTOR)))
}

// Translated from 009ee040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage::GetActorStartedConversation` (Xbox PDB): `pActor`.
pub fn dialogue_package_get_actor_started_conversation(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(field(this, ACTOR)))
}

// Translated from 009edd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009edd80 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DialoguePackage::DialoguePackage` (Xbox PDB): the `TESPackage`
/// constructor, the class's vtable, the sound handle's constructor, then
/// the conversation (with `Conversation::FirstItem` when there is one), no
/// item or response, listen timer 0, the actor and target (the actor's
/// vtable slot +0x200 with 0), no topic or talking actor, `bCreatedPack`
/// from its argument, the flags cleared but `bActorHeadTrack`, no generic
/// second location, the player's `CheckForClosesConversation` with actor,
/// target and the package, emotion 0 with value 50, the talking activator,
/// `bUseIdleManager` from its argument, and `bSoundFile` set. Answers
/// `this`. The exception-unwinding frame is not translated.
pub fn dialogue_package_dialogue_package(
    e: &mut Engine,
    this: Ptr,
    conversation: Ptr,
    actor: Ptr,
    target_actor: Ptr,
    talking_activator: Ptr,
    created_pack: u8,
    use_idle_manager: u8,
) -> Ptr {
    e.call(PACKAGE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), DIALOGUE_PACKAGE_VTABLE);
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![field(this, HANDLE)]);
    e.mem
        .set_u32(field(this, CONVERSATION), conversation.addr());
    if !conversation.is_null() {
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.call(CONVERSATION_FIRST_ITEM, &args![conversation]);
    }
    e.mem.set_u32(field(this, CURRENT_ITEM), 0);
    e.mem.set_u32(field(this, CURRENT_RESPONSE), 0);
    e.mem.set_u32(field(this, LISTEN_TIMER), x87_store32(0.0));
    e.mem.set_u32(field(this, ACTOR), actor.addr());
    e.mem
        .set_u32(field(this, TARGET_ACTOR), target_actor.addr());
    let started_by = e.mem.u32(field(this, ACTOR));
    e.vcall(started_by, ACTOR_SLOT_200, &args![0u32]);
    e.mem.set_u32(field(this, TOPIC), 0);
    e.mem.set_u32(field(this, TALKING_ACTOR), 0);
    e.mem.set_u8(field(this, CREATED_PACK), created_pack);
    e.mem.set_u8(field(this, WAITING_FOR_LIP_FILE), 0);
    e.mem.set_u8(field(this, CONTINUE_MOVING), 0);
    e.mem.set_u8(field(this, TARGET_CONTINUE_MOVING), 0);
    e.mem.set_u8(field(this, TARGET_MOVE_TOWARD_STARTER), 0);
    e.mem.set_u8(field(this, ACTOR_HEAD_TRACK), 1);
    e.mem.set_u8(field(this, DO_NOT_CONTROL_TARGET_MOVEMENT), 0);
    e.mem.set_u32(field(this, GENERIC_SECOND_LOCATION), 0);
    let player = e.mem.u32(PLAYER);
    e.call(
        CHECK_FOR_CLOSE_CONVERSATION,
        &args![player, actor, target_actor, this],
    );
    e.mem.set_u32(field(this, EMOTION), 0);
    e.mem.set_u32(field(this, EMOTION_VALUE), 0x32);
    e.mem
        .set_u32(field(this, TALKING_ACTIVATOR), talking_activator.addr());
    e.mem
        .set_u8(field(this, USE_IDLE_MANAGER), use_idle_manager);
    e.mem.set_u8(field(this, REACHED_AMBUSH_LOCATION), 0);
    e.mem.set_u8(field(this, SOUND_FILE), 1);
    this
}

// Translated from 009f01d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage::SaveGame` (Xbox PDB): `TESPackage::SaveGame`, then the
/// talking actor, actor, target, talking activator, generic second location
/// and topic as form IDs, the listen timer, the flags, emotion and its
/// value, `bUseIdleManager`, `bSoundFile` and `bReachedAmbushLocation` as
/// data, a byte saying whether there is a conversation, and with one the
/// conversation itself and the 16-bit indexes of the current item and
/// response (-1 when unset).
pub fn dialogue_package_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(PACKAGE_SAVE_GAME, &args![this, buffer]);
    for offset in [
        TALKING_ACTOR,
        ACTOR,
        TARGET_ACTOR,
        TALKING_ACTIVATOR,
        GENERIC_SECOND_LOCATION,
        TOPIC,
    ] {
        let form = e.mem.u32(field(this, offset));
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    for (offset, size) in [
        (LISTEN_TIMER, 4u32),
        (WAITING_FOR_LIP_FILE, 1),
        (ACTOR_HEAD_TRACK, 1),
        (CREATED_PACK, 1),
        (CONTINUE_MOVING, 1),
        (TARGET_CONTINUE_MOVING, 1),
        (TARGET_MOVE_TOWARD_STARTER, 1),
        (DO_NOT_CONTROL_TARGET_MOVEMENT, 1),
        (EMOTION, 4),
        (EMOTION_VALUE, 4),
        (USE_IDLE_MANAGER, 1),
        (SOUND_FILE, 1),
        (REACHED_AMBUSH_LOCATION, 1),
    ] {
        e.call(SAVE_DATA, &args![buffer, field(this, offset), size, 0u32]);
    }
    e.with_stack(0xc, |e, frame| {
        let has_conversation = frame.addr();
        let item_index = has_conversation + 4;
        let response_index = has_conversation + 8;
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.mem.set_u8(has_conversation, u8::from(conversation != 0));
        e.call(SAVE_DATA, &args![buffer, has_conversation, 1u32, 0u32]);
        if e.mem.u8(has_conversation) == 0 {
            return;
        }
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.call(CONVERSATION_SAVE_GAME, &args![conversation, buffer]);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let index = if item != 0 {
            let conversation = e.mem.u32(field(this, CONVERSATION));
            e.call(CONVERSATION_ITEM_INDEX, &args![conversation, item])
                .u16()
        } else {
            0xffff
        };
        e.mem.set_u16(item_index, index);
        e.call(SAVE_DATA, &args![buffer, item_index, 2u32, 0u32]);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let response = e.mem.u32(field(this, CURRENT_RESPONSE));
        let index = if item != 0 && response != 0 {
            e.call(ITEM_RESPONSE_INDEX, &args![item, response]).u16()
        } else {
            0xffff
        };
        e.mem.set_u16(response_index, index);
        e.call(SAVE_DATA, &args![buffer, response_index, 2u32, 0u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d80e0, dialogue_package_get_target_of_conversation(Ptr) -> Ptr),
        entry!(0x009ee040, dialogue_package_get_actor_started_conversation(Ptr) -> Ptr),
        entry!(
            0x009edd80,
            dialogue_package_dialogue_package(Ptr, Ptr, Ptr, Ptr, Ptr, u8, u8) -> Ptr
        ),
        entry!(0x009f01d0, dialogue_package_save_game(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn getters() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0xb0, 0x1111);
        e.mem.set_u32(this + 0xb4, 0x2222);
        assert_eq!(e.call(0x008d80e0, &args![this]).u32(), 0x2222);
        assert_eq!(e.call(0x009ee040, &args![this]).u32(), 0x1111);
    }

    #[test]
    fn constructor_sets_the_package_up() {
        const ACTOR_DOUBLE: u32 = 0x7200_0200;
        for with_conversation in [true, false] {
            let mut e = Engine::new();
            e.register(PACKAGE_CONSTRUCTOR, |_, _| Ret::default());
            e.register(SOUND_HANDLE_CONSTRUCTOR, |_, _| Ret::default());
            e.register(CONVERSATION_FIRST_ITEM, |_, _| Ret::default());
            e.register(CHECK_FOR_CLOSE_CONVERSATION, |_, _| Ret::default());
            e.register(ACTOR_DOUBLE, |_, _| Ret::default());
            e.put_vtable(0x7000_0000, &[0; 0x204 / 4]);
            e.mem.set_u32(0x7000_0000 + 0x200, ACTOR_DOUBLE);
            e.map(0x011d_e000, 0x1000);
            e.mem.set_u32(PLAYER, 0x9999);
            let actor = e.mem.alloc(4);
            e.mem.set_u32(actor, 0x7000_0000);
            let this = e.mem.alloc(0xd0);
            let conversation = if with_conversation { 0x5555u32 } else { 0 };
            e.call_log = Some(vec![]);
            let r = e
                .call(
                    0x009edd80,
                    &args![
                        this,
                        conversation,
                        actor,
                        0x3333u32,
                        0x4444u32,
                        1u32,
                        0x101u32
                    ],
                )
                .u32();
            assert_eq!(r, this);
            assert_eq!(e.mem.u32(this), DIALOGUE_PACKAGE_VTABLE);
            assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCTOR), vec![vec![this + 0x80]]);
            assert_eq!(
                calls(&e, CONVERSATION_FIRST_ITEM).len(),
                usize::from(with_conversation)
            );
            assert_eq!(calls(&e, ACTOR_DOUBLE), vec![vec![actor, 0]]);
            assert_eq!(
                calls(&e, CHECK_FOR_CLOSE_CONVERSATION),
                vec![vec![0x9999, actor, 0x3333, this]]
            );
            assert_eq!(e.mem.u32(this + 0xa4), conversation);
            assert_eq!(e.mem.u32(this + 0xb4), 0x3333);
            assert_eq!(e.mem.u32(this + 0xb8), 0x4444);
            assert_eq!(e.mem.u8(this + 0xbd), 1);
            assert_eq!(e.mem.u8(this + 0xa0), 1);
            assert_eq!(e.mem.u8(this + 0xbc), 1);
            assert_eq!(e.mem.u8(this + 0xcc), 1);
            assert_eq!(e.mem.u32(this + 0xc8), 50);
        }
    }

    #[test]
    fn save_game_writes_everything_and_the_conversation_indexes() {
        for (conversation, item, response) in [(0u32, 0u32, 0u32), (0x50, 0x60, 0x70), (0x50, 0, 0)]
        {
            let mut e = Engine::new();
            e.register(PACKAGE_SAVE_GAME, |_, _| Ret::default());
            e.register(SAVE_FORM_ID, |_, _| Ret::default());
            e.map(0x7300_0000, 0x10);
            e.register(SAVE_DATA, |e, a| {
                if a[2] == 2 {
                    let n = e.mem.u32(0x7300_0000);
                    e.mem.set_u16(0x7300_0004 + 2 * n, e.mem.u16(a[1]));
                    e.mem.set_u32(0x7300_0000, n + 1);
                }
                Ret::default()
            });
            e.register(CONVERSATION_SAVE_GAME, |_, _| Ret::default());
            e.register(CONVERSATION_ITEM_INDEX, |_, _| 3u32.into_ret());
            e.register(ITEM_RESPONSE_INDEX, |_, _| 0xffff_0004u32.into_ret());
            let this = e.mem.alloc(0xd0);
            e.mem.set_u32(this + 0xa4, conversation);
            e.mem.set_u32(this + 0xa8, item);
            e.mem.set_u32(this + 0xac, response);
            e.mem.set_u32(this + 0x94, 0x1001);
            e.call_log = Some(vec![]);
            e.call(0x009f01d0, &args![this, 0x8000u32]);
            let forms = calls(&e, SAVE_FORM_ID);
            assert_eq!(forms.len(), 6);
            assert_eq!(forms[0], vec![0x8000, 0x1001, 0]);
            let data = calls(&e, SAVE_DATA);
            assert_eq!(data[0], vec![0x8000, this + 0x90, 4, 0]);
            if conversation == 0 {
                assert_eq!(data.len(), 14);
                assert!(calls(&e, CONVERSATION_SAVE_GAME).is_empty());
            } else {
                assert_eq!(data.len(), 16);
                assert_eq!(calls(&e, CONVERSATION_SAVE_GAME), vec![vec![0x50, 0x8000]]);
                let saved = (e.mem.u16(0x7300_0004), e.mem.u16(0x7300_0006));
                if item != 0 {
                    assert_eq!(saved, (3, 4));
                } else {
                    assert_eq!(saved, (0xffff, 0xffff));
                }
            }
        }
    }
}
