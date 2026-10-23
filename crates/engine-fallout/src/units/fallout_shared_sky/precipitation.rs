//! `fallout shared/sky/precipitation.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00639b00` calls on the member (`009938b0`).
const CALLEE_009938B0: u32 = 0x0099_38b0;

// Translated from 00639b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `009938b0` with `ECX` = `this + 0x54` and passes on its 16-bit result.
pub fn fn_00639b00(e: &mut Engine, this: Ptr) -> u16 {
    e.call(CALLEE_009938B0, &args![this.byte_add(0x54)]).u16()
}

layout! {
    /// `Precipitation` (Xbox PDB), 0x18 bytes; the PC code uses the Xbox PDB's offsets.
    pub struct Precipitation: 0x18 {
        /// `spCurrentPrecipRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x04 spCurrentPrecipRoot: u32,
        /// `spLastPrecipRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 spLastPrecipRoot: u32,
        /// `pParent` (Xbox PDB): `NiNode*`.
        0x0C pParent: u32,
        /// `fBirthRate` (Xbox PDB).
        0x10 fBirthRate: f32,
        /// `pQueuedModel` (Xbox PDB): `const TESModel*`.
        0x14 pQueuedModel: u32,
    }
}

/// The `Precipitation` vtable the destructor sets (`0104ed98`).
const PRECIPITATION_VTABLE: u32 = 0x0104_ed98;
/// `NiPointer` helpers (`thiscall` on the pointer's address): the pointer's value (`00559450`,
/// no stack words), assignment of a raw pointer (`0066b0d0`, `RET 4`), assignment of another
/// `NiPointer` (`006e5cc0`, `RET 4`: its address) and the destructor (`0045cec0`).
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The function `fn_00636bb0` stores the result of (`0054f4f0`: no arguments, the value of the
/// `NiPointer` global at `011deda4`).
const POINTER_GLOBAL_011DEDA4_GET: u32 = 0x0054_f4f0;
/// `0048cee0` (`thiscall`, no stack words), asked of a model before it is queued.
const MODEL_TEST_0048CEE0: u32 = 0x0048_cee0;
/// The model loader singleton (a pointer global), `ModelLoader::QueueModel` (`00444040`,
/// `RET 0x1c`), `ModelLoader::LoadFile` (`00447080`, `RET 0x18`), `ModelLoader::FindModel`
/// (`004472a0`, `RET 8`) and `0045a5e0` (`RET 4`, the path) called after a found model is loaded.
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_4040;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
const MODEL_LOADER_AFTER_LOAD_0045A5E0: u32 = 0x0045_a5e0;
/// `_stricmp`-shaped compare (`00404dc0`, `cdecl`, two strings; 0 when equal) and the text
/// `"Sky\Snow.NIF"` (`0104ed9c`).
const STRING_COMPARE: u32 = 0x0040_4dc0;
const SNOW_MODEL_PATH: u32 = 0x0104_ed9c;
/// `0043d410` (`thiscall` on a 12-byte local, `RET 0xc`: a `float` and two words).
const LOCAL_INIT_0043D410: u32 = 0x0043_d410;
/// The allocation `00aa13e0` (`cdecl`, size) and `NiNode::NiNode` (`00a5ecb0`, `RET 4`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The loaded model's child count (`00453470`, no stack words) and child (`0043b4a0`, `RET 4`:
/// the index); both also used on the precipitation roots.
const CHILD_COUNT: u32 = 0x0045_3470;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `0045bc00` (`thiscall`, `RET 4`) on what a child's virtual slot +0xc answers, then the
/// kind-of cast `00653270` (`cdecl`: the `NiRTTI` record, the object) to the record at
/// `012024e0`.
const CHILD_OBJECT_0045BC00: u32 = 0x0045_bc00;
const KIND_OF_CAST: u32 = 0x0065_3270;
const CAST_RTTI_012024E0: u32 = 0x0120_24e0;
/// `00461130` (`thiscall` on the parent node, no stack words) and `00637890` (this unit,
/// `thiscall`, `RET 0xc`), which builds the precipitation for one cast child.
const PARENT_0046130: u32 = 0x0046_1130;
const BUILD_CHILD_00637890: u32 = 0x0063_7890;
/// `NiAVObject::UpdateProperties` (`00a5a040`, no stack words).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The parent node's virtual slots: detach a child (+0xe8, one word) and attach one (+0xdc, two
/// words: the child and 1).
const SLOT_DETACH_CHILD: u32 = 0xe8;
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// A model's virtual slot +0x14 (no stack words): its path.
const SLOT_MODEL_PATH: u32 = 0x14;

/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0`, passed as an `f64` (two words).
const FTOL: u32 = 0x00ec_62c0;
/// `fn_00639730`'s data: the `float` divisor (`011ccb34`), the `double` numerator (`01028338`)
/// and the two tables of 512 `float`s (`011f52e0`, `011f4ae0`).
const TABLE_DIVISOR_011CCB34: u32 = 0x011c_cb34;
const TABLE_NUMERATOR_01028338: u32 = 0x0102_8338;
const TABLE_011F52E0: u32 = 0x011f_52e0;
const TABLE_011F4AE0: u32 = 0x011f_4ae0;

/// `NiBlendInterpolator::GetHighestWeightedIndex` (`00a368f0`, `u8` result),
/// `NiBlendInterpolator::GetInterpolator` (`00495d50`, `RET 4`), the invalid index byte
/// (`01096968`) and the two `NiRTTI` records `fn_00639b60` casts to (`011f3738`, `011f3e58`).
const BLEND_HIGHEST_WEIGHTED_INDEX: u32 = 0x00a3_68f0;
const BLEND_GET_INTERPOLATOR: u32 = 0x0049_5d50;
const INVALID_INDEX_01096968: u32 = 0x0109_6968;
const INTERPOLATOR_RTTI_011F3738: u32 = 0x011f_3738;
const BLEND_RTTI_011F3E58: u32 = 0x011f_3e58;

/// The value of the `NiPointer` at `slot`.
fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

/// The parent's detach of the `NiPointer` at `this + offset` when it holds a node, then the
/// pointer cleared (`00637800`'s and `00636ed0`'s inlined sequence).
fn detach_root(e: &mut Engine, this: Ptr<Precipitation>, offset: u32) {
    let slot = this.addr().wrapping_add(offset);
    if pointer_get(e, slot) != 0 {
        let node = pointer_get(e, slot);
        let parent = e.get(this, Precipitation::pParent);
        e.vcall(parent, SLOT_DETACH_CHILD, &args![node]);
        e.call(POINTER_ASSIGN, &args![slot, 0u32]);
    }
}

// Translated from 00636af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::~Precipitation`'s scalar deleting destructor (Xbox PDB name
/// `Precipitation::_scalar_deleting_destructor_`). `thiscall`, `RET 4` (the flags): runs the
/// destructor (`00636b20`), frees the object (`00401030`) when bit 0 of the flags is set and
/// answers `this`.
pub fn precipitation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Precipitation>,
    flags: u32,
) -> Ptr<Precipitation> {
    fn_00636b20(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00636b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `Precipitation` destructor (by its body). `thiscall`, no
/// stack words. Sets the vtable (`0104ed98`), assigns null to both precipitation roots
/// (`0066b0d0` on +4, then +8), clears `pQueuedModel`, then destroys the roots' `NiPointer`s
/// (`0045cec0` on +8, then +4).
// Translation incomplete: 00636b20 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two NiPointer members) is not translated.
pub fn fn_00636b20(e: &mut Engine, this: Ptr<Precipitation>) {
    let base = this.addr();
    e.mem.set_u32(base, PRECIPITATION_VTABLE);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(4), 0u32]);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(8), 0u32]);
    e.set(this, Precipitation::pQueuedModel, 0);
    e.call(POINTER_DESTRUCT, &args![base.wrapping_add(8)]);
    e.call(POINTER_DESTRUCT, &args![base.wrapping_add(4)]);
}

