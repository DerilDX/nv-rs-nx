//! `BSAudio/bsaudiomanager.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! # Where the unit stands
//!
//! The unit has 147 functions. Session 1 translated the first 60 open
//! functions in address order, `00ad9060` to `00adcd00`: the singleton
//! getter, constructor, destructor, initialization and shutdown, the audio
//! thread control, `Update`, `SynchVoiceStates`, `SetVoiceState`, the message
//! pool (`FreeMessage`, `FreeQueueMessage`), the message queuing function
//! `00adb1a0` and the thin wrappers around it (`Play`, `Stop`, `SetVolume`,
//! ...), and the sound-state queries (`IsValid`, `IsPlaying`, ...). The next
//! session continues at `00adcd80`.
//!
//! # Layout of the manager on PC
//!
//! `BSAudioManager` is a 0x188-byte singleton (the static object at
//! `011f6ef0`, built by `00ad9060`). Up to `CachedSoundList` (+0x94) its
//! offsets are the Xbox PDB's. The five critical sections are 0x18 bytes
//! here (0x28 in the PDB), so they sit at +0xac, +0xc4, +0xdc, +0xf4 and
//! +0x10c, `CacheLoadTaskList` is at +0x124 (PDB +0x174) and every later
//! field is 0x50 lower than in the PDB: `iDelayTimerDelta` at +0x130,
//! the volumes from +0x140, `iNextID` at +0x180.
//!
//! Message queuing: most public methods build a [`BSSoundMessage`] with the
//! type number, three words and (for a few) a smart pointer and three
//! floats, and hand it to `00adb1a0`, which takes a message from the
//! free-message stack, fills it and appends it to the current message list.
//! The methods that carry no extra data pass the three floats at
//! `011f426c` (a constant vector the audio code uses as its default
//! position) and a null smart pointer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTPointerList, NiTPointerMap};

/// The static `BSAudioManager` object `00ad9060` hands out.
const MANAGER_INSTANCE: u32 = 0x011f_6ef0;
/// Bit 0 of this word records that [`MANAGER_INSTANCE`] has been built (the
/// compiler's guard for the function-local static).
const MANAGER_BUILT_GUARD: u32 = 0x011f_7078;
/// The function `00ad9060` registers with `atexit` (destroys the manager).
const MANAGER_EXIT_THUNK: u32 = 0x00fd_bfd0;
/// `_atexit` (CRT, cdecl).
const ATEXIT: u32 = 0x00ec_658f;
/// The `BSAudio` object the audio code reads its flags from (see
/// `units/bsaudio/bsaudio.rs`): the byte at +5 says audio is initialized,
/// the byte at +6 says the audio runs on its own thread.
const AUDIO_GLOBAL: u32 = 0x011f_6d98;
/// The game's memory manager object: `ECX` of `Allocate` / `Deallocate`.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (platform): `(manager, size)`.
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate` (platform): `(manager, block)`.
const DEALLOCATE: u32 = 0x00aa_4060;
/// A free of a table (`00aa10f0`, cdecl, one argument), as the map
/// destructors do it.
const FREE_TABLE: u32 = 0x00aa_10f0;
/// The setting byte `Initialize` tests (audio enabled), `011f6da0`.
const AUDIO_ENABLED_SETTING: u32 = 0x011f_6da0;
/// The debug-information setting byte `SetVoiceState` tests
/// (`bUseAudioDebugInformation`, 1 byte).
const AUDIO_DEBUG_INFORMATION_SETTING: u32 = 0x011f_6e60;
/// The byte whose value `Shutdown` clears around its last cache flush.
const CACHE_FLUSH_FLAG: u32 = 0x011f_6ec0;
/// The three words of the default message vector.
const DEFAULT_MESSAGE_VECTOR: u32 = 0x011f_426c;
/// A double, 1000.0, by which seconds are scaled to milliseconds.
const MILLISECOND_SCALE: u32 = 0x0101_7b70;
/// The `fMasterVolumeMult` setting value (a float) that scales every volume.
const MASTER_VOLUME_MULT: u32 = 0x011f_6e78;
/// The TLS word at +0x2b4 of the thread block that the audio methods set to
/// 0xb while they run (see `units/bsaudio/bsaudio.rs`).
const AUDIO_TLS_CONTEXT: u32 = 0x2b4;
/// The value stored in [`AUDIO_TLS_CONTEXT`] while an audio method runs.
const AUDIO_CONTEXT_VALUE: u32 = 0xb;
/// The name `StartAudioThread` gives the audio thread (`"LibAudioUpdate"`).
const AUDIO_THREAD_NAME: u32 = 0x010a_33d8;
/// The `BSThread` processor setting word `StartAudioThread` hands to
/// `SetThreadProcessor` (`011f6ee4`).
const AUDIO_THREAD_PROCESSOR: u32 = 0x011f_6ee4;
/// The text `SetVoiceState` copies as the debug name when a sound has no
/// name (`"NotTESSound"`).
const NOT_TES_SOUND_TEXT: u32 = 0x010a_3478;
/// The double 3.4028235e+37 `SetVoiceState` compares distances against.
const HUGE_DISTANCE_LIMIT: u32 = 0x010a_3470;
/// The float written instead of a distance that exceeds
/// [`HUGE_DISTANCE_LIMIT`] (-1.0).
const HUGE_DISTANCE_REPLACEMENT: u32 = 0x0101_2054;

/// `EnterCriticalSection` import slot.
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection` import slot.
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `InitializeCriticalSection` import slot.
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `DeleteCriticalSection` import slot.
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `GetTickCount` import slot.
const IMPORT_GET_TICK_COUNT: u32 = 0x00fd_f060;
/// `InterlockedIncrement` import slot.
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` import slot.
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `WaitForSingleObject` import slot.
const IMPORT_WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;

/// `_eh_vector_constructor_iterator_` (CRT, cdecl: block, element size,
/// count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `_eh_vector_destructor_iterator_` (CRT, cdecl: block, element size,
/// count, destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The constructor of the `NiTPointerList<BSSoundMessage *>` elements.
const MESSAGE_LIST_CONSTRUCTOR: u32 = 0x00a1_b940;
/// The destructor of those list elements.
const MESSAGE_LIST_DESTRUCTOR: u32 = 0x00a1_bbd0;
/// `memset` (CRT, cdecl: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `strcpy_s` (CRT, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_ftol2`: truncates the float in `ST0` (the leading `f64` argument).
const FTOL2: u32 = 0x00ec_62c0;

/// Constructor of an `AudioMessageStack` (this at the stack).
const MESSAGE_STACK_CONSTRUCTOR: u32 = 0x00ae_7510;
/// Destructor of an `AudioMessageStack`: frees every message in it.
const MESSAGE_STACK_DESTRUCTOR: u32 = 0x00ae_7550;
/// `AudioMessageStack::Push` (Xbox PDB): `(stack, message)`.
const MESSAGE_STACK_PUSH: u32 = 0x00ae_75f0;
/// The pop of an `AudioMessageStack` (`00ae76b0`): the message on top, or a
/// new one when the stack is empty and bottomless.
const MESSAGE_STACK_POP: u32 = 0x00ae_76b0;
/// Constructor of the 0x4c-byte audio thread object
/// (`BSAudioManagerThread::BSAudioManagerThread`, Xbox PDB).
const THREAD_CONSTRUCTOR: u32 = 0x00ae_77d0;
/// `BSSemaphore::Signal` (Xbox PDB) on the audio thread's semaphore.
const THREAD_SEMAPHORE_SIGNAL: u32 = 0x00ae_78e0;
/// Constructor of a `BSSoundMessage` (`00ae8650`).
const MESSAGE_CONSTRUCTOR: u32 = 0x00ae_8650;
/// `BSSoundMessage::~BSSoundMessage` (Xbox PDB).
const MESSAGE_DESTRUCTOR: u32 = 0x00ae_86d0;
/// The copy-assignment of a `BSSoundMessage` (`00ae7990`).
const MESSAGE_ASSIGN: u32 = 0x00ae_7990;
/// `BSSoundInfo::BSSoundInfo` (Xbox PDB).
const SOUND_INFO_CONSTRUCTOR: u32 = 0x00ae_89f0;
/// `BSSoundInfo::BSSoundInfo` overload 2 (Xbox PDB): copy-constructs from a
/// `BSSoundInfo`.
const SOUND_INFO_COPY_CONSTRUCTOR: u32 = 0x00ae_8a40;
/// `BSSoundInfo::~BSSoundInfo` (Xbox PDB).
const SOUND_INFO_DESTRUCTOR: u32 = 0x00ae_8af0;
/// `BSSoundInfo::operator=` (Xbox PDB).
const SOUND_INFO_ASSIGN: u32 = 0x00ae_7920;
/// `BSDebugSoundInfo::BSDebugSoundInfo` (Xbox PDB).
const DEBUG_SOUND_INFO_CONSTRUCTOR: u32 = 0x00ae_8770;
/// `BSDebugSoundInfo::~BSDebugSoundInfo` (Xbox PDB).
const DEBUG_SOUND_INFO_DESTRUCTOR: u32 = 0x00ae_8830;
/// `BSDebugSoundInfo::operator=` (Xbox PDB).
const DEBUG_SOUND_INFO_ASSIGN: u32 = 0x00ae_88c0;
/// The append of a pointer, passed by address, to a message list: the
/// engine map names this `BSAudioManager::GetFromCache`, but the body
/// adds a node to the list and stores the pointer in it.
const LIST_APPEND_POINTER: u32 = 0x00ae_8bf0;
/// `NiTPointerListBase<..>::RemoveHead` (Xbox PDB name, for the message
/// lists): removes the first node and returns its pointer.
const LIST_REMOVE_HEAD: u32 = 0x00ae_7f90;
/// Removes every node of a pointer-map (`00ae91f0`).
const MAP_CLEAR: u32 = 0x00ae_91f0;
/// `NiTPointerList` destructor-like clear of a list (`00ae8370`).
const LIST_CLEAR: u32 = 0x00ae_8370;
/// Constructors of the `NiTPointerMap` instances, `(map, bucket count)`.
const MAP_CONSTRUCTOR_SOUND_LIST: u32 = 0x00ae_7b40;
const MAP_CONSTRUCTOR_SOUND_STATES: u32 = 0x00ae_7bc0;
const MAP_CONSTRUCTOR_MOVING_REFERENCES: u32 = 0x00ae_7c40;
/// The pointer-map operations the manager's four maps use; all take the map
/// as `this`.
/// `GetNext`: `(map, position slot, key out, value out)`.
const MAP_GET_NEXT: u32 = 0x00a0_c850;
/// `GetAt`: `(map, key, value out)`, true when the key is present.
const MAP_GET_VALUE: u32 = 0x00a1_e380;
/// `RemoveKey`: `(map, key)`, true when something was removed.
const MAP_REMOVE: u32 = 0x00a1_e240;
/// `SetAt`: `(map, key, value)`.
const MAP_SET_VALUE: u32 = 0x00a0_c6c0;

/// `NiPointer<..>::operator=` (`006e5cc0`): `(destination, source slot)`.
const SMART_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Destructor of a smart-pointer temporary (`0045cec0`).
const SMART_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `(0066b0d0)`: sets a smart pointer slot to a pointer, `(slot, pointer)`.
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// The function (`00c45830`, cdecl, one argument) `FreeMessage` calls with
/// the pointer a message was holding.
const RELEASE_MESSAGE_POINTER: u32 = 0x00c4_5830;
/// `BSThread::Initialize` (Xbox PDB): `(thread, stack size, name, 0)`.
const THREAD_INITIALIZE: u32 = 0x00aa_6430;
/// `BSThread::SetName` (Xbox PDB).
const THREAD_SET_NAME: u32 = 0x00aa_6540;
/// `BSThread::SetThreadProcessor` (Xbox PDB).
const THREAD_SET_PROCESSOR: u32 = 0x00aa_6550;
/// `SetThreadName` (Xbox PDB, cdecl): `(thread id, name)`.
const SET_THREAD_NAME: u32 = 0x00aa_2740;
/// The call `StopAudioThread` makes on the audio thread once it has stopped
/// (`00aa64f0`, unnamed in the map).
const THREAD_FINISH: u32 = 0x00aa_64f0;
/// `BSAudio::Precache` (Xbox PDB): `(BSAudio object, sound name, flags)`.
const AUDIO_PRECACHE: u32 = 0x00ad_7fa0;
/// The manager's `Process` (`00addc00`, Xbox PDB `BSAudioManager::Process`).
const MANAGER_PROCESS: u32 = 0x00ad_dc00;
/// The manager function `Shutdown` calls with 0 while the cache-flush flag
/// is cleared (`00adcd80`, still to be translated).
const MANAGER_CACHE_FLUSH: u32 = 0x00ad_cd80;
/// Tests the sound's flag the debug fill needs (`00ae9cf0`, bool).
const SOUND_IS_EMITTER_SOUND: u32 = 0x00ae_9cf0;
/// Second test of the same condition (`00ae9d10`, bool).
const SOUND_HAS_EMITTER: u32 = 0x00ae_9d10;

// Virtual slots of a sound (`BSGameSound`, Xbox PDB vtable offsets).
/// `BSGameSound::IsPlaying`.
const SOUND_SLOT_IS_PLAYING: u32 = 0x04;
/// `BSGameSound::GetVolume`.
const SOUND_SLOT_GET_VOLUME: u32 = 0x10;
/// `BSGameSound::GetEmitterPosition`.
const SOUND_SLOT_GET_EMITTER_POSITION: u32 = 0x54;
// Fields of a sound (`BSGameSound`, Xbox PDB names, offsets match on PC).
const SOUND_ID: u32 = 0x04;
const SOUND_TYPE_FLAGS: u32 = 0x08;
const SOUND_DURATION: u32 = 0x14;
const SOUND_STATIC_ATTENUATION: u32 = 0x18;
const SOUND_SYSTEM_ATTENUATION: u32 = 0x1c;
const SOUND_DISTANCE_ATTENUATION: u32 = 0x1e;
const SOUND_FILENAME: u32 = 0x36;
const SOUND_MAX_DISTANCE: u32 = 0x13c;
const SOUND_MIN_DISTANCE: u32 = 0x140;
const SOUND_DISTANCE_TO_LISTENER: u32 = 0x150;

