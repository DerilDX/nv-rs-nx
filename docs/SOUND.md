# Sound: the platform's audio layers, measured

How the platform answers the game's sound imports, and how far each answer has
been checked against the player's own libraries. Measurements are private
recordings under `build43-scratch/measure` (never committed), made by the tools
in `scripts/`.

| Layer | Module | What it replaces | Checked against | Result |
| --- | --- | --- | --- | --- |
| Music | `platform/dshow.rs`, `crates/mp3` | DirectShow's filter graph and MP3 decoder | `scripts/dshow-measure` | Graph answers as measured. All 199 tracks are the same length, and 99.9748% of samples are identical |
| Wave files | `platform/mmio.rs` | `mmio*` | `scripts/mmio-measure` | Edge cases as measured |
| DirectSound | `platform/dsound.rs` | DirectSound 8 objects, cursors | `scripts/dsound-capture` | Answers and cursors as measured |
| Mixer | `platform/mixer.rs`, `platform/sound_out.rs` | DirectSound's mixer, the device | `scripts/dsmix-measure` (WASAPI loopback) | 2D path bit-exact; resampler, 3D, distortion not provided |
| Voices and sounds | `platform/vorbisfile.rs`, `crates/vorbis` | `libvorbisfile.dll`/`libvorbis.dll` (1.2.0) | `scripts/vorbis-record` | Bit-exact at 24-bit and 53-bit x87; 64-bit not provided |
| Movie sound | `platform/bink.rs` | Bink's DirectSound sound system | `nv-bink --sound-system`, `--wait --sound` | `BinkSetSoundSystem` answers 1 as measured; the movie track plays as a mixer voice on the clock schedule (output off by default) |

## Mixer (DirectSound 8 to the device)

These were measured on a device without audio enhancements (Virtual Desktop Audio, 48 kHz,
32-bit float stereo). The default device's enhancements smear the signal by
-17 dB, so it cannot be used for these measurements.

**Exact:**
- Samples become `f32` as x/32768.
- The volume and pan gain law is exact for all 429 volumes and 25 pans (see `mixer.rs`).
- A mono buffer goes to both channels.
- Voices are added in `f32`, the one started last first. This holds for 47,996 of
  48,000 samples. The other 4 are the tail of a voice started 2 frames into a device
  period.

**Not exact, so the platform stops with "not provided":**
- **Resampling.** It is a long polyphase filter, not linear; least-squares fits only
  reach -34 dB.
- **3D.** Gains come in 0.1 dB steps, but the rule from position to gain is not derived
  yet.
- **Distortion.** Not analysed.

## Vorbis

`scripts/vorbis-record` drives the player's `libvorbisfile.dll` the way the game
does (`00af1e00`, `00af2100`). It reads every `.ogg` in the archives (63,575 files)
and records the decoded bytes. At 24-bit and at 53-bit x87 precision, `crates/vorbis`
gives the same bytes and the same `ov_pcm_total` for all of them. It follows the public
libvorbis 1.2.0 source's order of operations (BSD, see `LEGAL.md`).

The library computes on the x87 unit, so its output depends on the decoding
thread's precision control: 62,819 of the files differ between 24 and 53 bits.
At 24 bits every operation rounds to 24 bits, which is the same as `f32` in the
source's order.

**Which thread decodes is not measured yet.** The thread is the one that runs
`AudioLoadTask::Run` (`00ae7050`), whose path to the decoder is
`00aeb270`, `00aebff0`, `00af2100`. Load tasks go to the IO manager (`01202d98`),
so they probably run on a task-manager thread. Such a thread starts at
`0x027F` (53-bit) unless something on it changes the control word.

### 53 bits: how the match was found

The recorder's `--blocks` mode calls `libvorbis.dll`'s public API
(`vorbis_synthesis`) and records every block after the inverse MDCT.
`VORBIS_PACKETS=<file>` does the same for crafted streams. A crafted stream has
a floor of 1.0 and exact spectra (impulses, random values), so its blocks are
the DLL's inverse MDCT of a known input.

