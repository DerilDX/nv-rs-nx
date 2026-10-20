//! `fallout shared/region system/tesregiondataweather.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f50b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather` scalar deleting destructor (engine map name): runs the destructor body `004f50e0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_weather_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_50e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtable of `TESRegionDataWeather` (RTTI `.?AVTESRegionDataWeather@@`, read at the word before it).
const TES_REGION_DATA_WEATHER_VTABLE: u32 = 0x0102_3e18;
/// `00582320` (unnamed in the engine map): the destructor of the weather list at +8.
const WEATHER_LIST_DTOR_00582320: u32 = 0x0058_2320;
/// The `TESRegionData` destructor body (`004f14a0`).
const TES_REGION_DATA_DTOR: u32 = 0x004f_14a0;
/// `0046e8a0` (unnamed in the engine map; no arguments): a count the
/// loader compares before and after reading the chunk (the warnings so far).
const WARNING_COUNT_0046E8A0: u32 = 0x0046_e8a0;
/// `MessageHandler::IncDisableWarningCount` (`0043b2b0`, engine map; cdecl, one word).
const INC_DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;
/// `TESWeatherList::LoadWeatherListChunk` (`00582940`, engine map; cdecl,
/// the list, the file and the form).
const LOAD_WEATHER_LIST_CHUNK: u32 = 0x0058_2940;
/// The virtual slot (byte offset) of the form the warning names (its
/// identifying text, passed to the format).
const SLOT_FORM_TEXT: u32 = 0x130;
/// `005b5e40` (unnamed in the engine map; cdecl, a format and its
/// arguments): prints the warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `"Warnings were encountered while loading weather list chunk from ..."`
/// (the format, with the form's text).
const WEATHER_WARNING_FORMAT: u32 = 0x0102_3e40;

// Translated from 004f50e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f50e0 - its FS:[0] exception frame (the unwind state that runs the base destructor when the list's destructor throws) is not translated
/// `TESRegionDataWeather` destructor body (unnamed in the engine map):
/// stores the class's vtable, destroys the weather list at +8 (`00582320`),
/// then runs the `TESRegionData` destructor body.
pub fn fn_004f50e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_REGION_DATA_WEATHER_VTABLE);
    e.call(WEATHER_LIST_DTOR_00582320, &args![this.addr() + 8]);
    e.call(TES_REGION_DATA_DTOR, &args![this]);
}

// Translated from 004f51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather::LoadRegionDataWeather` (Xbox PDB; `thiscall`,
/// `RET 8`): reads the weather list chunk of `file` into the list at +8
/// (`TESWeatherList::LoadWeatherListChunk(list, file, form)`) between
/// `IncDisableWarningCount(1)` and `(0)`. When the count `0046e8a0` grew
/// (signed comparison) and `form` is set, it prints the weather-list warning
/// with the text of the form's virtual slot +0x130.
pub fn tes_region_data_weather_load_region_data_weather(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
    form: Ptr,
) {
    let before = e.call(WARNING_COUNT_0046E8A0, &args![]).i32();
    e.call(INC_DISABLE_WARNING_COUNT, &args![1u32]);
    e.call(LOAD_WEATHER_LIST_CHUNK, &args![this.addr() + 8, file, form]);
    e.call(INC_DISABLE_WARNING_COUNT, &args![0u32]);
    let after = e.call(WARNING_COUNT_0046E8A0, &args![]).i32();
    if after > before && !form.is_null() {
        let text = e.vcall(form.addr(), SLOT_FORM_TEXT, &args![]).u32();
        e.call(PRINT_WARNING_005B5E40, &args![WEATHER_WARNING_FORMAT, text]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_50b0, tes_region_data_weather_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_50e0, fn_004f50e0(Ptr)),
        entry!(
            0x004f_51f0,
            tes_region_data_weather_load_region_data_weather(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod weather_tests {
    use super::*;

    #[test]
    fn at_004f50e0_destroys_the_list_then_the_base() {
        let mut e = Engine::new();
        e.register(WEATHER_LIST_DTOR_00582320, |_, _| Ret::default());
        e.register(TES_REGION_DATA_DTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_50e0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_WEATHER_VTABLE);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_50e0).collect();
        assert_eq!(
            calls,
            vec![
                (WEATHER_LIST_DTOR_00582320, vec![this + 8]),
                (TES_REGION_DATA_DTOR, vec![this]),
            ]
        );
    }

    fn run(grows: bool, with_form: bool) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        let mut count = 0i32;
        e.register_double(WARNING_COUNT_0046E8A0, move |_, _| {
            let value = count;
            if grows {
                count += 1;
            }
            value.into_ret()
        });
        e.register(INC_DISABLE_WARNING_COUNT, |_, _| Ret::default());
        e.register(LOAD_WEATHER_LIST_CHUNK, |_, _| Ret::default());
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x4d];
        slots[0x4c] = 0x0f02_1000;
        e.put_vtable(0x0f02_0000, &slots);
        e.register(0x0f02_1000, |_, _| 0x0bad_f00du32.into_ret());
        let form = if with_form {
            let form = e.mem.alloc(0x20);
            e.mem.set_u32(form, 0x0f02_0000);
            form
        } else {
            0
        };
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_51f0, &args![this, 0x0123_4567u32, form]);
        let log = e.call_log.take().unwrap();
        let load = log.iter().find(|c| c.0 == LOAD_WEATHER_LIST_CHUNK).unwrap();
        assert_eq!(load.1, vec![this + 8, 0x0123_4567, form]);
        let disables: Vec<_> = log
            .iter()
            .filter(|c| c.0 == INC_DISABLE_WARNING_COUNT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(disables, vec![vec![1], vec![0]]);
        log
    }

    #[test]
    fn at_004f51f0_warns_when_warnings_grew_for_a_form() {
        let log = run(true, true);
        let print = log.iter().find(|c| c.0 == PRINT_WARNING_005B5E40).unwrap();
        assert_eq!(print.1, vec![WEATHER_WARNING_FORMAT, 0x0bad_f00d]);
    }

    #[test]
    fn at_004f51f0_stays_quiet_otherwise() {
        for (grows, with_form) in [(false, true), (true, false), (false, false)] {
            let log = run(grows, with_form);
            assert!(log.iter().all(|c| c.0 != PRINT_WARNING_005B5E40));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f50b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_50e0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_50b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_50b0)
                .collect();
            let mut expected = vec![(0x004f_50e0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