// Translated from 00636bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `pParent` to what `0054f4f0` answers (the value of the
/// `NiPointer` global at `011deda4`; it takes no arguments). `thiscall`, no stack words.
pub fn fn_00636bb0(e: &mut Engine, this: Ptr<Precipitation>) {
    let parent = e.call(POINTER_GLOBAL_011DEDA4_GET, &args![]).u32();
    e.set(this, Precipitation::pParent, parent);
}

// Translated from 00636bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::Load` (Xbox PDB), the model overload. `thiscall`, `RET 4` (the model).
/// Nothing while the last precipitation root still holds a node. Otherwise, a model that
/// `0048cee0` accepts is queued with the model loader (`ModelLoader::QueueModel` on its path,
/// six zero words) and becomes `pQueuedModel`; any other model (or none) loads no file
/// (`Load_ov2` with a null path).
pub fn precipitation_load(e: &mut Engine, this: Ptr<Precipitation>, model: Ptr) {
    let last = this.addr().wrapping_add(8);
    if pointer_get(e, last) != 0 {
        return;
    }
    if model.addr() != 0 && e.call(MODEL_TEST_0048CEE0, &args![model]).u32() != 0 {
        let path = e.vcall(model.addr(), SLOT_MODEL_PATH, &args![]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_MODEL,
            &args![loader, path, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        e.set(this, Precipitation::pQueuedModel, model.addr());
    } else {
        precipitation_load_ov2(e, this, Ptr::new(0));
    }
}

// Translated from 00636c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::Load` (Xbox PDB, overload 2), the path overload. `thiscall`, `RET 4` (the
/// path). When the current root holds a node it is detached from the parent (slot +0xe8),
/// becomes the last root (`006e5cc0`), the last root is attached again (slot +0xdc with 1) and
/// the current root is cleared. With a non-empty path whose file the model loader loads
/// (`ModelLoader::LoadFile`: path, 0, 1, 0, 0, 0): whether the path is `"Sky\Snow.NIF"`
/// (case-insensitive compare `00404dc0`) is kept, a 12-byte local is set up by `0043d410`
/// (0.0, 0, 0) and passed to the model's virtual slot +0xa4 (with 0), a new `NiNode` (0xac
/// bytes; null when the allocation fails) becomes the current root and `fBirthRate` 0.0; then
/// for each child of the model (`00453470` children, `0043b4a0`) whose virtual slot +0xc
/// answers an object, `0045bc00` (with 0) on that object is cast to the record at `012024e0`
/// (`00653270`); a cast child is built by `00637890` (the child, what `00461130` answers for
/// the parent, the snow flag) and a built one attached to the current root (slot +0xdc with
/// 1). Finally the current root is attached to the parent (slot +0xdc with 1) and its
/// properties updated (`NiAVObject::UpdateProperties`).
// Translation incomplete: 00636c40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the new NiNode) is not translated.
pub fn precipitation_load_ov2(e: &mut Engine, this: Ptr<Precipitation>, path: Ptr) {
    let base = this.addr();
    let current = base.wrapping_add(4);
    let last = base.wrapping_add(8);
    if pointer_get(e, current) != 0 {
        let node = pointer_get(e, current);
        let parent = e.get(this, Precipitation::pParent);
        e.vcall(parent, SLOT_DETACH_CHILD, &args![node]);
        e.call(POINTER_ASSIGN_POINTER, &args![last, current]);
        let node = pointer_get(e, last);
        let parent = e.get(this, Precipitation::pParent);
        e.vcall(parent, SLOT_ATTACH_CHILD, &args![node, 1u32]);
        e.call(POINTER_ASSIGN, &args![current, 0u32]);
    }
    let path = path.addr();
    if path == 0 || e.mem.u8(path) == 0 {
        return;
    }
    let loader = e.global::<u32>(MODEL_LOADER);
    let model = e
        .call(
            MODEL_LOADER_LOAD_FILE,
            &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if model == 0 {
        return;
    }
    let snow = e.call(STRING_COMPARE, &args![path, SNOW_MODEL_PATH]).u32() == 0;
    e.with_stack(0xc, |e, local| {
        e.call(LOCAL_INIT_0043D410, &args![local, 0.0f32, 0u32, 0u32]);
        e.vcall(model, 0xa4, &args![local, 0u32]);
    });
    let memory = e.call(NEW_OBJECT, &args![0xacu32]).u32();
    let node = if memory != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![memory, 0u32]).u32()
    } else {
        0
    };
    e.call(POINTER_ASSIGN, &args![current, node]);
    e.set(
        this,
        Precipitation::fBirthRate,
        f32::from_bits(x87_store32(0.0)),
    );
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![model]).u32() {
        let child = e.call(CHILD_AT, &args![model, index]).u32();
        let object = e.vcall(child, 0xc, &args![]).u32();
        let cast = if object != 0 {
            let inner = e.call(CHILD_OBJECT_0045BC00, &args![object, 0u32]).u32();
            e.call(KIND_OF_CAST, &args![CAST_RTTI_012024E0, inner])
                .u32()
        } else {
            0
        };
        if cast != 0 {
            let parent = e.get(this, Precipitation::pParent);
            let from_parent = e.call(PARENT_0046130, &args![parent]).u32();
            let built = e
                .call(
                    BUILD_CHILD_00637890,
                    &args![this, cast, from_parent, u32::from(snow)],
                )
                .u32();
            if built != 0 {
                let root = pointer_get(e, current);
                e.vcall(root, SLOT_ATTACH_CHILD, &args![built, 1u32]);
            }
        }
        index = index.wrapping_add(1);
    }
    let root = pointer_get(e, current);
    let parent = e.get(this, Precipitation::pParent);
    e.vcall(parent, SLOT_ATTACH_CHILD, &args![root, 1u32]);
    let root = pointer_get(e, current);
    e.call(UPDATE_PROPERTIES, &args![root]);
}

