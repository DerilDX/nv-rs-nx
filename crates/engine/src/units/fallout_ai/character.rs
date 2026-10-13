//! `fallout/ai/character.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_ai::actor::Actor;

/// `Actor::Actor` (`0087d4a0`): the base constructor, one stack word
/// (`add_to_process_lists`).
const ACTOR_CONSTRUCT: u32 = 0x0087_d4a0;
/// Writes `TESForm::cFormType` (`004f15a0`, one stack word).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// Sets the armour rating and threshold to -1.0 (`008d4e80`).
const RESET_ARMOUR: u32 = 0x008d_4e80;
/// The `float` constant -1.0 the code loads from the exe's data.
const MINUS_ONE: u32 = 0x0101_2054;

/// The vtables `Character` installs: its own (+0x00), the `TESChildCell`
/// part (+0x18), `MagicCaster` (+0x88), `MagicTarget` (+0x94),
/// `ActorValueOwner` (+0xa4) and `CachedValuesOwner` (+0xa8), Xbox PDB part
/// names at the PC offsets.
const VTABLES: [(u32, u32); 6] = [
    (0x00, 0x0108_6a6c),
    (0x18, 0x0108_6a60),
    (0x88, 0x0108_6a0c),
    (0x94, 0x0108_69d4),
    (0xa4, 0x0108_69a4),
    (0xa8, 0x0108_695c),
];
/// The form type of a `Character` (`ACHR`).
const FORM_TYPE_CHARACTER: u32 = 0x3b;

layout! {
    /// `Character` (Xbox PDB), 0x1c8 bytes on PC (the Xbox PDB's size);
    /// derived from [`Actor`], whose fields come first (use the `Actor`
    /// layout for them). Only the fields the constructor sets are listed.
    pub struct Character: 0x1c8 {
        /// `pBiped` (Xbox PDB): `BipedAnim*`.
        0x1B4 pBiped: Ptr,
        /// `fArmorRating` (Xbox PDB).
        0x1B8 fArmorRating: f32,
        /// `fArmorThreshold` (Xbox PDB).
        0x1BC fArmorThreshold: f32,
        /// `bTrespassing` (Xbox PDB).
        0x1C0 bTrespassing: bool,
        /// `bGuard` (Xbox PDB).
        0x1C1 bGuard: bool,
        /// `fequippedweight` (Xbox PDB).
        0x1C4 fequippedweight: f32,
    }
}

// Translated from 008d1d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Character::Character` (Xbox PDB): constructs the `Actor` base (with
/// `add_to_process_lists` 1), installs the six vtables, sets the form type to
/// 0x3b, clears `pBiped`, sets the armour rating and threshold to -1.0
/// (`008d4e80`), clears `bGuard`, sets `fequippedweight` to -1.0, clears
/// `bTrespassing` and the actor's crime fields (`bAttackOnNextTheft`,
/// `iThiefCrimeStamp`, `iMinorCrimes`, `iMajorCrimes`, `bIgnoreCrime`).
/// Returns `this`.
// Translation incomplete: 008d1d30 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn character_character(e: &mut Engine, this: Ptr<Character>) -> Ptr<Character> {
    e.call(ACTOR_CONSTRUCT, &args![this, 1u32]);
    for (offset, vtable) in VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_CHARACTER]);
    e.set(this, Character::pBiped, Ptr::NULL);
    e.call(RESET_ARMOUR, &args![this]);
    e.set(this, Character::bGuard, false);
    let minus_one: f32 = e.global(MINUS_ONE);
    e.set(this, Character::fequippedweight, minus_one);
    e.set(this, Character::bTrespassing, false);
    let actor = this.cast::<Actor>();
    e.set(actor, Actor::bAttackOnNextTheft, false);
    e.set(actor, Actor::iThiefCrimeStamp, 0);
    e.set(actor, Actor::iMinorCrimes, 0);
    e.set(actor, Actor::iMajorCrimes, 0);
    e.set(actor, Actor::bIgnoreCrime, false);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x008d1d30,
        character_character(Ptr<Character>) -> Ptr<Character>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_every_field_in_the_games_order() {
        let mut e = Engine::new();
        e.register(ACTOR_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(SET_FORM_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        // The armour reset is a real function of this crate (`008d4e80`);
        // double it to see that it is called and to keep the test local.
        e.register(RESET_ARMOUR, |e, a| {
            e.mem.set_f32(a[0] + 0x1b8, -1.0);
            e.mem.set_f32(a[0] + 0x1bc, -1.0);
            Ret::default()
        });
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);

        let character: Ptr<Character> = e.new_object();
        let actor = character.cast::<Actor>();
        // Dirty values the constructor must overwrite.
        e.set(character, Character::pBiped, Ptr::new(0x1234));
        e.set(character, Character::bGuard, true);
        e.set(character, Character::bTrespassing, true);
        e.set(character, Character::fequippedweight, 5.0);
        e.set(actor, Actor::bAttackOnNextTheft, true);
        e.set(actor, Actor::iThiefCrimeStamp, 9);
        e.set(actor, Actor::iMinorCrimes, 8);
        e.set(actor, Actor::iMajorCrimes, 7);
        e.set(actor, Actor::bIgnoreCrime, true);

        e.call_log = Some(vec![]);
        let back = e.call(0x008d_1d30, &args![character]).ptr::<Character>();
        assert_eq!(back, character);
        let base = character.addr();
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (ACTOR_CONSTRUCT, vec![base, 1]),
                (SET_FORM_TYPE, vec![base, 0x3b]),
                (RESET_ARMOUR, vec![base]),
            ]
        );
        for (offset, vtable) in VTABLES {
            assert_eq!(e.mem.u32(base + offset), vtable);
        }
        assert_eq!(e.mem.u8(base + 4), 0x3b);
        assert_eq!(e.get(character, Character::pBiped), Ptr::NULL);
        assert_eq!(e.get(character, Character::fArmorRating), -1.0);
        assert_eq!(e.get(character, Character::fArmorThreshold), -1.0);
        assert!(!e.get(character, Character::bGuard));
        assert!(!e.get(character, Character::bTrespassing));
        assert_eq!(e.get(character, Character::fequippedweight), -1.0);
        assert!(!e.get(actor, Actor::bAttackOnNextTheft));
        assert_eq!(e.get(actor, Actor::iThiefCrimeStamp), 0);
        assert_eq!(e.get(actor, Actor::iMinorCrimes), 0);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 0);
        assert!(!e.get(actor, Actor::bIgnoreCrime));
    }
}
