//! `BSHavok/bhkcollisionfilter.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The collision filter info is one 32-bit word (`CFilter`): the low 7 bits are the layer,
//! bits 8 to 12 the biped part (for the layers 8 and 0x1d), bit 13 "scaled", bit 14 "no
//! collision", bit 15 "linked" and the high 16 bits the group.
//!
//! The five small `isCollisionEnabled`-shaped functions are vtable entries of the filter: they
//! are called with the filter in `ECX` (never read), answer through a result pointer (their
//! first stack word, which they also return in `EAX`) and compare two filter infos with
//! `bhkCollisionFilter::IsCollisionEnabled` (`00c84740`).
//!
//! The `/GS` cookie check (`00ec408c`) of the function marked `Translation incomplete` is not
//! modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCollisionFilter::IsCollisionEnabled` (engine map name, `00c84740`, cdecl: the two
/// filter infos; answers a bool).
const IS_COLLISION_ENABLED: u32 = 0x00c8_4740;
/// `sprintf_s` (`00406d00`, cdecl: buffer, size, format, values).
const FORMAT: u32 = 0x0040_6d00;
/// `NiGetViewerString` overloads (engine map names): `_ov11` (`00a7d5e0`, cdecl: label,
/// string), `_ov2` (`00a7d370`, cdecl: label, bool word) and `_ov8` (`00a7d510`, cdecl: label,
/// integer); each answers the text pointer.
const VIEWER_STRING_OF_TEXT: u32 = 0x00a7_d5e0;
const VIEWER_STRING_OF_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_OF_INT: u32 = 0x00a7_d510;
/// The string array's resize (`0096ad30`) and store (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The format strings `"0x%08X"` and `"0x%X"` and the labels `"COLFILTER"`, `"-GROUP"`,
/// `"-LAYER"`, `"-LINK"`, `"-PART"`, `"-NOCOL"` and `"-SCALED"` of the viewer strings.
const FORMAT_FILTER: u32 = 0x010c_54b0;
const FORMAT_GROUP: u32 = 0x010c_584c;
const LABEL_FILTER: u32 = 0x010c_5854;
const LABEL_GROUP: u32 = 0x010c_5844;
const LABEL_LAYER: u32 = 0x010c_583c;
const LABEL_LINK: u32 = 0x010c_5834;
const LABEL_PART: u32 = 0x010c_582c;
const LABEL_NO_COLLISION: u32 = 0x010c_5824;
const LABEL_SCALED: u32 = 0x010c_581c;
/// The table of the layer names (`011b0810`, indexed by the layer) and of the biped part names
/// (`011b08c0`, indexed by the part).
const LAYER_NAMES: u32 = 0x011b_0810;
const PART_NAMES: u32 = 0x011b_08c0;
/// The debug color (three words) of a layer outside the table (`011a9b7c`).
const DEFAULT_DEBUG_COLOR: u32 = 0x011a_9b7c;

/// One component of a layer's debug color: the x87 constant 0 or 1, or the `float` at an
/// address (copied through `FLD`/`FSTP`).
#[derive(Clone, Copy)]
enum Component {
    Zero,
    One,
    Global(u32),
}

/// The case (an index into [`debug_color_components`], 18 for the default) of each layer from
/// 1 to 31, as the exe's byte table at `00c84720` has it.
const LAYER_CASE: [u8; 31] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 1, 9, 10, 11, 10, 18, 12, 13, 14, 18, 18, 18, 18, 18, 15, 18, 18,
    18, 18, 18, 16, 17,
];

