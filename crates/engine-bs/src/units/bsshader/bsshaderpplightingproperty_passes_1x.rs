//! `BSShader/bsshaderpplightingproperty_passes_1x.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The pass builders here share one shape: from their flag words they pick a render-pass
//! enum and, when the pass-count byte is 1, add that pass to the property's render-pass list
//! (`pRenderPassList`, +0x3c) with `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`);
//! otherwise they count one pass in the caller's 16-bit counter. Either way the caller's
//! first-pass byte is cleared afterwards.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

/// `BSShaderProperty::RenderPassArray::Add` (`thiscall`, eight words: geometry, pass enum,
/// first-pass flag, light count, four light words).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// The formatter `00406d00` (`cdecl`: buffer, size, format, arguments).
const FORMAT: u32 = 0x0040_6d00;
/// `"Parent:%s,Child:%s"`.
const PARENT_CHILD_FORMAT: u32 = 0x010b_c5f4;
/// `"Parent:%s"`.
const PARENT_FORMAT: u32 = 0x010b_c5e8;
/// The one-string format at `01019f08` the code uses for the geometry's own name.
const NAME_FORMAT_01019F08: u32 = 0x0101_9f08;
/// The format without arguments at `010250cc` the code uses when there is no name.
const NO_NAME_FORMAT_010250CC: u32 = 0x0102_50cc;
/// `"SHADER ERROR (%s)(%08X) : no shader to handle TEXTURE_SFgVc ( texture skinned facegen
/// vertexcolors )"`.
const SKINNED_FACEGEN_ERROR_FORMAT: u32 = 0x010b_c580;

/// The `Add` call every builder makes: the pass with the first-pass byte, no lights.
fn add_pass(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass: u32,
    first_pass: Ptr<u8>,
) {
    let first = u32::from(e.mem.u8(first_pass.addr()));
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, first, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

/// Adds `pass` when `pass_count` is 1, else counts one more pass in `counter`.
fn add_or_count(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
) {
    if pass_count == 1 {
        add_pass(e, this, geometry, pass, first_pass);
    } else {
        let count = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), count.wrapping_add(1));
    }
}

