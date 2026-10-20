//! `fallout/ai/falloutcharacterprogression.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The integer setting whose value (`0043d4d0`) is the level cap.
const MAX_LEVEL_SETTING: u32 = 0x011d_0c60;
/// `"UILevelUp"`.
const LEVEL_UP_SOUND: u32 = 0x0106_f354;

/// `00559450` (`thiscall`): the word at `this` (here: the address of the
/// ready-to-level-up byte).
const WORD_AT: u32 = 0x0055_9450;
/// `0087f9f0` (`thiscall` on the player): the player's level (`AX`).
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
/// An integer setting's value address (`0043d4d0`, `thiscall`).
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `BGSEntryPoint::HandleEntryPoint` (engine map), `cdecl`: entry point,
/// perk owner, address of the value.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `BSAudio::QInstance` (engine map), no stack words: the audio singleton.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `00ad7fa0` (`thiscall`, `RET 8`; the engine map names it
/// `BSAudio::Precache`): plays the named sound with the flags word.
const PLAY_SOUND: u32 = 0x00ad_7fa0;
/// `004e4430` (`cdecl`, one `float`): the experience after the game's
/// adjustment, in `ST0`.
const ADJUST_EXPERIENCE: u32 = 0x004e_4430;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0` (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// `GamePlayFormulas::GetRequiredExperiencePoints` (engine map), `cdecl`:
/// level.
const REQUIRED_EXPERIENCE: u32 = 0x0064_8b50;

// Translated from 008d51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharacterProgression::QReadyToLevelUp` (Xbox PDB): the byte at the
/// address `00559450` gives for `this`.
pub fn character_progression_q_ready_to_level_up(e: &mut Engine, this: Ptr) -> u8 {
    let address = e.call(WORD_AT, &args![this]).u32();
    e.mem.u8(address)
}