/// The three components of the debug color of case `case` (the jump table at `00c846d4`), or
/// `None` for the default case.
fn debug_color_components(case: u8) -> Option<[Component; 3]> {
    use Component::{Global, One, Zero};
    Some(match case {
        0 => [One, Zero, Zero],
        1 => [
            Global(0x0101_77e8),
            Global(0x010c_5818),
            Global(0x010c_5814),
        ],
        2 => [
            Global(0x010c_5810),
            Global(0x0103_0ff0),
            Global(0x0103_0ff0),
        ],
        3 => [
            Global(0x010c_580c),
            Global(0x010c_5808),
            Global(0x010c_5804),
        ],
        4 => [
            Global(0x0101_9de0),
            Global(0x010c_5800),
            Global(0x0108_aff0),
        ],
        5 => [
            Global(0x0101_77e8),
            Global(0x010c_57fc),
            Global(0x010c_57f8),
        ],
        6 => [
            Global(0x010c_57f4),
            Global(0x0101_45a8),
            Global(0x0101_77e8),
        ],
        7 => [Zero, One, Zero],
        8 => [
            Global(0x0103_0ff0),
            Global(0x0108_da38),
            Global(0x0104_e2b8),
        ],
        9 => [
            Global(0x010c_57f0),
            Global(0x0102_05fc),
            Global(0x0102_05fc),
        ],
        10 => [
            Global(0x010c_57fc),
            Global(0x010c_57ec),
            Global(0x0101_6264),
        ],
        11 | 13 => [
            Global(0x010c_5810),
            Global(0x0101_45a8),
            Global(0x010c_57e8),
        ],
        12 => [
            Global(0x010c_57fc),
            Global(0x0102_d92c),
            Global(0x0101_8238),
        ],
        14 => [Zero, Global(0x0101_8238), Zero],
        15 => [
            Global(0x0104_e2b8),
            Global(0x0101_8180),
            Global(0x0101_e2bc),
        ],
        16 => [One, One, Zero],
        17 => [One, Global(0x0103_0ff0), Zero],
        _ => return None,
    })
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

/// The root of a collision body: its parent chain (`+0xc`) followed to the end.
fn root_of(e: &Engine, body: u32) -> u32 {
    let mut node = body;
    loop {
        let parent = e.mem.u32(node + 0xc);
        if parent == 0 {
            return node;
        }
        node = parent;
    }
}

/// The collision filter info of a collidable (`+0x1c`).
fn filter_info_of(e: &Engine, collidable: u32) -> u32 {
    e.mem.u32(collidable + 0x1c)
}

/// Compares two filter infos (`00c84740`), stores the answer as a byte at `out` and answers
/// `out`.
fn answer(e: &mut Engine, out: Ptr, first: u32, second: u32) -> Ptr {
    let enabled = e.call(IS_COLLISION_ENABLED, &args![first, second]).bool();
    e.mem.set_u8(out.addr(), enabled as u8);
    out
}

// Translated from 00c84470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionFilter::GetDebugColor` (Xbox PDB), cdecl `(filter, color)`: writes the three
/// words of the debug color of the filter's layer (`filter & 0x7f`, 1 to 31) to `color`: each
/// is the constant 0 or 1 or a `float` copied through `FLD`/`FSTP` (a signalling NaN comes out
/// quiet), the cases chosen by the exe's tables (`00c84720`, `00c846d4`). Any other layer
/// copies the three words at `011a9b7c`. Answers `color`.
pub fn bhk_collision_filter_get_debug_color(e: &mut Engine, filter: u32, color: Ptr) -> Ptr {
    let layer = filter & 0x7f;
    let case = LAYER_CASE
        .get(layer.wrapping_sub(1) as usize)
        .copied()
        .unwrap_or(18);
    match debug_color_components(case) {
        Some(components) => {
            for (index, component) in components.into_iter().enumerate() {
                let bits = match component {
                    Component::Zero => 0.0f32.to_bits(),
                    Component::One => 1.0f32.to_bits(),
                    Component::Global(address) => x87_quiet_bits(e.mem.u32(address)),
                };
                e.mem.set_u32(color.addr() + 4 * index as u32, bits);
            }
        }
        None => {
            for index in 0..3 {
                let word = e.mem.u32(DEFAULT_DEBUG_COLOR + 4 * index);
                e.mem.set_u32(color.addr() + 4 * index, word);
            }
        }
    }
    color
}

// Translated from 00c84880 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload of two
/// collidables has this shape), `(out, collidable_a, collidable_b)`: stores whether
/// `IsCollisionEnabled` (`00c84740`) answers true for the two collidables' filter infos
/// (`+0x1c`) as a byte at `out`. Answers `out`.
pub fn fn_00c84880(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    collidable_a: Ptr,
    collidable_b: Ptr,
) -> Ptr {
    let first = filter_info_of(e, collidable_a.addr());
    let second = filter_info_of(e, collidable_b.addr());
    answer(e, out, first, second)
}

// Translated from 00c848b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload of
/// two collision bodies has this shape), `(out, _, body_a, body_b, _, _)`: the same as
/// [`fn_00c84880`], with the filter info taken from the root (the end of the parent chain at
/// `+0xc`) of each body.
pub fn fn_00c848b0(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_0: u32,
    body_a: Ptr,
    body_b: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) -> Ptr {
    let second_root = root_of(e, body_b.addr());
    let second = filter_info_of(e, second_root);
    let first_root = root_of(e, body_a.addr());
    let first = filter_info_of(e, first_root);
    answer(e, out, first, second)
}

// Translated from 00c84900 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload for a
/// ray cast against a shape has this shape), `(out, input, _, container, key)`: compares the
/// filter info at `input + 0x20` with the one the container answers from its virtual `+0x10`
/// for `key`, and stores the answer of `IsCollisionEnabled` as a byte at `out`. Answers `out`.
pub fn fn_00c84900(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    input: Ptr,
    _unused_0: u32,
    container: Ptr,
    key: u32,
) -> Ptr {
    let shape_info = e.vcall(container.addr(), 0x10, &args![key]).u32();
    let ray_info = e.mem.u32(input.addr() + 0x20);
    answer(e, out, ray_info, shape_info)
}

// Translated from 00c84930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload for a
/// world ray cast against a collidable has this shape), `(out, input, collidable)`: compares
/// the filter info at `input + 0x24` with the collidable's, and stores the answer of
/// `IsCollisionEnabled` as a byte at `out`. Answers `out`.
pub fn fn_00c84930(e: &mut Engine, _this: Ptr, out: Ptr, input: Ptr, collidable: Ptr) -> Ptr {
    let ray_info = e.mem.u32(input.addr() + 0x24);
    let collidable_info = filter_info_of(e, collidable.addr());
    answer(e, out, ray_info, collidable_info)
}

// Translated from 00c84960 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's longest `isCollisionEnabled`
/// overload has this shape), `(out, _, body_a, body_b, _, _, _, _)`: the same as
/// [`fn_00c848b0`].
pub fn fn_00c84960(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_0: u32,
    body_a: Ptr,
    body_b: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    let first_root = root_of(e, body_a.addr());
    let first = filter_info_of(e, first_root);
    let second_root = root_of(e, body_b.addr());
    let second = filter_info_of(e, second_root);
    answer(e, out, first, second)
}

// Translated from 00c849b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c849b0 - the /GS cookie check is not modeled.
/// `CFilter::GetViewerStrings` (engine map name, from a string reference), `(strings)`: stores
/// one viewer string per field of the filter info word at `this`, in this order: `COLFILTER`
/// with the word as `0x%08X` (`sprintf_s`, `00406d00`, then `00a7d5e0`), `-GROUP` with the high
/// half as `0x%X`, `-LAYER` with the layer's name from the table at `011b0810`, `-LINK` with
/// bit 15, then `-PART`: for the layers 8 and 0x1d the name from the table at `011b08c0`,
/// otherwise, when bit 15 is set, the part number (bits 8 to 12) through `00a7d510`; none
/// else. `-NOCOL` with bit 14 and `-SCALED` with bit 13 follow. The bool strings (`00a7d370`)
/// are passed the dword of the exe's stack slot holding the previous string pointer with its
/// low byte replaced by the bit.
pub fn c_filter_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let array = strings.addr();
    let info_of = |e: &Engine| e.mem.u32(this.addr());
    e.with_stack(0x80, |e, buffer| {
        let info = info_of(e);
        e.call(FORMAT, &args![buffer, 0x80u32, FORMAT_FILTER, info]);
        let text = e
            .call(VIEWER_STRING_OF_TEXT, &args![LABEL_FILTER, buffer])
            .u32();
        store_text(e, array, text);
        let group = u32::from(e.mem.u16(this.addr() + 2));
        e.call(FORMAT, &args![buffer, 0x80u32, FORMAT_GROUP, group]);
        let text = e
            .call(VIEWER_STRING_OF_TEXT, &args![LABEL_GROUP, buffer])
            .u32();
        store_text(e, array, text);
    });
    let info = info_of(e);
    let layer_name = e.mem.u32(LAYER_NAMES + 4 * (info & 0x7f));
    let mut slot = e
        .call(VIEWER_STRING_OF_TEXT, &args![LABEL_LAYER, layer_name])
        .u32();
    store_text(e, array, slot);
    let bit = (info_of(e) >> 0xf) & 1;
    slot = e
        .call(
            VIEWER_STRING_OF_BOOL,
            &args![LABEL_LINK, (slot & 0xffff_ff00) | bit],
        )
        .u32();
    store_text(e, array, slot);
    let info = info_of(e);
    let layer = info & 0x7f;
    let part = (info >> 8) & 0x1f;
    if layer == 8 || layer == 0x1d {
        let part_name = e.mem.u32(PART_NAMES + 4 * part);
        slot = e
            .call(VIEWER_STRING_OF_TEXT, &args![LABEL_PART, part_name])
            .u32();
        store_text(e, array, slot);
    } else if (info >> 0xf) & 1 != 0 {
        slot = e.call(VIEWER_STRING_OF_INT, &args![LABEL_PART, part]).u32();
        store_text(e, array, slot);
    }
    for (label, shift) in [(LABEL_NO_COLLISION, 0xe), (LABEL_SCALED, 0xd)] {
        let bit = (info_of(e) >> shift) & 1;
        slot = e
            .call(
                VIEWER_STRING_OF_BOOL,
                &args![label, (slot & 0xffff_ff00) | bit],
            )
            .u32();
        store_text(e, array, slot);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c84470, bhk_collision_filter_get_debug_color(u32, Ptr) -> Ptr),
        entry!(0x00c84880, fn_00c84880(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c848b0, fn_00c848b0(Ptr, Ptr, u32, Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00c84900, fn_00c84900(Ptr, Ptr, Ptr, u32, Ptr, u32) -> Ptr),
        entry!(0x00c84930, fn_00c84930(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c84960, fn_00c84960(Ptr, Ptr, u32, Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00c849b0, c_filter_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The words of the color a call of `GetDebugColor` for `filter` wrote.
    fn debug_color(e: &mut Engine, filter: u32) -> [u32; 3] {
        let color = e.mem.alloc(0xc);
        let result = e.call(0x00c8_4470, &args![filter, color]).u32();
        assert_eq!(result, color);
        [0, 4, 8].map(|offset| e.mem.u32(color + offset))
    }

    #[test]
    fn bhk_collision_filter_get_debug_color_picks_the_color_of_the_layer() {
        let mut e = logged_engine();
        // Every constant the table reads holds its own address as a float.
        for address in [
            0x0101_77e8u32,
            0x010c_5818,
            0x010c_5814,
            0x010c_5810,
            0x0103_0ff0,
            0x0101_8238,
            0x0104_e2b8,
        ] {
            e.mem.set_f32(address, address as f32);
        }
        for (index, word) in [0x1111u32, 0x2222, 0x3333].into_iter().enumerate() {
            e.mem.set_u32(DEFAULT_DEBUG_COLOR + 4 * index as u32, word);
        }
        let one = 1.0f32.to_bits();
        let value = |address: u32| (address as f32).to_bits();
        // Layer 1: (1, 0, 0).
        assert_eq!(debug_color(&mut e, 1), [one, 0, 0]);
        // The bits above the layer are ignored; layers 2 and 10 share a color.
        let layer_two = [value(0x0101_77e8), value(0x010c_5818), value(0x010c_5814)];
        assert_eq!(debug_color(&mut e, 0xabcd_0082), layer_two);
        assert_eq!(debug_color(&mut e, 10), layer_two);
        // Layer 3 repeats the second constant in the third place.
        assert_eq!(
            debug_color(&mut e, 3),
            [value(0x010c_5810), value(0x0103_0ff0), value(0x0103_0ff0)]
        );
        // Layers 8, 18, 30 and 31.
        assert_eq!(debug_color(&mut e, 8), [0, one, 0]);
        assert_eq!(debug_color(&mut e, 18), [0, value(0x0101_8238), 0]);
        assert_eq!(debug_color(&mut e, 30), [one, one, 0]);
        assert_eq!(debug_color(&mut e, 31), [one, value(0x0103_0ff0), 0]);
        // Layers outside the table (0, 15, 20 and 32 and up) copy the default words.
        for filter in [0u32, 15, 20, 32, 0x7f, 0x80] {
            assert_eq!(
                debug_color(&mut e, filter),
                [0x1111, 0x2222, 0x3333],
                "{filter}"
            );
        }
        // A signalling NaN comes out quiet; the default words are copied as they are.
        e.mem.set_u32(0x0101_77e8, 0x7f80_0001);
        assert_eq!(debug_color(&mut e, 2)[0], 0x7fc0_0001);
        e.mem.set_u32(DEFAULT_DEBUG_COLOR, 0x7f80_0001);
        assert_eq!(debug_color(&mut e, 0)[0], 0x7f80_0001);
    }

    #[test]
    fn fn_00c84880_compares_the_filter_infos_of_two_collidables() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] < a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x1c, 5);
        e.mem.set_u32(second + 0x1c, 9);
        let result = e
            .call(0x00c8_4880, &args![0x1234u32, out, first, second])
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![5, 9]]);
        e.call(0x00c8_4880, &args![0x1234u32, out, second, first]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED)[1], vec![9, 5]);
    }

    /// Two bodies, each a chain `body -> parent -> root` whose root has the filter info.
    fn body_chain(e: &mut Engine, info: u32) -> u32 {
        let root = e.mem.alloc(0x20);
        e.mem.set_u32(root + 0x1c, info);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(parent + 0xc, root);
        e.mem.set_u32(parent + 0x1c, 0xdead);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0xc, parent);
        e.mem.set_u32(body + 0x1c, 0xbeef);
        body
    }

    #[test]
    fn fn_00c848b0_compares_the_filter_infos_of_the_roots_of_two_bodies() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] < a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let first = body_chain(&mut e, 3);
        let second = body_chain(&mut e, 8);
        let result = e
            .call(
                0x00c8_48b0,
                &args![0u32, out, 1u32, first, second, 2u32, 3u32],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![3, 8]]);
        // A body without parent is its own root.
        let alone = e.mem.alloc(0x20);
        e.mem.set_u32(alone + 0x1c, 1);
        e.call(
            0x00c8_48b0,
            &args![0u32, out, 1u32, alone, second, 2u32, 3u32],
        );
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED)[1], vec![1, 8]);
    }

    #[test]
    fn fn_00c84900_compares_the_ray_filter_with_the_one_the_container_answers() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] == a[1]) as u32).into_ret()
        });
        e.register_double(0x7000_0010, |_, a| (a[1] + 0x100).into_ret());
        e.put_vtable(0x7001_1000, &[0, 0, 0, 0, 0x7000_0010]);
        let container = e.mem.alloc(0x10);
        e.mem.set_u32(container, 0x7001_1000);
        let out = e.mem.alloc(4);
        let input = e.mem.alloc(0x30);
        e.mem.set_u32(input + 0x20, 0x107);
        let result = e
            .call(0x00c8_4900, &args![0u32, out, input, 0u32, container, 7u32])
            .u32();
        assert_eq!(result, out);
        assert_eq!(calls_to(&e, 0x7000_0010), vec![vec![container, 7]]);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![0x107, 0x107]]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(input + 0x20, 0x108);
        e.call(0x00c8_4900, &args![0u32, out, input, 0u32, container, 7u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00c84930_compares_the_world_ray_filter_with_the_collidable() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] > a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let input = e.mem.alloc(0x30);
        e.mem.set_u32(input + 0x24, 20);
        let collidable = e.mem.alloc(0x20);
        e.mem.set_u32(collidable + 0x1c, 10);
        let result = e
            .call(0x00c8_4930, &args![0u32, out, input, collidable])
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![20, 10]]);
    }

    #[test]
    fn fn_00c84960_compares_the_filter_infos_of_the_roots_of_two_bodies() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] > a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let first = body_chain(&mut e, 30);
        let second = body_chain(&mut e, 4);
        let result = e
            .call(
                0x00c8_4960,
                &args![0u32, out, 0u32, first, second, 1u32, 2u32, 3u32, 4u32],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![30, 4]]);
    }

    #[test]
    fn c_filter_get_viewer_strings_stores_one_string_per_field() {
        let mut e = logged_engine();
        e.register(FORMAT, |_, _| Ret::default());
        e.register(VIEWER_STRING_OF_TEXT, |_, a| {
            (0x6000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_OF_BOOL, |_, a| {
            (0x7000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_OF_INT, |_, a| {
            (0x8000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        // The layer and part name tables.
        for index in 0..0x80u32 {
            e.mem.set_u32(LAYER_NAMES + 4 * index, 0x7500_0000 + index);
        }
        for index in 0..0x20u32 {
            e.mem.set_u32(PART_NAMES + 4 * index, 0x7600_0000 + index);
        }
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x80);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0x20);
        e.mem.set_u16(array + 0xe, 4);
        let filter = e.mem.alloc(8);
        // Group 0x1234, linked, scaled, part 5, layer 8.
        let info = 0x1234_0000u32 | 0x8000 | 0x2000 | 0x0500 | 0x08;
        e.mem.set_u32(filter, info);
        e.call(0x00c8_49b0, &args![filter, array]);
        let texts: Vec<u32> = (0..7).map(|index| e.mem.u32(base + 4 * index)).collect();
        assert_eq!(
            texts,
            vec![
                0x6000_0000 | (LABEL_FILTER & 0xffff),
                0x6000_0000 | (LABEL_GROUP & 0xffff),
                0x6000_0000 | (LABEL_LAYER & 0xffff),
                0x7000_0000 | (LABEL_LINK & 0xffff),
                0x6000_0000 | (LABEL_PART & 0xffff),
                0x7000_0000 | (LABEL_NO_COLLISION & 0xffff),
                0x7000_0000 | (LABEL_SCALED & 0xffff),
            ]
        );
        assert_eq!(e.mem.u16(array + 0xa), 7);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[0][1..], [0x80, FORMAT_FILTER, info]);
        assert_eq!(formats[1][1..], [0x80, FORMAT_GROUP, 0x1234]);
        // Each text is made from the formatted buffer.
        let buffer = formats[0][0];
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_TEXT)[0],
            vec![LABEL_FILTER, buffer]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_TEXT)[2],
            vec![LABEL_LAYER, 0x7500_0008]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_TEXT)[3],
            vec![LABEL_PART, 0x7600_0005]
        );
        // The bool words keep the high bytes of the previous string pointer: the layer text
        // before "-LINK", the part text before "-NOCOL", "-NOCOL"'s before "-SCALED".
        let bools = calls_to(&e, VIEWER_STRING_OF_BOOL);
        assert_eq!(bools[0], vec![LABEL_LINK, 0x6000_5800 | 1]);
        assert_eq!(bools[1], vec![LABEL_NO_COLLISION, 0x6000_5800]);
        assert_eq!(bools[2], vec![LABEL_SCALED, 0x7000_5800 | 1]);
    }

    #[test]
    fn c_filter_get_viewer_strings_uses_the_part_number_of_a_linked_filter() {
        let mut e = logged_engine();
        e.register(FORMAT, |_, _| Ret::default());
        e.register(VIEWER_STRING_OF_TEXT, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_OF_BOOL, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_OF_INT, |_, a| {
            (0x8000_0000 | (a[1] & 0xffff)).into_ret()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x80);
        e.mem.set_u32(array + 4, base);
        // A full array grows by its grow-by before the first store.
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let filter = e.mem.alloc(8);
        // Linked, part 6, layer 3: the part number is the string; no "-PART" name lookup.
        e.mem.set_u32(filter, 0x8000 | 0x0600 | 0x03);
        e.call(0x00c8_49b0, &args![filter, array]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_INT),
            vec![vec![LABEL_PART, 6]]
        );
        assert_eq!(e.mem.u16(array + 0xa), 7);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE)[0], vec![array, 4]);
        // Without the link bit, a layer other than 8 and 0x1d stores no "-PART" string.
        e.mem.set_u16(array + 0xa, 0);
        e.mem.set_u32(filter, 0x0600 | 0x03);
        e.call(0x00c8_49b0, &args![filter, array]);
        assert_eq!(calls_to(&e, VIEWER_STRING_OF_INT).len(), 1);
        assert_eq!(e.mem.u16(array + 0xa), 6);
    }
}