/// `Sky::GetInstance` (`0046dd00`, no arguments) and the sky's accessors `fn_00636ed0` reads
/// (`thiscall`, no stack words): `00476c90` (compared with 1 and 0), `0044edb0` (the engine map
/// names the folded body `BaseProcess::GetCurrentProcedureIndex`; here a weather), `00825c00`
/// (another weather) and `006447d0` (a `float` in `ST0`), and `00634870` (`u8` result).
const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
const SKY_MODE_00476C90: u32 = 0x0047_6c90;
const SKY_WEATHER_0044EDB0: u32 = 0x0044_edb0;
const SKY_WEATHER_00825C00: u32 = 0x0082_5c00;
const SKY_FRACTION_006447D0: u32 = 0x0064_47d0;
const SKY_TEST_00634870: u32 = 0x0063_4870;
/// The weather's `float` read (`004ed230`, `thiscall`, `RET 0xc`: an index and two `float`s;
/// `float` in `ST0`) and the `float`s `fn_00636ed0` passes (`010231e0`) and falls back to
/// (`01017d00`).
const WEATHER_VALUE_004ED230: u32 = 0x004e_d230;
const WEATHER_ARGUMENT_010231E0: u32 = 0x0102_31e0;
const WEATHER_FALLBACK_01017D00: u32 = 0x0101_7d00;
/// The singleton pointer global `011de45c` and its test `0047c850` (`u8` result).
const SINGLETON_011DE45C: u32 = 0x011d_e45c;
const SINGLETON_TEST_0047C850: u32 = 0x0047_c850;
/// The 4-byte local `00633c90` constructs (`RET 4`, with 0) and `0040c110` destroys.
const LOCAL_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
const LOCAL_DESTRUCT_0040C110: u32 = 0x0040_c110;
/// `1.0` (`double`, `01012070`).
const ONE_01012070: u32 = 0x0101_2070;
/// `006377c0` (this unit, `cdecl`, one `float`: the elapsed time).
const ELAPSED_006377C0: u32 = 0x0063_77c0;
/// The player (a pointer global), its parent cell (`008d6f30`), the cell test `004518e0`
/// (`u8` result), the camera chain (`0045c670` with no arguments, `00558310`, `0045bb80`: a
/// position whose `z` is at +8) and `TESObjectREFR::GetRelevantWaterHeight` (`0057b0a0`, `float`
/// in `ST0`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_PARENT_CELL: u32 = 0x008d_6f30;
const CELL_TEST_004518E0: u32 = 0x0045_18e0;
const CAMERA_HOLDER_GET: u32 = 0x0045_c670;
const CAMERA_NODE_00558310: u32 = 0x0055_8310;
const NODE_POSITION_0045BB80: u32 = 0x0045_bb80;
const RELEVANT_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// The child's property by type (`NiAVObject::GetProperty`, `00a59d30`, `RET 4`, type 3), the
/// property's kind (`00441110`, compared with 0x12), the child's particle data (`005495f0`),
/// its 16-bit count (`004567e0`), the property's word (`005f5f80`) and the data's 16-bit value
/// `004a8ae0` (engine map: `TESActorBaseData::GetFatigue`, folded code).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_KIND_00441110: u32 = 0x0044_1110;
const CHILD_DATA_005495F0: u32 = 0x0054_95f0;
const DATA_COUNT_004567E0: u32 = 0x0045_67e0;
const PROPERTY_COUNT_005F5F80: u32 = 0x005f_5f80;
const DATA_VALUE_004A8AE0: u32 = 0x004a_8ae0;
/// The root's `00450f90` (`RET 4`: the underwater flag).
const ROOT_SET_FLAG_00450F90: u32 = 0x0045_0f90;

/// `fn_00636ed0`'s property lookup: type 3 when its kind (`00441110`) is 0x12, else 0.
fn kind_0x12_property(e: &mut Engine, child: u32) -> u32 {
    if e.call(GET_PROPERTY, &args![child, 3u32]).u32() == 0 {
        return 0;
    }
    let property = e.call(GET_PROPERTY, &args![child, 3u32]).u32();
    if e.call(PROPERTY_KIND_00441110, &args![property]).u32() == 0x12 {
        e.call(GET_PROPERTY, &args![child, 3u32]).u32()
    } else {
        0
    }
}

/// `fn_00636ed0`'s per-child step: scales the child's active count by `factor` (as bits) and
/// adds its share to `fBirthRate`.
fn scale_child(e: &mut Engine, this: Ptr<Precipitation>, child: u32, property: u32, factor: u32) {
    let data = e.call(CHILD_DATA_005495F0, &args![child]).u32();
    let count = i32::from(e.call(DATA_COUNT_004567E0, &args![data]).u16());
    let divisor = e.call(PROPERTY_COUNT_005F5F80, &args![property]).i32();
    // `CDQ; IDIV`: a zero divisor faults in the exe as here.
    let per_unit = (count / divisor) as u16;
    let word = e.call(PROPERTY_COUNT_005F5F80, &args![property]).i32();
    let scaled = crate::x87::fistp_chop(x87_mul(f64::from(word), x87_m32(factor))) as u16;
    let product = per_unit.wrapping_mul(scaled);
    let current = e.vcall(data, 0xa8, &args![]).u16();
    if product != current {
        let full = u32::from(per_unit) * u32::from(scaled);
        e.vcall(data, 0xa4, &args![full]);
        if scaled != 0 {
            let value = e.call(DATA_VALUE_004A8AE0, &args![data]).u16();
            e.vcall(data, 0x8c, &args![u32::from(value)]);
        } else {
            e.vcall(data, 0x8c, &args![0u32]);
        }
    }
    let word = e.call(PROPERTY_COUNT_005F5F80, &args![property]).i32();
    let rate = e.get(this, Precipitation::fBirthRate).to_bits();
    let sum = x87_add(x87_mul(f64::from(word), x87_m32(factor)), x87_m32(rate));
    e.set(
        this,
        Precipitation::fBirthRate,
        f32::from_bits(x87_store32(sum)),
    );
}

/// `fn_00636ed0`'s loop over the children of the root at `this + offset`.
fn scale_children(e: &mut Engine, this: Ptr<Precipitation>, offset: u32, factor: u32) {
    let slot = this.addr().wrapping_add(offset);
    let mut index = 0u32;
    loop {
        let root = pointer_get(e, slot);
        let child = e.call(CHILD_AT, &args![root, index]).u32();
        index = index.wrapping_add(1);
        if child == 0 {
            break;
        }
        let property = kind_0x12_property(e, child);
        if property != 0 {
            scale_child(e, this, child, property, factor);
        }
    }
}