// Translated from 00bd9e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`: geometry, pass counter, pass count,
/// first-pass byte, flag): pass `0x1e6` when the flag is set, `0x1e5` otherwise; added when
/// the pass count is 1, counted otherwise. Clears the first-pass byte and returns 0.
pub fn fn_00bd9e60(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
) -> u32 {
    let pass = if flag { 0x1e6 } else { 0x1e5 };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd9f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: geometry, pass counter, pass count,
/// first-pass byte, two flags): pass 3 when the first flag is set, otherwise 2 when the
/// second flag is set and 1 when not; added when the pass count is 1, counted otherwise.
/// Clears the first-pass byte and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9f00(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
    second_flag: bool,
) -> u32 {
    let pass = if flag { 3 } else { u32::from(second_flag) + 1 };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bdac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddTexturePass_1x` (engine map; the body matches;
/// `thiscall`, `RET 0x2c`: geometry, an unread word, pass counter, pass count, first-pass
/// byte, then the flags at stack words 6 and 8 to 11, word 7 unread). The pass:
///
/// - flag 6 clear: `0x151` when flag 10 is set; else with flag 9 set `0x14c` (flag 11 set)
///   or `0x14e`; else `0x14b` (flag 8 set) or `0x14a`;
/// - flag 6 set, flag 8 clear: `0x14d` when flag 9 is clear, else `0x152` (flag 11 set) or
///   `0x14f`;
/// - flags 6 and 8 set: `0x150` when flag 9 is clear. With flag 9 set there is no pass: the
///   code formats the geometry's description (`"Parent:%s,Child:%s"` with its parent's and
///   its own name, `"Parent:%s"`, the one-name format `01019f08`, or the format `010250cc`
///   without a name or geometry; names are the `NiFixedString` handles at +0x08 of the
///   geometry and of its parent at +0x18) into a 128-byte buffer and then the
///   `TEXTURE_SFgVc` shader error message into a 255-byte one, and neither adds nor counts.
///
/// A chosen pass is added when the pass count is 1 and counted otherwise. The first-pass
/// byte is cleared on every path; the result is 0.
///
/// The /GS stack cookie check (`00ec408c`) is not translated.
// Translation incomplete: 00bdac00 - the /GS stack cookie check (00ec408c) is not translated
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_texture_pass_1x(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    _unused_2: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag6: bool,
    _unused_7: u32,
    flag8: bool,
    flag9: bool,
    flag10: bool,
    flag11: bool,
) -> u32 {
    let pass = if !flag6 {
        if flag10 {
            Some(0x151)
        } else if flag9 {
            Some(if flag11 { 0x14c } else { 0x14e })
        } else if flag8 {
            Some(0x14b)
        } else {
            Some(0x14a)
        }
    } else if !flag8 {
        if !flag9 {
            Some(0x14d)
        } else if flag11 {
            Some(0x152)
        } else {
            Some(0x14f)
        }
    } else if !flag9 {
        Some(0x150)
    } else {
        None
    };
    match pass {
        Some(pass) => add_or_count(e, this, geometry, pass, counter, pass_count, first_pass),
        None => e.with_stack(0x180, |e, buffers| {
            let description = buffers.addr();
            let message = description + 0x80;
            if geometry == 0 {
                e.call(
                    FORMAT,
                    &args![description, 0x80u32, NO_NAME_FORMAT_010250CC],
                );
            } else {
                // NiAVObject::m_pkParent (Xbox PDB) +0x18, m_kName +0x08.
                let parent = e.mem.u32(geometry + 0x18);
                let parent_name = if parent != 0 {
                    e.mem.u32(parent + 8)
                } else {
                    0
                };
                let name = e.mem.u32(geometry + 8);
                if parent_name != 0 {
                    if name != 0 {
                        e.call(
                            FORMAT,
                            &args![description, 0x80u32, PARENT_CHILD_FORMAT, parent_name, name],
                        );
                    } else {
                        e.call(
                            FORMAT,
                            &args![description, 0x80u32, PARENT_FORMAT, parent_name],
                        );
                    }
                } else if name != 0 {
                    e.call(
                        FORMAT,
                        &args![description, 0x80u32, NAME_FORMAT_01019F08, name],
                    );
                } else {
                    e.call(
                        FORMAT,
                        &args![description, 0x80u32, NO_NAME_FORMAT_010250CC],
                    );
                }
            }
            e.call(
                FORMAT,
                &args![
                    message,
                    0xffu32,
                    SKINNED_FACEGEN_ERROR_FORMAT,
                    description,
                    geometry
                ],
            );
        }),
    }
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bd_9e60,
            fn_00bd9e60(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool) -> u32
        ),
        entry!(
            0x00bd_9f00,
            fn_00bd9f00(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool, bool) -> u32
        ),
        entry!(
            0x00bd_ac00,
            bs_shader_pp_lighting_property_add_texture_pass_1x(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
                u32,
                bool,
                bool,
                bool,
                bool,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: u32 = 0x0070_0000;

    /// A property whose render-pass list is `LIST`, a counter of 5, a first-pass byte of 1,
    /// and `Add` and the formatter doubled.
    fn setup() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        e.register(FORMAT, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x3c, LIST);
        let counter = e.mem.alloc(2);
        e.mem.set_u16(counter, 5);
        let first = e.mem.alloc(1);
        e.mem.set_u8(first, 1);
        e.call_log = Some(vec![]);
        (e, this, counter, first)
    }

    fn calls_to(e: &mut Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn fn_00bd9e60_adds_1e5_or_1e6_or_counts() {
        for (flag, count, pass) in [(0u32, 1u32, Some(0x1e5)), (1, 1, Some(0x1e6)), (1, 2, None)] {
            let (mut e, this, counter, first) = setup();
            let r = e.call(0x00bd_9e60, &[this, 0x1234, counter, count, first, flag]);
            assert_eq!(r.u32(), 0);
            assert_eq!(e.mem.u8(first), 0);
            let calls = calls_to(&mut e, RENDER_PASS_ARRAY_ADD);
            match pass {
                Some(pass) => {
                    assert_eq!(calls, vec![vec![LIST, 0x1234, pass, 1, 0, 0, 0, 0, 0]]);
                    assert_eq!(e.mem.u16(counter), 5);
                }
                None => {
                    assert!(calls.is_empty());
                    assert_eq!(e.mem.u16(counter), 6);
                }
            }
        }
    }

    #[test]
    fn fn_00bd9f00_picks_3_2_or_1() {
        for (flag, second, pass) in [(1u32, 0u32, 3u32), (0, 1, 2), (0, 0, 1), (0, 0x100, 1)] {
            let (mut e, this, counter, first) = setup();
            e.call(0x00bd_9f00, &[this, 0x10, counter, 1, first, flag, second]);
            assert_eq!(
                calls_to(&mut e, RENDER_PASS_ARRAY_ADD),
                vec![vec![LIST, 0x10, pass, 1, 0, 0, 0, 0, 0]]
            );
            assert_eq!(e.mem.u8(first), 0);
        }
        // Only the low byte of the pass count is compared.
        let (mut e, this, counter, first) = setup();
        e.call(0x00bd_9f00, &[this, 0x10, counter, 0x102, first, 0, 0]);
        assert!(calls_to(&mut e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u16(counter), 6);
        assert_eq!(e.mem.u8(first), 0);
    }

    /// Flags in order 6, 8, 9, 10, 11.
    fn texture_pass(
        e: &mut Engine,
        this: u32,
        counter: u32,
        first: u32,
        f: [u32; 5],
        geometry: u32,
    ) {
        let r = e.call(
            0x00bd_ac00,
            &[
                this, geometry, 0, counter, 1, first, f[0], 0, f[1], f[2], f[3], f[4],
            ],
        );
        assert_eq!(r.u32(), 0);
    }

    #[test]
    fn add_texture_pass_1x_chooses_each_pass() {
        let cases = [
            ([0u32, 0, 0, 1, 0], 0x151u32),
            ([0, 0, 1, 0, 1], 0x14c),
            ([0, 0, 1, 0, 0], 0x14e),
            ([0, 1, 0, 0, 0], 0x14b),
            ([0, 0, 0, 0, 0], 0x14a),
            ([1, 0, 0, 0, 0], 0x14d),
            ([1, 0, 1, 0, 1], 0x152),
            ([1, 0, 1, 0, 0], 0x14f),
            ([1, 1, 0, 0, 0], 0x150),
        ];
        for (flags, pass) in cases {
            let (mut e, this, counter, first) = setup();
            texture_pass(&mut e, this, counter, first, flags, 0x40);
            assert_eq!(
                calls_to(&mut e, RENDER_PASS_ARRAY_ADD),
                vec![vec![LIST, 0x40, pass, 1, 0, 0, 0, 0, 0]],
                "{flags:?}"
            );
            assert_eq!(e.mem.u8(first), 0);
        }
        // A pass count other than 1 counts.
        let (mut e, this, counter, first) = setup();
        e.call(
            0x00bd_ac00,
            &[this, 0x40, 0, counter, 2, first, 0, 0, 0, 0, 0, 0],
        );
        assert!(calls_to(&mut e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u16(counter), 6);
    }

    #[test]
    fn add_texture_pass_1x_reports_skinned_facegen_without_counting() {
        // No geometry.
        let (mut e, this, counter, first) = setup();
        texture_pass(&mut e, this, counter, first, [1, 1, 1, 0, 0], 0);
        let calls = calls_to(&mut e, FORMAT);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][1..], [0x80, NO_NAME_FORMAT_010250CC]);
        assert_eq!(
            calls[1][1..],
            [0xff, SKINNED_FACEGEN_ERROR_FORMAT, calls[0][0], 0]
        );
        assert_eq!(calls[1][0], calls[0][0] + 0x80);
        assert_eq!(e.mem.u16(counter), 5);
        assert_eq!(e.mem.u8(first), 0);

        // Parent and child names, parent only, own name only, neither.
        for (parent_name, name, expected) in [
            (0x111u32, 0x222u32, vec![PARENT_CHILD_FORMAT, 0x111, 0x222]),
            (0x111, 0, vec![PARENT_FORMAT, 0x111]),
            (0, 0x222, vec![NAME_FORMAT_01019F08, 0x222]),
            (0, 0, vec![NO_NAME_FORMAT_010250CC]),
        ] {
            let (mut e, this, counter, first) = setup();
            let geometry = e.mem.alloc(0x20);
            let parent = e.mem.alloc(0x20);
            e.mem.set_u32(geometry + 0x18, parent);
            e.mem.set_u32(parent + 8, parent_name);
            e.mem.set_u32(geometry + 8, name);
            texture_pass(&mut e, this, counter, first, [1, 1, 1, 0, 0], geometry);
            let calls = calls_to(&mut e, FORMAT);
            assert_eq!(calls[0][2..], expected[..]);
            assert_eq!(calls[1][4], geometry);
            assert_eq!(e.mem.u16(counter), 5);
        }
        // A geometry without a parent uses its own name.
        let (mut e, this, counter, first) = setup();
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 8, 0x333);
        texture_pass(&mut e, this, counter, first, [1, 1, 1, 0, 0], geometry);
        assert_eq!(
            calls_to(&mut e, FORMAT)[0][2..],
            [NAME_FORMAT_01019F08, 0x333]
        );
    }
}