layout! {
    /// A Win32 `CRITICAL_SECTION` (24 bytes), as the manager embeds it.
    pub struct BSCriticalSection: 0x18 {}

    /// `AudioMessageStack` (Xbox PDB), 0x10 bytes: a stack of free
    /// messages linked through their `pData` word.
    pub struct AudioMessageStack: 0x10 {
        /// `iCount` (Xbox PDB).
        0x00 iCount: u32,
        /// `bBottomless` (Xbox PDB): a pop on an empty stack makes a new
        /// message.
        0x04 bBottomless: bool,
        /// `pTop` (Xbox PDB).
        0x08 pTop: u32,
        /// `iTimeLastAllocated` (Xbox PDB), a `GetTickCount` value.
        0x0C iTimeLastAllocated: u32,
    }

    /// `BSSoundMessage` (Xbox PDB), 0x20 bytes.
    pub struct BSSoundMessage: 0x20 {
        /// `msg` (Xbox PDB): the message type.
        0x00 msg: u32,
        /// `iID` (Xbox PDB): the sound id.
        0x04 iID: u32,
        /// `iData` (Xbox PDB).
        0x08 iData: u32,
        /// `pData` (Xbox PDB), also the link of a free message.
        0x0C pData: u32,
        /// `spData` (Xbox PDB): a smart pointer to a reference-counted
        /// object (an `NiAVObject` for the moving-sound messages).
        0x10 spData: u32,
        /// `vector3[0]` (Xbox PDB).
        0x14 vector3_x: f32,
        /// `vector3[1]` (Xbox PDB).
        0x18 vector3_y: f32,
        /// `vector3[2]` (Xbox PDB).
        0x1C vector3_z: f32,
        /// `curve[0]` (Xbox PDB `u16[10]` overlaying the vector).
        0x14 curve_0: u16,
        /// `curve[1]`.
        0x16 curve_1: u16,
        /// `curve[2]`.
        0x18 curve_2: u16,
        /// `curve[3]`.
        0x1A curve_3: u16,
        /// `curve[4]`.
        0x1C curve_4: u16,
    }

    /// `BSDebugSoundInfo` (Xbox PDB), 0x23c bytes. The PC code fills only
    /// the fields below.
    pub struct BSDebugSoundInfo: 0x23C {
        /// `pFileName` (Xbox PDB): points at the name buffer that follows.
        0x108 pFileName: u32,
        /// `FileName` (Xbox PDB), a 260-byte buffer.
        0x10C FileName: u8,
        /// `fRadiusMin` (Xbox PDB), the sound's minimum distance.
        0x210 fRadiusMin: f32,
        /// `fRadiusMax` (Xbox PDB), the sound's maximum distance.
        0x214 fRadiusMax: f32,
        /// First coordinate of `Location` (Xbox PDB), the emitter x.
        0x218 locationX: f32,
        /// Second coordinate of `Location`, the emitter y.
        0x21C locationY: f32,
        /// The word the PDB calls the third coordinate of `Location`; the
        /// PC code stores the sound's distance to the listener here.
        0x220 distanceToListener: f32,
        /// `iStaticAttenuation` (Xbox PDB).
        0x230 iStaticAttenuation: u32,
        /// `iSystemAttenuation` (Xbox PDB).
        0x234 iSystemAttenuation: u32,
        /// `iDistanceAttenuation` (Xbox PDB).
        0x238 iDistanceAttenuation: u32,
    }

    /// `BSSoundInfo` (Xbox PDB), 0x254 bytes: the manager's record of a
    /// sound for the voice-state sync, the debug record at +0x18.
    pub struct BSSoundInfo: 0x254 {
        /// `iID` (Xbox PDB).
        0x00 iID: u32,
        /// `fFrequency` (Xbox PDB).
        0x04 fFrequency: f32,
        /// `fVolume` (Xbox PDB).
        0x08 fVolume: f32,
        /// `uiSoundType` (Xbox PDB).
        0x0C uiSoundType: u32,
        /// `uiDuration` (Xbox PDB).
        0x10 uiDuration: u32,
        /// `bIsPlaying` (Xbox PDB).
        0x14 bIsPlaying: u8,
        /// `pDebugInfo` (Xbox PDB).
        0x18 pDebugInfo: Inline<BSDebugSoundInfo>,
    }

    /// `BSAudioManager` (Xbox PDB), 0x188 bytes on PC.
    pub struct BSAudioManager: 0x188 {
        /// The vtable (`010a3368`, the scalar deleting destructor).
        0x00 vtable: u32,
        /// `iMessageListIndex` (Xbox PDB): the list new messages join.
        0x04 iMessageListIndex: u32,
        /// `iMessageProcessingListIndex` (Xbox PDB).
        0x08 iMessageProcessingListIndex: u32,
        /// `Messages` (Xbox PDB): two lists of 0xc bytes.
        0x0C Messages: Inline<NiTPointerList>,
        /// `FreeMessages` (Xbox PDB).
        0x24 FreeMessages: Inline<AudioMessageStack>,
        /// `FreeQueueMessages` (Xbox PDB).
        0x34 FreeQueueMessages: Inline<AudioMessageStack>,
        /// `GarbageMessages` (Xbox PDB).
        0x44 GarbageMessages: Inline<AudioMessageStack>,
        /// `SoundList` (Xbox PDB): sound id to sound.
        0x54 SoundList: Inline<NiTPointerMap>,
        /// `SoundStateMap` (Xbox PDB): sound id to `BSSoundInfo`.
        0x64 SoundStateMap: Inline<NiTPointerMap>,
        /// `StateUpdateMap` (Xbox PDB): pending `BSSoundInfo` updates.
        0x74 StateUpdateMap: Inline<NiTPointerMap>,
        /// `MovingRefrList` (Xbox PDB).
        0x84 MovingRefrList: Inline<NiTPointerMap>,
        /// `CachedSoundList` (Xbox PDB).
        0x94 CachedSoundList: Inline<NiTPointerList>,
        /// `iCurrentCachSize` (Xbox PDB).
        0xA0 iCurrentCachSize: u32,
        /// `iWeatherFlags` (Xbox PDB).
        0xA4 iWeatherFlags: u32,
        /// `fCurrentGamehour` (Xbox PDB).
        0xA8 fCurrentGamehour: f32,
        /// `MessageCritSection` (Xbox PDB).
        0xAC MessageCritSection: Inline<BSCriticalSection>,
        /// `StateCritSection` (Xbox PDB).
        0xC4 StateCritSection: Inline<BSCriticalSection>,
        /// `CacheCritSection` (Xbox PDB).
        0xDC CacheCritSection: Inline<BSCriticalSection>,
        /// `ProcessingCritSection` (Xbox PDB).
        0xF4 ProcessingCritSection: Inline<BSCriticalSection>,
        /// `CacheLoadTaskCritSection` (Xbox PDB).
        0x10C CacheLoadTaskCritSection: Inline<BSCriticalSection>,
        /// `CacheLoadTaskList` (Xbox PDB).
        0x124 CacheLoadTaskList: Inline<NiTPointerList>,
        /// `iDelayTimerDelta` (Xbox PDB, 0x180 there): the constructor
        /// stores `GetTickCount` here.
        0x130 iDelayTimerDelta: u32,
        /// `bPrecacheCompleted` (Xbox PDB): set once `Update` has
        /// precached the interface sounds.
        0x134 bPrecacheCompleted: bool,
        /// `bListenerMoved` (Xbox PDB).
        0x135 bListenerMoved: bool,
        /// `dwAudioThreadID` (Xbox PDB).
        0x138 dwAudioThreadID: u32,
        /// `pUpdateThread` (Xbox PDB): the `BSAudioManagerThread`.
        0x13C pUpdateThread: u32,
        /// `fMasterVolume` (Xbox PDB).
        0x140 fMasterVolume: f32,
        /// `fFootVolume` (Xbox PDB).
        0x144 fFootVolume: f32,
        /// `fVoiceVolume` (Xbox PDB).
        0x148 fVoiceVolume: f32,
        /// `fEffectsVolume` (Xbox PDB).
        0x14C fEffectsVolume: f32,
        /// `fMusicVolume` (Xbox PDB).
        0x150 fMusicVolume: f32,
        /// `fRadioVolume` (Xbox PDB).
        0x154 fRadioVolume: f32,
        /// `fMasterSecondaryVolume` (Xbox PDB).
        0x158 fMasterSecondaryVolume: f32,
        /// `fFootSecondaryVolume` (Xbox PDB).
        0x15C fFootSecondaryVolume: f32,
        /// `fVoiceSecondaryVolume` (Xbox PDB).
        0x160 fVoiceSecondaryVolume: f32,
        /// `fEffectsSecondaryVolume` (Xbox PDB).
        0x164 fEffectsSecondaryVolume: f32,
        /// `fMusicSecondaryVolume` (Xbox PDB).
        0x168 fMusicSecondaryVolume: f32,
        /// `fRadioSecondaryVolume` (Xbox PDB).
        0x16C fRadioSecondaryVolume: f32,
        /// `iMuteMask` (Xbox PDB).
        0x170 iMuteMask: u32,
        /// `bDialogueFade` (Xbox PDB).
        0x174 bDialogueFade: bool,
        /// `bListenerUnderwater` (Xbox PDB).
        0x175 bListenerUnderwater: bool,
        /// `pPauseEntries` (Xbox PDB).
        0x178 pPauseEntries: u32,
        /// `iPauseListSize` (Xbox PDB).
        0x17C iPauseListSize: u32,
        /// `iNextID` (Xbox PDB).
        0x180 iNextID: u32,
        /// `bIgnoreGlobalTimeMultiplier` (Xbox PDB).
        0x184 bIgnoreGlobalTimeMultiplier: bool,
    }
}

/// Enters the critical section at `section`.
fn enter_section(e: &mut Engine, section: u32) {
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &[section]);
}

/// Leaves the critical section at `section`.
fn leave_section(e: &mut Engine, section: u32) {
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[section]);
}

/// Adds a reference to the reference-counted object at `object` (the count
/// is at +4), as the inlined smart-pointer constructor does for a non-null
/// pointer.
fn add_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &[object + 4]);
    }
}

/// Drops a reference of the object at `object`; when the count reaches
/// zero calls its virtual method at +4 (the inlined smart-pointer
/// destructor).
fn drop_reference(e: &mut Engine, object: u32) {
    if object != 0 && e.call(IMPORT_INTERLOCKED_DECREMENT, &[object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// Destroys a manager-owned object: when `object` is not null runs its
/// destructor at `destructor` and then frees it (the scalar deleting
/// destructor the compiler inlined).
fn destroy_object(e: &mut Engine, destructor: u32, object: u32) {
    if object != 0 {
        e.call(destructor, &[object]);
        e.call(DEALLOCATE, &[MEMORY_MANAGER, object]);
    }
}

/// The inlined start of an `NiTPointerMap` walk: the first item of the
/// first non-empty bucket, or 0 for an empty map.
fn map_first_item(e: &Engine, map: Ptr<NiTPointerMap>) -> u32 {
    let buckets = e.get(map, NiTPointerMap::m_uiHashSize);
    let table = e.get(map, NiTPointerMap::m_ppkHashTable);
    for index in 0..buckets {
        let item = e.mem.u32(table.wrapping_add(index * 4));
        if item != 0 {
            return item;
        }
    }
    0
}

/// `GetNext` (`00a0c850`) on a map: returns the item's key and value and
/// moves `position` to the next item (0 at the end).
fn map_next(e: &mut Engine, map: Ptr<NiTPointerMap>, position: &mut u32) -> (u32, u32) {
    let current = *position;
    e.with_stack(12, |e, slots| {
        e.mem.set_u32(slots.addr(), current);
        e.call(
            MAP_GET_NEXT,
            &[map.addr(), slots.addr(), slots.addr() + 4, slots.addr() + 8],
        );
        *position = e.mem.u32(slots.addr());
        (e.mem.u32(slots.addr() + 4), e.mem.u32(slots.addr() + 8))
    })
}

/// `GetAt` (`00a1e380`) on a map: whether the key is present, and the value
/// slot afterwards (it keeps `initial` when the key is absent).
fn map_get(e: &mut Engine, map: Ptr<NiTPointerMap>, key: u32, initial: u32) -> (bool, u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), initial);
        let found = e
            .call(MAP_GET_VALUE, &[map.addr(), key, slot.addr()])
            .bool();
        (found, e.mem.u32(slot.addr()))
    })
}

/// Reads the three words at the default message vector.
fn default_message_vector(e: &Engine) -> (f32, f32, f32) {
    (
        e.global::<f32>(DEFAULT_MESSAGE_VECTOR),
        e.global::<f32>(DEFAULT_MESSAGE_VECTOR + 4),
        e.global::<f32>(DEFAULT_MESSAGE_VECTOR + 8),
    )
}

/// What most public methods do: queue a message of type `kind` with the
/// three words, a null smart pointer and the default vector.
fn queue_plain(e: &mut Engine, this: Ptr<BSAudioManager>, kind: u32, a: u32, b: u32, c: u32) {
    let (x, y, z) = default_message_vector(e);
    fn_00adb1a0(e, this, kind, a, b, c, 0, x, y, z);
}

/// The `float * 1000` then truncate that several wrappers do inline
/// (`FISTP` with truncation, the low word of the 64-bit result).
fn scaled_milliseconds_inline(e: &Engine, seconds: f32) -> u32 {
    let scale = e.mem.f64(MILLISECOND_SCALE);
    let scaled = seconds as f64 * scale;
    if scaled.is_nan() || scaled >= i64::MAX as f64 || scaled < i64::MIN as f64 {
        // FISTP stores the "integer indefinite" value 0x8000000000000000.
        return 0;
    }
    (scaled.trunc() as i64) as u32
}

/// The `float * 1000` converted through `_ftol2` (`00ec62c0`).
fn scaled_milliseconds_ftol(e: &mut Engine, seconds: f32) -> u32 {
    let scale = e.mem.f64(MILLISECOND_SCALE);
    let scaled = seconds as f64 * scale;
    e.call(FTOL2, &args![scaled]).u32()
}

/// Runs `body` with the audio TLS context word set to 0xb, restoring the
/// previous value afterwards (the prologue/epilogue of the audio methods
/// that save and set `+0x2b4` of the thread block).
fn in_audio_context<R>(e: &mut Engine, body: impl FnOnce(&mut Engine) -> R) -> R {
    let at = e.tls() + AUDIO_TLS_CONTEXT;
    let previous = e.mem.u32(at);
    e.mem.set_u32(at, AUDIO_CONTEXT_VALUE);
    let result = body(e);
    e.mem.set_u32(at, previous);
    result
}

// Translated from 00ad9060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::QInstance` (Xbox PDB): the manager singleton. The first
/// call runs the constructor on the static object and registers its
/// destruction with `atexit` (the compiler's function-local static guard in
/// the word at `011f7078`).
// Translation incomplete: 00ad9060 - C++ exception unwinding (SEH frame state) of the static initialization is not translated.
pub fn bs_audio_manager_q_instance(e: &mut Engine) -> Ptr<BSAudioManager> {
    let guard = e.mem.u32(MANAGER_BUILT_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(MANAGER_BUILT_GUARD, guard | 1);
        fn_00ad90e0(e, Ptr::new(MANAGER_INSTANCE));
        e.call(ATEXIT, &[MANAGER_EXIT_THUNK]);
    }
    Ptr::new(MANAGER_INSTANCE)
}

// Translated from 00ad90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSAudioManager` constructor (unnamed in the engine map): sets the
/// vtable, constructs the two message lists, the three message stacks, the
/// four maps (37 buckets each) and the lists, initializes the critical
/// sections and zeroes the state; `iDelayTimerDelta` takes `GetTickCount`.
/// Returns `this`.
// Translation incomplete: 00ad90e0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00ad90e0(e: &mut Engine, this: Ptr<BSAudioManager>) -> Ptr<BSAudioManager> {
    let base = this.addr();
    e.mem.set_u32(base, 0x010a_3368);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &[
            base + 0x0c,
            0x0c,
            2,
            MESSAGE_LIST_CONSTRUCTOR,
            MESSAGE_LIST_DESTRUCTOR,
        ],
    );
    e.call(MESSAGE_STACK_CONSTRUCTOR, &[base + 0x24]);
    e.call(MESSAGE_STACK_CONSTRUCTOR, &[base + 0x34]);
    e.call(MESSAGE_STACK_CONSTRUCTOR, &[base + 0x44]);
    e.call(MAP_CONSTRUCTOR_SOUND_LIST, &[base + 0x54, 0x25]);
    e.mem.set_u32(base + 0x54, 0x010a_3348);
    e.call(MAP_CONSTRUCTOR_SOUND_STATES, &[base + 0x64, 0x25]);
    e.mem.set_u32(base + 0x64, 0x010a_3328);
    e.call(MAP_CONSTRUCTOR_SOUND_STATES, &[base + 0x74, 0x25]);
    e.mem.set_u32(base + 0x74, 0x010a_3328);
    e.call(MAP_CONSTRUCTOR_MOVING_REFERENCES, &[base + 0x84, 0x25]);
    e.mem.set_u32(base + 0x84, 0x010a_3308);
    // CachedSoundList: an empty NiTPointerList.
    e.set(
        this.at(BSAudioManager::CachedSoundList),
        NiTPointerList::m_uiCount,
        0,
    );
    e.set(
        this.at(BSAudioManager::CachedSoundList),
        NiTPointerList::m_pkHead,
        0,
    );
    e.set(
        this.at(BSAudioManager::CachedSoundList),
        NiTPointerList::m_pkTail,
        0,
    );
    for section in [0xac, 0xc4, 0xdc, 0xf4, 0x10c] {
        e.call(IMPORT_INITIALIZE_CRITICAL_SECTION, &[base + section]);
    }
    // CacheLoadTaskList: an empty NiTPointerList.
    e.set(
        this.at(BSAudioManager::CacheLoadTaskList),
        NiTPointerList::m_uiCount,
        0,
    );
    e.set(
        this.at(BSAudioManager::CacheLoadTaskList),
        NiTPointerList::m_pkHead,
        0,
    );
    e.set(
        this.at(BSAudioManager::CacheLoadTaskList),
        NiTPointerList::m_pkTail,
        0,
    );
    e.set(this, BSAudioManager::dwAudioThreadID, 0);
    for volume in [
        BSAudioManager::fMasterVolume,
        BSAudioManager::fFootVolume,
        BSAudioManager::fVoiceVolume,
        BSAudioManager::fEffectsVolume,
        BSAudioManager::fMusicVolume,
        BSAudioManager::fRadioVolume,
        BSAudioManager::fMasterSecondaryVolume,
        BSAudioManager::fFootSecondaryVolume,
        BSAudioManager::fVoiceSecondaryVolume,
        BSAudioManager::fEffectsSecondaryVolume,
        BSAudioManager::fMusicSecondaryVolume,
        BSAudioManager::fRadioSecondaryVolume,
    ] {
        e.set(this, volume, 0.0);
    }
    e.set(this, BSAudioManager::bPrecacheCompleted, false);
    e.set(this, BSAudioManager::iWeatherFlags, 0);
    e.set(this, BSAudioManager::iNextID, 1);
    e.set(this, BSAudioManager::iMessageListIndex, 0);
    e.set(this, BSAudioManager::iMessageProcessingListIndex, 1);
    e.set(this, BSAudioManager::iCurrentCachSize, 0);
    e.set(this, BSAudioManager::pUpdateThread, 0);
    e.set(this, BSAudioManager::iMuteMask, 0);
    e.set(this, BSAudioManager::bListenerMoved, false);
    e.set(this, BSAudioManager::iWeatherFlags, 0);
    e.set(this, BSAudioManager::fCurrentGamehour, 0.0);
    let ticks = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    e.set(this, BSAudioManager::iDelayTimerDelta, ticks);
    e.set(this, BSAudioManager::bDialogueFade, false);
    e.set(this, BSAudioManager::bListenerUnderwater, false);
    this
}

// Translated from 00ad93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSAudioManager` destructor body (unnamed in the engine map): resets
/// the vtable, empties the cache-load list and the cached-sound list,
/// deletes the critical sections, clears and frees the four maps (each map
/// sets its two base vtables on the way), empties the three message stacks
/// and destroys the two message lists.
// Translation incomplete: 00ad93f0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00ad93f0(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010a_3368);
    e.call(LIST_CLEAR, &[base + 0x124]);
    for section in [0x10c, 0xf4, 0xdc, 0xc4, 0xac] {
        e.call(IMPORT_DELETE_CRITICAL_SECTION, &[base + section]);
    }
    e.call(LIST_CLEAR, &[base + 0x94]);
    // Each map: the derived vtable, a clear, the base vtable, a clear and
    // the table freed.
    for (map, derived, base_vtable) in [
        (base + 0x84, 0x010a_3308, 0x010a_33b0),
        (base + 0x74, 0x010a_3328, 0x010a_3390),
        (base + 0x64, 0x010a_3328, 0x010a_3390),
        (base + 0x54, 0x010a_3348, 0x010a_3370),
    ] {
        e.mem.set_u32(map, derived);
        e.call(MAP_CLEAR, &[map]);
        e.mem.set_u32(map, base_vtable);
        e.call(MAP_CLEAR, &[map]);
        let table = e.mem.u32(map + 8);
        e.call(FREE_TABLE, &[table]);
    }
    e.call(MESSAGE_STACK_DESTRUCTOR, &[base + 0x44]);
    e.call(MESSAGE_STACK_DESTRUCTOR, &[base + 0x34]);
    e.call(MESSAGE_STACK_DESTRUCTOR, &[base + 0x24]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &[base + 0x0c, 0x0c, 2, MESSAGE_LIST_DESTRUCTOR],
    );
}

// Translated from 00ad9640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Initialize` (Xbox PDB): when the audio setting at
/// `011f6da0` is on, allocates and clears the 8-entry pause list, sets the
/// volumes (`00ad9da0`), fills the free-message stack and the free-queue
/// stack with 0x100 constructed messages each (the stacks then become
/// bottomless, the second one's third byte stays 0) and constructs the
/// audio thread object.
// Translation incomplete: 00ad9640 - C++ exception unwinding (SEH frame state) around the allocations is not translated.
pub fn bs_audio_manager_initialize(e: &mut Engine, this: Ptr<BSAudioManager>) {
    if e.mem.u8(AUDIO_ENABLED_SETTING) == 0 {
        return;
    }
    e.set(this, BSAudioManager::iWeatherFlags, 0);
    e.set(this, BSAudioManager::iPauseListSize, 8);
    // Size of the pause list: 8 entries of 8 bytes, saturating at 32 bits.
    let entries = e.get(this, BSAudioManager::iPauseListSize);
    let bytes = entries as u64 * 8;
    let size = if bytes >> 32 != 0 {
        u32::MAX
    } else {
        bytes as u32
    };
    let block = e.call(ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, BSAudioManager::pPauseEntries, block);
    let entries = e.get(this, BSAudioManager::iPauseListSize);
    e.call(MEMSET, &[block, 0, entries << 3]);
    fn_00ad9da0(e, this);
    for _ in 0..0x100 {
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x20]).u32();
        let message = if raw != 0 {
            e.call(MESSAGE_CONSTRUCTOR, &[raw]).u32()
        } else {
            0
        };
        e.call(MESSAGE_STACK_PUSH, &[this.addr() + 0x24, message]);
    }
    e.set(
        this.at(BSAudioManager::FreeMessages),
        AudioMessageStack::bBottomless,
        true,
    );
    for _ in 0..0x100 {
        let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x20]).u32();
        let message = if raw != 0 {
            e.call(MESSAGE_CONSTRUCTOR, &[raw]).u32()
        } else {
            0
        };
        e.call(MESSAGE_STACK_PUSH, &[this.addr() + 0x34, message]);
    }
    e.set(
        this.at(BSAudioManager::FreeQueueMessages),
        AudioMessageStack::bBottomless,
        true,
    );
    e.set(
        this.at(BSAudioManager::GarbageMessages),
        AudioMessageStack::bBottomless,
        false,
    );
    let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x4c]).u32();
    let thread = if raw != 0 {
        e.call(THREAD_CONSTRUCTOR, &[raw]).u32()
    } else {
        0
    };
    e.set(this, BSAudioManager::pUpdateThread, thread);
}