/// `fn_00636ed0`'s load of a weather's precipitation model (`fn_006377e0`, the model's path
/// from slot +0x14, `Load_ov2`).
fn load_weather_model(e: &mut Engine, this: Ptr<Precipitation>, weather: u32) {
    let model = fn_006377e0(e, Ptr::new(weather));
    let path = e.vcall(model.addr(), SLOT_MODEL_PATH, &args![]).u32();
    precipitation_load_ov2(e, this, Ptr::new(path));
}

// Translated from 00636ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the precipitation's per-frame update. `thiscall`, `RET 4` (a
/// `float`, passed on to `006377c0`). From the sky (`Sky::GetInstance`): an underwater flag
/// (set when `00476c90` answers 1, or 0 on its second call), the current (`0044edb0`) and
/// other (`00825c00`) weathers and a fraction (`006447d0`); a low bound (the current weather's
/// `004ed230` with 6, the `float` at `010231e0` and 0.0; 0.0 without one) and a high bound (the
/// other weather's `004ed230` with 7, 1.0 and the `float` at `01017d00`; that `float` without
/// one). When the singleton test `0047c850` passes, both roots are detached and cleared and
/// the models of the other and the current weather are loaded. Otherwise a queued model whose
/// `0048cee0` and path match the current weather's model is loaded once the loader finds it
/// (`ModelLoader::FindModel`, then `0045a5e0`), and a queued model that does not match is
/// replaced by `Load` with the weather's model; without a queued model, `Load` runs when the
/// sky's `00634870` passes and there is a current weather. Then: the last root is detached
/// when the fraction is at least 1.0; `fBirthRate` is reset; with a root, `006377c0` runs, the
/// underwater flag is also set when the camera is below the player's relevant water height
/// (player in a cell that passes `004518e0`), each child of the current root is scaled by
/// `(fraction - low) / (1 - low)` (0 unless `low < fraction`) and each of the last root by
/// `1 - fraction / high` (0 unless `fraction < high`), adding to `fBirthRate`. Finally each
/// root gets the underwater flag (`00450f90`).
// Translation incomplete: 00636ed0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the 00633c90 local) is not translated.
pub fn fn_00636ed0(e: &mut Engine, this: Ptr<Precipitation>, elapsed: f32) {
    let current_slot = this.addr().wrapping_add(4);
    let last_slot = this.addr().wrapping_add(8);
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    let mut underwater = e.call(SKY_MODE_00476C90, &args![sky]).u32() == 1
        || e.call(SKY_MODE_00476C90, &args![sky]).u32() == 0;
    let current = e.call(SKY_WEATHER_0044EDB0, &args![sky]).u32();
    let other = e.call(SKY_WEATHER_00825C00, &args![sky]).u32();
    let fraction = x87_store32(e.call(SKY_FRACTION_006447D0, &args![sky]).f64());
    let low = if current != 0 {
        let argument = x87_quiet_bits(e.global::<u32>(WEATHER_ARGUMENT_010231E0));
        let value = e
            .call(
                WEATHER_VALUE_004ED230,
                &args![current, 6u32, argument, 0.0f32],
            )
            .f64();
        x87_store32(value)
    } else {
        x87_store32(0.0)
    };
    let low = x87_quiet_bits(low);
    let high = if other != 0 {
        let fallback = x87_quiet_bits(e.global::<u32>(WEATHER_FALLBACK_01017D00));
        let value = e
            .call(
                WEATHER_VALUE_004ED230,
                &args![other, 7u32, 1.0f32, fallback],
            )
            .f64();
        x87_store32(value)
    } else {
        x87_quiet_bits(e.global::<u32>(WEATHER_FALLBACK_01017D00))
    };
    let high = x87_quiet_bits(high);

    let singleton = e.global::<u32>(SINGLETON_011DE45C);
    if e.call(SINGLETON_TEST_0047C850, &args![singleton]).u8() != 0 {
        detach_root(e, this, 8);
        detach_root(e, this, 4);
        if e.call(SKY_WEATHER_00825C00, &args![sky]).u32() != 0 {
            let weather = e.call(SKY_WEATHER_00825C00, &args![sky]).u32();
            load_weather_model(e, this, weather);
        }
        if e.call(SKY_WEATHER_0044EDB0, &args![sky]).u32() != 0 {
            let weather = e.call(SKY_WEATHER_0044EDB0, &args![sky]).u32();
            load_weather_model(e, this, weather);
        }
    } else if e.get(this, Precipitation::pQueuedModel) != 0 {
        let model = fn_006377e0(e, Ptr::new(current)).addr();
        let mut done = false;
        if model != 0 {
            let queued = e.get(this, Precipitation::pQueuedModel);
            let queued_test = e.call(MODEL_TEST_0048CEE0, &args![queued]).u32();
            if queued_test == e.call(MODEL_TEST_0048CEE0, &args![model]).u32() {
                let model_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
                let queued = e.get(this, Precipitation::pQueuedModel);
                let queued_path = e.vcall(queued, SLOT_MODEL_PATH, &args![]).u32();
                if e.call(STRING_COMPARE, &args![queued_path, model_path])
                    .u32()
                    == 0
                {
                    let model = fn_006377e0(e, Ptr::new(current)).addr();
                    let path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
                    e.with_stack(4, |e, local| {
                        e.call(LOCAL_CONSTRUCT_00633C90, &args![local, 0u32]);
                        let loader = e.global::<u32>(MODEL_LOADER);
                        if e.call(MODEL_LOADER_FIND_MODEL, &args![loader, path, local])
                            .u8()
                            != 0
                        {
                            e.set(this, Precipitation::pQueuedModel, 0);
                            precipitation_load_ov2(e, this, Ptr::new(path));
                            let loader = e.global::<u32>(MODEL_LOADER);
                            e.call(MODEL_LOADER_AFTER_LOAD_0045A5E0, &args![loader, path]);
                        }
                        e.call(LOCAL_DESTRUCT_0040C110, &args![local]);
                    });
                    done = true;
                }
            }
        }
        if !done {
            e.set(this, Precipitation::pQueuedModel, 0);
            precipitation_load(e, this, Ptr::new(model));
        }
    } else if e.call(SKY_TEST_00634870, &args![sky]).u8() != 0 && current != 0 {
        let model = fn_006377e0(e, Ptr::new(current));
        precipitation_load(e, this, model);
    }

    // `FCOMP double 1.0; TEST AH,1`: detached unless below 1.0 or unordered.
    let one: f64 = e.global(ONE_01012070);
    let fraction_value = x87_load32(fraction);
    if fraction_value >= one {
        detach_root(e, this, 8);
    }
    e.set(
        this,
        Precipitation::fBirthRate,
        f32::from_bits(x87_store32(0.0)),
    );
    if pointer_get(e, current_slot) != 0 || pointer_get(e, last_slot) != 0 {
        e.call(ELAPSED_006377C0, &args![x87_quiet(elapsed)]);
        let player = e.global::<u32>(PLAYER);
        if player != 0 && e.call(PLAYER_PARENT_CELL, &args![player]).u32() != 0 {
            let cell = e.call(PLAYER_PARENT_CELL, &args![player]).u32();
            if e.call(CELL_TEST_004518E0, &args![cell]).u8() != 0 {
                let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
                let node = e.call(CAMERA_NODE_00558310, &args![holder]).u32();
                let position = e.call(NODE_POSITION_0045BB80, &args![node]).u32();
                let water = e.call(RELEVANT_WATER_HEIGHT, &args![player]).f64();
                let z = x87_load32(e.mem.u32(position.wrapping_add(8)));
                if z < water {
                    underwater = true;
                }
            }
        }
        if pointer_get(e, current_slot) != 0 {
            let low_value = x87_load32(low);
            let mut factor = x87_store32(0.0);
            if low_value < fraction_value {
                let numerator = x87_sub(fraction_value, x87_m32(low));
                let denominator = x87_sub(1.0, low_value);
                factor = x87_store32(x87_div(numerator, denominator));
            }
            scale_children(e, this, 4, factor);
        }
        if pointer_get(e, last_slot) != 0 {
            let high_value = x87_load32(high);
            let mut factor = x87_store32(0.0);
            if high_value > fraction_value {
                let ratio = x87_div(fraction_value, x87_m32(high));
                factor = x87_store32(x87_sub(1.0, ratio));
            }
            scale_children(e, this, 8, factor);
        }
    }
    for slot in [current_slot, last_slot] {
        if pointer_get(e, slot) != 0 {
            let root = pointer_get(e, slot);
            e.call(ROOT_SET_FLAG_00450F90, &args![root, u32::from(underwater)]);
        }
    }
}

