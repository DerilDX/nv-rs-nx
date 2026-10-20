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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c_6130,
            actor_package_move_to(Ptr<ActorPackage>, Ptr<ActorPackage>)
        ),
        entry!(0x008c5ef0, actorpackage_clear(Ptr)),
        entry!(0x008c6060, fn_008c6060(Ptr)),
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
}