// Translated from 00ad9870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Shutdown` (Xbox PDB): runs `Update` three times when the
/// audio does not run on its own thread, empties both message lists
/// (destroying every message), deletes the audio thread object, removes
/// every sound from the sound list (setting its voice state to none and
/// deleting it), clears the moving-reference list, deletes every record in
/// the two state maps, flushes the cache (`00adcd80`, with its flag byte
/// cleared meanwhile) and frees the pause list.
pub fn bs_audio_manager_shutdown(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 6) == 0 {
        for _ in 0..3 {
            bs_audio_manager_update(e, this, 0);
        }
    }
    for list in [this.addr() + 0x0c, this.addr() + 0x18] {
        while e.mem.u32(list + 8) != 0 {
            let message = e.call(LIST_REMOVE_HEAD, &[list]).u32();
            destroy_object(e, MESSAGE_DESTRUCTOR, message);
        }
    }
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        // The scalar deleting destructor, first slot, with the delete flag.
        e.vcall(thread, 0, &[1]);
        e.set(this, BSAudioManager::pUpdateThread, 0);
    }
    // Every sound: remove it from the list, clear its voice state, delete it.
    let sounds = this.at(BSAudioManager::SoundList);
    let mut position = map_first_item(e, sounds);
    while position != 0 {
        let (_key, sound) = map_next(e, sounds, &mut position);
        let id = e.mem.u32(sound + SOUND_ID);
        e.call(MAP_REMOVE, &[sounds.addr(), id]);
        let id = e.mem.u32(sound + SOUND_ID);
        bs_audio_manager_set_voice_state(e, this, id, Ptr::NULL);
        if sound != 0 {
            e.vcall(sound, 0, &[1]);
        }
    }
    e.call(MAP_CLEAR, &[this.addr() + 0x84]);
    // Every pending state update, with the record it replaces.
    let updates = this.at(BSAudioManager::StateUpdateMap);
    let states = this.at(BSAudioManager::SoundStateMap);
    let mut position = map_first_item(e, updates);
    let mut old = 0;
    while position != 0 {
        let (key, update) = map_next(e, updates, &mut position);
        loop {
            let (found, record) = map_get(e, states, key, old);
            old = record;
            if !found {
                break;
            }
            while e.call(MAP_REMOVE, &[states.addr(), key]).bool() {}
            destroy_object(e, SOUND_INFO_DESTRUCTOR, old);
        }
        destroy_object(e, SOUND_INFO_DESTRUCTOR, update);
    }
    e.call(MAP_CLEAR, &[this.addr() + 0x74]);
    let saved = e.mem.u8(CACHE_FLUSH_FLAG);
    e.mem.set_u8(CACHE_FLUSH_FLAG, 0);
    e.call(MANAGER_CACHE_FLUSH, &[this.addr(), 0]);
    e.mem.set_u8(CACHE_FLUSH_FLAG, saved);
    let pause_entries = e.get(this, BSAudioManager::pPauseEntries);
    e.call(DEALLOCATE, &[MEMORY_MANAGER, pause_entries]);
}

// Translated from 00ad9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the six volumes from the `f...VolumeDefault` settings (the float
/// values at `011f6e18`, `e24`, `e30`, `e3c`, `e48`, `e54`: master, foot,
/// voice, effects, music, radio) and the six secondary volumes to 1.0.
/// (The compiler's null-setting fallbacks are folded away.)
pub fn fn_00ad9da0(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let defaults = [
        (0x011f_6e18, BSAudioManager::fMasterVolume),
        (0x011f_6e24, BSAudioManager::fFootVolume),
        (0x011f_6e30, BSAudioManager::fVoiceVolume),
        (0x011f_6e3c, BSAudioManager::fEffectsVolume),
        (0x011f_6e48, BSAudioManager::fMusicVolume),
        (0x011f_6e54, BSAudioManager::fRadioVolume),
    ];
    for (setting, field) in defaults {
        let value = e.global::<f32>(setting);
        e.set(this, field, value);
    }
    for field in [
        BSAudioManager::fMasterSecondaryVolume,
        BSAudioManager::fFootSecondaryVolume,
        BSAudioManager::fVoiceSecondaryVolume,
        BSAudioManager::fEffectsSecondaryVolume,
        BSAudioManager::fMusicSecondaryVolume,
        BSAudioManager::fRadioSecondaryVolume,
    ] {
        e.set(this, field, 1.0);
    }
}

// Translated from 00ad9f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::StartAudioThread` (Xbox PDB): when the audio thread
/// object exists and has no thread id yet, initializes the thread (0x8000
/// bytes of stack, named `LibAudioUpdate`), remembers its id, sets the
/// processor and the name and names the thread for the debugger; then
/// signals the thread's semaphore.
pub fn bs_audio_manager_start_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread == 0 {
        return;
    }
    // BSAudioManagerThread: the thread id is at +0x24 on PC.
    if e.mem.u32(thread + 0x24) == 0 {
        e.call(THREAD_INITIALIZE, &[thread, 0x8000, AUDIO_THREAD_NAME, 0]);
        let id = e.mem.u32(thread + 0x24);
        e.set(this, BSAudioManager::dwAudioThreadID, id);
        let processor = e.global::<u32>(AUDIO_THREAD_PROCESSOR);
        e.call(THREAD_SET_PROCESSOR, &[thread, processor]);
        e.call(THREAD_SET_NAME, &[thread, AUDIO_THREAD_NAME]);
        let id = e.mem.u32(thread + 0x24);
        e.call(SET_THREAD_NAME, &[id, AUDIO_THREAD_NAME]);
    }
    e.call(THREAD_SEMAPHORE_SIGNAL, &[thread]);
}

/// The shared tail of `SuspendAudioThread` and `StopAudioThread`: after the
/// message is queued, waits on the thread's semaphore handle (+0x40,
/// infinite) and decrements the counter at +0x3c.
fn wait_for_thread_semaphore(e: &mut Engine, thread: u32) {
    let handle = e.mem.u32(thread + 0x40);
    e.call(IMPORT_WAIT_FOR_SINGLE_OBJECT, &[handle, 0xffff_ffff]);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &[thread + 0x3c]);
}

// Translated from 00ada020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SuspendAudioThread` (Xbox PDB): signals the thread,
/// queues the suspend message (0x35) and waits for the thread to
/// acknowledge on its semaphore.
pub fn bs_audio_manager_suspend_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        e.call(THREAD_SEMAPHORE_SIGNAL, &[thread]);
    }
    queue_plain(e, this, 0x35, 0, 0, 0);
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        wait_for_thread_semaphore(e, thread);
    }
}

// Translated from 00ada0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::StopAudioThread` (Xbox PDB): like `SuspendAudioThread`
/// with the stop message (0x34), and afterwards finishes the thread
/// (`00aa64f0`).
pub fn bs_audio_manager_stop_audio_thread(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        e.call(THREAD_SEMAPHORE_SIGNAL, &[thread]);
    }
    queue_plain(e, this, 0x34, 0, 0, 0);
    let thread = e.get(this, BSAudioManager::pUpdateThread);
    if thread != 0 {
        wait_for_thread_semaphore(e, thread);
        let thread = e.get(this, BSAudioManager::pUpdateThread);
        e.call(THREAD_FINISH, &[thread]);
    }
}

/// The interface sounds `Update` precaches the first time it runs: name
/// text address and the flags passed with it.
const PRECACHED_INTERFACE_SOUNDS: [(u32, u32); 10] = [
    (0x0106_f060, 0),
    (0x0106_f39c, 0x20000),
    (0x0106_f360, 0x20001),
    (0x010a_3444, 0x20000),
    (0x0106_f380, 0x20001),
    (0x0106_f38c, 1),
    (0x010a_342c, 0x102),
    (0x010a_3418, 0x102),
    (0x010a_33fc, 0x101),
    (0x010a_33e8, 0x101),
];

// Translated from 00ada1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Update` (Xbox PDB): the first time, precaches ten
/// interface and weapon sounds through `BSAudio::Precache`; then syncs the
/// voice states and, when the audio does not run on its own thread, runs
/// `Process` on the singleton. The one stack word is not read.
pub fn bs_audio_manager_update(e: &mut Engine, this: Ptr<BSAudioManager>, _unused_1: u32) {
    if !e.get(this, BSAudioManager::bPrecacheCompleted) {
        let audio = e.global::<u32>(AUDIO_GLOBAL);
        for (name, flags) in PRECACHED_INTERFACE_SOUNDS {
            e.call(AUDIO_PRECACHE, &[audio, name, flags]);
        }
        e.set(this, BSAudioManager::bPrecacheCompleted, true);
    }
    bs_audio_manager_synch_voice_states(e, this);
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 6) == 0 {
        let manager = bs_audio_manager_q_instance(e);
        e.call(MANAGER_PROCESS, &[manager.addr()]);
    }
}

// Translated from 00ada300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the weather flags and the game hour (unnamed in the engine map).
pub fn fn_00ada300(e: &mut Engine, this: Ptr<BSAudioManager>, game_hour: f32, weather_flags: u32) {
    e.set(this, BSAudioManager::iWeatherFlags, weather_flags);
    e.set(this, BSAudioManager::fCurrentGamehour, game_hour);
}

// Translated from 00ada330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SynchVoiceStates` (Xbox PDB): under the state critical
/// section, walks the pending-update map: each entry replaces the record in
/// the state map (the old record is deleted); an entry whose id is 0 is
/// deleted instead. The pending-update map is then cleared. (A debug
/// listing of the state map, guarded by a compile-time zero, is folded
/// away.)
pub fn bs_audio_manager_synch_voice_states(e: &mut Engine, this: Ptr<BSAudioManager>) {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    if e.get(
        this.at(BSAudioManager::StateUpdateMap),
        NiTPointerMap::m_uiCount,
    ) != 0
    {
        let updates = this.at(BSAudioManager::StateUpdateMap);
        let states = this.at(BSAudioManager::SoundStateMap);
        let mut position = map_first_item(e, updates);
        let mut old = 0;
        while position != 0 {
            let (key, update) = map_next(e, updates, &mut position);
            let (found, record) = map_get(e, states, key, old);
            old = record;
            if found {
                while e.call(MAP_REMOVE, &[states.addr(), key]).bool() {}
                destroy_object(e, SOUND_INFO_DESTRUCTOR, old);
            }
            if e.mem.u32(update) != 0 {
                e.call(MAP_SET_VALUE, &[states.addr(), key, update]);
            } else {
                destroy_object(e, SOUND_INFO_DESTRUCTOR, update);
            }
        }
        e.call(MAP_CLEAR, &[updates.addr()]);
    }
    leave_section(e, section);
}

/// Clamps a distance the way `SetVoiceState` does for the debug record: a
/// value greater than 3.4028235e+37 (a "no limit" marker) becomes the
/// float at `01012054` (-1.0); anything else, NaN included, is kept.
fn clamp_distance(e: &Engine, distance: f32) -> f32 {
    let limit = e.mem.f64(HUGE_DISTANCE_LIMIT);
    if distance as f64 > limit {
        e.global::<f32>(HUGE_DISTANCE_REPLACEMENT)
    } else {
        distance
    }
}

/// The debug-record fill of `SetVoiceState`: the file name, the distances
/// and the attenuations of `sound` into the debug record at `record`
/// (cleared first).
fn fill_debug_record(e: &mut Engine, record: Ptr<BSDebugSoundInfo>, sound: u32) {
    e.call(MEMSET, &[record.addr(), 0, 0x23c]);
    let buffer = record.addr() + 0x10c;
    if e.get(record, BSDebugSoundInfo::pFileName) != buffer {
        e.set(record, BSDebugSoundInfo::pFileName, buffer);
    }
    let name_pointer = e.get(record, BSDebugSoundInfo::pFileName);
    // The PC code tests `sound + 0x36 != 0`, false only for the address
    // 0xffffffca.
    if sound.wrapping_add(SOUND_FILENAME) != 0 {
        e.call(
            STRCPY_S,
            &[name_pointer, 0x104, sound.wrapping_add(SOUND_FILENAME)],
        );
    } else {
        e.call(STRCPY_S, &[name_pointer, 0x104, NOT_TES_SOUND_TEXT]);
    }
    let positioned = e.call(SOUND_IS_EMITTER_SOUND, &[sound]).bool()
        || e.call(SOUND_HAS_EMITTER, &[sound]).bool();
    if positioned {
        let distance = clamp_distance(e, e.mem.f32(sound + SOUND_DISTANCE_TO_LISTENER));
        e.set(record, BSDebugSoundInfo::distanceToListener, distance);
        let x = e.with_stack(12, |e, out| {
            let position = e
                .vcall(sound, SOUND_SLOT_GET_EMITTER_POSITION, &[out.addr()])
                .u32();
            e.mem.f32(position)
        });
        e.set(record, BSDebugSoundInfo::locationX, x);
        let y = e.with_stack(12, |e, out| {
            let position = e
                .vcall(sound, SOUND_SLOT_GET_EMITTER_POSITION, &[out.addr()])
                .u32();
            e.mem.f32(position + 4)
        });
        e.set(record, BSDebugSoundInfo::locationY, y);
        let minimum = clamp_distance(e, e.mem.f32(sound + SOUND_MIN_DISTANCE));
        e.set(record, BSDebugSoundInfo::fRadiusMin, minimum);
        let maximum = clamp_distance(e, e.mem.f32(sound + SOUND_MAX_DISTANCE));
        e.set(record, BSDebugSoundInfo::fRadiusMax, maximum);
    } else {
        e.set(record, BSDebugSoundInfo::distanceToListener, 0.0);
        e.set(record, BSDebugSoundInfo::locationX, 0.0);
        e.set(record, BSDebugSoundInfo::locationY, 0.0);
        e.set(record, BSDebugSoundInfo::fRadiusMin, 0.0);
        e.set(record, BSDebugSoundInfo::fRadiusMax, 0.0);
    }
    let static_attenuation = e.mem.u16(sound + SOUND_STATIC_ATTENUATION) as u32;
    e.set(
        record,
        BSDebugSoundInfo::iStaticAttenuation,
        static_attenuation,
    );
    let system_attenuation = e.mem.u16(sound + SOUND_SYSTEM_ATTENUATION) as u32;
    e.set(
        record,
        BSDebugSoundInfo::iSystemAttenuation,
        system_attenuation,
    );
    let distance_attenuation = e.mem.u16(sound + SOUND_DISTANCE_ATTENUATION) as u32;
    e.set(
        record,
        BSDebugSoundInfo::iDistanceAttenuation,
        distance_attenuation,
    );
}

/// Copies the debug record at `record` into a temporary (the by-value
/// argument copy the compiler makes), assigns it to the debug part of
/// `target` (`BSDebugSoundInfo::operator=` on `target + 0x18`) and destroys
/// the temporary.
fn assign_debug_record(e: &mut Engine, target: u32, record: Ptr<BSDebugSoundInfo>) {
    let bytes = e.mem.bytes(record.addr(), 0x23c);
    e.with_stack(0x23c, |e, copy| {
        e.mem.write(copy.addr(), &bytes);
        e.call(DEBUG_SOUND_INFO_ASSIGN, &[target + 0x18, copy.addr()]);
        e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[copy.addr()]);
    });
}

// Translated from 00ada5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetVoiceState` (Xbox PDB): records the state of the
/// sound `sound` (null after `Shutdown`) under `id` in the pending-update
/// map. A local `BSSoundInfo` is built from the sound (its id, a frequency
/// of 1.0, its volume, type flags, duration and whether it is playing) and,
/// when the debug-information setting is on, a debug record (file name,
/// distances, position, attenuations). Under the state critical section the
/// record either overwrites the entry already in the map or becomes a new
/// 0x254-byte entry.
// Translation incomplete: 00ada5d0 - C++ exception unwinding (SEH frame state) and the stack cookie check are not translated.
pub fn bs_audio_manager_set_voice_state(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    sound: Ptr,
) {
    let sound = sound.addr();
    in_audio_context(e, |e| {
        // Locals the game keeps on its stack: the working record, two debug
        // records, the temporary built from the sound and a copy slot.
        e.with_stack(0xb5c, |e, frame| {
            let info = frame.addr();
            let debug_a = Ptr::<BSDebugSoundInfo>::new(info + 0x254);
            let debug_b = info + 0x490;
            let temporary = info + 0x6cc;
            e.call(SOUND_INFO_CONSTRUCTOR, &[info]);
            e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[debug_a.addr()]);
            e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[debug_b]);
            let debug = e.mem.u8(AUDIO_DEBUG_INFORMATION_SETTING) != 0;
            if sound != 0 {
                let duration = e.mem.u32(sound + SOUND_DURATION);
                let sound_type = e.mem.u32(sound + SOUND_TYPE_FLAGS);
                let sound_id = e.mem.u32(sound + SOUND_ID);
                let volume = e.vcall(sound, SOUND_SLOT_GET_VOLUME, &[]).f32();
                let playing = e.vcall(sound, SOUND_SLOT_IS_PLAYING, &[]).u8();
                e.call(DEBUG_SOUND_INFO_CONSTRUCTOR, &[temporary + 0x18]);
                e.mem.set_u32(temporary, sound_id);
                e.mem.set_f32(temporary + 4, 1.0);
                e.mem.set_f32(temporary + 8, volume);
                e.mem.set_u32(temporary + 0xc, sound_type);
                e.mem.set_u32(temporary + 0x10, duration);
                e.mem.set_u8(temporary + 0x14, playing);
                e.call(SOUND_INFO_ASSIGN, &[info, temporary]);
                e.call(SOUND_INFO_DESTRUCTOR, &[temporary]);
                if debug {
                    fill_debug_record(e, debug_a, sound);
                }
            }
            let section = this.addr() + 0xc4;
            enter_section(e, section);
            let updates = this.at(BSAudioManager::StateUpdateMap);
            let (found, existing) = map_get(e, updates, id, 0);
            if found {
                if debug {
                    assign_debug_record(e, existing, debug_a);
                }
                let record = Ptr::<BSSoundInfo>::new(existing);
                let source = Ptr::<BSSoundInfo>::new(info);
                let value = e.get(source, BSSoundInfo::iID);
                e.set(record, BSSoundInfo::iID, value);
                let value = e.get(source, BSSoundInfo::fFrequency);
                e.set(record, BSSoundInfo::fFrequency, value);
                let value = e.get(source, BSSoundInfo::fVolume);
                e.set(record, BSSoundInfo::fVolume, value);
                let value = e.get(source, BSSoundInfo::uiSoundType);
                e.set(record, BSSoundInfo::uiSoundType, value);
                let value = e.get(source, BSSoundInfo::uiDuration);
                e.set(record, BSSoundInfo::uiDuration, value);
                let value = e.get(source, BSSoundInfo::bIsPlaying);
                e.set(record, BSSoundInfo::bIsPlaying, value);
            } else {
                let raw = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x254]).u32();
                let created = if raw != 0 {
                    e.call(SOUND_INFO_COPY_CONSTRUCTOR, &[raw, info]).u32()
                } else {
                    0
                };
                if debug {
                    assign_debug_record(e, created, debug_a);
                }
                e.call(MAP_SET_VALUE, &[updates.addr(), id, created]);
            }
            leave_section(e, section);
            e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[debug_b]);
            e.call(DEBUG_SOUND_INFO_DESTRUCTOR, &[debug_a.addr()]);
            e.call(SOUND_INFO_DESTRUCTOR, &[info]);
        });
    });
}