// Translated from 006377e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers `this + 0xc8` (a member's address; by its callers the
/// weather's precipitation model). `thiscall`, no stack words.
pub fn fn_006377e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xc8)
}

// Translated from 00637800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::FlushAll` (Xbox PDB). `thiscall`, no stack words. Detaches the last and then
/// the current precipitation root from the parent (slot +0xe8) when they hold a node, and
/// clears each detached one (`0066b0d0` with null).
pub fn precipitation_flush_all(e: &mut Engine, this: Ptr<Precipitation>) {
    detach_root(e, this, 8);
    detach_root(e, this, 4);
}

// Translated from 00639730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a table lookup of an angle. `cdecl`, three words (a `float` and
/// two `float` pointers). The index is `_ftol2(([01028338] / [011ccb34]) * angle) & 0x1ff`
/// (x87, in that order); the first out pointer gets the index's entry of the table at
/// `011f52e0`, the second that of the table at `011f4ae0` (`FLD`/`FSTP` copies).
pub fn fn_00639730(e: &mut Engine, angle: f32, first: Ptr, second: Ptr) {
    let divisor = x87_load32(e.global::<u32>(TABLE_DIVISOR_011CCB34));
    let numerator: f64 = e.global(TABLE_NUMERATOR_01028338);
    let scaled = x87_mul(x87_div(numerator, divisor), x87_m32(angle.to_bits()));
    let index = e.call(FTOL, &args![scaled]).u32() & 0x1ff;
    let a = x87_quiet_bits(e.global::<u32>(TABLE_011F52E0 + 4 * index));
    e.mem.set_u32(first.addr(), a);
    let b = x87_quiet_bits(e.global::<u32>(TABLE_011F4AE0 + 4 * index));
    e.mem.set_u32(second.addr(), b);
}

// Translated from 00639940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the engine map places it here; its references are vtable
/// entries): clears the six words at +4 to +0x18 and the byte at +0, sets the mode bits with
/// `fn_006399a0(0)` and answers `this`. `thiscall`, no stack words.
pub fn fn_00639940(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base, 0);
    fn_006399a0(e, this, 0);
    this
}

// Translated from 006399a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at +0 becomes `(byte & 0xf9) | fn_006399e0(value)`.
/// `thiscall`, `RET 4`.
pub fn fn_006399a0(e: &mut Engine, this: Ptr, value: u32) {
    let bits = fn_006399e0(e, this, value);
    let byte = e.mem.u8(this.addr());
    e.mem.set_u8(this.addr(), (byte & 0xf9) | bits);
}

// Translated from 006399e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 2 for 0, 3 for 0x4000 and 3 for anything else (`AL`).
/// `thiscall` (`this` unused), `RET 4`.
pub fn fn_006399e0(_e: &mut Engine, _this: Ptr, value: u32) -> u8 {
    match value {
        0 => 2,
        0x4000 => 3,
        _ => 3,
    }
}

// Translated from 00639aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at +0x48 (`FLD`, in `ST0`). `thiscall`.
pub fn fn_00639aa0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x48)))
}

// Translated from 00639b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at +0x5c (`FLD`, in `ST0`). `thiscall`.
pub fn fn_00639b20(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x5c)))
}

// Translated from 00639b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at +0x3c. `thiscall`.
pub fn fn_00639b40(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x3c))
}

