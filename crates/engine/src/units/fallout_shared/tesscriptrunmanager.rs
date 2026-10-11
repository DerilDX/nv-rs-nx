//! `fallout shared/tesscriptrunmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ScriptRunner` (Xbox PDB), size and offsets checked against PC
    /// constructor 005e0c60 and busy predicate 005e2850.
    pub struct ScriptRunner: 0xa4 {
        0x14 m_scriptRunning: Ptr,
    }
    /// `ScriptRunManager` (Xbox PDB), unchanged eight-byte PC layout.
    pub struct ScriptRunManager: 8 {
        0 m_pScriptRunner: Ptr<ScriptRunner>,
        4 pCurrentRunner: Ptr<ScriptRunner>,
    }
}

const MANAGER: u32 = 0x011c_aed0;
const INITIALIZED: u32 = 0x011c_aed8;

// Translated from 005e0c60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `ScriptRunner::ScriptRunner` (Xbox PDB). Preserves the two padding bytes.
pub fn runner_construct(e: &mut Engine, this: Ptr<ScriptRunner>) -> Ptr<ScriptRunner> {
    for offset in [0x20, 0x4c, 0x18, 0x1c] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in [0x24, 0x50, 0x78] {
        e.call(0x0040_3d30, &args![this.addr() + offset, 0u32, 0x28u32]);
    }
    for offset in [0, 4, 8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    e.mem.set_u8(this.addr() + 0xa0, 0);
    e.mem.set_u8(this.addr() + 0xa1, 0);
    this
}

// Translated from 005e24d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e24d0 - compiler SEH exception cleanup.
/// `ScriptRunManager::Instance` (Xbox PDB). Sets the guard before construction.
pub fn instance(e: &mut Engine) -> Ptr<ScriptRunManager> {
    if e.mem.u32(INITIALIZED) & 1 == 0 {
        e.mem.set_u32(INITIALIZED, e.mem.u32(INITIALIZED) | 1);
        e.call(0x0096_a2d0, &args![MANAGER]);
        e.call(0x00ec_658f, &args![0x00fc_b1f0u32]);
    }
    Ptr::new(MANAGER)
}

// Translated from 005e2730 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e2730 - compiler SEH exception cleanup.
/// `ScriptRunManager::InitActionList` (Xbox PDB): uses the cached runner,
/// or a temporary runner if the cached one is busy. The interpreter entry
/// 005e1160 remains a dependency, not a replacement implementation here.
pub fn init_action_list(
    e: &mut Engine,
    this: Ptr<ScriptRunManager>,
    script: Ptr,
    reference: Ptr,
    locals: Ptr,
) -> bool {
    if e.get(this, ScriptRunManager::m_pScriptRunner).is_null() {
        let block = e.call(0x0040_1000, &args![0xa4u32]).u32();
        let runner = if block == 0 {
            Ptr::NULL
        } else {
            e.call(0x005e_0c60, &args![block]).ptr::<ScriptRunner>()
        };
        e.set(this, ScriptRunManager::m_pScriptRunner, runner);
    }
    let runner = e.get(this, ScriptRunManager::m_pScriptRunner);
    if e.call(0x005e_2850, &args![runner]).bool() {
        let block = e.call(0x0040_1000, &args![0xa4u32]).u32();
        let temporary = if block == 0 {
            0
        } else {
            e.call(0x005e_0c60, &args![block]).u32()
        };
        e.call(0x005e_1160, &args![temporary, script, reference, locals]);
        e.call(0x0040_1030, &args![temporary]);
    } else {
        let runner = e.get(this, ScriptRunManager::m_pScriptRunner);
        e.call(0x005e_1160, &args![runner, script, reference, locals]);
    }
    true
}

// Translated from 005e2850 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Tests the full script pointer, returning the result in AL.
pub fn runner_is_busy(e: &mut Engine, this: Ptr<ScriptRunner>) -> bool {
    !e.get(this, ScriptRunner::m_scriptRunning).is_null()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005e0c60,
            runner_construct(Ptr<ScriptRunner>) -> Ptr<ScriptRunner>
        ),
        entry!(0x005e24d0, instance() -> Ptr<ScriptRunManager>),
        entry!(
            0x005e2730,
            init_action_list(Ptr<ScriptRunManager>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x005e2850, runner_is_busy(Ptr<ScriptRunner>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_clears_fields_and_keeps_padding_and_guards() {
        // Three guarded nv-call vectors agree with PC 005e0c60, unpacked
        // exe SHA-256 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        for fill in [0xa5, 0xcc, 0xff] {
            let mut e = Engine::new();
            let block = e.mem.alloc(0xac);
            e.mem.write(block, &[fill; 0xac]);
            let this = block + 4;
            assert_eq!(e.call(0x005e0c60, &[this]).u32(), this);
            assert_eq!(e.mem.bytes(block, 4), vec![fill; 4]);
            assert_eq!(e.mem.bytes(this, 0xa2), vec![0; 0xa2]);
            assert_eq!(e.mem.bytes(this + 0xa2, 6), vec![fill; 6]);
        }
    }

    #[test]
    fn constructor_preserves_native_call_and_store_order() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa4);
        e.mem.write(this, &[0xcc; 0xa4]);
        let mut step = 0;
        e.register_double(0x00403d30, move |e, a| {
            assert_eq!(a, &[this + [0x24, 0x50, 0x78][step], 0, 0x28]);
            assert_eq!(e.mem.u32(this + 0x20), 0);
            assert_eq!(e.mem.u32(this + 0x4c), 0);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            assert_eq!(e.mem.u32(this + 0x1c), 0);
            assert_eq!(e.mem.u32(this), 0xcccccccc);
            step += 1;
            e.mem.write(a[0], &[step as u8; 0x28]);
            if step == 3 {
                e.mem.set_u32(this + 0x20, 7);
            }
            Ret::default()
        });
        e.call(0x005e0c60, &[this]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x20), 7);
        for (offset, fill) in [(0x24, 1), (0x50, 2), (0x78, 3)] {
            assert_eq!(e.mem.bytes(this + offset, 0x28), vec![fill; 0x28]);
        }
    }

    #[test]
    fn singleton_guard_precedes_constructor_and_preserves_other_bits() {
        let mut e = Engine::new();
        e.map(MANAGER, 12);
        e.mem.set_u32(INITIALIZED, 0xa4);
        e.register_double(0x0096a2d0, |e, a| {
            assert_eq!(a, &[MANAGER]);
            assert_eq!(e.mem.u32(INITIALIZED), 0xa5);
            assert_eq!(e.call(0x005e24d0, &[]).u32(), MANAGER);
            e.mem.set_u32(MANAGER, 0x1234);
            Ret::default()
        });
        e.register_double(0x00ec658f, |e, a| {
            assert_eq!(a, &[0x00fcb1f0]);
            assert_eq!(e.mem.u32(MANAGER), 0x1234);
            Ret::default()
        });
        assert_eq!(e.call(0x005e24d0, &[]).u32(), MANAGER);
        assert_eq!(e.call(0x005e24d0, &[]).u32(), MANAGER);
    }

    #[test]
    fn busy_predicate_tests_full_pointer() {
        // Native 005e2850 returns AL=0,1,1 for these pointers, with no
        // memory writes. Upper EAX is incidental and not part of bool ABI.
        let mut e = Engine::new();
        let this = e.new_object::<ScriptRunner>();
        for pointer in [0, 0x100, u32::MAX] {
            e.set(this, ScriptRunner::m_scriptRunning, Ptr::new(pointer));
            assert_eq!(e.call(0x005e2850, &[this.addr()]).bool(), pointer != 0);
        }
    }

    #[test]
    fn cached_runner_with_null_script_matches_native_manager_result() {
        // Native 005e2730 with a cached idle runner and three null arguments
        // returns AL=1, pops twelve bytes and leaves the objects untouched.
        // The interpreter's null-script early return is the only double.
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let runner = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, runner);
        e.mem.set_u32(manager.addr() + 4, 0xa5a5a5a5);
        let before = e.mem.bytes(manager.addr(), 8);
        e.register_double(0x005e1160, move |_, a| {
            assert_eq!(a, &[runner.addr(), 0, 0, 0]);
            0u32.into_ret()
        });
        assert!(e.call(0x005e2730, &[manager.addr(), 0, 0, 0]).bool());
        assert_eq!(e.mem.bytes(manager.addr(), 8), before);
        assert_eq!(e.mem.bytes(runner.addr(), 0xa4), vec![0; 0xa4]);
    }

    #[test]
    fn idle_cached_runner_is_reloaded_after_busy_callback() {
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let first = e.new_object::<ScriptRunner>();
        let second = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, first);
        e.register_double(0x005e2850, move |e, a| {
            assert_eq!(a, &[first.addr()]);
            e.set(manager, ScriptRunManager::m_pScriptRunner, second);
            0u32.into_ret()
        });
        e.register_double(0x005e1160, move |_, a| {
            assert_eq!(a, &[second.addr(), 0x10, 0x20, 0x30]);
            0u32.into_ret()
        });
        assert!(e
            .call(0x005e2730, &[manager.addr(), 0x10, 0x20, 0x30])
            .bool());
    }

    #[test]
    fn busy_runner_uses_and_frees_temporary_without_replacing_cache() {
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let cached = e.new_object::<ScriptRunner>();
        let temporary = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, cached);
        e.set(cached, ScriptRunner::m_scriptRunning, Ptr::new(0x100));
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, &[0xa4]);
            temporary.into_ret()
        });
        e.register_double(0x005e1160, move |e, a| {
            assert_eq!(a, &[temporary.addr(), 1, 2, 3]);
            assert_eq!(e.mem.u32(temporary.addr() + 0x14), 0);
            assert_eq!(e.get(manager, ScriptRunManager::m_pScriptRunner), cached);
            e.mem.set_u32(temporary.addr(), 0x1234);
            0u32.into_ret()
        });
        e.register_double(0x00401030, move |e, a| {
            assert_eq!(a, &[temporary.addr()]);
            assert_eq!(e.mem.u32(temporary.addr()), 0x1234);
            Ret::default()
        });
        assert!(e.call(0x005e2730, &[manager.addr(), 1, 2, 3]).bool());
    }

    #[test]
    fn empty_cache_allocates_and_keeps_constructor_return() {
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let allocated = e.new_object::<ScriptRunner>();
        let returned = e.new_object::<ScriptRunner>();
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, &[0xa4]);
            allocated.into_ret()
        });
        e.register_double(0x005e0c60, move |_, a| {
            assert_eq!(a, &[allocated.addr()]);
            returned.into_ret()
        });
        e.register_double(0x005e1160, move |e, a| {
            assert_eq!(a, &[returned.addr(), 1, 2, 3]);
            assert_eq!(e.get(manager, ScriptRunManager::m_pScriptRunner), returned);
            Ret::default()
        });
        assert!(e.call(0x005e2730, &[manager.addr(), 1, 2, 3]).bool());
    }
}