// Translated from 00adac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The volume a sound of type mask `flags` plays at (unnamed in the engine
/// map): `volume` times the master volume, its secondary volume and the
/// `fMasterVolumeMult` setting, then times the volume and secondary volume
/// of the category the mask selects: music for 0x800000 (volume then
/// secondary), foot (8), voice (4), music (0x800), radio (0x100000), else
/// effects.
pub fn fn_00adac30(e: &mut Engine, this: Ptr<BSAudioManager>, volume: f32, flags: u32) -> f32 {
    let master = e.get(this, BSAudioManager::fMasterVolume) as f64;
    let master_secondary = e.get(this, BSAudioManager::fMasterSecondaryVolume) as f64;
    let multiplier = e.global::<f32>(MASTER_VOLUME_MULT) as f64;
    let scaled = (((master * master_secondary) * multiplier) * volume as f64) as f32;
    let category = |e: &Engine,
                    volume_field: Field<BSAudioManager, f32>,
                    secondary: Field<BSAudioManager, f32>| {
        (
            e.get(this, volume_field) as f64,
            e.get(this, secondary) as f64,
        )
    };
    if flags & 0x80_0000 != 0 {
        let (music, music_secondary) = category(
            e,
            BSAudioManager::fMusicVolume,
            BSAudioManager::fMusicSecondaryVolume,
        );
        return ((scaled as f64 * music) * music_secondary) as f32;
    }
    let (category_volume, category_secondary) = if flags & 8 != 0 {
        category(
            e,
            BSAudioManager::fFootVolume,
            BSAudioManager::fFootSecondaryVolume,
        )
    } else if flags & 4 != 0 {
        category(
            e,
            BSAudioManager::fVoiceVolume,
            BSAudioManager::fVoiceSecondaryVolume,
        )
    } else if flags & 0x800 != 0 {
        category(
            e,
            BSAudioManager::fMusicVolume,
            BSAudioManager::fMusicSecondaryVolume,
        )
    } else if flags & 0x10_0000 != 0 {
        category(
            e,
            BSAudioManager::fRadioVolume,
            BSAudioManager::fRadioSecondaryVolume,
        )
    } else {
        category(
            e,
            BSAudioManager::fEffectsVolume,
            BSAudioManager::fEffectsSecondaryVolume,
        )
    };
    ((category_volume * category_secondary) * scaled as f64) as f32
}

// Translated from 00adad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a message from the free-message stack (unnamed in the engine map):
/// pops `FreeMessages` (+0x24) and returns the message, or a new one when
/// the stack is empty and bottomless.
pub fn fn_00adad60(e: &mut Engine, this: Ptr<BSAudioManager>) -> Ptr<BSSoundMessage> {
    e.call(MESSAGE_STACK_POP, &[this.addr() + 0x24]).ptr()
}

/// The common part of `FreeMessage` and `FreeQueueMessage`: empties the
/// message and pushes it on the stack at `stack`.
fn free_message_to(e: &mut Engine, stack: u32, message: Ptr<BSSoundMessage>) {
    let message_address = message.addr();
    let (x, y, z) = default_message_vector(e);
    // A smart-pointer temporary (a local the game keeps on its stack) takes
    // over the message's pointer; it lives to the end of the function.
    e.with_stack(4, |e, held| {
        e.call(SMART_POINTER_ASSIGN, &[held.addr(), message_address + 0x10]);
        let object = e.mem.u32(held.addr());
        if object != 0 {
            // The message lets go of the object, which is then handed on.
            e.with_stack(4, |e, empty| {
                e.call(
                    SMART_POINTER_ASSIGN,
                    &[message_address + 0x10, empty.addr()],
                );
                e.call(SMART_POINTER_DESTRUCTOR, &[empty.addr()]);
            });
            e.call(RELEASE_MESSAGE_POINTER, &[object]);
            e.call(SMART_POINTER_SET, &[held.addr(), 0]);
        }
        e.set(message, BSSoundMessage::msg, 0);
        e.set(message, BSSoundMessage::iID, 0);
        e.set(message, BSSoundMessage::iData, 0);
        e.set(message, BSSoundMessage::pData, 0);
        // The cleared message takes a null smart pointer (its previous
        // pointer is released by the assignment).
        e.with_stack(4, |e, empty| {
            e.call(
                SMART_POINTER_ASSIGN,
                &[message_address + 0x10, empty.addr()],
            );
        });
        e.set(message, BSSoundMessage::vector3_x, x);
        e.set(message, BSSoundMessage::vector3_y, y);
        e.set(message, BSSoundMessage::vector3_z, z);
        e.call(MESSAGE_STACK_PUSH, &[stack, message_address]);
        // End of the temporary's scope.
        let remaining = e.mem.u32(held.addr());
        drop_reference(e, remaining);
    });
}

// Translated from 00adad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::FreeMessage` (Xbox PDB): clears a message (releasing the
/// object its smart pointer held) and pushes it on `FreeMessages`.
// Translation incomplete: 00adad80 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bs_audio_manager_free_message(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    free_message_to(e, this.addr() + 0x24, message);
}

// Translated from 00adaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a message from the free-queue-message stack (unnamed in the engine
/// map): pops `FreeQueueMessages` (+0x34).
pub fn fn_00adaf80(e: &mut Engine, this: Ptr<BSAudioManager>) -> Ptr<BSSoundMessage> {
    e.call(MESSAGE_STACK_POP, &[this.addr() + 0x34]).ptr()
}

// Translated from 00adafa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::FreeQueueMessage` (Xbox PDB): the same as `FreeMessage`
/// for the `FreeQueueMessages` stack (+0x34).
// Translation incomplete: 00adafa0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bs_audio_manager_free_queue_message(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    message: Ptr<BSSoundMessage>,
) {
    free_message_to(e, this.addr() + 0x34, message);
}

// Translated from 00adb1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a message (unnamed in the engine map; the callers' common step):
/// when the audio is initialized (byte +5 of the `BSAudio` object), takes a
/// message from `FreeMessages` under the message critical section, fills it
/// with the type, the three words, the smart pointer and the three floats,
/// and appends it to the message list `iMessageListIndex` selects.
/// `pointer` is passed by value as a smart pointer: the message takes its
/// own reference and the argument's reference is dropped on return.
// Translation incomplete: 00adb1a0 - C++ exception unwinding (SEH frame state) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00adb1a0(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    kind: u32,
    a: u32,
    b: u32,
    c: u32,
    pointer: u32,
    x: f32,
    y: f32,
    z: f32,
) {
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 5) != 0 {
        in_audio_context(e, |e| {
            let section = this.addr() + 0xac;
            enter_section(e, section);
            let message = fn_00adad60(e, this);
            if !message.is_null() {
                // The smart-pointer temporary built from the argument.
                e.with_stack(4, |e, temporary| {
                    e.mem.set_u32(temporary.addr(), pointer);
                    add_reference(e, pointer);
                    e.set(message, BSSoundMessage::msg, kind);
                    e.set(message, BSSoundMessage::iID, a);
                    e.set(message, BSSoundMessage::iData, b);
                    e.set(message, BSSoundMessage::pData, c);
                    e.call(
                        SMART_POINTER_ASSIGN,
                        &[message.addr() + 0x10, temporary.addr()],
                    );
                    e.set(message, BSSoundMessage::vector3_x, x);
                    e.set(message, BSSoundMessage::vector3_y, y);
                    e.set(message, BSSoundMessage::vector3_z, z);
                    let held = e.mem.u32(temporary.addr());
                    drop_reference(e, held);
                });
                append_message(e, this, message);
            }
            leave_section(e, section);
        });
    }
    drop_reference(e, pointer);
}

/// Appends `message` to the message list `iMessageListIndex` selects.
fn append_message(e: &mut Engine, this: Ptr<BSAudioManager>, message: Ptr<BSSoundMessage>) {
    let index = e.get(this, BSAudioManager::iMessageListIndex);
    let list = this.addr() + 0x0c + index.wrapping_mul(0x0c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), message.addr());
        e.call(LIST_APPEND_POINTER, &[list, slot.addr()]);
    });
}

// Translated from 00adb3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Play` (Xbox PDB): unless the sound is already playing,
/// queues message 0x2d (when `flag` is set) and message 1 for it. Always
/// true.
pub fn bs_audio_manager_play(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, flag: u8) -> bool {
    if bs_audio_manager_is_playing(e, this, id) {
        return true;
    }
    if flag != 0 {
        queue_plain(e, this, 0x2d, id, 0, 0);
    }
    queue_plain(e, this, 0x01, id, 0, 0);
    true
}

// Translated from 00adb4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::PlayAfter` (Xbox PDB): unless the sound is already
/// playing, queues message 5 with the time `delay` milliseconds (at most
/// 10000) after `GetTickCount`. Always true. The third stack word is not
/// read.
pub fn bs_audio_manager_play_after(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    delay: u32,
    _unused_3: u32,
) -> bool {
    if bs_audio_manager_is_playing(e, this, id) {
        return true;
    }
    let delay = delay.min(10000);
    let now = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    queue_plain(e, this, 0x05, id, now.wrapping_add(delay), 0);
    true
}

// Translated from 00adb590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Pause` (Xbox PDB): queues message 3 for the sound.
/// Returns false.
pub fn bs_audio_manager_pause(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    queue_plain(e, this, 0x03, id, 0, 0);
    false
}

// Translated from 00adb610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Stop` (Xbox PDB): queues message 4 for the sound.
/// Returns false.
pub fn bs_audio_manager_stop(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    queue_plain(e, this, 0x04, id, 0, 0);
    false
}

// Translated from 00adb690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::StopMovingSounds` (Xbox PDB): queues message 0x29 with
/// the object (a reference-counted object; the call takes a reference for
/// the smart-pointer argument), `seconds` converted to milliseconds
/// (`_ftol2` of seconds times the double at `01017b70`) and the last word.
pub fn bs_audio_manager_stop_moving_sounds(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    object: u32,
    seconds: f32,
    value: u32,
) {
    let (x, y, z) = default_message_vector(e);
    add_reference(e, object);
    let milliseconds = scaled_milliseconds_ftol(e, seconds);
    fn_00adb1a0(e, this, 0x29, 0, milliseconds, value, object, x, y, z);
}

// Translated from 00adb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetStaticAttenuation` (Xbox PDB): queues message 0x21
/// with the attenuation limited to 10000. Returns false.
pub fn bs_audio_manager_set_static_attenuation(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    attenuation: u16,
) -> bool {
    let attenuation = attenuation.min(10000) as u32;
    queue_plain(e, this, 0x21, id, attenuation, 0);
    false
}

// Translated from 00adb7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x13 with `seconds` converted to milliseconds through
/// `_ftol2` (unnamed in the engine map). Returns false.
pub fn fn_00adb7c0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, seconds: f32) -> bool {
    let milliseconds = scaled_milliseconds_ftol(e, seconds);
    queue_plain(e, this, 0x13, id, milliseconds, 0);
    false
}

// Translated from 00adb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetModFrequency` (Xbox PDB): queues message 0x42 with
/// the value times 1000 through `_ftol2`. Returns false.
pub fn bs_audio_manager_set_mod_frequency(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    value: f32,
) -> bool {
    let scaled = scaled_milliseconds_ftol(e, value);
    queue_plain(e, this, 0x42, id, scaled, 0);
    false
}

// Translated from 00adb8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x14 with a byte (unnamed in the engine map). Returns
/// false.
pub fn fn_00adb8e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, flag: u8) -> bool {
    queue_plain(e, this, 0x14, id, flag as u32, 0);
    false
}

// Translated from 00adb960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing and returns false (unnamed in the engine map); both stack
/// words are ignored.
pub fn fn_00adb960(
    _e: &mut Engine,
    _this: Ptr<BSAudioManager>,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    false
}

/// Queues a message whose three floats are `x`, `y`, `z` instead of the
/// default vector (`SetPosition` and its sibling).
fn queue_with_vector(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    kind: u32,
    id: u32,
    (x, y, z): (f32, f32, f32),
) {
    fn_00adb1a0(e, this, kind, id, 0, 0, 0, x, y, z);
}

// Translated from 00adb970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetPosition` (Xbox PDB): queues message 0x1e with the
/// three coordinates as the message vector. Returns false.
pub fn bs_audio_manager_set_position(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    queue_with_vector(e, this, 0x1e, id, (x, y, z));
    false
}

// Translated from 00adb9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x1f with three floats as the message vector (unnamed in
/// the engine map). Returns false.
pub fn fn_00adb9f0(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    queue_with_vector(e, this, 0x1f, id, (x, y, z));
    false
}

// Translated from 00adba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetMinMax` (Xbox PDB): queues message 0x23 with the
/// minimum and maximum distances times 1000, each truncated by an inline
/// `FISTP` (low 32 bits of the 64-bit result).
pub fn bs_audio_manager_set_min_max(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    minimum: f32,
    maximum: f32,
) {
    let maximum = scaled_milliseconds_inline(e, maximum);
    let minimum = scaled_milliseconds_inline(e, minimum);
    queue_plain(e, this, 0x23, id, minimum, maximum);
}

// Translated from 00adbb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x24 with five 16-bit values, each limited to 100
/// (unnamed in the engine map). Takes a message from `FreeMessages`,
/// builds a local message with the type and the sound id, assigns it to the
/// pooled message (`00ae7990`), destroys the local one, stores the five
/// limited values over the message's vector words (`curve[0..5]`) and
/// appends the message to the current list.
// Translation incomplete: 00adbb30 - C++ exception unwinding (SEH frame state) and the stack cookie check are not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00adbb30(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    value_0: u16,
    value_1: u16,
    value_2: u16,
    value_3: u16,
    value_4: u16,
) {
    let values = [value_0, value_1, value_2, value_3, value_4].map(|value| value.min(100));
    let audio = e.global::<u32>(AUDIO_GLOBAL);
    if e.mem.u8(audio + 5) == 0 {
        return;
    }
    in_audio_context(e, |e| {
        let section = this.addr() + 0xac;
        enter_section(e, section);
        let message = fn_00adad60(e, this);
        let (x, y, z) = default_message_vector(e);
        // Locals: the null smart pointer that is assigned into the local
        // message, and the local message itself.
        e.with_stack(4 + 0x20, |e, frame| {
            let pointer_slot = frame.addr();
            let local = frame.addr() + 4;
            e.mem.set_u32(local, 0x24);
            e.mem.set_u32(local + 4, id);
            e.mem.set_u32(local + 8, 0);
            e.mem.set_u32(local + 0xc, 0);
            e.call(SMART_POINTER_ASSIGN, &[local + 0x10, pointer_slot]);
            e.mem.set_f32(local + 0x14, x);
            e.mem.set_f32(local + 0x18, y);
            e.mem.set_f32(local + 0x1c, z);
            e.call(SMART_POINTER_DESTRUCTOR, &[pointer_slot]);
            e.call(MESSAGE_ASSIGN, &[message.addr(), local]);
            e.call(MESSAGE_DESTRUCTOR, &[local]);
        });
        e.set(message, BSSoundMessage::curve_0, values[0]);
        e.set(message, BSSoundMessage::curve_1, values[1]);
        e.set(message, BSSoundMessage::curve_2, values[2]);
        e.set(message, BSSoundMessage::curve_3, values[3]);
        e.set(message, BSSoundMessage::curve_4, values[4]);
        append_message(e, this, message);
        leave_section(e, section);
    });
}

// Translated from 00adbec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x22 with a 16-bit value (unnamed in the engine map).
pub fn fn_00adbec0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u16) {
    queue_plain(e, this, 0x22, id, value as u32, 0);
}

// Translated from 00adbf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing and returns false (unnamed in the engine map); all four
/// stack words are ignored.
pub fn fn_00adbf40(
    _e: &mut Engine,
    _this: Ptr<BSAudioManager>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> bool {
    false
}

// Translated from 00adbf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::Release` (Xbox PDB): queues message 0x26 for the sound.
/// Returns true.
pub fn bs_audio_manager_release(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    queue_plain(e, this, 0x26, id, 0, 0);
    true
}

// Translated from 00adbfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetCompletionCallback` (Xbox PDB): queues message 0x3a
/// with the first stack word after the sound id as the message's third
/// word and the second as its second. Returns true.
pub fn bs_audio_manager_set_completion_callback(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    first: u32,
    second: u32,
) -> bool {
    queue_plain(e, this, 0x3a, id, second, first);
    true
}

// Translated from 00adc050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3b with the two words laid out like
/// `SetCompletionCallback` (unnamed in the engine map). Returns true.
pub fn fn_00adc050(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    first: u32,
    second: u32,
) -> bool {
    queue_plain(e, this, 0x3b, id, second, first);
    true
}

// Translated from 00adc0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetVolume` (Xbox PDB): queues message 0x10 with the
/// volume times 1000 through `_ftol2`. Returns true.
pub fn bs_audio_manager_set_volume(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    volume: f32,
) -> bool {
    let scaled = scaled_milliseconds_ftol(e, volume);
    queue_plain(e, this, 0x10, id, scaled, 0);
    true
}

// Translated from 00adc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::VolumeChanged` (Xbox PDB): queues message 0x0e with no
/// data.
pub fn bs_audio_manager_volume_changed(e: &mut Engine, this: Ptr<BSAudioManager>) {
    queue_plain(e, this, 0x0e, 0, 0, 0);
}

// Translated from 00adc1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetObjectToFollow` (Xbox PDB): queues message 0x27 with
/// the object as the message's smart pointer (a reference is taken for the
/// argument).
pub fn bs_audio_manager_set_object_to_follow(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    object: u32,
) {
    let (x, y, z) = default_message_vector(e);
    add_reference(e, object);
    fn_00adb1a0(e, this, 0x27, id, 0, 0, object, x, y, z);
}

// Translated from 00adc260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3f with a word (unnamed in the engine map).
pub fn fn_00adc260(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u32) {
    queue_plain(e, this, 0x3f, id, value, 0);
}

// Translated from 00adc2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x40 with a byte (unnamed in the engine map).
pub fn fn_00adc2e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, flag: u8) {
    queue_plain(e, this, 0x40, id, flag as u32, 0);
}