// Translated from 00639b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetBirthRateInterpolator` (Xbox PDB). `thiscall`, no stack words. The
/// interpolator held at +0x34 (`NiPointer` value) cast to the record at `011f3738`
/// (`00653270`) when that succeeds; otherwise, cast to the record at `011f3e58`, the blend
/// interpolator's interpolator at its highest weighted index (`00a368f0`, `00495d50`) unless
/// that index is the invalid byte at `01096968`; otherwise 0.
pub fn ni_p_sys_emitter_ctlr_get_birth_rate_interpolator(e: &mut Engine, this: Ptr) -> u32 {
    let slot = this.addr().wrapping_add(0x34);
    let interpolator = pointer_get(e, slot);
    let cast = e
        .call(
            KIND_OF_CAST,
            &args![INTERPOLATOR_RTTI_011F3738, interpolator],
        )
        .u32();
    if cast != 0 {
        return cast;
    }
    let interpolator = pointer_get(e, slot);
    let blend = e
        .call(KIND_OF_CAST, &args![BLEND_RTTI_011F3E58, interpolator])
        .u32();
    if blend != 0 {
        let index = e.call(BLEND_HIGHEST_WEIGHTED_INDEX, &args![blend]).u8();
        if index != e.global::<u8>(INVALID_INDEX_01096968) {
            return e
                .call(BLEND_GET_INTERPOLATOR, &args![blend, u32::from(index)])
                .u32();
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_9b00, fn_00639b00(Ptr) -> u16),
        entry!(
            0x0063_6af0,
            precipitation_scalar_deleting_destructor(Ptr<Precipitation>, u32) -> Ptr<Precipitation>
        ),
        entry!(0x0063_6b20, fn_00636b20(Ptr<Precipitation>)),
        entry!(0x0063_6bb0, fn_00636bb0(Ptr<Precipitation>)),
        entry!(0x0063_6bd0, precipitation_load(Ptr<Precipitation>, Ptr)),
        entry!(0x0063_6c40, precipitation_load_ov2(Ptr<Precipitation>, Ptr)),
        entry!(0x0063_6ed0, fn_00636ed0(Ptr<Precipitation>, f32)),
        entry!(0x0063_77e0, fn_006377e0(Ptr) -> Ptr),
        entry!(0x0063_7800, precipitation_flush_all(Ptr<Precipitation>)),
        entry!(0x0063_9730, fn_00639730(f32, Ptr, Ptr)),
        entry!(0x0063_9940, fn_00639940(Ptr) -> Ptr),
        entry!(0x0063_99a0, fn_006399a0(Ptr, u32)),
        entry!(0x0063_99e0, fn_006399e0(Ptr, u32) -> u8),
        entry!(0x0063_9aa0, fn_00639aa0(Ptr) -> f32),
        entry!(0x0063_9b20, fn_00639b20(Ptr) -> f32),
        entry!(0x0063_9b40, fn_00639b40(Ptr) -> u32),
        entry!(
            0x0063_9b60,
            ni_p_sys_emitter_ctlr_get_birth_rate_interpolator(Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00639b00_passes_this_plus_54_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_009938B0, |_, a| Ret {
            eax: a[0] | 0x5_0000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_9b00, &args![0x4100u32]).u16(), 0x4154);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_009938B0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x54]);
    }
}

#[cfg(test)]
mod b064_tests {
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

    const DETACH: u32 = 0x0f00_1000;
    const ATTACH: u32 = 0x0f00_1004;
    const MODEL_PATH: u32 = 0x0f00_1008;