Measurement alone did not find the 53-bit model. None of these came close:
rounding at each C assignment to `float`, with `float` locals rounded or kept;
angles at 24 or 53 bits; 486 per-stage combinations.

With the maintainer's leave (2026-10-24), the DLL's transform was then read as
instructions: its own Ghidra project in the agent's scratch folder, not the
shared servers.

**The 53-bit path follows the DLL's instruction order.** That order is the C
source's, statement for statement:
- every `float` local of the transform is stored to the stack, so it is rounded to
  `f32`;
- each expression is evaluated in registers at the thread's precision and rounded
  to `f32` when stored.

The one difference from the public source is the trig tables' `M_PI`: the DLL
uses `pi` rounded to `f32` (3.14159274...). At 24 bits that makes no difference,
because `pi / n` rounds to the same value. At 53 bits it changes every angle.
With that constant, the crafted blocks and all 63,575 game files match at
53 bits as well.

No bytes from the DLL are in the repository. The constant is the `f32` value of
`pi`, computed in the code.

`platform/vorbisfile.rs` decodes at the calling thread's precision: 24 or 53
bits. 64-bit precision is not provided (not measured).

## Bink with sound

The game calls `BinkSetSoundSystem(BinkOpenDirectSound, 0)` (`00ec1680`). An
earlier measurement (2026-10-23) recorded the answer as 0, and `platform/bink.rs`
was written to match it: no sound, frames timed by the clock. That measurement
was wrong: `nv-bink` passed a garbled export name to `GetProcAddress`, so it
handed the DLL a NULL function.

Measured again on 2026-10-24 (`nv-bink --sound-system`, private recordings in
`bink-sound-2026-10-24`):

- **The answer is 1**, with or without a window.
- **That first run without a window** (`nv-bink --wait 300 --sound`): the frames came
  about 3 ms late with about ±2 ms of jitter. With `--hook-dsound` it turned out the
  library's `Play` had failed there (no window for the priority level), so that run
  did not have sound. The runs below have a window and do.

### What the library does with DirectSound (measured)

`nv-bink --wait --sound --window --hook-dsound` records every DirectSound call the
library makes while the intro plays with sound:
- **Device:** its own (`DirectSoundCreate(NULL)`), at the priority level on a window.
  Without a window the level fails and so does `Play` (`DSERR_PRIOLEVELNEEDED`).
- **Primary buffer:** given a format and played looping.
- **One secondary buffer:** 115,712 bytes of 48 kHz 16-bit stereo (0.6 s), flags `0x180e0`
  (frequency, pan and volume controls, `GETCURRENTPOSITION2`, `GLOBALFOCUS`). Frequency is
  set to 48000, pan and volume to 0.
- **Filling:** at the first `BinkDoFrame` the buffer is cleared, then filled whole.
  `Play` (looping) follows. Then, about every two frames, the part already played is
  refilled: 11,520 or 13,440 bytes, wrapping round. The play cursor is polled about every
  13 ms.
- **When `Play` comes:** 50 to 540 ms after the fill, varying from run to run.
- **Frames with sound:** frame 1 is due one frame time after the sound starts. From
  there the frames keep exactly the clock schedule they keep without sound: 600 frames,
  the same per-frame times to within 0.05 ms.

### What the platform does

`BinkSetSoundSystem` answers 1, as measured. The game calls it at start-up
(`BGSMoviePlayer`, `008670e0`) whether or not a movie plays.

A movie with a sound track has its track decoded whole at `BinkOpen` by
`crates/bink` (within one step of the DLL, docs/MOVIES.md). The track becomes one
mixer voice (`dsound::play_stream`), started at the first `BinkDoFrame`, when the
clock schedule starts. It is held while the movie is paused (`BinkPause`) and ended
at `BinkClose`. It is heard only when the host has turned the output on, which is
off by default.

This gives the same samples on the same schedule as the DLL. What is not
reproduced:
- The DLL streams the track through a 0.6 s buffer instead of one voice; only the
  samples heard matter, and they are the same.
- The DLL's 50 to 540 ms device start, which varies from run to run.

Still not provided: more than one sound track, a track the decoder cannot read, and a
rate other than the device's.
