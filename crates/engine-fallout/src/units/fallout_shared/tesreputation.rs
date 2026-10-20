//! `fallout shared/tesreputation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `00484ab0`: the form flag setter.
const FORM_SET_FLAG: u32 = 0x0048_4ab0;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 00616290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00616290 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESReputation::Load` (Xbox PDB): `TESForm::LoadForm`, `00484ab0(0)` and
/// each chunk: `EDID` (vtable +0x134), `OBND` (vtable +0xE0), `FULL`
/// (+0x18, `00487050`), `ICON` (+0x24, `TESTexture::LoadTextureChunk`),
/// `MICO` (+0x30, `0047a290`) and `DATA` (four bytes at +0x40,
/// `TESForm::LoadData`, swapped by `00503210` when the file asks). Returns
/// true.
pub fn tes_reputation_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    let or_null = |offset: u32| if this.is_null() { 0 } else { base + offset };
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_FULL => {
                e.call(0x0048_7050, &args![or_null(0x18), file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x40, 4u32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x0050_3210, &args![base + 0x40]);
                }
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ICON => {
                e.call(0x0048_e3d0, &args![or_null(0x24), file]);
            }
            CHUNK_MICO => {
                e.call(0x0047_a290, &args![or_null(0x30), file]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061_6290, tes_reputation_load(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn load(swap: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let chunks = Rc::new(RefCell::new((
            vec![
                CHUNK_EDID, CHUNK_OBND, CHUNK_FULL, CHUNK_ICON, CHUNK_MICO, CHUNK_DATA,
            ],
            0usize,
        )));
        let file = e.mem.alloc(0x10);
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        for addr in [
            0x7000_00e0u32,
            0x7000_0134,
            FILE_CHUNK_DATA_SIZED,
            0x0048_5110,
            FORM_SET_FLAG,
            0x0048_7050,
            0x0048_e3d0,
            0x0047_a290,
            0x0048_61f0,
            0x0050_3210,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_6290, &args![this, file]).bool());
        (e, this, file)
    }

    #[test]
    fn load_sends_each_chunk_to_its_component() {
        let (e, this, file) = load(true);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, FILE_CHUNK_DATA_SIZED)[0][2], 0x200);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(calls(&e, 0x0048_e3d0), vec![vec![this + 0x24, file]]);
        assert_eq!(calls(&e, 0x0047_a290), vec![vec![this + 0x30, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x40, 4]]
        );
        assert_eq!(calls(&e, 0x0050_3210), vec![vec![this + 0x40]]);

        let (e, _, _) = load(false);
        assert!(calls(&e, 0x0050_3210).is_empty());
    }
}
