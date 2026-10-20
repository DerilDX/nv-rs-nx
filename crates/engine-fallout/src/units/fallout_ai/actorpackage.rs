//! `fallout/ai/actorpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores 0 in the word at `this + 4`. The engine map names it
/// `ActorPackage::NullInstanceData` (Xbox PDB), but the body is shared by
/// identical-code folding (constructors such as `00487420` call it on their
/// own objects, as do `00475350`, `0048e7a0`, ...), so it is named by
/// address.
pub fn fn_008c6060(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 008c5ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::Clear` (engine map name; the object is the four word record `008c5e20`
/// builds). Lets go of the package at `+0` and the object at `+4` and restores the start values.
///
/// - The package at `+0`, when set: the pointer in the word at `+0x208` of the object at
///   `0x011dea3c` is cleared when it is this package; the package's count is lowered
///   (`008c6030`) and when it reaches 0 and `00674d40` agrees the package is handed to
///   `0085a2e0` of the object at `0x011de45c` if `0047c850` of it answers true, otherwise
///   destroyed through virtual slot `+0x10` (argument 1). The word is cleared in either case.
/// - The object at `+4`, when set, is destroyed through virtual slot `+0` (argument 1) and the
///   word cleared.
/// - The float at `+0x10` becomes the float at `0x01012054`, the word at `+0xc` 0xffffffff and
///   the words at `+0x14` and `+8` 0.
pub fn actorpackage_clear(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let package = e.mem.u32(base);
    if package != 0 {
        let globals = e.global::<u32>(0x011d_ea3c);
        if package == e.mem.u32(globals + 0x208) {
            let globals = e.global::<u32>(0x011d_ea3c);
            e.mem.set_u32(globals + 0x208, 0);
        }
        let package = e.mem.u32(base);
        let remaining = e.call(0x008c_6030, &args![package]).u32();
        if remaining == 0 && e.call(0x0067_4d40, &args![package]).bool() {
            let package = e.mem.u32(base);
            e.mem.set_u32(base, 0);
            let manager = e.global::<u32>(0x011d_e45c);
            if e.call(0x0047_c850, &args![manager]).bool() {
                let manager = e.global::<u32>(0x011d_e45c);
                e.call(0x0085_a2e0, &args![manager, package]);
            } else {
                e.vcall(package, 0x10, &args![1u32]);
            }
        } else {
            e.mem.set_u32(base, 0);
        }
    }
    let object = e.mem.u32(base + 4);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
        e.mem.set_u32(base + 4, 0);
    }
    let value = x87_quiet(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(base + 0x10, value);
    e.mem.set_u32(base + 0xc, 0xffff_ffff);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 8, 0);
}

layout! {
    /// `ActorPackage` (Xbox PDB, 0x18 bytes), the fields `MoveTo` uses; the PC offsets are
    /// the ones the code reads and writes (`+0`, `+4`, `+0xc`, `+0x10`).
    pub struct ActorPackage: 0x18 {
        /// `pPackage` (Xbox PDB): `TESPackage*`.
        0x00 pPackage: u32,
        /// `pData` (Xbox PDB): `ActorPackageData*`.
        0x04 pData: u32,
        /// `iCurrentProcedureIndex` (Xbox PDB).
        0x0C iCurrentProcedureIndex: i32,
        /// `fPackageStartTime` (Xbox PDB).
        0x10 fPackageStartTime: f32,
    }
}

// Translated from 008c6130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::MoveTo` (Xbox PDB; `thiscall`, `RET 4`): `this` is the package that is
/// moved out, `destination` the one that takes it (the word on the stack). Copies `+0`,
/// `+4`, `+0xc` and the float at `+0x10` (`FLD`/`FSTP`, which quiets a signalling NaN) to
/// `destination`, then resets `this` to the empty values: `+0` and `+4` to 0, `+0xc` to
/// 0xffffffff and the float at `+0x10` to the float at `0x01012054` (read through memory,
/// the same load and store). `+8` and `+0x14` are not touched.
pub fn actor_package_move_to(
    e: &mut Engine,
    this: Ptr<ActorPackage>,
    destination: Ptr<ActorPackage>,
) {
    let package = e.get(this, ActorPackage::pPackage);
    e.set(destination, ActorPackage::pPackage, package);
    let data = e.get(this, ActorPackage::pData);
    e.set(destination, ActorPackage::pData, data);
    let procedure = e.get(this, ActorPackage::iCurrentProcedureIndex);
    e.set(destination, ActorPackage::iCurrentProcedureIndex, procedure);
    let start_time = x87_quiet(e.get(this, ActorPackage::fPackageStartTime));
    e.set(destination, ActorPackage::fPackageStartTime, start_time);
    e.set(this, ActorPackage::pPackage, 0);
    e.set(this, ActorPackage::pData, 0);
    e.set(this, ActorPackage::iCurrentProcedureIndex, -1);
    let empty_start_time = x87_quiet(e.global::<f32>(0x0101_2054));
    e.set(this, ActorPackage::fPackageStartTime, empty_start_time);
}

/// Interlocked decrement of the word at the address (`004019a0`, cdecl;
/// the engine map's name `WaitForSingleObjectEx` belongs to folded code).
const INTERLOCKED_DECREMENT: u32 = 0x0040_19a0;
/// Interlocked increment of the word at the address (`0040b460`, cdecl).
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
/// `TESPackage`'s reference count (+0x7C).
const PACKAGE_REF_COUNT: u32 = 0x7c;
/// The `Calendar` singleton (`011de7b8`).
const CALENDAR: u32 = 0x011d_e7b8;
/// `Calendar::GetHour` (`00867da0`, a `float` in `ST0`).
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// The package's type byte (`0041ca90`, `fallout shared/extradatalist.cpp`
/// by the engine map).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `ActorPackageData::CreateActorPackageData` (`008c6ca0`, cdecl, the
/// package type).
const CREATE_ACTOR_PACKAGE_DATA: u32 = 0x008c_6ca0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, the buffer, the form and
/// a flag word).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// Writes bytes into the save buffer (`00865e50`, the buffer, the address,
/// the size and a flag word).
const SAVE_DATA: u32 = 0x0086_5e50;
/// A `TESForm` getter (`0084e3a0`) whose result `00469860` checks.
const FORM_VALUE_0084E3A0: u32 = 0x0084_e3a0;
/// A `TESDataHandler` check (`00469860`, the data handler and the value).
const DATA_HANDLER_CHECK_00469860: u32 = 0x0046_9860;
/// The `TESDataHandler` singleton pointer (`011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;

// Translated from 008c6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::DecRefCount` (Xbox PDB): interlocked decrement of the
/// package's reference count (+0x7C); returns the new count.
pub fn tes_package_dec_ref_count(e: &mut Engine, this: Ptr) -> u32 {
    let count = this.addr().wrapping_add(PACKAGE_REF_COUNT);
    e.call(INTERLOCKED_DECREMENT, &args![count]).u32()
}

// Translated from 008c6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IncRefCount` (Xbox PDB): interlocked increment of the
/// package's reference count (+0x7C); returns the new count.
pub fn tes_package_inc_ref_count(e: &mut Engine, this: Ptr) -> u32 {
    let count = this.addr().wrapping_add(PACKAGE_REF_COUNT);
    e.call(INTERLOCKED_INCREMENT, &args![count]).u32()
}

// Translated from 008c6080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::InitFromPackage` (Xbox PDB): nothing when `package` is
/// already this one's; otherwise clears this one, and for a package that is
/// not null takes a reference on it, stores the calendar's hour as the start
/// time, the package, new data for the package's type
/// (`ActorPackageData::CreateActorPackageData`), procedure index 0 and
/// `target` at +8.
pub fn actor_package_init_from_package(e: &mut Engine, this: Ptr, package: Ptr, target: u32) {
    let base = this.addr();
    if package.addr() == e.mem.u32(base) {
        return;
    }
    actorpackage_clear(e, this);
    if package.is_null() {
        return;
    }
    tes_package_inc_ref_count(e, package);
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    e.mem.set_u32(base + 0x10, x87_store32(hour));
    e.mem.set_u32(base, package.addr());
    let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    let data = e.call(CREATE_ACTOR_PACKAGE_DATA, &args![kind]).u32();
    e.mem.set_u32(base + 4, data);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 8, target);
}

