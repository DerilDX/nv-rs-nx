//! `(unplaced)/run_00ad6bb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsaudio::bsaudio::BSAudio;

/// `BSAudio`'s virtual table (RTTI `.?AVBSAudio@@`).
const BS_AUDIO_VTABLE: u32 = 0x010a_323c;
/// `BSAudio::SynchTimer` (Xbox PDB, a `BSAudioSynchTimer`) +0x14.
const BS_AUDIO_SYNCH_TIMER: u32 = 0x14;
/// `BSAudio::iMusicStartOffset` (Xbox PDB) +0x08, `bSeekMusic` +0x0c, `pfnIDCallback`
/// +0x20, `pfnNameCallback` +0x24, `pfnRandomFilenameCallback` +0x28, `pfnSettingsCallback`
/// +0x2c.
const BS_AUDIO_MUSIC_START_OFFSET: u32 = 0x08;
const BS_AUDIO_SEEK_MUSIC: u32 = 0x0c;
const BS_AUDIO_ID_CALLBACK: u32 = 0x20;
const BS_AUDIO_NAME_CALLBACK: u32 = 0x24;
const BS_AUDIO_RANDOM_FILENAME_CALLBACK: u32 = 0x28;
const BS_AUDIO_SETTINGS_CALLBACK: u32 = 0x2c;
/// The setting the constructor reads its multithread byte from (`011f6dfc`) and that
/// setting's value (`011f6e00`, the word after its virtual table).
const MULTITHREAD_SETTING: u32 = 0x011f_6dfc;
const MULTITHREAD_SETTING_VALUE: u32 = 0x011f_6e00;
/// A zero byte in `.data` the code falls back to when the setting is missing.
const FALLBACK_01202800: u32 = 0x0120_2800;

// Callees outside this unit.
/// `00aea0a0` and `00aea0d0` (`__thiscall` on the timer, `BSAudio` subsystem, unnamed): the
/// constructor and destructor bodies of the `BSAudioSynchTimer` at +0x14.
const SYNCH_TIMER_CONSTRUCTOR: u32 = 0x00ae_a0a0;
const SYNCH_TIMER_DESTRUCTOR: u32 = 0x00ae_a0d0;

// Translated from 00ad6f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio` constructor (unnamed in the map; it stores `BSAudio`'s virtual table): the
/// virtual table, the synch timer (`00aea0a0` on +0x14), then audio not enabled, the
/// multithread byte from the setting at `011f6dfc` (its value byte at `011f6e00`), not
/// initialized, and every other field zero. Answers `this`.
pub fn fn_00ad6f40(e: &mut Engine, this: Ptr<BSAudio>) -> Ptr<BSAudio> {
    let base = this.addr();
    e.mem.set_u32(base, BS_AUDIO_VTABLE);
    e.call(SYNCH_TIMER_CONSTRUCTOR, &args![base + BS_AUDIO_SYNCH_TIMER]);
    e.set(this, BSAudio::bAudioEnabled, 0);
    // The exe tests the setting's (constant, non-zero) address here, so the value is always
    // the setting's; the fallback to a cleared zero byte is kept as the exe has it.
    let setting = MULTITHREAD_SETTING;
    let value_at = if setting != 0 {
        MULTITHREAD_SETTING_VALUE
    } else {
        e.mem.set_u8(FALLBACK_01202800, 0);
        FALLBACK_01202800
    };
    let multithread = e.mem.u8(value_at);
    e.set(this, BSAudio::bMultiThread, multithread);
    e.set(this, BSAudio::bInitialized, 0);
    e.mem.set_u32(base + BS_AUDIO_MUSIC_START_OFFSET, 0);
    e.mem.set_u8(base + BS_AUDIO_SEEK_MUSIC, 0);
    e.mem.set_u32(base + BS_AUDIO_ID_CALLBACK, 0);
    e.mem.set_u32(base + BS_AUDIO_NAME_CALLBACK, 0);
    e.mem.set_u32(base + BS_AUDIO_RANDOM_FILENAME_CALLBACK, 0);
    e.mem.set_u32(base + BS_AUDIO_SETTINGS_CALLBACK, 0);
    e.set(this, BSAudio::pfnSynchPausedCallback, 0);
    e.set(this, BSAudio::pfnSynchUnPausedCallback, 0);
    e.set(this, BSAudio::pListener, Ptr::new(0));
    this
}

// Translated from 00ad7000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio` destructor body (unnamed in the map): `BSAudio`'s virtual table, then the synch
/// timer's destructor (`00aea0d0` on +0x14).
pub fn fn_00ad7000(e: &mut Engine, this: Ptr<BSAudio>) {
    e.mem.set_u32(this.addr(), BS_AUDIO_VTABLE);
    e.call(
        SYNCH_TIMER_DESTRUCTOR,
        &args![this.addr() + BS_AUDIO_SYNCH_TIMER],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ad6f40, fn_00ad6f40(Ptr<BSAudio>) -> Ptr<BSAudio>),
        entry!(0x00ad7000, fn_00ad7000(Ptr<BSAudio>)),
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

    #[test]
    fn fn_00ad6f40_builds_the_audio_object() {
        let mut e = Engine::new();
        e.register(SYNCH_TIMER_CONSTRUCTOR, |_, _| Ret::default());
        e.map(MULTITHREAD_SETTING, 8);
        e.mem.set_u8(MULTITHREAD_SETTING_VALUE, 1);
        let this = e.mem.alloc(0x38);
        for off in (0..0x38).step_by(4) {
            e.mem.set_u32(this + off, 0x5a5a_5a5a);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad6f40, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, SYNCH_TIMER_CONSTRUCTOR),
            vec![vec![this + 0x14]]
        );
        assert_eq!(e.mem.u32(this), BS_AUDIO_VTABLE);
        assert_eq!(
            (e.mem.u8(this + 4), e.mem.u8(this + 5), e.mem.u8(this + 6)),
            (0, 0, 1)
        );
        assert_eq!(e.mem.u8(this + 7), 0x5a);
        for off in [0x08, 0x10, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x0c), 0x5a5a_5a00);
        // The timer's words are left to its constructor.
        assert_eq!(e.mem.u32(this + 0x14), 0x5a5a_5a5a);
        e.mem.set_u8(MULTITHREAD_SETTING_VALUE, 0);
        e.call(0x00ad6f40, &args![this]);
        assert_eq!(e.mem.u8(this + 6), 0);
    }

    #[test]
    fn fn_00ad7000_destroys_the_timer() {
        let mut e = Engine::new();
        e.register(SYNCH_TIMER_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x00ad7000, &args![this]);
        assert_eq!(e.mem.u32(this), BS_AUDIO_VTABLE);
        assert_eq!(
            calls_to(&e, SYNCH_TIMER_DESTRUCTOR),
            vec![vec![this + 0x14]]
        );
    }
}
