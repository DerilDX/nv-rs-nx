//! `NiParticle/nipsyssphereemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c20020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_sphere_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

// Translated from 00c1fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2510.
pub fn ni_p_sys_sphere_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2510
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00c31af0` (unnamed, `thiscall`, no words): the `NiPSysVolumeEmitter` part
/// of the constructor.
const VOLUME_EMITTER_CONSTRUCT: u32 = 0x00c3_1af0;
/// `00c31b10` (`thiscall`, two words: the clone, the cloning process): the
/// emitter part of the member copy.
const VOLUME_EMITTER_COPY_MEMBERS: u32 = 0x00c3_1b10;
/// `NiPSysVolumeEmitter::LoadBinary` (`00c31b20`, `thiscall`, one word).
const VOLUME_EMITTER_LOAD_BINARY: u32 = 0x00c3_1b20;
/// The `NiPSysSphereEmitter` vtable.
const SPHERE_EMITTER_VTABLE: u32 = 0x010b_d974;
/// `m_fEmitterRadius` (Xbox PDB), the float at `+0x58`, as bits.
const RADIUS: u32 = 0x58;

/// The inlined constructor: `00c31af0`, the radius 0 (`FLDZ`), the vtable.
fn construct_sphere_emitter(e: &mut Engine, block: u32) -> Ptr {
    e.call(VOLUME_EMITTER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block + RADIUS, 0);
    e.mem.set_u32(block, SPHERE_EMITTER_VTABLE);
    Ptr::new(block)
}

// Translated from 00c1ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::CreateClone` (Xbox PDB): 0x5C bytes constructed as a
/// sphere emitter (null when the allocation fails, then used anyway as the
/// exe does), the emitter members copied (`00c31b10`), the radius copied
/// (`FLD`/`FSTP`, quieting a signalling NaN). Answers the clone.
pub fn ni_p_sys_sphere_emitter_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x5cu32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct_sphere_emitter(e, block)
    };
    e.call(VOLUME_EMITTER_COPY_MEMBERS, &args![this, clone, process]);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    e.mem.set_u32(clone.addr() + RADIUS, radius);
    clone
}

// Translated from 00c1fff0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysSphereEmitter` in `00c1aa00`'s loader table; `cdecl`, no words):
/// 0x5C bytes constructed as a sphere emitter, or null.
pub fn fn_00c1fff0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x5cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_sphere_emitter(e, block)
}

// Translated from 00c20040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::LoadBinary` (Xbox PDB): the volume-emitter part,
/// then the radius, four bytes read from the stream's input file (`+0x24c`,
/// its read function at `+8`, `cdecl (file, dest, size, &count, 1)`, the
/// result added to its position `+4`); the count word is the argument slot.
pub fn ni_p_sys_sphere_emitter_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(VOLUME_EMITTER_LOAD_BINARY, &args![this, stream]);
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let got = e
            .call(read, &args![file, this.addr() + RADIUS, 4u32, count, 1u32])
            .u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(got));
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1_ffa0, ni_p_sys_sphere_emitter_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c1_fff0, fn_00c1fff0() -> Ptr),
        entry!(0x00c2_0040, ni_p_sys_sphere_emitter_load_binary(Ptr, Ptr)),
        entry!(0x00c1fe10, ni_p_sys_sphere_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c20020, ni_p_sys_sphere_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_sphere_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c20020, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c20020, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_sphere_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1_fe10, &args![]).u32(), 0x0120_2510);
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn create_clone_constructs_and_copies_the_radius() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| Ret::default());
        e.register(VOLUME_EMITTER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + RADIUS, 0x7f80_0002);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c1_ffa0, &args![this, 0x7000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x5c]]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.u32(clone), SPHERE_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(clone + RADIUS), 0x7fc0_0002);
    }

    #[test]
    fn create_function_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c1_fff0, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| Ret::default());
        let made = e.call(0x00c1_fff0, &args![]).u32();
        assert_eq!(e.mem.u32(made), SPHERE_EMITTER_VTABLE);
    }

    #[test]
    fn load_binary_reads_the_radius() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_f32(a[1], 2.5);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 0x20);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0040, &args![this, stream]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.f32(this + RADIUS), 2.5);
        assert_eq!(e.mem.u32(file + 4), 0x24);
    }
}