// Translated from 008c65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::SaveGame` (Xbox PDB): writes the package's form id;
/// for a package, when the data handler check (`00469860` on `0084e3a0`'s
/// value) passes, its type byte (0xFF when the package's virtual `+0x15C`
/// refuses the buffer's version) and, unless 0xFF, the package's own data
/// (virtual `+0x54`); then the data's type byte (its virtual `+8`, 0xFF
/// without data) and, unless 0xFF, the data's own save (virtual `+0xC`), the
/// words at +0xC, +0x10 and +0x14 and the form id of the target at +8.
pub fn actor_package_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    let version = e.vcall(buffer.addr(), 8, &args![]).u32();
    let package = e.mem.u32(base);
    e.call(SAVE_FORM_ID, &args![buffer, package, 0u32]);
    if e.mem.u32(base) == 0 {
        return;
    }
    e.with_stack(2, |e, bytes| {
        let package_type = bytes.addr();
        let data_type = bytes.addr() + 1;
        let package = e.mem.u32(base);
        let value = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
        let data_handler = e.global::<u32>(DATA_HANDLER);
        if e.call(DATA_HANDLER_CHECK_00469860, &args![data_handler, value])
            .bool()
        {
            let package = e.mem.u32(base);
            let kind = e.call(PACKAGE_TYPE, &args![package]).u8();
            e.mem.set_u8(package_type, kind);
            let package = e.mem.u32(base);
            if !e.vcall(package, 0x15c, &args![version]).bool() {
                e.mem.set_u8(package_type, 0xff);
            }
            e.call(SAVE_DATA, &args![buffer, package_type, 1u32, 0u32]);
            if e.mem.u8(package_type) as i8 != -1 {
                let package = e.mem.u32(base);
                e.vcall(package, 0x54, &args![buffer]);
            }
        }
        let data = e.mem.u32(base + 4);
        let kind = if data != 0 {
            e.vcall(data, 8, &args![]).u32()
        } else {
            0xffff_ffff
        };
        e.mem.set_u8(data_type, kind as u8);
        e.call(SAVE_DATA, &args![buffer, data_type, 1u32, 0u32]);
        if e.mem.u8(data_type) as i8 != -1 {
            let data = e.mem.u32(base + 4);
            e.vcall(data, 0xc, &args![buffer]);
        }
    });
    e.call(SAVE_DATA, &args![buffer, base + 0xc, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, base + 0x10, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, base + 0x14, 4u32, 0u32]);
    let target = e.mem.u32(base + 8);
    e.call(SAVE_FORM_ID, &args![buffer, target, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c_6130,
            actor_package_move_to(Ptr<ActorPackage>, Ptr<ActorPackage>)
        ),
        entry!(0x008c5ef0, actorpackage_clear(Ptr)),
        entry!(0x008c6060, fn_008c6060(Ptr)),
        entry!(0x008c6030, tes_package_dec_ref_count(Ptr) -> u32),
        entry!(0x008c6100, tes_package_inc_ref_count(Ptr) -> u32),
        entry!(0x008c6080, actor_package_init_from_package(Ptr, Ptr, u32)),
        entry!(0x008c65d0, actor_package_save_game(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_only_the_word_at_4() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        for offset in [0, 4, 8] {
            e.mem.set_u32(object + offset, 0xdead_0000 + offset);
        }
        e.call(0x008c_6060, &[object]);
        assert_eq!(e.mem.u32(object), 0xdead_0000);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0xdead_0008);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use super::*;

    const STATE: u32 = 0x011d_ea3c;
    const MANAGER: u32 = 0x011d_e45c;
    const OWNER: u32 = 0x0e00_0000;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with an owner object (the pending package of the owner at `+0x208` is
    /// `pending`), a package whose slot `+0x10` and a sub-object whose slot `+0` are doubles that
    /// log, and doubles for the callees; `count` is what `008c6030` answers, `agrees` what
    /// `00674d40` answers and `manager` what `0047c850` answers.
    fn setup(
        pending: u32,
        count: u32,
        agrees: bool,
        manager: bool,
    ) -> (Engine, Log, u32, u32, u32) {
        let mut e = Engine::new();
        let log: Log = Log::default();
        let record = e.mem.alloc(0x18);
        let package = e.mem.alloc(8);
        let object = e.mem.alloc(8);
        e.map(OWNER, 0x300);
        for global in [STATE, MANAGER, 0x0101_2054] {
            e.map(global, 4);
        }
        e.mem.set_u32(OWNER + 0x208, pending);
        e.set_global(STATE, OWNER);
        e.set_global(MANAGER, 0x0f00_0000u32);
        e.set_global(0x0101_2054, 0x3f80_0000u32);
        e.put_vtable(0x0090_1000, &[0x0090_0100, 0, 0, 0, 0x0090_0110]);
        e.mem.set_u32(package, 0x0090_1000);
        e.mem.set_u32(object, 0x0090_1000);
        for (address, eax) in [
            (0x008c_6030, count),
            (0x0067_4d40, u32::from(agrees)),
            (0x0047_c850, u32::from(manager)),
            (0x0085_a2e0, 0),
            (0x0090_0100, 0),
            (0x0090_0110, 0),
        ] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        (e, log, record, package, object)
    }

    #[test]
    fn an_empty_record_is_only_reset() {
        let (mut e, log, record, _, _) = setup(0, 0, true, true);
        for offset in [0, 4, 8, 0x14] {
            e.mem.set_u32(record + offset, 0);
        }
        e.mem.set_u32(record + 8, 5);
        e.call(0x008c_5ef0, &[record]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(record + 8), 0);
        assert_eq!(e.mem.u32(record + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(record + 0x10), 0x3f80_0000);
        assert_eq!(e.mem.u32(record + 0x14), 0);
    }

    #[test]
    fn a_package_that_is_still_counted_is_only_let_go() {
        let (mut e, log, record, package, _) = setup(0x1234, 2, true, true);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(*log.borrow(), [(0x008c_6030, vec![package])]);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn the_last_reference_hands_the_package_to_the_manager() {
        let (mut e, log, record, package, _) = setup(0, 0, true, true);
        e.mem.set_u32(OWNER + 0x208, package);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(
            *log.borrow(),
            [
                (0x008c_6030, vec![package]),
                (0x0067_4d40, vec![package]),
                (0x0047_c850, vec![0x0f00_0000]),
                (0x0085_a2e0, vec![0x0f00_0000, package]),
            ]
        );
        assert_eq!(e.mem.u32(record), 0);
        assert_eq!(e.mem.u32(OWNER + 0x208), 0);
    }

    #[test]
    fn without_a_manager_the_package_is_destroyed() {
        let (mut e, log, record, package, _) = setup(0x99, 0, true, false);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(
            log.borrow().last().unwrap(),
            &(0x0090_0110, vec![package, 1])
        );
        assert_eq!(e.mem.u32(OWNER + 0x208), 0x99);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn a_package_that_disagrees_is_only_let_go() {
        let (mut e, log, record, package, _) = setup(0, 0, false, true);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn the_sub_object_is_destroyed() {
        let (mut e, log, record, _, object) = setup(0, 0, true, true);
        e.mem.set_u32(record + 4, object);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(*log.borrow(), [(0x0090_0100, vec![object, 1])]);
        assert_eq!(e.mem.u32(record + 4), 0);
    }
}

#[cfg(test)]
mod move_to_tests {
    use super::*;

    const EMPTY_START_TIME: u32 = 0x0101_2054;

    fn fixture() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(EMPTY_START_TIME, 4);
        e.set_global(EMPTY_START_TIME, (-1.0f32).to_bits());
        let from = e.mem.alloc(0x18);
        let to = e.mem.alloc(0x18);
        (e, from, to)
    }

    #[test]
    fn moves_the_package_and_resets_the_source() {
        let (mut e, from, to) = fixture();
        e.mem.set_u32(from, 0x1111);
        e.mem.set_u32(from + 4, 0x2222);
        e.mem.set_u32(from + 8, 0x3333);
        e.mem.set_u32(from + 0xc, 5);
        e.mem.set_u32(from + 0x10, 12.5f32.to_bits());
        e.mem.set_u32(from + 0x14, 0x4444);
        e.mem.set_u32(to + 8, 0x7777);
        e.mem.set_u32(to + 0x14, 0x8888);
        e.call(0x008c_6130, &args![from, to]);
        assert_eq!(e.mem.u32(to), 0x1111);
        assert_eq!(e.mem.u32(to + 4), 0x2222);
        assert_eq!(e.mem.u32(to + 8), 0x7777);
        assert_eq!(e.mem.u32(to + 0xc), 5);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0x10)), 12.5);
        assert_eq!(e.mem.u32(to + 0x14), 0x8888);
        assert_eq!(e.mem.u32(from), 0);
        assert_eq!(e.mem.u32(from + 4), 0);
        assert_eq!(e.mem.u32(from + 8), 0x3333);
        assert_eq!(e.mem.u32(from + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(from + 0x10), (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(from + 0x14), 0x4444);
    }

    #[test]
    fn quiets_signalling_nans_in_the_start_time() {
        let (mut e, from, to) = fixture();
        e.set_global(EMPTY_START_TIME, 0x7fa0_0003u32);
        e.mem.set_u32(from + 0x10, 0xffa0_0001);
        e.call(0x008c_6130, &args![from, to]);
        assert_eq!(e.mem.u32(to + 0x10), 0xffe0_0001);
        assert_eq!(e.mem.u32(from + 0x10), 0x7fe0_0003);
    }

    fn calls_to_008c(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn package_ref_counts_go_through_the_interlocked_helpers() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |_, _| 4u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| 6u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_6030, &args![0x1000u32]).u32(), 4);
        assert_eq!(e.call(0x008c_6100, &args![0x1000u32]).u32(), 6);
        assert_eq!(calls_to_008c(&e, INTERLOCKED_DECREMENT), vec![vec![0x107c]]);
        assert_eq!(calls_to_008c(&e, INTERLOCKED_INCREMENT), vec![vec![0x107c]]);
    }

    fn init_doubles(e: &mut Engine) {
        e.map(0x011d_ea3c, 4);
        let holder = e.mem.alloc(0x210);
        e.set_global(0x011d_ea3c, holder);
        e.map(0x0101_2054, 4);
        e.set_global(0x0101_2054, -1.0f32);
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| 2u32.into_ret());
        e.register(CALENDAR_GET_HOUR, |_, _| 7.5f32.into_ret());
        e.register(PACKAGE_TYPE, |_, _| 0x12u32.into_ret());
        e.register(CREATE_ACTOR_PACKAGE_DATA, |_, _| 0x5150u32.into_ret());
    }

    #[test]
    fn init_from_package_takes_the_new_package() {
        let mut e = Engine::new();
        init_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        let package = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x008c_6080, &args![this, package, 0x77u32]);
        assert_eq!(e.mem.u32(this), package);
        assert_eq!(e.mem.u32(this + 4), 0x5150);
        assert_eq!(e.mem.u32(this + 8), 0x77);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.f32(this + 0x10), 7.5);
        assert_eq!(
            calls_to_008c(&e, INTERLOCKED_INCREMENT),
            vec![vec![package + 0x7c]]
        );
        assert_eq!(
            calls_to_008c(&e, CREATE_ACTOR_PACKAGE_DATA),
            vec![vec![0x12]]
        );
        // The same package again: nothing.
        e.call_log = Some(vec![]);
        e.call(0x008c_6080, &args![this, package, 0x99u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 8), 0x77);
    }

    #[test]
    fn init_from_a_null_package_only_clears() {
        let mut e = Engine::new();
        init_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, 0x44);
        // With +0 already null a null package returns at once: start from
        // an old package.
        let old = e.mem.alloc(0x80);
        e.mem.set_u32(this, old);
        e.register(0x0067_4d40, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008c_6080, &args![this, 0u32, 0x55u32]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(calls_to_008c(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to_008c(&e, CALENDAR_GET_HOUR).is_empty());
    }

    /// A save buffer whose virtual `+8` answers the version, and doubles of
    /// the save helpers that record the bytes written.
    fn save_setup(e: &mut Engine, check: bool) -> u32 {
        e.put_vtable(0x0310_0000, &[0, 0, 0x0310_1000]);
        e.register(0x0310_1000, |_, _| 9u32.into_ret());
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x0310_0000);
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.register(FORM_VALUE_0084E3A0, |_, _| 0x33u32.into_ret());
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x0400_0000u32);
        if check {
            e.register(DATA_HANDLER_CHECK_00469860, |_, _| true.into_ret());
        } else {
            e.register(DATA_HANDLER_CHECK_00469860, |_, _| false.into_ret());
        }
        e.register(PACKAGE_TYPE, |_, _| 0x05u32.into_ret());
        buffer
    }

    fn saved_bytes(e: &Engine) -> Vec<(u32, u32)> {
        calls_to_008c(e, SAVE_DATA)
            .iter()
            .map(|w| {
                (
                    w[2],
                    if w[2] == 1 {
                        u32::from(e.mem.u8(w[1]))
                    } else {
                        w[1]
                    },
                )
            })
            .collect()
    }

    #[test]
    fn save_game_without_a_package_writes_only_its_form_id() {
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, true);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        assert_eq!(calls_to_008c(&e, SAVE_FORM_ID), vec![vec![buffer, 0, 0]]);
        assert!(calls_to_008c(&e, SAVE_DATA).is_empty());
    }

    #[test]
    fn save_game_writes_the_package_and_its_data() {
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, true);
        // Package vtable: +0x54 saves, +0x15C accepts the version.
        let mut slots = vec![0u32; 0x58];
        slots[0x54 / 4] = 0x0310_2054;
        slots[0x15c / 4] = 0x0310_215c;
        e.put_vtable(0x0320_0000, &slots);
        e.register(0x0310_2054, |_, _| Ret::default());
        e.register(0x0310_215c, |_, a| (a[1] == 9).into_ret());
        let package = e.mem.alloc(0x80);
        e.mem.set_u32(package, 0x0320_0000);
        // Data vtable: +8 type 3, +0xC saves.
        e.put_vtable(0x0330_0000, &[0, 0, 0x0310_3008, 0x0310_300c]);
        e.register(0x0310_3008, |_, _| 3u32.into_ret());
        e.register(0x0310_300c, |_, _| Ret::default());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0330_0000);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, package);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, 0x0abc);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        assert_eq!(
            calls_to_008c(&e, SAVE_FORM_ID),
            vec![vec![buffer, package, 0], vec![buffer, 0x0abc, 0]]
        );
        assert_eq!(
            calls_to_008c(&e, DATA_HANDLER_CHECK_00469860),
            vec![vec![0x0400_0000, 0x33]]
        );
        assert_eq!(
            saved_bytes(&e),
            vec![
                (1, 5),
                (1, 3),
                (4, this + 0xc),
                (4, this + 0x10),
                (4, this + 0x14)
            ]
        );
        assert_eq!(calls_to_008c(&e, 0x0310_2054), vec![vec![package, buffer]]);
        assert_eq!(calls_to_008c(&e, 0x0310_300c), vec![vec![data, buffer]]);
    }

    #[test]
    fn save_game_marks_refused_packages_and_missing_data_with_ff() {
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, true);
        let mut slots = vec![0u32; 0x58];
        slots[0x54 / 4] = 0x0310_2054;
        slots[0x15c / 4] = 0x0310_215c;
        e.put_vtable(0x0320_0000, &slots);
        e.register(0x0310_2054, |_, _| Ret::default());
        e.register(0x0310_215c, |_, _| false.into_ret());
        let package = e.mem.alloc(0x80);
        e.mem.set_u32(package, 0x0320_0000);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, package);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        let bytes = saved_bytes(&e);
        assert_eq!(bytes[0], (1, 0xff));
        assert_eq!(bytes[1], (1, 0xff));
        assert!(calls_to_008c(&e, 0x0310_2054).is_empty());
        // The data handler check failing skips the package's byte entirely.
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, false);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, package);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        assert_eq!(saved_bytes(&e)[0], (1, 0xff));
        assert_eq!(saved_bytes(&e).len(), 4);
    }
}