// Translated from 00adc360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetSynchStartTime` (Xbox PDB): queues message 0x3c with
/// the time in both the second and third word.
pub fn bs_audio_manager_set_synch_start_time(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    time: u32,
) {
    queue_plain(e, this, 0x3c, id, time, time);
}

// Translated from 00adc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3d for the sound (unnamed in the engine map).
pub fn fn_00adc3e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) {
    queue_plain(e, this, 0x3d, id, 0, 0);
}

// Translated from 00adc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::SetPriority` (Xbox PDB): queues message 0x41 with the
/// priority.
pub fn bs_audio_manager_set_priority(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    priority: u32,
) {
    queue_plain(e, this, 0x41, id, priority, 0);
}

// Translated from 00adc4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x3e with two bytes (unnamed in the engine map): the
/// first as the second word, the second as the third.
pub fn fn_00adc4e0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, first: u8, second: u8) {
    queue_plain(e, this, 0x3e, id, first as u32, second as u32);
}

// Translated from 00adc560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::FadeOut` (Xbox PDB): queues message 9 with `value` as
/// the third word and `time`, converted to a float, as the first of the
/// message's three floats (the other two are 0.0).
pub fn bs_audio_manager_fade_out(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    id: u32,
    value: u32,
    time: u32,
) {
    fn_00adb1a0(e, this, 0x09, id, 0, value, 0, time as f32, 0.0, 0.0);
}

// Translated from 00adc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x0b with a word as the third word (unnamed in the engine
/// map). The third stack word is not read.
pub fn fn_00adc5f0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u32, _unused_3: u32) {
    queue_plain(e, this, 0x0b, id, 0, value);
}

// Translated from 00adc670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless the sound is already playing, queues messages 7 and 1 for it and
/// then message 0x0b with `value` as the third word (unnamed in the engine
/// map).
pub fn fn_00adc670(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32, value: u32) {
    if bs_audio_manager_is_playing(e, this, id) {
        return;
    }
    queue_plain(e, this, 0x07, id, 0, 0);
    queue_plain(e, this, 0x01, id, 0, 0);
    queue_plain(e, this, 0x0b, id, 0, value);
}