// Translated from 008d5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharacterProgression::RewardExperience` (Xbox PDB; `ECX` not read,
/// `RET 4`). Below the level cap (the player's level from `0087f9f0` as a
/// 16-bit value, compared signed with the setting at `011d0c60`): the
/// experience as a float runs through entry point 9 (`005e58f0`), the
/// `"UILevelUp"` sound plays, the adjusted amount (`004e4430`, truncated)
/// is cut so that the player's experience (actor value 0x18, through the
/// value owner at +0xa4, slot +8) does not pass the experience the cap level
/// needs (`00648b50`), and the player's slot +0x3a8 adds it to actor value
/// 0x18.
pub fn character_progression_reward_experience(e: &mut Engine, _unused_0: Ptr, experience: i32) {
    let player = e.global::<u32>(PLAYER_POINTER);
    let level = e.call(PLAYER_LEVEL, &args![player]).u16();
    let cap_address = e.call(INT_SETTING_VALUE, &args![MAX_LEVEL_SETTING]).u32();
    if i32::from(level) >= e.mem.i32(cap_address) {
        return;
    }
    let amount_bits = x87_store32(f64::from(experience));
    let amount_bits = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), amount_bits);
        let player = e.global::<u32>(PLAYER_POINTER);
        e.call(HANDLE_ENTRY_POINT, &args![9u32, player, slot]);
        e.mem.u32(slot.addr())
    });
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    e.call(PLAY_SOUND, &args![audio, LEVEL_UP_SOUND, 0x121u32]);
    let adjusted = e
        .call(ADJUST_EXPERIENCE, &args![x87_quiet_bits(amount_bits)])
        .f64();
    let mut reward = e.call(FTOL, &args![adjusted]).i32();
    let player = e.global::<u32>(PLAYER_POINTER);
    let current = e.vcall(player.wrapping_add(0xa4), 8, &args![0x18u32]).i32();
    let total = current.wrapping_add(reward);
    let cap_address = e.call(INT_SETTING_VALUE, &args![MAX_LEVEL_SETTING]).u32();
    let cap = e.mem.u32(cap_address);
    let required = e.call(REQUIRED_EXPERIENCE, &args![cap]).i32();
    if total > required {
        reward = reward.wrapping_sub(total.wrapping_sub(required));
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    e.vcall(player, 0x3a8, &args![0x18u32, reward, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_51f0, character_progression_q_ready_to_level_up(Ptr) -> u8),
        entry!(
            0x008d_5100,
            character_progression_reward_experience(Ptr, i32)
        ),
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

    #[test]
    fn q_ready_to_level_up_reads_the_byte() {
        let mut e = Engine::new();
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(WORD_AT, move |_, _| word(flag));
        assert_eq!(e.call(0x008d_51f0, &args![0x5000u32]).u8(), 1);
    }

    /// The player with a value owner at +0xa4 whose slot +8 answers
    /// `current`, and its slot +0x3a8 recorded.
    fn setup(e: &mut Engine, level: u32, cap: u32, current: u32) {
        let player = e.mem.alloc(0x100);
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, player);
        e.put_vtable(0x0300_0000, &[0; 0xeb]);
        e.mem.set_u32(0x0300_0000 + 0x3a8, 0x0300_2000);
        e.mem.set_u32(player, 0x0300_0000);
        e.put_vtable(0x0300_1000, &[0, 0, 0x0300_3000]);
        e.mem.set_u32(player + 0xa4, 0x0300_1000);
        e.register(0x0300_2000, |_, _| Ret::default());
        e.register_double(0x0300_3000, move |_, _| word(current));
        e.register_double(PLAYER_LEVEL, move |_, _| word(0xffff_0000 | level));
        let cap_address = e.mem.alloc(4);
        e.mem.set_u32(cap_address, cap);
        e.register_double(INT_SETTING_VALUE, move |_, _| word(cap_address));
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            // The perk doubles the value.
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value * 2.0);
            Ret::default()
        });
        e.register(AUDIO_INSTANCE, |_, _| word(0x0400_0000));
        e.register(PLAY_SOUND, |_, _| Ret::default());
        e.register(ADJUST_EXPERIENCE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + 0.75,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(value as i32 as u32)
        });
        e.register(REQUIRED_EXPERIENCE, |_, a| word(a[0] * 1000));
    }

    #[test]
    fn reward_experience_adds_the_adjusted_amount() {
        let mut e = Engine::new();
        setup(&mut e, 3, 30, 100);
        e.call_log = Some(vec![]);
        e.call(0x008d_5100, &args![0u32, 25i32]);
        assert_eq!(
            calls_to(&e, PLAY_SOUND),
            vec![vec![0x0400_0000, LEVEL_UP_SOUND, 0x121]]
        );
        assert_eq!(calls_to(&e, HANDLE_ENTRY_POINT)[0][0], 9);
        assert_eq!(calls_to(&e, ADJUST_EXPERIENCE), vec![vec![50f32.to_bits()]]);
        let player = e.global::<u32>(PLAYER_POINTER);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![player, 0x18, 50, 0]]);
        assert_eq!(calls_to(&e, REQUIRED_EXPERIENCE), vec![vec![30]]);
    }

    #[test]
    fn reward_experience_stops_at_the_cap_experience() {
        let mut e = Engine::new();
        setup(&mut e, 3, 30, 29_980);
        e.call_log = Some(vec![]);
        e.call(0x008d_5100, &args![0u32, 25i32]);
        let player = e.global::<u32>(PLAYER_POINTER);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![player, 0x18, 20, 0]]);
    }

    #[test]
    fn reward_experience_does_nothing_at_the_level_cap() {
        let mut e = Engine::new();
        setup(&mut e, 30, 30, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_5100, &args![0u32, 25i32]);
        assert!(calls_to(&e, HANDLE_ENTRY_POINT).is_empty());
        assert!(calls_to(&e, 0x0300_2000).is_empty());
    }
}