    /// The `NiPointer` helpers as plain word reads and writes, a parent whose slots +0xe8
    /// and +0xdc are doubles, and a precipitation object with that parent.
    fn rig() -> (Engine, Ptr<Precipitation>, u32) {
        let mut e = Engine::new();
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN_POINTER, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(DETACH, |_, _| Ret::default());
        e.register(ATTACH, |_, _| Ret::default());
        // A model's path slot answers the word at the model's +4.
        e.register(MODEL_PATH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let parent_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(parent_vtable + SLOT_DETACH_CHILD, DETACH);
        e.mem.set_u32(parent_vtable + SLOT_ATTACH_CHILD, ATTACH);
        let parent = e.mem.alloc(0x10);
        e.mem.set_u32(parent, parent_vtable);
        let this: Ptr<Precipitation> = e.new_object();
        e.set(this, Precipitation::pParent, parent);
        (e, this, parent)
    }

    /// A model object whose slot +0x14 answers `path`.
    fn model(e: &mut Engine, path: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + SLOT_MODEL_PATH, MODEL_PATH);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model, vtable);
        e.mem.set_u32(model + 4, path);
        model
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        for (flags, freed) in [(1u32, true), (2, false)] {
            let (mut e, this, _) = rig();
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let r = e.call(0x0063_6af0, &args![this, flags]).u32();
            assert_eq!(r, this.addr());
            assert_eq!(e.mem.u32(this.addr()), PRECIPITATION_VTABLE);
            let want: Vec<Vec<u32>> = if freed {
                vec![vec![this.addr()]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, OPERATOR_DELETE), want);
        }
    }

    #[test]
    fn destructor_clears_both_roots_then_destroys_them() {
        let (mut e, this, _) = rig();
        let base = this.addr();
        e.mem.set_u32(base + 4, 0x11);
        e.mem.set_u32(base + 8, 0x22);
        e.set(this, Precipitation::pQueuedModel, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x0063_6b20, &args![this]);
        assert_eq!(e.mem.u32(base), PRECIPITATION_VTABLE);
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![base + 4, 0], vec![base + 8, 0]]
        );
        assert_eq!(
            calls(&e, POINTER_DESTRUCT),
            vec![vec![base + 8], vec![base + 4]]
        );
        assert_eq!(e.get(this, Precipitation::pQueuedModel), 0);
    }

    #[test]
    fn fn_00636bb0_sets_the_parent_from_the_global_pointer() {
        let (mut e, this, _) = rig();
        e.register(POINTER_GLOBAL_011DEDA4_GET, |_, a| {
            assert!(a.is_empty());
            0x4444u32.into_ret()
        });
        e.call(0x0063_6bb0, &args![this]);
        assert_eq!(e.get(this, Precipitation::pParent), 0x4444);
    }

    #[test]
    fn load_queues_an_accepted_model_and_skips_while_the_last_root_is_held() {
        let (mut e, this, _) = rig();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x7777u32);
        e.register(MODEL_TEST_0048CEE0, |_, _| 1u32.into_ret());
        e.register(MODEL_LOADER_QUEUE_MODEL, |_, _| Ret::default());
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| Ret::default());
        let m = model(&mut e, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x0063_6bd0, &args![this, m]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![vec![0x7777, 0x5150, 0, 0, 0, 0, 0, 0]]
        );
        assert_eq!(e.get(this, Precipitation::pQueuedModel), m);

        // A held last root: nothing at all.
        e.set(this, Precipitation::pQueuedModel, 0);
        e.mem.set_u32(this.addr() + 8, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0063_6bd0, &args![this, m]);
        assert!(calls(&e, MODEL_TEST_0048CEE0).is_empty());
        assert_eq!(e.get(this, Precipitation::pQueuedModel), 0);
    }

    #[test]
    fn load_of_a_rejected_model_loads_no_file() {
        let (mut e, this, _) = rig();
        e.register(MODEL_TEST_0048CEE0, |_, _| 0u32.into_ret());
        let m = model(&mut e, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x0063_6bd0, &args![this, m]);
        assert_eq!(calls(&e, MODEL_TEST_0048CEE0), vec![vec![m]]);
        assert_eq!(e.get(this, Precipitation::pQueuedModel), 0);
        // `Load_ov2(null)` with an empty current root only reads it.
        assert_eq!(
            calls(&e, POINTER_GET),
            vec![vec![this.addr() + 8], vec![this.addr() + 4]]
        );
    }

    #[test]
    fn load_ov2_rotates_the_roots_and_builds_the_cast_children() {
        let (mut e, this, parent) = rig();
        let base = this.addr();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x7777u32);
        e.mem.set_u32(base + 4, 0xaaaa);
        let path = e.mem.alloc(16);
        e.mem.set_cstr(path, b"Sky\\Rain.NIF");
        // The loaded model: two children; the first answers an object, the second none.
        let loaded_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(loaded_vtable + 0xa4, 0x0f00_2000);
        e.register(0x0f00_2000, |_, _| Ret::default());
        let loaded = e.mem.alloc(8);
        e.mem.set_u32(loaded, loaded_vtable);
        let child_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(child_vtable + 0xc, 0x0f00_2004);
        e.register(0x0f00_2004, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, child_vtable);
        e.mem.set_u32(first + 4, 0x6000);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, child_vtable);
        e.register_double(MODEL_LOADER_LOAD_FILE, move |_, _| loaded.into_ret());
        e.register(STRING_COMPARE, |_, _| 1u32.into_ret());
        e.register(LOCAL_INIT_0043D410, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        // The new node shares the parent's vtable (its attach slot).
        let parent_vtable = e.mem.u32(parent);
        e.register_double(NI_NODE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], parent_vtable);
            a[0].into_ret()
        });
        e.register(CHILD_COUNT, |_, _| 2u32.into_ret());
        e.register_double(CHILD_AT, move |_, a| {
            [first, second][a[1] as usize].into_ret()
        });
        e.register(CHILD_OBJECT_0045BC00, |_, a| (a[0] + 1).into_ret());
        e.register(KIND_OF_CAST, |_, a| a[1].into_ret());
        e.register(PARENT_0046130, |_, _| 0x8888u32.into_ret());
        e.register(BUILD_CHILD_00637890, |_, _| 0x9999u32.into_ret());
        e.register(UPDATE_PROPERTIES, |_, _| Ret::default());
        e.set(this, Precipitation::fBirthRate, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x0063_6c40, &args![this, path]);

        assert_eq!(calls(&e, DETACH), vec![vec![parent, 0xaaaa]]);
        assert_eq!(e.mem.u32(base + 8), 0xaaaa);
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0x7777, path, 0, 1, 0, 0, 0]]
        );
        assert_eq!(calls(&e, STRING_COMPARE), vec![vec![path, SNOW_MODEL_PATH]]);
        let local = calls(&e, LOCAL_INIT_0043D410)[0][0];
        assert_eq!(calls(&e, LOCAL_INIT_0043D410), vec![vec![local, 0, 0, 0]]);
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![loaded, local, 0]]);
        let node = e.mem.u32(base + 4);
        assert_eq!(calls(&e, NI_NODE_CONSTRUCT), vec![vec![node, 0]]);
        assert_eq!(e.get(this, Precipitation::fBirthRate), 0.0);
        assert_eq!(
            calls(&e, KIND_OF_CAST),
            vec![vec![CAST_RTTI_012024E0, 0x6001]]
        );
        assert_eq!(
            calls(&e, BUILD_CHILD_00637890),
            vec![vec![base, 0x6001, 0x8888, 0]]
        );
        assert_eq!(
            calls(&e, ATTACH),
            vec![
                vec![parent, 0xaaaa, 1],
                vec![node, 0x9999, 1],
                vec![parent, node, 1]
            ]
        );
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![node]]);
    }

    #[test]
    fn load_ov2_with_an_empty_path_or_no_file_stops() {
        let (mut e, this, _) = rig();
        e.map(0x011c_3000, 0x1000);
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| 0u32.into_ret());
        let path = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0063_6c40, &args![this, path]);
        assert!(calls(&e, MODEL_LOADER_LOAD_FILE).is_empty());
        e.mem.set_u8(path, 0x61);
        e.call(0x0063_6c40, &args![this, path]);
        assert_eq!(calls(&e, MODEL_LOADER_LOAD_FILE).len(), 1);
        assert!(calls(&e, POINTER_ASSIGN).is_empty());
    }

    /// Doubles for the update: the sky's accessors answer the values given.
    fn update_rig(test: u8, fraction: f32, current: u32) -> (Engine, Ptr<Precipitation>, u32) {
        let (mut e, this, parent) = rig();
        for page in [
            0x0101_7000,
            0x0101_2000,
            0x0102_3000,
            0x011d_e000,
            0x011c_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(WEATHER_ARGUMENT_010231E0, 0.25f32);
        e.set_global(WEATHER_FALLBACK_01017D00, 0.001f32);
        e.register(SKY_GET_INSTANCE, |_, _| 0x5000u32.into_ret());
        e.register(SKY_MODE_00476C90, |_, _| 2u32.into_ret());
        e.register_double(SKY_WEATHER_0044EDB0, move |_, _| current.into_ret());
        e.register(SKY_WEATHER_00825C00, |_, _| 0u32.into_ret());
        e.register_double(SKY_FRACTION_006447D0, move |_, _| Ret {
            st0: f64::from(fraction),
            ..Ret::default()
        });
        e.register(WEATHER_VALUE_004ED230, |_, a| Ret {
            st0: if a[1] == 6 { 0.5 } else { 2.0 },
            ..Ret::default()
        });
        e.register_double(SINGLETON_TEST_0047C850, move |_, _| {
            u32::from(test).into_ret()
        });
        e.register(SKY_TEST_00634870, |_, _| 0u32.into_ret());
        e.register(ROOT_SET_FLAG_00450F90, |_, _| Ret::default());
        e.register(ELAPSED_006377C0, |_, _| Ret::default());
        (e, this, parent)
    }

    #[test]
    fn update_with_the_singleton_test_reloads_the_weathers_models() {
        let (mut e, this, parent) = update_rig(1, 0.5, 0);
        let base = this.addr();
        e.mem.set_u32(base + 8, 0xbbbb);
        // The current weather's model (at weather + 0xc8) has an empty path.
        let weather_block = e.mem.alloc(0x100);
        let m = model(&mut e, 0);
        let vtable = e.mem.u32(m);
        e.mem.set_u32(weather_block + 0xc8, vtable);
        e.register_double(SKY_WEATHER_0044EDB0, move |_, _| weather_block.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0063_6ed0, &args![this, 0.1f32]);
        assert_eq!(calls(&e, DETACH), vec![vec![parent, 0xbbbb]]);
        assert_eq!(e.mem.u32(base + 8), 0);
        // The low bound came from the weather with 6, 0.25, 0.0; the high one is the fallback.
        assert_eq!(
            calls(&e, WEATHER_VALUE_004ED230),
            vec![vec![weather_block, 6, 0.25f32.to_bits(), 0]]
        );
        // Only the current weather's model path was asked for (the other weather is null).
        assert_eq!(calls(&e, MODEL_PATH), vec![vec![weather_block + 0xc8]]);
        // No root: no elapsed-time call and no flags.
        assert!(calls(&e, ELAPSED_006377C0).is_empty());
        assert!(calls(&e, ROOT_SET_FLAG_00450F90).is_empty());
        assert_eq!(e.get(this, Precipitation::fBirthRate), 0.0);
    }

    #[test]
    fn update_scales_the_current_roots_children_and_sets_the_underwater_flag() {
        let (mut e, this, _) = update_rig(0, 0.75, 0x100);
        let base = this.addr();
        e.mem.set_u32(base + 4, 0xc000);
        e.register(SKY_MODE_00476C90, |_, _| 1u32.into_ret());
        // The root's children: one with a kind-0x12 property, then none.
        e.register(CHILD_AT, |_, a| {
            if a[1] == 0 { 0xd000u32 } else { 0 }.into_ret()
        });
        e.register(GET_PROPERTY, |_, _| 0xe000u32.into_ret());
        e.register(PROPERTY_KIND_00441110, |_, _| 0x12u32.into_ret());
        let data_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data_vtable + 0x8c, 0x0f00_3000);
        e.mem.set_u32(data_vtable + 0xa4, 0x0f00_3004);
        e.mem.set_u32(data_vtable + 0xa8, 0x0f00_3008);
        e.register(0x0f00_3000, |_, _| Ret::default());
        e.register(0x0f00_3004, |_, _| Ret::default());
        e.register(0x0f00_3008, |_, _| 7u32.into_ret());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, data_vtable);
        e.register_double(CHILD_DATA_005495F0, move |_, _| data.into_ret());
        e.register(DATA_COUNT_004567E0, |_, _| 100u32.into_ret());
        e.register(PROPERTY_COUNT_005F5F80, |_, _| 10u32.into_ret());
        e.register(DATA_VALUE_004A8AE0, |_, _| 0x1_0042u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0063_6ed0, &args![this, 0.1f32]);
        // A queued model is absent and 00634870 fails: nothing loads.
        assert!(calls(&e, MODEL_LOADER_LOAD_FILE).is_empty());
        // low = 0.5, fraction 0.75: factor (0.75 - 0.5) / 0.5 = 0.5; 10 * 0.5 = 5 per child.
        assert_eq!(calls(&e, ELAPSED_006377C0), vec![vec![0.1f32.to_bits()]]);
        // 100 / 10 = 10 per unit, times 5 = 50 differs from 7.
        assert_eq!(calls(&e, 0x0f00_3004), vec![vec![data, 50]]);
        assert_eq!(calls(&e, 0x0f00_3000), vec![vec![data, 0x42]]);
        assert_eq!(e.get(this, Precipitation::fBirthRate), 5.0);
        assert_eq!(calls(&e, ROOT_SET_FLAG_00450F90), vec![vec![0xc000, 1]]);
    }

    #[test]
    fn fn_006377e0_answers_the_member_at_0xc8() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0063_77e0, &args![0x1000u32]).u32(), 0x10c8);
    }

    #[test]
    fn flush_all_detaches_the_last_then_the_current_root() {
        let (mut e, this, parent) = rig();
        let base = this.addr();
        e.mem.set_u32(base + 4, 0x44);
        e.mem.set_u32(base + 8, 0x88);
        e.call_log = Some(vec![]);
        e.call(0x0063_7800, &args![this]);
        assert_eq!(
            calls(&e, DETACH),
            vec![vec![parent, 0x88], vec![parent, 0x44]]
        );
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![base + 8, 0], vec![base + 4, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0063_7800, &args![this]);
        assert!(calls(&e, DETACH).is_empty());
    }

    #[test]
    fn fn_00639730_reads_both_tables_at_the_masked_index() {
        let mut e = Engine::new();
        for page in [0x011c_c000, 0x0102_8000, 0x011f_4000, 0x011f_5000] {
            e.map(page, 0x1000);
        }
        e.set_global(TABLE_DIVISOR_011CCB34, 2.0f32);
        e.set_global(TABLE_NUMERATOR_01028338, 512.0f64);
        e.set_global(TABLE_011F52E0 + 4 * 0x10, 0x7f80_0001u32);
        e.set_global(TABLE_011F4AE0 + 4 * 0x10, 0.5f32);
        e.register(FTOL, |_, a| {
            (f64::take(a, &mut 0).trunc() as i32 as u32).into_ret()
        });
        let out = e.mem.alloc(8);
        // 512 / 2 * 2.0625 = 528 = 0x210, masked to 0x10.
        e.call(0x0063_9730, &args![2.0625f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 4), 0.5);
    }

    #[test]
    fn fn_00639940_clears_the_words_and_sets_mode_2() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        for offset in (0..0x1c).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0063_9940, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 2);
        assert_eq!(e.mem.bytes(this + 4, 0x18), vec![0; 0x18]);
        assert_eq!(e.mem.bytes(this + 1, 3), vec![0xff; 3]);
    }

    #[test]
    fn fn_006399a0_keeps_the_other_bits_and_fn_006399e0_maps_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u8(this, 0xff);
        e.call(0x0063_99a0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this), 0xfb);
        e.mem.set_u8(this, 0xf0);
        e.call(0x0063_99a0, &args![this, 0x4000u32]);
        assert_eq!(e.mem.u8(this), 0xf3);
        for (value, want) in [(0u32, 2u8), (0x4000, 3), (7, 3)] {
            assert_eq!(e.call(0x0063_99e0, &args![this, value]).u8(), want);
        }
    }

    #[test]
    fn the_three_getters_read_their_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x48, 0x7f80_0002);
        e.mem.set_f32(this + 0x5c, -3.5);
        e.mem.set_u32(this + 0x3c, 0xabcd);
        assert!(e.call(0x0063_9aa0, &args![this]).f64().is_nan());
        assert_eq!(e.call(0x0063_9b20, &args![this]).f64(), -3.5);
        assert_eq!(e.call(0x0063_9b40, &args![this]).u32(), 0xabcd);
    }

    fn interpolator_case(direct: bool, blend: bool, index: u8) -> u32 {
        let mut e = Engine::new();
        e.map(0x0109_6000, 0x1000);
        e.set_global(INVALID_INDEX_01096968, 0xffu8);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(KIND_OF_CAST, move |_, a| {
            let ok = if a[0] == INTERPOLATOR_RTTI_011F3738 {
                direct
            } else {
                blend
            };
            if ok { a[1] + (a[0] & 0xf) } else { 0 }.into_ret()
        });
        e.register_double(BLEND_HIGHEST_WEIGHTED_INDEX, move |_, _| {
            (0xab00 | u32::from(index)).into_ret()
        });
        e.register(BLEND_GET_INTERPOLATOR, |_, a| (0x7000 + a[1]).into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 0x3000);
        e.call(0x0063_9b60, &args![this]).u32()
    }

    #[test]
    fn birth_rate_interpolator_casts_then_falls_back_to_the_blend() {
        assert_eq!(interpolator_case(true, true, 1), 0x3008);
        assert_eq!(interpolator_case(false, true, 2), 0x7002);
        assert_eq!(interpolator_case(false, true, 0xff), 0);
        assert_eq!(interpolator_case(false, false, 1), 0);
    }
}
