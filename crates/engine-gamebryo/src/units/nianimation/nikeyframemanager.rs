//! `NiAnimation/nikeyframemanager.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyframeManager::ms_RTTI` (the class's `NiRTTI` record).
const NI_KEYFRAME_MANAGER_RTTI: u32 = 0x011f_3fb4;

// Translated from 00a4da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_keyframe_manager_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_KEYFRAME_MANAGER_RTTI)
}

// Translated from 00a4d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A string hash for a table whose bucket count is the word at +4 (thiscall, `RET 4`):
/// `h = h * 33 + c` over the bytes of `name` (each sign-extended), then `h % buckets`
/// (unsigned).
pub fn fn_00a4d6d0(e: &mut Engine, this: Ptr, name: Ptr) -> u32 {
    let mut hash = 0u32;
    let mut p = name.addr();
    loop {
        let c = e.mem.u8(p);
        if c == 0 {
            break;
        }
        hash = (hash << 5)
            .wrapping_add(hash)
            .wrapping_add(c as i8 as i32 as u32);
        p += 1;
    }
    hash % e.mem.u32(this.addr() + 4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4da80, ni_keyframe_manager_get_rtti() -> Ptr),
        entry!(0x00a4_d6d0, fn_00a4d6d0(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_keyframe_manager_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4da80, &args![]).u32(), NI_KEYFRAME_MANAGER_RTTI);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn string_hash_modulo_the_bucket_count() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 37);
        let name = e.mem.alloc(8);
        for (i, b) in [b'a', b'b', 0xe9u8].iter().enumerate() {
            e.mem.set_u8(name + i as u32, *b);
        }
        // 0xe9 is a negative char: -23.
        let hash = (u32::from(b'a') * 33 + u32::from(b'b')) * 33;
        let want = hash.wrapping_sub(23) % 37;
        assert_eq!(e.call(0x00a4_d6d0, &args![this, name]).u32(), want);
        let empty = e.mem.alloc(1);
        assert_eq!(e.call(0x00a4_d6d0, &args![this, empty]).u32(), 0);
    }
}
