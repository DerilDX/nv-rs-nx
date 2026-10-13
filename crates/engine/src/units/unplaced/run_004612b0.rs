//! `(unplaced)/run_004612b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The form-type table: 12-byte rows, the first word of a row is the address
/// of the type's name string (the row for type 0 begins with `"NONE"`).
const FORM_TYPE_TABLE: u32 = 0x0118_7004;
/// Bytes per row of [`FORM_TYPE_TABLE`].
const FORM_TYPE_ROW: u32 = 12;
/// Number of types in the table (0x00 to 0x78).
const FORM_TYPE_COUNT: u32 = 0x79;
/// The empty string (`""`) answered for a type outside the table.
const EMPTY_STRING: u32 = 0x0101_1584;

// Translated from 004612b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name of a form type (the byte `TESForm::cFormType`): the address of
/// its string in the table of 0x79 rows, or of the empty string for a type
/// from 0x79 up. `CreateFormOfType` uses it when it meets a type it has no
/// constructor for.
pub fn fn_004612b0(e: &mut Engine, form_type: u8) -> u32 {
    if (form_type as u32) < FORM_TYPE_COUNT {
        e.mem
            .u32(FORM_TYPE_TABLE + form_type as u32 * FORM_TYPE_ROW)
    } else {
        EMPTY_STRING
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004612b0, fn_004612b0(u8) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_row_name_and_the_empty_string_past_the_table() {
        let mut e = Engine::new();
        e.map(FORM_TYPE_TABLE & !0xfff, 0x2000);
        e.set_global(FORM_TYPE_TABLE, 0x0101_a6bfu32);
        e.set_global(FORM_TYPE_TABLE + 5 * FORM_TYPE_ROW, 0x0102_0000u32);
        e.set_global(FORM_TYPE_TABLE + 0x78 * FORM_TYPE_ROW, 0x0102_8000u32);
        assert_eq!(e.call(0x0046_12b0, &args![0u8]).u32(), 0x0101_a6bf);
        assert_eq!(e.call(0x0046_12b0, &args![5u8]).u32(), 0x0102_0000);
        assert_eq!(e.call(0x0046_12b0, &args![0x78u8]).u32(), 0x0102_8000);
        assert_eq!(e.call(0x0046_12b0, &args![0x79u8]).u32(), EMPTY_STRING);
        assert_eq!(e.call(0x0046_12b0, &args![0xffu8]).u32(), EMPTY_STRING);
    }
}