// Translated from 00adc7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::IsValid` (Xbox PDB): whether the sound state map has a
/// record for `id` whose own id is `id`.
pub fn bs_audio_manager_is_valid(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    let (_found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    record != 0 && e.mem.u32(record) == id
}

// Translated from 00adc830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetDuration` (Xbox PDB): the duration in the sound's
/// state record (+0x10), 0 when there is none. Reads the map under the
/// state critical section.
pub fn bs_audio_manager_get_duration(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> u32 {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (_found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    leave_section(e, section);
    if record != 0 {
        e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::uiDuration)
    } else {
        0
    }
}

// Translated from 00adc8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::GetVolume` (Xbox PDB): the volume in the sound's state
/// record (+8), 0.0 when there is none. Reads the map under the state
/// critical section.
pub fn bs_audio_manager_get_volume(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> f32 {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (_found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    leave_section(e, section);
    if record != 0 {
        e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::fVolume)
    } else {
        0.0
    }
}

// Translated from 00adc910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::IsPlaying` (Xbox PDB): whether the sound's state
/// record says it is playing. Reads the map under the state critical
/// section.
pub fn bs_audio_manager_is_playing(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> bool {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    let playing = found
        && record != 0
        && e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::bIsPlaying) != 0;
    leave_section(e, section);
    playing
}

// Translated from 00adc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sound type word (+0xc) of the sound's state record, 0 when there is
/// none (unnamed in the engine map). Reads the map under the state critical
/// section.
pub fn fn_00adc9a0(e: &mut Engine, this: Ptr<BSAudioManager>, id: u32) -> u32 {
    let section = this.addr() + 0xc4;
    enter_section(e, section);
    let (found, record) = map_get(e, this.at(BSAudioManager::SoundStateMap), id, 0);
    let value = if found && record != 0 {
        e.get(Ptr::<BSSoundInfo>::new(record), BSSoundInfo::uiSoundType)
    } else {
        0
    };
    leave_section(e, section);
    value
}

// Translated from 00adca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::MuteAllOfType` (Xbox PDB): queues message 0x18 with the
/// third stack word as the first message word and the first two as the
/// second and third.
pub fn bs_audio_manager_mute_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    first: u32,
    second: u32,
    third: u32,
) {
    queue_plain(e, this, 0x18, third, first, second);
}

// Translated from 00adcaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x19 with the two words as the second and third message
/// word (unnamed in the engine map).
pub fn fn_00adcaa0(e: &mut Engine, this: Ptr<BSAudioManager>, first: u32, second: u32) {
    queue_plain(e, this, 0x19, 0, first, second);
}

// Translated from 00adcb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x1c when `flag` is set and 0x1a otherwise, with `value`
/// as the second message word (unnamed in the engine map).
pub fn fn_00adcb20(e: &mut Engine, this: Ptr<BSAudioManager>, value: u32, flag: u8) {
    let kind = if flag != 0 { 0x1c } else { 0x1a };
    queue_plain(e, this, kind, 0, value, 0);
}

// Translated from 00adcc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues message 0x1d when `flag` is set and 0x1b otherwise, with `value`
/// as the second message word (unnamed in the engine map).
pub fn fn_00adcc10(e: &mut Engine, this: Ptr<BSAudioManager>, value: u32, flag: u8) {
    let kind = if flag != 0 { 0x1d } else { 0x1b };
    queue_plain(e, this, kind, 0, value, 0);
}

// Translated from 00adcd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioManager::KillAllOfType` (Xbox PDB): queues message 0x2a with the
/// two words as the second and third message word.
pub fn bs_audio_manager_kill_all_of_type(
    e: &mut Engine,
    this: Ptr<BSAudioManager>,
    first: u32,
    second: u32,
) {
    queue_plain(e, this, 0x2a, 0, first, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ad9060, bs_audio_manager_q_instance() -> Ptr<BSAudioManager>),
        entry!(
            0x00ad90e0,
            fn_00ad90e0(Ptr<BSAudioManager>) -> Ptr<BSAudioManager>
        ),
        entry!(0x00ad93f0, fn_00ad93f0(Ptr<BSAudioManager>)),
        entry!(0x00ad9640, bs_audio_manager_initialize(Ptr<BSAudioManager>)),
        entry!(0x00ad9870, bs_audio_manager_shutdown(Ptr<BSAudioManager>)),
        entry!(0x00ad9da0, fn_00ad9da0(Ptr<BSAudioManager>)),
        entry!(
            0x00ad9f30,
            bs_audio_manager_start_audio_thread(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada020,
            bs_audio_manager_suspend_audio_thread(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada0f0,
            bs_audio_manager_stop_audio_thread(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada1d0,
            bs_audio_manager_update(Ptr<BSAudioManager>, u32)
        ),
        entry!(0x00ada300, fn_00ada300(Ptr<BSAudioManager>, f32, u32)),
        entry!(
            0x00ada330,
            bs_audio_manager_synch_voice_states(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00ada5d0,
            bs_audio_manager_set_voice_state(Ptr<BSAudioManager>, u32, Ptr)
        ),
        entry!(
            0x00adac30,
            fn_00adac30(Ptr<BSAudioManager>, f32, u32) -> f32
        ),
        entry!(
            0x00adad60,
            fn_00adad60(Ptr<BSAudioManager>) -> Ptr<BSSoundMessage>
        ),
        entry!(
            0x00adad80,
            bs_audio_manager_free_message(Ptr<BSAudioManager>, Ptr<BSSoundMessage>)
        ),
        entry!(
            0x00adaf80,
            fn_00adaf80(Ptr<BSAudioManager>) -> Ptr<BSSoundMessage>
        ),
        entry!(
            0x00adafa0,
            bs_audio_manager_free_queue_message(Ptr<BSAudioManager>, Ptr<BSSoundMessage>)
        ),
        entry!(
            0x00adb1a0,
            fn_00adb1a0(Ptr<BSAudioManager>, u32, u32, u32, u32, u32, f32, f32, f32)
        ),
        entry!(
            0x00adb3e0,
            bs_audio_manager_play(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00adb4e0,
            bs_audio_manager_play_after(Ptr<BSAudioManager>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adb590,
            bs_audio_manager_pause(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adb610,
            bs_audio_manager_stop(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adb690,
            bs_audio_manager_stop_moving_sounds(Ptr<BSAudioManager>, u32, f32, u32)
        ),
        entry!(
            0x00adb720,
            bs_audio_manager_set_static_attenuation(Ptr<BSAudioManager>, u32, u16) -> bool
        ),
        entry!(
            0x00adb7c0,
            fn_00adb7c0(Ptr<BSAudioManager>, u32, f32) -> bool
        ),
        entry!(
            0x00adb850,
            bs_audio_manager_set_mod_frequency(Ptr<BSAudioManager>, u32, f32) -> bool
        ),
        entry!(
            0x00adb8e0,
            fn_00adb8e0(Ptr<BSAudioManager>, u32, u8) -> bool
        ),
        entry!(
            0x00adb960,
            fn_00adb960(Ptr<BSAudioManager>, u32, u32) -> bool
        ),
        entry!(
            0x00adb970,
            bs_audio_manager_set_position(Ptr<BSAudioManager>, u32, f32, f32, f32) -> bool
        ),
        entry!(
            0x00adb9f0,
            fn_00adb9f0(Ptr<BSAudioManager>, u32, f32, f32, f32) -> bool
        ),
        entry!(
            0x00adba70,
            bs_audio_manager_set_min_max(Ptr<BSAudioManager>, u32, f32, f32)
        ),
        entry!(
            0x00adbb30,
            fn_00adbb30(Ptr<BSAudioManager>, u32, u16, u16, u16, u16, u16)
        ),
        entry!(0x00adbec0, fn_00adbec0(Ptr<BSAudioManager>, u32, u16)),
        entry!(
            0x00adbf40,
            fn_00adbf40(Ptr<BSAudioManager>, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adbf50,
            bs_audio_manager_release(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adbfd0,
            bs_audio_manager_set_completion_callback(Ptr<BSAudioManager>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adc050,
            fn_00adc050(Ptr<BSAudioManager>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00adc0d0,
            bs_audio_manager_set_volume(Ptr<BSAudioManager>, u32, f32) -> bool
        ),
        entry!(
            0x00adc160,
            bs_audio_manager_volume_changed(Ptr<BSAudioManager>)
        ),
        entry!(
            0x00adc1e0,
            bs_audio_manager_set_object_to_follow(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adc260, fn_00adc260(Ptr<BSAudioManager>, u32, u32)),
        entry!(0x00adc2e0, fn_00adc2e0(Ptr<BSAudioManager>, u32, u8)),
        entry!(
            0x00adc360,
            bs_audio_manager_set_synch_start_time(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adc3e0, fn_00adc3e0(Ptr<BSAudioManager>, u32)),
        entry!(
            0x00adc460,
            bs_audio_manager_set_priority(Ptr<BSAudioManager>, u32, u32)
        ),
        entry!(0x00adc4e0, fn_00adc4e0(Ptr<BSAudioManager>, u32, u8, u8)),
        entry!(
            0x00adc560,
            bs_audio_manager_fade_out(Ptr<BSAudioManager>, u32, u32, u32)
        ),
        entry!(0x00adc5f0, fn_00adc5f0(Ptr<BSAudioManager>, u32, u32, u32)),
        entry!(0x00adc670, fn_00adc670(Ptr<BSAudioManager>, u32, u32)),
        entry!(
            0x00adc7d0,
            bs_audio_manager_is_valid(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(
            0x00adc830,
            bs_audio_manager_get_duration(Ptr<BSAudioManager>, u32) -> u32
        ),
        entry!(
            0x00adc8b0,
            bs_audio_manager_get_volume(Ptr<BSAudioManager>, u32) -> f32
        ),
        entry!(
            0x00adc910,
            bs_audio_manager_is_playing(Ptr<BSAudioManager>, u32) -> bool
        ),
        entry!(0x00adc9a0, fn_00adc9a0(Ptr<BSAudioManager>, u32) -> u32),
        entry!(
            0x00adca20,
            bs_audio_manager_mute_all_of_type(Ptr<BSAudioManager>, u32, u32, u32)
        ),
        entry!(0x00adcaa0, fn_00adcaa0(Ptr<BSAudioManager>, u32, u32)),
        entry!(0x00adcb20, fn_00adcb20(Ptr<BSAudioManager>, u32, u8)),
        entry!(0x00adcc10, fn_00adcc10(Ptr<BSAudioManager>, u32, u8)),
        entry!(
            0x00adcd00,
            bs_audio_manager_kill_all_of_type(Ptr<BSAudioManager>, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the KERNEL32 imports the manager uses, the pages of
    /// exe data it reads mapped, and the constants set.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let imports = [
            (IMPORT_ENTER_CRITICAL_SECTION, "EnterCriticalSection"),
            (IMPORT_LEAVE_CRITICAL_SECTION, "LeaveCriticalSection"),
            (
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                "InitializeCriticalSection",
            ),
            (IMPORT_DELETE_CRITICAL_SECTION, "DeleteCriticalSection"),
            (IMPORT_GET_TICK_COUNT, "GetTickCount"),
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (IMPORT_INTERLOCKED_DECREMENT, "InterlockedDecrement"),
        ]
        .map(|(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 7);
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x010a_3000,
            0x011f_4000,
            0x011f_5000,
            0x011f_6000,
            0x011f_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(MILLISECOND_SCALE, 1000.0);
        e.mem
            .set_f64(HUGE_DISTANCE_LIMIT, 3.402_823_466_385_288_6e37);
        e.mem.set_f32(HUGE_DISTANCE_REPLACEMENT, -1.0);
        e.mem.set_cstr(NOT_TES_SOUND_TEXT, b"NotTESSound");
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_MESSAGE_VECTOR + 4 * i as u32, v);
        }
        e
    }

    // ---- a small model of the pointer maps and lists the manager uses ----

    fn map_node(e: &Engine, map: u32, key: u32) -> Option<u32> {
        let size = e.mem.u32(map + 4);
        let table = e.mem.u32(map + 8);
        let mut node = e.mem.u32(table + (key % size) * 4);
        while node != 0 {
            if e.mem.u32(node + 4) == key {
                return Some(node);
            }
            node = e.mem.u32(node);
        }
        None
    }

    fn fake_get_value(e: &mut Engine, a: &[u32]) -> Ret {
        match map_node(e, a[0], a[1]) {
            Some(node) => {
                let value = e.mem.u32(node + 8);
                e.mem.set_u32(a[2], value);
                ret(1)
            }
            None => ret(0),
        }
    }

    fn fake_set_value(e: &mut Engine, a: &[u32]) -> Ret {
        if let Some(node) = map_node(e, a[0], a[1]) {
            e.mem.set_u32(node + 8, a[2]);
            return ret(0);
        }
        let size = e.mem.u32(a[0] + 4);
        let table = e.mem.u32(a[0] + 8);
        let slot = table + (a[1] % size) * 4;
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, e.mem.u32(slot));
        e.mem.set_u32(node + 4, a[1]);
        e.mem.set_u32(node + 8, a[2]);
        e.mem.set_u32(slot, node);
        let count = e.mem.u32(a[0] + 0xc);
        e.mem.set_u32(a[0] + 0xc, count + 1);
        ret(0)
    }

    fn fake_remove(e: &mut Engine, a: &[u32]) -> Ret {
        let size = e.mem.u32(a[0] + 4);
        let table = e.mem.u32(a[0] + 8);
        let mut link = table + (a[1] % size) * 4;
        loop {
            let node = e.mem.u32(link);
            if node == 0 {
                return ret(0);
            }
            if e.mem.u32(node + 4) == a[1] {
                let next = e.mem.u32(node);
                e.mem.set_u32(link, next);
                let count = e.mem.u32(a[0] + 0xc);
                e.mem.set_u32(a[0] + 0xc, count - 1);
                return ret(1);
            }
            link = node;
        }
    }

    fn fake_get_next(e: &mut Engine, a: &[u32]) -> Ret {
        let current = e.mem.u32(a[1]);
        let key = e.mem.u32(current + 4);
        let value = e.mem.u32(current + 8);
        e.mem.set_u32(a[2], key);
        e.mem.set_u32(a[3], value);
        let mut next = e.mem.u32(current);
        if next == 0 {
            let size = e.mem.u32(a[0] + 4);
            let table = e.mem.u32(a[0] + 8);
            let mut index = key % size + 1;
            while index < size {
                let head = e.mem.u32(table + index * 4);
                if head != 0 {
                    next = head;
                    break;
                }
                index += 1;
            }
        }
        e.mem.set_u32(a[1], next);
        ret(0)
    }

    fn fake_map_clear(e: &mut Engine, a: &[u32]) -> Ret {
        let size = e.mem.u32(a[0] + 4);
        let table = e.mem.u32(a[0] + 8);
        for index in 0..size {
            e.mem.set_u32(table + index * 4, 0);
        }
        e.mem.set_u32(a[0] + 0xc, 0);
        ret(0)
    }

    fn install_map_model(e: &mut Engine) {
        e.register(MAP_GET_VALUE, fake_get_value);
        e.register(MAP_SET_VALUE, fake_set_value);
        e.register(MAP_REMOVE, fake_remove);
        e.register(MAP_GET_NEXT, fake_get_next);
        e.register(MAP_CLEAR, fake_map_clear);
    }

    /// Appends `element` to a pointer list (head, tail, count; nodes are
    /// `[next, previous, element]`).
    fn list_push(e: &mut Engine, list: u32, element: u32) {
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, element);
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(node + 4, tail);
        if tail == 0 {
            e.mem.set_u32(list, node);
        } else {
            e.mem.set_u32(tail, node);
        }
        e.mem.set_u32(list + 4, node);
        let count = e.mem.u32(list + 8);
        e.mem.set_u32(list + 8, count + 1);
    }

    fn fake_list_remove_head(e: &mut Engine, a: &[u32]) -> Ret {
        let node = e.mem.u32(a[0]);
        let next = e.mem.u32(node);
        e.mem.set_u32(a[0], next);
        if next == 0 {
            e.mem.set_u32(a[0] + 4, 0);
        } else {
            e.mem.set_u32(next + 4, 0);
        }
        let count = e.mem.u32(a[0] + 8);
        e.mem.set_u32(a[0] + 8, count - 1);
        ret(e.mem.u32(node + 8))
    }

    /// `NiPointer::operator=` as a test double: the destination takes the
    /// source's pointer, adding a reference to it and dropping one from the
    /// old one (the real function does this through its callees).
    fn fake_pointer_assign(e: &mut Engine, a: &[u32]) -> Ret {
        let new = e.mem.u32(a[1]);
        let old = e.mem.u32(a[0]);
        if new != old {
            if old != 0 {
                let count = e.mem.u32(old + 4);
                e.mem.set_u32(old + 4, count - 1);
            }
            e.mem.set_u32(a[0], new);
            if new != 0 {
                let count = e.mem.u32(new + 4);
                e.mem.set_u32(new + 4, count + 1);
            }
        }
        ret(a[0])
    }

    // ---- the message queue ----

    /// What a queued message holds.
    #[derive(Debug, PartialEq)]
    struct Message {
        kind: u32,
        a: u32,
        b: u32,
        c: u32,
        pointer: u32,
        vector: [f32; 3],
    }

    /// The message with the default vector and no pointer.
    fn plain(kind: u32, a: u32, b: u32, c: u32) -> Message {
        Message {
            kind,
            a,
            b,
            c,
            pointer: 0,
            vector: [1.0, 2.0, 3.0],
        }
    }

    /// A manager whose message pool hands out fresh messages and whose
    /// message list records what is appended, with the audio initialized.
    struct Fixture {
        e: Engine,
        this: Ptr<BSAudioManager>,
        queued: Rc<RefCell<Vec<u32>>>,
    }

    impl Fixture {
        fn new() -> Self {
            let mut e = engine();
            install_map_model(&mut e);
            let audio = e.mem.alloc(0x10);
            e.mem.set_u8(audio + 5, 1);
            e.set_global(AUDIO_GLOBAL, audio);
            e.register(MESSAGE_STACK_POP, |e, _| {
                let message = e.mem.alloc(0x20);
                ret(message)
            });
            e.register(SMART_POINTER_ASSIGN, fake_pointer_assign);
            let queued = Rc::new(RefCell::new(Vec::new()));
            let log = queued.clone();
            e.register_double(LIST_APPEND_POINTER, move |e, a| {
                log.borrow_mut().push(e.mem.u32(a[1]));
                Ret::default()
            });
            let this: Ptr<BSAudioManager> = e.new_object();
            for offset in [0x54, 0x64, 0x74, 0x84] {
                let table = e.mem.alloc(7 * 4);
                e.mem.set_u32(this.addr() + offset + 4, 7);
                e.mem.set_u32(this.addr() + offset + 8, table);
            }
            Fixture { e, this, queued }
        }

        fn messages(&self) -> Vec<Message> {
            self.queued
                .borrow()
                .iter()
                .map(|&m| Message {
                    kind: self.e.mem.u32(m),
                    a: self.e.mem.u32(m + 4),
                    b: self.e.mem.u32(m + 8),
                    c: self.e.mem.u32(m + 0xc),
                    pointer: self.e.mem.u32(m + 0x10),
                    vector: [
                        self.e.mem.f32(m + 0x14),
                        self.e.mem.f32(m + 0x18),
                        self.e.mem.f32(m + 0x1c),
                    ],
                })
                .collect()
        }

        /// Puts a record for sound `id` into the state map (+0x64): its id,
        /// volume, type word, duration and playing byte.
        fn add_state(&mut self, id: u32, volume: f32, sound_type: u32, duration: u32, playing: u8) {
            let record = self.e.mem.alloc(0x20);
            self.e.mem.set_u32(record, id);
            self.e.mem.set_f32(record + 8, volume);
            self.e.mem.set_u32(record + 0xc, sound_type);
            self.e.mem.set_u32(record + 0x10, duration);
            self.e.mem.set_u8(record + 0x14, playing);
            let map = self.this.addr() + 0x64;
            fake_set_value(&mut self.e, &[map, id, record]);
        }

        fn call(&mut self, address: u32, rest: &[u32]) -> Ret {
            let mut words = vec![self.this.addr()];
            words.extend_from_slice(rest);
            self.e.call(address, &words)
        }
    }

    #[test]
    fn q_instance_builds_the_manager_once() {
        let mut e = engine();
        let exits = Rc::new(RefCell::new(Vec::new()));
        let log = exits.clone();
        e.register_double(ATEXIT, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(MESSAGE_STACK_CONSTRUCTOR, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_LIST, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_STATES, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_MOVING_REFERENCES, |_, _| ret(0));
        assert_eq!(e.call(0x00ad9060, &[]).u32(), MANAGER_INSTANCE);
        assert_eq!(e.mem.u32(MANAGER_BUILT_GUARD) & 1, 1);
        assert_eq!(e.mem.u32(MANAGER_INSTANCE), 0x010a_3368);
        assert_eq!(*exits.borrow(), vec![MANAGER_EXIT_THUNK]);
        // The second call returns the same object without building again.
        e.mem.set_u32(MANAGER_INSTANCE, 0);
        assert_eq!(e.call(0x00ad9060, &[]).u32(), MANAGER_INSTANCE);
        assert_eq!(e.mem.u32(MANAGER_INSTANCE), 0);
        assert_eq!(exits.borrow().len(), 1);
    }

    #[test]
    fn constructor_sets_the_vtables_and_initial_state() {
        let mut e = engine();
        e.clock_ms = 4321;
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(MESSAGE_STACK_CONSTRUCTOR, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_LIST, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_SOUND_STATES, |_, _| ret(0));
        e.register(MAP_CONSTRUCTOR_MOVING_REFERENCES, |_, _| ret(0));
        let this: Ptr<BSAudioManager> = e.new_object();
        e.mem.write(this.addr(), &[0xaa; 0x188]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad90e0, &args![this]).u32(), this.addr());
        let log: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            log,
            vec![
                0x00ad90e0,
                VECTOR_CONSTRUCTOR_ITERATOR,
                MESSAGE_STACK_CONSTRUCTOR,
                MESSAGE_STACK_CONSTRUCTOR,
                MESSAGE_STACK_CONSTRUCTOR,
                MAP_CONSTRUCTOR_SOUND_LIST,
                MAP_CONSTRUCTOR_SOUND_STATES,
                MAP_CONSTRUCTOR_SOUND_STATES,
                MAP_CONSTRUCTOR_MOVING_REFERENCES,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_INITIALIZE_CRITICAL_SECTION,
                IMPORT_GET_TICK_COUNT,
            ]
        );
        assert_eq!(e.get(this, BSAudioManager::vtable), 0x010a_3368);
        assert_eq!(e.mem.u32(this.addr() + 0x54), 0x010a_3348);
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0x010a_3328);
        assert_eq!(e.mem.u32(this.addr() + 0x74), 0x010a_3328);
        assert_eq!(e.mem.u32(this.addr() + 0x84), 0x010a_3308);
        assert_eq!(e.get(this, BSAudioManager::iMessageListIndex), 0);
        assert_eq!(e.get(this, BSAudioManager::iMessageProcessingListIndex), 1);
        assert_eq!(e.get(this, BSAudioManager::iNextID), 1);
        assert_eq!(e.get(this, BSAudioManager::iDelayTimerDelta), 4321);
        assert_eq!(e.get(this, BSAudioManager::fMasterVolume), 0.0);
        assert_eq!(e.get(this, BSAudioManager::fRadioSecondaryVolume), 0.0);
        assert_eq!(e.get(this, BSAudioManager::pUpdateThread), 0);
        assert_eq!(e.get(this, BSAudioManager::iMuteMask), 0);
        assert!(!e.get(this, BSAudioManager::bPrecacheCompleted));
        assert!(!e.get(this, BSAudioManager::bListenerUnderwater));
        assert_eq!(e.mem.u32(this.addr() + 0x94), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x98), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xa0), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x124 + 8), 0);
    }

    #[test]
    fn destructor_tears_the_members_down_in_order() {
        let mut e = engine();
        e.register(LIST_CLEAR, |_, _| ret(0));
        e.register(MAP_CLEAR, |_, _| ret(0));
        e.register(FREE_TABLE, |_, _| ret(0));
        e.register(MESSAGE_STACK_DESTRUCTOR, |_, _| ret(0));
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| ret(0));
        let this: Ptr<BSAudioManager> = e.new_object();
        let base = this.addr();
        e.mem.set_u32(base + 0x54 + 8, 0x1111);
        e.mem.set_u32(base + 0x64 + 8, 0x2222);
        e.mem.set_u32(base + 0x74 + 8, 0x3333);
        e.mem.set_u32(base + 0x84 + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00ad93f0, &args![this]);
        let log = e.call_log.take().unwrap();
        let expected: Vec<(u32, Vec<u32>)> = vec![
            (0x00ad93f0, vec![base]),
            (LIST_CLEAR, vec![base + 0x124]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0x10c]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xf4]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xdc]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xc4]),
            (IMPORT_DELETE_CRITICAL_SECTION, vec![base + 0xac]),
            (LIST_CLEAR, vec![base + 0x94]),
            (MAP_CLEAR, vec![base + 0x84]),
            (MAP_CLEAR, vec![base + 0x84]),
            (FREE_TABLE, vec![0x4444]),
            (MAP_CLEAR, vec![base + 0x74]),
            (MAP_CLEAR, vec![base + 0x74]),
            (FREE_TABLE, vec![0x3333]),
            (MAP_CLEAR, vec![base + 0x64]),
            (MAP_CLEAR, vec![base + 0x64]),
            (FREE_TABLE, vec![0x2222]),
            (MAP_CLEAR, vec![base + 0x54]),
            (MAP_CLEAR, vec![base + 0x54]),
            (FREE_TABLE, vec![0x1111]),
            (MESSAGE_STACK_DESTRUCTOR, vec![base + 0x44]),
            (MESSAGE_STACK_DESTRUCTOR, vec![base + 0x34]),
            (MESSAGE_STACK_DESTRUCTOR, vec![base + 0x24]),
            (
                VECTOR_DESTRUCTOR_ITERATOR,
                vec![base + 0x0c, 0x0c, 2, MESSAGE_LIST_DESTRUCTOR],
            ),
        ];
        assert_eq!(log, expected);
        assert_eq!(e.get(this, BSAudioManager::vtable), 0x010a_3368);
        // The last map to be torn down ends with its base vtable.
        assert_eq!(e.mem.u32(base + 0x54), 0x010a_3370);
        assert_eq!(e.mem.u32(base + 0x84), 0x010a_33b0);
    }

    #[test]
    fn initialize_fills_the_message_stacks_and_makes_the_thread() {
        let mut e = engine();
        e.mem.set_f32(0x011f_6e18, 0.5);
        e.register(MEMSET, |_, _| ret(0));
        e.register(MESSAGE_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(THREAD_CONSTRUCTOR, |_, a| ret(a[0]));
        let pushes = Rc::new(RefCell::new(Vec::new()));
        let log = pushes.clone();
        e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let this: Ptr<BSAudioManager> = e.new_object();
        // Audio disabled: nothing happens.
        e.call(0x00ad9640, &args![this]);
        assert_eq!(e.get(this, BSAudioManager::iPauseListSize), 0);
        assert!(pushes.borrow().is_empty());
        e.mem.set_u8(AUDIO_ENABLED_SETTING, 1);
        e.mem.set_u32(this.addr() + 0xa4, 77);
        e.call_log = Some(vec![]);
        e.call(0x00ad9640, &args![this]);
        assert_eq!(e.get(this, BSAudioManager::iWeatherFlags), 0);
        assert_eq!(e.get(this, BSAudioManager::iPauseListSize), 8);
        let pause_entries = e.get(this, BSAudioManager::pPauseEntries);
        assert_ne!(pause_entries, 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMSET, vec![pause_entries, 0, 64])));
        assert_eq!(e.get(this, BSAudioManager::fMasterVolume), 0.5);
        let pushes = pushes.borrow();
        assert_eq!(pushes.len(), 0x200);
        assert!(pushes[..0x100].iter().all(|p| p.0 == this.addr() + 0x24));
        assert!(pushes[0x100..].iter().all(|p| p.0 == this.addr() + 0x34));
        assert!(pushes.iter().all(|p| p.1 != 0));
        assert!(e.get(
            this.at(BSAudioManager::FreeMessages),
            AudioMessageStack::bBottomless
        ));
        assert!(e.get(
            this.at(BSAudioManager::FreeQueueMessages),
            AudioMessageStack::bBottomless
        ));
        assert!(!e.get(
            this.at(BSAudioManager::GarbageMessages),
            AudioMessageStack::bBottomless
        ));
        let thread = e.get(this, BSAudioManager::pUpdateThread);
        assert_ne!(thread, 0);
    }

    #[test]
    fn volumes_come_from_the_default_settings() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        for (i, setting) in [
            0x011f_6e18,
            0x011f_6e24,
            0x011f_6e30,
            0x011f_6e3c,
            0x011f_6e48,
            0x011f_6e54,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_f32(setting, 0.1 * (i + 1) as f32);
        }
        e.call(0x00ad9da0, &args![this]);
        assert_eq!(e.get(this, BSAudioManager::fMasterVolume), 0.1f32);
        assert_eq!(e.get(this, BSAudioManager::fFootVolume), 0.2f32);
        assert_eq!(e.get(this, BSAudioManager::fVoiceVolume), 0.3f32);
        assert_eq!(e.get(this, BSAudioManager::fEffectsVolume), 0.4f32);
        assert_eq!(e.get(this, BSAudioManager::fMusicVolume), 0.5f32);
        assert_eq!(e.get(this, BSAudioManager::fRadioVolume), 0.6f32);
        for field in [
            BSAudioManager::fMasterSecondaryVolume,
            BSAudioManager::fFootSecondaryVolume,
            BSAudioManager::fVoiceSecondaryVolume,
            BSAudioManager::fEffectsSecondaryVolume,
            BSAudioManager::fMusicSecondaryVolume,
            BSAudioManager::fRadioSecondaryVolume,
        ] {
            assert_eq!(e.get(this, field), 1.0);
        }
    }

    #[test]
    fn start_audio_thread_initializes_a_thread_without_an_id() {
        let mut e = engine();
        e.register(THREAD_INITIALIZE, |e, a| {
            // The initialization gives the thread its id.
            e.mem.set_u32(a[0] + 0x24, 0x77);
            ret(0)
        });
        e.register(THREAD_SET_PROCESSOR, |_, _| ret(0));
        e.register(THREAD_SET_NAME, |_, _| ret(0));
        e.register(SET_THREAD_NAME, |_, _| ret(0));
        e.register(THREAD_SEMAPHORE_SIGNAL, |_, _| ret(0));
        e.set_global(AUDIO_THREAD_PROCESSOR, 3u32);
        let this: Ptr<BSAudioManager> = e.new_object();
        // No thread object: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x00ad9f30, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let thread = e.mem.alloc(0x4c);
        e.set(this, BSAudioManager::pUpdateThread, thread);
        e.call_log = Some(vec![]);
        e.call(0x00ad9f30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00ad9f30, vec![this.addr()]),
                (
                    THREAD_INITIALIZE,
                    vec![thread, 0x8000, AUDIO_THREAD_NAME, 0]
                ),
                (THREAD_SET_PROCESSOR, vec![thread, 3]),
                (THREAD_SET_NAME, vec![thread, AUDIO_THREAD_NAME]),
                (SET_THREAD_NAME, vec![0x77, AUDIO_THREAD_NAME]),
                (THREAD_SEMAPHORE_SIGNAL, vec![thread]),
            ]
        );
        assert_eq!(e.get(this, BSAudioManager::dwAudioThreadID), 0x77);
        // A thread that already has an id is only signalled.
        e.call_log = Some(vec![]);
        e.call(0x00ad9f30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (THREAD_SEMAPHORE_SIGNAL, vec![thread]));
    }

    /// The manager of a fixture with an audio thread object whose
    /// semaphore handle is 0x55 and whose counter is 3.
    fn thread_fixture() -> (Fixture, u32) {
        let mut f = Fixture::new();
        f.e.register(THREAD_SEMAPHORE_SIGNAL, |_, _| ret(0));
        f.e.register(IMPORT_WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        f.e.register(THREAD_FINISH, |_, _| ret(0));
        let thread = f.e.mem.alloc(0x4c);
        f.e.mem.set_u32(thread + 0x3c, 3);
        f.e.mem.set_u32(thread + 0x40, 0x55);
        f.e.set(f.this, BSAudioManager::pUpdateThread, thread);
        (f, thread)
    }

    #[test]
    fn suspend_audio_thread_queues_the_message_and_waits() {
        let (mut f, thread) = thread_fixture();
        f.e.call_log = Some(vec![]);
        f.e.call(0x00ada020, &args![f.this]);
        let log = f.e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert!(addresses.starts_with(&[0x00ada020, THREAD_SEMAPHORE_SIGNAL]));
        assert!(addresses.ends_with(&[IMPORT_WAIT_FOR_SINGLE_OBJECT, IMPORT_INTERLOCKED_DECREMENT]));
        assert!(log.contains(&(IMPORT_WAIT_FOR_SINGLE_OBJECT, vec![0x55, 0xffff_ffff])));
        assert_eq!(f.messages(), vec![plain(0x35, 0, 0, 0)]);
        assert_eq!(f.e.mem.u32(thread + 0x3c), 2);
        assert!(!addresses.contains(&THREAD_FINISH));
    }

    #[test]
    fn stop_audio_thread_also_finishes_the_thread() {
        let (mut f, thread) = thread_fixture();
        f.e.call_log = Some(vec![]);
        f.e.call(0x00ada0f0, &args![f.this]);
        let log = f.e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(THREAD_FINISH, vec![thread]));
        assert_eq!(f.messages(), vec![plain(0x34, 0, 0, 0)]);
        assert_eq!(f.e.mem.u32(thread + 0x3c), 2);
        // Without a thread object only the message is queued.
        f.e.set(f.this, BSAudioManager::pUpdateThread, 0);
        f.e.call_log = Some(vec![]);
        f.e.call(0x00ada0f0, &args![f.this]);
        let log = f.e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == THREAD_FINISH));
        assert_eq!(f.messages().len(), 2);
    }

    #[test]
    fn update_precaches_once_then_syncs_and_processes() {
        let mut f = Fixture::new();
        let precached = Rc::new(RefCell::new(Vec::new()));
        let log = precached.clone();
        f.e.register_double(AUDIO_PRECACHE, move |_, a| {
            log.borrow_mut().push((a[1], a[2]));
            ret(0)
        });
        let processed = Rc::new(RefCell::new(0));
        let count = processed.clone();
        f.e.register_double(MANAGER_PROCESS, move |_, a| {
            assert_eq!(a[0], MANAGER_INSTANCE);
            *count.borrow_mut() += 1;
            ret(0)
        });
        f.e.mem.set_u32(MANAGER_BUILT_GUARD, 1);
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.call(0x00ada1d0, &[0]);
        assert_eq!(precached.borrow().len(), 10);
        assert_eq!(precached.borrow()[0], (0x0106_f060, 0));
        assert_eq!(precached.borrow()[9], (0x010a_33e8, 0x101));
        assert!(f.e.get(f.this, BSAudioManager::bPrecacheCompleted));
        assert_eq!(*processed.borrow(), 1);
        // Second run: already precached; the audio thread runs: no Process.
        f.e.mem.set_u8(audio + 6, 1);
        f.call(0x00ada1d0, &[0]);
        assert_eq!(precached.borrow().len(), 10);
        assert_eq!(*processed.borrow(), 1);
    }

    #[test]
    fn weather_and_hour_are_stored() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        e.call(0x00ada300, &args![this, 13.5f32, 0x25u32]);
        assert_eq!(e.get(this, BSAudioManager::fCurrentGamehour), 13.5);
        assert_eq!(e.get(this, BSAudioManager::iWeatherFlags), 0x25);
    }

    #[test]
    fn synch_voice_states_moves_updates_into_the_state_map() {
        let mut f = Fixture::new();
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        f.e.register_double(SOUND_INFO_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        let updates = f.this.addr() + 0x74;
        let states = f.this.addr() + 0x64;
        // The old record for sound 5, the new one for 5 and an update with
        // id 0 for sound 6.
        let old = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(old, 5);
        fake_set_value(&mut f.e, &[states, 5, old]);
        let new = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(new, 5);
        fake_set_value(&mut f.e, &[updates, 5, new]);
        let empty = f.e.mem.alloc(0x20);
        fake_set_value(&mut f.e, &[updates, 6, empty]);
        f.e.call_log = Some(vec![]);
        f.call(0x00ada330, &[]);
        let log = f.e.call_log.take().unwrap();
        assert_eq!(log.first().unwrap().0, 0x00ada330);
        assert_eq!(
            log[1],
            (IMPORT_ENTER_CRITICAL_SECTION, vec![f.this.addr() + 0xc4])
        );
        assert_eq!(
            log.last().unwrap(),
            &(IMPORT_LEAVE_CRITICAL_SECTION, vec![f.this.addr() + 0xc4])
        );
        assert_eq!(
            map_node(&f.e, states, 5).map(|n| f.e.mem.u32(n + 8)),
            Some(new)
        );
        assert_eq!(map_node(&f.e, states, 6), None);
        assert_eq!(f.e.mem.u32(updates + 0xc), 0);
        let mut gone = destroyed.borrow().clone();
        gone.sort_unstable();
        let mut expected = vec![old, empty];
        expected.sort_unstable();
        assert_eq!(gone, expected);
        // Nothing pending: only the critical section is entered and left.
        f.e.call_log = Some(vec![]);
        f.call(0x00ada330, &[]);
        assert_eq!(f.e.call_log.take().unwrap().len(), 3);
    }

    /// Doubles for the callees of `SetVoiceState` that build and copy
    /// records.
    fn install_record_doubles(e: &mut Engine) {
        e.register(SOUND_INFO_CONSTRUCTOR, |e, a| {
            e.mem.write(a[0], &[0; 0x18]);
            ret(a[0])
        });
        e.register(DEBUG_SOUND_INFO_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(DEBUG_SOUND_INFO_DESTRUCTOR, |_, _| ret(0));
        e.register(SOUND_INFO_DESTRUCTOR, |_, _| ret(0));
        e.register(SOUND_INFO_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x18);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(SOUND_INFO_COPY_CONSTRUCTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x18);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(DEBUG_SOUND_INFO_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x23c);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            ret(a[0])
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(0)
        });
    }

    /// A sound object with a vtable whose `GetVolume` answers `volume`,
    /// `IsPlaying` answers `playing` and `GetEmitterPosition` points at
    /// the position `(7, 8, 9)`.
    fn make_sound(e: &mut Engine, id: u32, volume: f32, playing: u8) -> u32 {
        let sound = e.mem.alloc(0x198);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 7.0);
        e.mem.set_f32(position + 4, 8.0);
        e.mem.set_f32(position + 8, 9.0);
        let vtable = 0x0300_0100;
        if !e.mem.is_mapped(vtable) {
            e.mem.map(vtable, 0x100);
        }
        e.mem.set_u32(vtable + SOUND_SLOT_IS_PLAYING, 0x0300_0200);
        e.mem.set_u32(vtable + SOUND_SLOT_GET_VOLUME, 0x0300_0204);
        e.mem
            .set_u32(vtable + SOUND_SLOT_GET_EMITTER_POSITION, 0x0300_0208);
        e.mem.set_u32(sound, vtable);
        e.mem.set_u32(sound + SOUND_ID, id);
        e.mem.set_u32(sound + SOUND_TYPE_FLAGS, 0x44);
        e.mem.set_u32(sound + SOUND_DURATION, 1500);
        e.register_double(0x0300_0200, move |_, _| ret(playing as u32));
        e.register_double(0x0300_0204, move |_, _| Ret {
            st0: volume as f64,
            ..Ret::default()
        });
        e.register_double(0x0300_0208, move |_, _| ret(position));
        sound
    }

    #[test]
    fn set_voice_state_creates_the_record_for_a_new_sound() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        let sound = make_sound(&mut f.e, 9, 0.75, 1);
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        f.e.mem.set_u32(context, 0x1234);
        f.e.call_log = Some(vec![]);
        f.call(0x00ada5d0, &[9, sound]);
        let log = f.e.call_log.take().unwrap();
        assert!(log.iter().any(|c| c.0 == SOUND_INFO_COPY_CONSTRUCTOR));
        let updates = f.this.addr() + 0x74;
        let record = map_node(&f.e, updates, 9)
            .map(|n| f.e.mem.u32(n + 8))
            .unwrap();
        assert_eq!(f.e.mem.u32(record), 9);
        assert_eq!(f.e.mem.f32(record + 4), 1.0);
        assert_eq!(f.e.mem.f32(record + 8), 0.75);
        assert_eq!(f.e.mem.u32(record + 0xc), 0x44);
        assert_eq!(f.e.mem.u32(record + 0x10), 1500);
        assert_eq!(f.e.mem.u8(record + 0x14), 1);
        // The audio context word is restored.
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        assert_eq!(f.e.mem.u32(context), 0x1234);
        // The critical section brackets the map work.
        let positions: Vec<u32> = log.iter().map(|c| c.0).collect();
        let enter = positions
            .iter()
            .position(|a| *a == IMPORT_ENTER_CRITICAL_SECTION)
            .unwrap();
        let leave = positions
            .iter()
            .position(|a| *a == IMPORT_LEAVE_CRITICAL_SECTION)
            .unwrap();
        let set = positions.iter().position(|a| *a == MAP_SET_VALUE).unwrap();
        assert!(enter < set && set < leave);
    }

    #[test]
    fn set_voice_state_overwrites_an_existing_record() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        let sound = make_sound(&mut f.e, 9, 0.25, 0);
        let updates = f.this.addr() + 0x74;
        let existing = f.e.mem.alloc(0x254);
        f.e.mem.set_u32(existing + 0xc, 1);
        fake_set_value(&mut f.e, &[updates, 9, existing]);
        f.call(0x00ada5d0, &[9, sound]);
        assert_eq!(f.e.mem.u32(existing), 9);
        assert_eq!(f.e.mem.f32(existing + 8), 0.25);
        assert_eq!(f.e.mem.u32(existing + 0xc), 0x44);
        assert_eq!(f.e.mem.u8(existing + 0x14), 0);
        assert_eq!(f.e.mem.u32(updates + 0xc), 1);
    }

    #[test]
    fn set_voice_state_without_a_sound_stores_an_empty_record() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.call(0x00ada5d0, &[4, 0]);
        let updates = f.this.addr() + 0x74;
        let record = map_node(&f.e, updates, 4)
            .map(|n| f.e.mem.u32(n + 8))
            .unwrap();
        assert_eq!(f.e.mem.u32(record), 0);
        assert_eq!(f.e.mem.u32(record + 0xc), 0);
    }

    #[test]
    fn set_voice_state_fills_the_debug_record_when_enabled() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.mem.set_u8(AUDIO_DEBUG_INFORMATION_SETTING, 1);
        f.e.register(SOUND_IS_EMITTER_SOUND, |_, _| ret(0));
        f.e.register(SOUND_HAS_EMITTER, |_, _| ret(1));
        let copies = Rc::new(RefCell::new(Vec::new()));
        let log = copies.clone();
        f.e.register_double(DEBUG_SOUND_INFO_ASSIGN, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.bytes(a[1], 0x23c)));
            ret(a[0])
        });
        let sound = make_sound(&mut f.e, 9, 0.5, 1);
        f.e.mem
            .set_cstr(sound + SOUND_FILENAME, b"Sound\\fx\\boom.wav");
        f.e.mem.set_f32(sound + SOUND_DISTANCE_TO_LISTENER, 50.0);
        f.e.mem.set_f32(sound + SOUND_MIN_DISTANCE, 3.0e38);
        f.e.mem.set_f32(sound + SOUND_MAX_DISTANCE, 400.0);
        f.e.mem.set_u16(sound + SOUND_STATIC_ATTENUATION, 11);
        f.e.mem.set_u16(sound + SOUND_SYSTEM_ATTENUATION, 22);
        f.e.mem.set_u16(sound + SOUND_DISTANCE_ATTENUATION, 33);
        f.call(0x00ada5d0, &[9, sound]);
        let copies = copies.borrow();
        assert_eq!(copies.len(), 1);
        let record = &copies[0].1;
        let word =
            |offset: usize| u32::from_le_bytes(record[offset..offset + 4].try_into().unwrap());
        let float = |offset: usize| f32::from_bits(word(offset));
        assert_eq!(&record[0x10c..0x10c + 18], b"Sound\\fx\\boom.wav\0");
        assert_ne!(word(0x108), 0);
        assert_eq!(float(0x210), -1.0);
        assert_eq!(float(0x214), 400.0);
        assert_eq!(float(0x218), 7.0);
        assert_eq!(float(0x21c), 8.0);
        assert_eq!(float(0x220), 50.0);
        assert_eq!(word(0x230), 11);
        assert_eq!(word(0x234), 22);
        assert_eq!(word(0x238), 33);
    }

    #[test]
    fn set_voice_state_zeroes_the_distances_of_an_unpositioned_sound() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.mem.set_u8(AUDIO_DEBUG_INFORMATION_SETTING, 1);
        f.e.register(SOUND_IS_EMITTER_SOUND, |_, _| ret(0));
        f.e.register(SOUND_HAS_EMITTER, |_, _| ret(0));
        let copies = Rc::new(RefCell::new(Vec::new()));
        let log = copies.clone();
        f.e.register_double(DEBUG_SOUND_INFO_ASSIGN, move |e, a| {
            log.borrow_mut().push(e.mem.bytes(a[1], 0x23c));
            ret(a[0])
        });
        let sound = make_sound(&mut f.e, 9, 0.5, 1);
        f.e.mem.set_f32(sound + SOUND_DISTANCE_TO_LISTENER, 50.0);
        f.call(0x00ada5d0, &[9, sound]);
        let copies = copies.borrow();
        assert!(copies[0][0x210..0x224].iter().all(|b| *b == 0));
        // No file name: the text for a sound without one is not used while
        // the name exists; an empty name stays empty.
        assert_eq!(copies[0][0x10c], 0);
    }

    #[test]
    fn volume_scales_by_the_category_of_the_sound_type() {
        let mut e = engine();
        e.mem.set_f32(MASTER_VOLUME_MULT, 2.0);
        let this: Ptr<BSAudioManager> = e.new_object();
        let set = |e: &mut Engine, field: Field<BSAudioManager, f32>, value: f32| {
            e.set(this, field, value)
        };
        set(&mut e, BSAudioManager::fMasterVolume, 0.5);
        set(&mut e, BSAudioManager::fMasterSecondaryVolume, 0.5);
        set(&mut e, BSAudioManager::fFootVolume, 2.0);
        set(&mut e, BSAudioManager::fFootSecondaryVolume, 3.0);
        set(&mut e, BSAudioManager::fVoiceVolume, 4.0);
        set(&mut e, BSAudioManager::fVoiceSecondaryVolume, 0.5);
        set(&mut e, BSAudioManager::fEffectsVolume, 0.25);
        set(&mut e, BSAudioManager::fEffectsSecondaryVolume, 8.0);
        set(&mut e, BSAudioManager::fMusicVolume, 10.0);
        set(&mut e, BSAudioManager::fMusicSecondaryVolume, 0.5);
        set(&mut e, BSAudioManager::fRadioVolume, 6.0);
        set(&mut e, BSAudioManager::fRadioSecondaryVolume, 0.5);
        // Master part: 0.5 * 0.5 * 2.0 * 4.0 = 2.0.
        let volume =
            |e: &mut Engine, flags: u32| e.call(0x00adac30, &args![this, 4.0f32, flags]).f32();
        assert_eq!(volume(&mut e, 0), 2.0 * 2.0); // effects: 0.25 * 8
        assert_eq!(volume(&mut e, 8), 2.0 * 6.0); // foot: 2 * 3
        assert_eq!(volume(&mut e, 4), 2.0 * 2.0); // voice: 4 * 0.5
        assert_eq!(volume(&mut e, 0x800), 2.0 * 5.0); // music: 10 * 0.5
        assert_eq!(volume(&mut e, 0x10_0000), 2.0 * 3.0); // radio: 6 * 0.5
        assert_eq!(volume(&mut e, 0x80_0000), 2.0 * 10.0 * 0.5); // music, other order
                                                                 // The flag 0x800000 wins over the others, 8 over 4.
        assert_eq!(volume(&mut e, 0x80_0008), 10.0);
        assert_eq!(volume(&mut e, 0xc), 2.0 * 6.0);
    }

    #[test]
    fn the_message_pools_pop_their_own_stacks() {
        let mut e = engine();
        let this: Ptr<BSAudioManager> = e.new_object();
        e.register(MESSAGE_STACK_POP, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x00adad60, &args![this]).u32(), this.addr() + 0x25);
        assert_eq!(e.call(0x00adaf80, &args![this]).u32(), this.addr() + 0x35);
    }

    /// A message holding a reference-counted object whose count is 2.
    fn message_with_object(e: &mut Engine) -> (u32, u32) {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 2);
        let message = e.mem.alloc(0x20);
        e.mem.set_u32(message, 0x13);
        e.mem.set_u32(message + 4, 5);
        e.mem.set_u32(message + 8, 6);
        e.mem.set_u32(message + 0xc, 7);
        e.mem.set_u32(message + 0x10, object);
        (message, object)
    }

    /// The `(stack, message)` pairs pushed so far.
    type PushLog = Rc<RefCell<Vec<(u32, u32)>>>;

    fn free_message_fixture() -> (Engine, Ptr<BSAudioManager>, PushLog) {
        let mut e = engine();
        e.register(SMART_POINTER_ASSIGN, fake_pointer_assign);
        e.register(SMART_POINTER_DESTRUCTOR, |_, _| ret(0));
        e.register(RELEASE_MESSAGE_POINTER, |_, _| ret(0));
        e.register(SMART_POINTER_SET, |e, a| {
            // Sets the slot to the pointer, dropping the old reference.
            let old = e.mem.u32(a[0]);
            if old != 0 {
                let count = e.mem.u32(old + 4);
                e.mem.set_u32(old + 4, count - 1);
            }
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let pushes = Rc::new(RefCell::new(Vec::new()));
        let log = pushes.clone();
        e.register_double(MESSAGE_STACK_PUSH, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let this: Ptr<BSAudioManager> = e.new_object();
        (e, this, pushes)
    }

    #[test]
    fn free_message_clears_the_message_and_pushes_it() {
        let (mut e, this, pushes) = free_message_fixture();
        let (message, object) = message_with_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00adad80, &args![this, message]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RELEASE_MESSAGE_POINTER, vec![object])));
        assert_eq!(*pushes.borrow(), vec![(this.addr() + 0x24, message)]);
        assert_eq!(e.mem.u32(message), 0);
        assert_eq!(e.mem.u32(message + 4), 0);
        assert_eq!(e.mem.u32(message + 8), 0);
        assert_eq!(e.mem.u32(message + 0xc), 0);
        assert_eq!(e.mem.u32(message + 0x10), 0);
        assert_eq!(e.mem.f32(message + 0x14), 1.0);
        assert_eq!(e.mem.f32(message + 0x18), 2.0);
        assert_eq!(e.mem.f32(message + 0x1c), 3.0);
        // The reference count: 2 + 1 (the temporary) - 1 (the message let
        // go) - 1 (the temporary released) = 1.
        assert_eq!(e.mem.u32(object + 4), 1);
    }

    #[test]
    fn free_message_without_an_object_skips_the_release() {
        let (mut e, this, pushes) = free_message_fixture();
        let message = e.mem.alloc(0x20);
        e.mem.set_u32(message, 0x13);
        e.call_log = Some(vec![]);
        e.call(0x00adad80, &args![this, message]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == RELEASE_MESSAGE_POINTER));
        assert_eq!(*pushes.borrow(), vec![(this.addr() + 0x24, message)]);
        assert_eq!(e.mem.u32(message), 0);
    }

    #[test]
    fn free_queue_message_pushes_on_the_queue_stack() {
        let (mut e, this, pushes) = free_message_fixture();
        let (message, object) = message_with_object(&mut e);
        e.call(0x00adafa0, &args![this, message]);
        assert_eq!(*pushes.borrow(), vec![(this.addr() + 0x34, message)]);
        assert_eq!(e.mem.u32(message + 0x10), 0);
        assert_eq!(e.mem.u32(object + 4), 1);
    }

    #[test]
    fn queue_message_fills_a_message_and_appends_it() {
        let mut f = Fixture::new();
        f.e.set(f.this, BSAudioManager::iMessageListIndex, 1);
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        f.e.mem.set_u32(context, 9);
        f.e.call_log = Some(vec![]);
        f.call(
            0x00adb1a0,
            &[
                0x31,
                1,
                2,
                3,
                0,
                4.5f32.to_bits(),
                5.5f32.to_bits(),
                6.5f32.to_bits(),
            ],
        );
        let log = f.e.call_log.take().unwrap();
        let append = log.iter().find(|c| c.0 == LIST_APPEND_POINTER).unwrap();
        assert_eq!(append.1[0], f.this.addr() + 0x0c + 0x0c);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x31,
                a: 1,
                b: 2,
                c: 3,
                pointer: 0,
                vector: [4.5, 5.5, 6.5],
            }]
        );
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        assert_eq!(f.e.mem.u32(context), 9);
        let enters = log
            .iter()
            .filter(|c| **c == (IMPORT_ENTER_CRITICAL_SECTION, vec![f.this.addr() + 0xac]))
            .count();
        assert_eq!(enters, 1);
    }

    #[test]
    fn queue_message_does_nothing_while_the_audio_is_not_initialized() {
        let mut f = Fixture::new();
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(audio + 5, 0);
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 2);
        f.e.mem.set_u32(object, 0x0300_0300);
        f.call(0x00adb1a0, &[0x31, 1, 2, 3, object, 0, 0, 0]);
        assert!(f.messages().is_empty());
        // The argument's reference is still dropped.
        assert_eq!(f.e.mem.u32(object + 4), 1);
    }

    #[test]
    fn queue_message_keeps_the_reference_the_message_takes() {
        let mut f = Fixture::new();
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 2);
        f.call(0x00adb1a0, &[0x31, 1, 2, 3, object, 0, 0, 0]);
        assert_eq!(f.messages()[0].pointer, object);
        // 2 (caller and argument) + 1 (temporary) + 1 (message) - 1 - 1 = 2.
        assert_eq!(f.e.mem.u32(object + 4), 2);
    }

    #[test]
    fn queue_message_with_an_empty_pool_appends_nothing() {
        let mut f = Fixture::new();
        f.e.register(MESSAGE_STACK_POP, |_, _| ret(0));
        f.e.call_log = Some(vec![]);
        f.call(0x00adb1a0, &[0x31, 1, 2, 3, 0, 0, 0, 0]);
        let log = f.e.call_log.take().unwrap();
        assert!(f.messages().is_empty());
        assert_eq!(
            log.last().unwrap(),
            &(IMPORT_LEAVE_CRITICAL_SECTION, vec![f.this.addr() + 0xac])
        );
    }

    #[test]
    fn play_queues_the_play_messages_unless_playing() {
        let mut f = Fixture::new();
        assert!(f.call(0x00adb3e0, &[7, 1]).bool());
        assert_eq!(f.messages(), vec![plain(0x2d, 7, 0, 0), plain(1, 7, 0, 0)]);
        f.queued.borrow_mut().clear();
        assert!(f.call(0x00adb3e0, &[7, 0]).bool());
        assert_eq!(f.messages(), vec![plain(1, 7, 0, 0)]);
        f.queued.borrow_mut().clear();
        f.add_state(7, 0.0, 0, 0, 1);
        assert!(f.call(0x00adb3e0, &[7, 1]).bool());
        assert!(f.messages().is_empty());
    }

    #[test]
    fn play_after_adds_the_delay_to_the_tick_count() {
        let mut f = Fixture::new();
        f.e.clock_ms = 100_000;
        assert!(f.call(0x00adb4e0, &[7, 250, 0]).bool());
        assert!(f.call(0x00adb4e0, &[8, 99_999, 0]).bool());
        assert_eq!(
            f.messages(),
            vec![plain(5, 7, 100_250, 0), plain(5, 8, 110_000, 0)]
        );
        f.queued.borrow_mut().clear();
        f.add_state(7, 0.0, 0, 0, 1);
        assert!(f.call(0x00adb4e0, &[7, 250, 0]).bool());
        assert!(f.messages().is_empty());
    }

    #[test]
    fn pause_and_stop_queue_their_messages() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb590, &[3]).bool());
        assert!(!f.call(0x00adb610, &[3]).bool());
        assert_eq!(f.messages(), vec![plain(3, 3, 0, 0), plain(4, 3, 0, 0)]);
    }

    #[test]
    fn stop_moving_sounds_passes_the_object_and_the_milliseconds() {
        let mut f = Fixture::new();
        install_ftol(&mut f.e);
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 1);
        f.call(0x00adb690, &[object, 2.5f32.to_bits(), 77]);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x29,
                a: 0,
                b: 2500,
                c: 77,
                pointer: object,
                vector: [1.0, 2.0, 3.0],
            }]
        );
        // 1 (caller) + 1 (argument) + 1 (temporary) + 1 (message) - 1 - 1.
        assert_eq!(f.e.mem.u32(object + 4), 2);
    }

    #[test]
    fn static_attenuation_is_limited_to_10000() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb720, &[7, 500]).bool());
        assert!(!f.call(0x00adb720, &[7, 20000]).bool());
        assert_eq!(
            f.messages(),
            vec![plain(0x21, 7, 500, 0), plain(0x21, 7, 10000, 0)]
        );
    }

    /// `_ftol2` as a double: truncates the leading `f64`.
    fn install_ftol(e: &mut Engine) {
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits((a[0] as u64) | ((a[1] as u64) << 32));
            ret(value as i32 as u32)
        });
    }

    #[test]
    fn the_millisecond_wrappers_scale_through_ftol() {
        let mut f = Fixture::new();
        install_ftol(&mut f.e);
        assert!(!f.call(0x00adb7c0, &[7, 0.5f32.to_bits()]).bool());
        assert!(!f.call(0x00adb850, &[7, 1.25f32.to_bits()]).bool());
        assert!(f.call(0x00adc0d0, &[7, 0.75f32.to_bits()]).bool());
        assert_eq!(
            f.messages(),
            vec![
                plain(0x13, 7, 500, 0),
                plain(0x42, 7, 1250, 0),
                plain(0x10, 7, 750, 0)
            ]
        );
    }

    #[test]
    fn byte_and_word_wrappers_queue_their_values() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb8e0, &[7, 0x1ff]).bool());
        f.call(0x00adbec0, &[7, 0xffff_1234]);
        f.call(0x00adc260, &[7, 0xdead]);
        f.call(0x00adc2e0, &[7, 0x1ab]);
        f.call(0x00adc4e0, &[7, 0x1cd, 0x2ef]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x14, 7, 0xff, 0),
                plain(0x22, 7, 0x1234, 0),
                plain(0x3f, 7, 0xdead, 0),
                plain(0x40, 7, 0xab, 0),
                plain(0x3e, 7, 0xcd, 0xef),
            ]
        );
    }

    #[test]
    fn the_empty_wrappers_return_false() {
        let mut f = Fixture::new();
        assert!(!f.call(0x00adb960, &[1, 2]).bool());
        assert!(!f.call(0x00adbf40, &[1, 2, 3, 4]).bool());
        assert!(f.messages().is_empty());
    }

    #[test]
    fn position_wrappers_put_the_coordinates_in_the_vector() {
        let mut f = Fixture::new();
        assert!(!f
            .call(
                0x00adb970,
                &[7, 10.0f32.to_bits(), 20.0f32.to_bits(), 30.0f32.to_bits()]
            )
            .bool());
        assert!(!f
            .call(
                0x00adb9f0,
                &[7, 1.5f32.to_bits(), 2.5f32.to_bits(), 3.5f32.to_bits()]
            )
            .bool());
        let message = |kind, vector| Message {
            kind,
            a: 7,
            b: 0,
            c: 0,
            pointer: 0,
            vector,
        };
        assert_eq!(
            f.messages(),
            vec![
                message(0x1e, [10.0, 20.0, 30.0]),
                message(0x1f, [1.5, 2.5, 3.5])
            ]
        );
    }

    #[test]
    fn set_min_max_truncates_the_scaled_distances() {
        let mut f = Fixture::new();
        f.call(0x00adba70, &[7, 1.5f32.to_bits(), 2.0004f32.to_bits()]);
        f.call(0x00adba70, &[7, (-1.5f32).to_bits(), f32::NAN.to_bits()]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x23, 7, 1500, 2000),
                plain(0x23, 7, (-1500i32) as u32, 0),
            ]
        );
    }

    #[test]
    fn five_values_are_limited_to_100_and_overwrite_the_vector() {
        let mut f = Fixture::new();
        f.e.register(MESSAGE_ASSIGN, |e, a| {
            // Copies the first 0x14 bytes (the type, id and words).
            let bytes = e.mem.bytes(a[1], 0x14);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        f.e.register(MESSAGE_DESTRUCTOR, |_, _| ret(0));
        f.e.register(SMART_POINTER_DESTRUCTOR, |_, _| ret(0));
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        f.e.mem.set_u32(context, 5);
        f.call(0x00adbb30, &[7, 1, 100, 101, 65535, 99]);
        let message = f.queued.borrow()[0];
        assert_eq!(f.e.mem.u32(message), 0x24);
        assert_eq!(f.e.mem.u32(message + 4), 7);
        let curve: Vec<u16> = (0..5)
            .map(|i| f.e.mem.u16(message + 0x14 + 2 * i))
            .collect();
        assert_eq!(curve, vec![1, 100, 100, 100, 99]);
        let context = f.e.tls() + AUDIO_TLS_CONTEXT;
        assert_eq!(f.e.mem.u32(context), 5);
        // Audio not initialized: nothing happens.
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(audio + 5, 0);
        f.call(0x00adbb30, &[7, 1, 2, 3, 4, 5]);
        assert_eq!(f.queued.borrow().len(), 1);
    }

    #[test]
    fn release_and_the_completion_callbacks() {
        let mut f = Fixture::new();
        assert!(f.call(0x00adbf50, &[7]).bool());
        assert!(f.call(0x00adbfd0, &[7, 0x111, 0x222]).bool());
        assert!(f.call(0x00adc050, &[7, 0x333, 0x444]).bool());
        assert_eq!(
            f.messages(),
            vec![
                plain(0x26, 7, 0, 0),
                plain(0x3a, 7, 0x222, 0x111),
                plain(0x3b, 7, 0x444, 0x333),
            ]
        );
    }

    #[test]
    fn volume_changed_and_priority_and_synch_time() {
        let mut f = Fixture::new();
        f.call(0x00adc160, &[]);
        f.call(0x00adc360, &[7, 4000]);
        f.call(0x00adc3e0, &[7]);
        f.call(0x00adc460, &[7, 3]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x0e, 0, 0, 0),
                plain(0x3c, 7, 4000, 4000),
                plain(0x3d, 7, 0, 0),
                plain(0x41, 7, 3, 0),
            ]
        );
    }

    #[test]
    fn set_object_to_follow_passes_the_object() {
        let mut f = Fixture::new();
        let object = f.e.mem.alloc(8);
        f.e.mem.set_u32(object + 4, 1);
        f.call(0x00adc1e0, &[7, object]);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 0x27,
                a: 7,
                b: 0,
                c: 0,
                pointer: object,
                vector: [1.0, 2.0, 3.0],
            }]
        );
        assert_eq!(f.e.mem.u32(object + 4), 2);
        // A null object is queued without any reference counting.
        f.call(0x00adc1e0, &[8, 0]);
        assert_eq!(f.messages()[1].pointer, 0);
    }

    #[test]
    fn fade_out_puts_the_time_in_the_vector() {
        let mut f = Fixture::new();
        f.call(0x00adc560, &[7, 0x55, 3000]);
        assert_eq!(
            f.messages(),
            vec![Message {
                kind: 9,
                a: 7,
                b: 0,
                c: 0x55,
                pointer: 0,
                vector: [3000.0, 0.0, 0.0],
            }]
        );
    }

    #[test]
    fn word_message_and_the_conditional_play_sequence() {
        let mut f = Fixture::new();
        f.call(0x00adc5f0, &[7, 0x99, 0x1234]);
        assert_eq!(f.messages(), vec![plain(0x0b, 7, 0, 0x99)]);
        f.queued.borrow_mut().clear();
        f.call(0x00adc670, &[7, 0x66]);
        assert_eq!(
            f.messages(),
            vec![
                plain(7, 7, 0, 0),
                plain(1, 7, 0, 0),
                plain(0x0b, 7, 0, 0x66)
            ]
        );
        f.queued.borrow_mut().clear();
        f.add_state(7, 0.0, 0, 0, 1);
        f.call(0x00adc670, &[7, 0x66]);
        assert!(f.messages().is_empty());
    }

    #[test]
    fn state_queries_read_the_record() {
        let mut f = Fixture::new();
        f.add_state(7, 0.75, 0x1234, 4500, 1);
        f.add_state(8, 0.5, 0, 0, 0);
        // An entry whose record id differs from its key is not valid.
        let stray = f.e.mem.alloc(0x20);
        f.e.mem.set_u32(stray, 3);
        let states = f.this.addr() + 0x64;
        fake_set_value(&mut f.e, &[states, 9, stray]);
        assert!(f.call(0x00adc7d0, &[7]).bool());
        assert!(f.call(0x00adc7d0, &[8]).bool());
        assert!(!f.call(0x00adc7d0, &[9]).bool());
        assert!(!f.call(0x00adc7d0, &[10]).bool());
        assert_eq!(f.call(0x00adc830, &[7]).u32(), 4500);
        assert_eq!(f.call(0x00adc830, &[10]).u32(), 0);
        assert_eq!(f.call(0x00adc8b0, &[7]).f32(), 0.75);
        assert_eq!(f.call(0x00adc8b0, &[10]).f32(), 0.0);
        assert!(f.call(0x00adc910, &[7]).bool());
        assert!(!f.call(0x00adc910, &[8]).bool());
        assert!(!f.call(0x00adc910, &[10]).bool());
        assert_eq!(f.call(0x00adc9a0, &[7]).u32(), 0x1234);
        assert_eq!(f.call(0x00adc9a0, &[10]).u32(), 0);
    }

    #[test]
    fn the_queries_hold_the_state_critical_section() {
        let mut f = Fixture::new();
        f.add_state(7, 0.75, 0x1234, 4500, 1);
        f.e.call_log = Some(vec![]);
        f.call(0x00adc830, &[7]);
        let log = f.e.call_log.take().unwrap();
        let section = f.this.addr() + 0xc4;
        assert_eq!(log[1], (IMPORT_ENTER_CRITICAL_SECTION, vec![section]));
        assert_eq!(log[3], (IMPORT_LEAVE_CRITICAL_SECTION, vec![section]));
        // IsValid reads the map without the critical section.
        f.e.call_log = Some(vec![]);
        f.call(0x00adc7d0, &[7]);
        let log = f.e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == IMPORT_ENTER_CRITICAL_SECTION));
    }

    #[test]
    fn type_mask_wrappers_queue_their_messages() {
        let mut f = Fixture::new();
        f.call(0x00adca20, &[0x11, 0x22, 0x33]);
        f.call(0x00adcaa0, &[0x44, 0x55]);
        f.call(0x00adcb20, &[0x66, 1]);
        f.call(0x00adcb20, &[0x67, 0]);
        f.call(0x00adcc10, &[0x77, 1]);
        f.call(0x00adcc10, &[0x78, 0]);
        f.call(0x00adcd00, &[0x88, 0x99]);
        assert_eq!(
            f.messages(),
            vec![
                plain(0x18, 0x33, 0x11, 0x22),
                plain(0x19, 0, 0x44, 0x55),
                plain(0x1c, 0, 0x66, 0),
                plain(0x1a, 0, 0x67, 0),
                plain(0x1d, 0, 0x77, 0),
                plain(0x1b, 0, 0x78, 0),
                plain(0x2a, 0, 0x88, 0x99),
            ]
        );
    }

    #[test]
    fn shutdown_empties_everything() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.register(LIST_REMOVE_HEAD, fake_list_remove_head);
        f.e.register(MESSAGE_DESTRUCTOR, |_, _| ret(0));
        f.e.register(MANAGER_CACHE_FLUSH, |e, a| {
            // The flag byte is clear while the cache is flushed.
            assert_eq!(a[1], 0);
            assert_eq!(e.mem.u8(CACHE_FLUSH_FLAG), 0);
            ret(0)
        });
        f.e.mem.set_u8(CACHE_FLUSH_FLAG, 1);
        let audio = f.e.global::<u32>(AUDIO_GLOBAL);
        f.e.mem.set_u8(audio + 6, 1);
        // Two messages in the first list, one in the second.
        let messages: Vec<u32> = (0..3).map(|_| f.e.mem.alloc(0x20)).collect();
        list_push(&mut f.e, f.this.addr() + 0x0c, messages[0]);
        list_push(&mut f.e, f.this.addr() + 0x0c, messages[1]);
        list_push(&mut f.e, f.this.addr() + 0x18, messages[2]);
        // The audio thread object: a vtable whose first slot records the
        // deleting-destructor call.
        let deletions = Rc::new(RefCell::new(Vec::new()));
        let log = deletions.clone();
        let vtable = 0x0300_0400;
        f.e.mem.map(vtable, 0x10);
        f.e.mem.set_u32(vtable, 0x0300_0410);
        f.e.register_double(0x0300_0410, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        let thread = f.e.mem.alloc(0x4c);
        f.e.mem.set_u32(thread, vtable);
        f.e.set(f.this, BSAudioManager::pUpdateThread, thread);
        // Two sounds (their deleting destructor is the same slot).
        let sounds = f.this.addr() + 0x54;
        let first = f.e.mem.alloc(0x198);
        f.e.mem.set_u32(first, vtable);
        f.e.mem.set_u32(first + SOUND_ID, 3);
        let second = f.e.mem.alloc(0x198);
        f.e.mem.set_u32(second, vtable);
        f.e.mem.set_u32(second + SOUND_ID, 4);
        fake_set_value(&mut f.e, &[sounds, 3, first]);
        fake_set_value(&mut f.e, &[sounds, 4, second]);
        // A pending update and the state it replaces.
        let updates = f.this.addr() + 0x74;
        let states = f.this.addr() + 0x64;
        let pending = f.e.mem.alloc(0x20);
        fake_set_value(&mut f.e, &[updates, 20, pending]);
        let stale = f.e.mem.alloc(0x20);
        fake_set_value(&mut f.e, &[states, 20, stale]);
        let pause_entries = f.e.mem.alloc(64);
        f.e.set(f.this, BSAudioManager::pPauseEntries, pause_entries);
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        f.e.register_double(SOUND_INFO_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        f.e.call_log = Some(vec![]);
        f.call(0x00ad9870, &[]);
        let log = f.e.call_log.take().unwrap();
        let removed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == LIST_REMOVE_HEAD)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(
            removed,
            vec![
                f.this.addr() + 0x0c,
                f.this.addr() + 0x0c,
                f.this.addr() + 0x18
            ]
        );
        assert_eq!(f.e.mem.u32(f.this.addr() + 0x14), 0);
        assert_eq!(f.e.mem.u32(f.this.addr() + 0x20), 0);
        // The audio thread was deleted and forgotten.
        assert_eq!(f.e.get(f.this, BSAudioManager::pUpdateThread), 0);
        let deletions = deletions.borrow();
        assert!(deletions.contains(&(thread, 1)));
        assert!(deletions.contains(&(first, 1)));
        assert!(deletions.contains(&(second, 1)));
        // Sound removed from the list; the state update replaced the stale
        // record, which was deleted along with the pending one; both maps
        // end up empty of pending work.
        assert_eq!(f.e.mem.u32(sounds + 0xc), 0);
        assert_eq!(f.e.mem.u32(updates + 0xc), 0);
        let mut gone = destroyed.borrow().clone();
        gone.sort_unstable();
        assert!(gone.contains(&stale));
        assert!(gone.contains(&pending));
        // The moving-sound map was cleared and the pause list freed.
        assert!(log.contains(&(MAP_CLEAR, vec![f.this.addr() + 0x84])));
        assert!(log.contains(&(DEALLOCATE, vec![MEMORY_MANAGER, pause_entries])));
        assert!(log.contains(&(MANAGER_CACHE_FLUSH, vec![f.this.addr(), 0])));
        assert_eq!(f.e.mem.u8(CACHE_FLUSH_FLAG), 1);
    }

    #[test]
    fn shutdown_updates_the_audio_first_when_not_multithreaded() {
        let mut f = Fixture::new();
        install_record_doubles(&mut f.e);
        f.e.register(MANAGER_CACHE_FLUSH, |_, _| ret(0));
        f.e.register(LIST_REMOVE_HEAD, fake_list_remove_head);
        f.e.mem.set_u32(MANAGER_BUILT_GUARD, 1);
        f.e.register(MANAGER_PROCESS, |_, _| ret(0));
        f.e.register(AUDIO_PRECACHE, |_, _| ret(0));
        f.e.call_log = Some(vec![]);
        f.call(0x00ad9870, &[]);
        let log = f.e.call_log.take().unwrap();
        // Three `Update` runs, each ending with `Process`.
        assert_eq!(log.iter().filter(|c| c.0 == MANAGER_PROCESS).count(), 3);
    }
}
