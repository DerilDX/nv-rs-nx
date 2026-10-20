//! `BSMain/bstexturedata.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c5f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c5ed60` and frees 0x1c bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_texture_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c5_ed60, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00c5f560 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01204378 without reading the receiver. The map names this body `BSTextureData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c5f560(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_4378
}

/// The global holding `BSTextureData`'s extra-data name (an allocated `NiFixedString`).
const TEXTURE_DATA_NAME: u32 = 0x0120_4374;
/// `"TXD"`.
const TEXTURE_DATA_NAME_TEXT: u32 = 0x010c_3194;
/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `NiFixedString` maker (`00a5b690`, cdecl: the text; returns the handle).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The empty `NiFixedString` handle.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c5f240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5f240 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ff15, state variable) is not translated
/// The start-up initializer of the texture-data name: a 4-byte `NiFixedString` from the
/// memory manager set to `"TXD"` (`00a5b690`); the compiler kept a test of the constant
/// text's address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) never runs (translated anyway). The string (or null when the allocation
/// failed) goes to `01204374`.
pub fn fn_00c5f240(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(TEXTURE_DATA_NAME_TEXT);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![TEXTURE_DATA_NAME_TEXT])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(TEXTURE_DATA_NAME, stored);
}

// Translated from 00c5f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the name exists, releases its handle (unless it is the
/// empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the text),
/// frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always true),
/// and clears the global.
pub fn fn_00c5f2e0(e: &mut Engine) {
    let string = e.global::<u32>(TEXTURE_DATA_NAME);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(TEXTURE_DATA_NAME, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_f560, fn_00c5f560(Ptr) -> u32),
        entry!(0x00c5_f360, bs_texture_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c5_f240, fn_00c5f240()),
        entry!(0x00c5_f2e0, fn_00c5f2e0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_texture_data_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c5_f360, 0x00c5_ed60, 0x1c);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c5f560_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_f560, &args![0x1000u32]).u32(), 0x0120_4378);
        assert_eq!(e.call(0x00c5_f560, &args![0u32]).u32(), 0x0120_4378);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn name_initializer_and_destructor() {
        let mut e = Engine::new();
        e.map(TEXTURE_DATA_NAME, 4);
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
        let block = e.mem.alloc(4);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(FIXED_STRING_CREATE, |_, _| 0x8008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_f240, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_f240),
            vec![
                (ALLOCATE, vec![MEMORY_MANAGER, 4]),
                (FIXED_STRING_CREATE, vec![TEXTURE_DATA_NAME_TEXT]),
            ]
        );
        assert_eq!(e.mem.u32(TEXTURE_DATA_NAME), block);
        assert_eq!(e.mem.u32(block), 0x8008);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f2e0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_f2e0),
            vec![
                (INTERLOCKED_DECREMENT, vec![0x8000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(TEXTURE_DATA_NAME), 0);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00c5_f240, &args![]);
        assert_eq!(e.mem.u32(TEXTURE_DATA_NAME), 0);
        e.mem.set_u32(block, 0x4444);
        e.mem.set_u32(TEXTURE_DATA_NAME, block);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f2e0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_f2e0),
            vec![(DEALLOCATE, vec![MEMORY_MANAGER, block])]
        );
    }
}
