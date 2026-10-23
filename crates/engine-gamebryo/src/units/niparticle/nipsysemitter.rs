//! `NiParticle/nipsysemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c220a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x54 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_emitter_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x54u32]);
    }
    this
}

// Translated from 00c21fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2540.
pub fn ni_p_sys_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2540
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{stream_read, NiPSysModifier, STREAM_VERSION};

layout! {
    /// `NiPSysEmitter` (Xbox PDB), 0x54 bytes; the PC code uses the same
    /// offsets. Float fields are declared as their bit patterns.
    pub struct NiPSysEmitter: 0x54 {
        /// `m_fSpeed` (Xbox PDB).
        0x18 m_fSpeed: u32,
        /// `m_fSpeedVar` (Xbox PDB).
        0x1C m_fSpeedVar: u32,
        /// `m_fDeclination` (Xbox PDB).
        0x20 m_fDeclination: u32,
        /// `m_fDeclinationVar` (Xbox PDB).
        0x24 m_fDeclinationVar: u32,
        /// `m_fPlanarAngle` (Xbox PDB).
        0x28 m_fPlanarAngle: u32,
        /// `m_fPlanarAngleVar` (Xbox PDB).
        0x2C m_fPlanarAngleVar: u32,
        /// `m_kInitialColor` (Xbox PDB): an `NiColorA` (four floats).
        0x30 m_kInitialColor: u32,
        /// `m_fInitialRadius` (Xbox PDB).
        0x40 m_fInitialRadius: u32,
        /// `m_fRadiusVar` (Xbox PDB).
        0x44 m_fRadiusVar: u32,
        /// `m_fLifeSpan` (Xbox PDB).
        0x48 m_fLifeSpan: u32,
        /// `m_fLifeSpanVar` (Xbox PDB).
        0x4C m_fLifeSpanVar: u32,
        /// `m_fScale` (Xbox PDB).
        0x50 m_fScale: u32,
    }
}

/// `_rand` (`00ecadb8`) and `_ftol2` (`00ec62c0`: truncates the `ST0`
/// value, passed as a leading `f64`).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
/// `double` constants: 32767.0 (`RAND_MAX`), 0.5, 1.0 and 512.0.
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const HALF: u32 = 0x0101_1588;
const ONE: u32 = 0x0101_2070;
const TABLE_SIZE: u32 = 0x0102_8338;
/// The `float` 2π the angle-to-table-index scale divides by (`011aeae8`).
const TWO_PI: u32 = 0x011a_eae8;
/// The two 512-entry `float` tables the emitter reads by angle index:
/// the one at `011f52e0` gives the factors the decompiler shows multiplying
/// into x and y, the one at `011f4ae0` those for z and x (the fast sine and
/// cosine tables, by their use; not confirmed by name).
const TABLE_A: u32 = 0x011f_52e0;
const TABLE_B: u32 = 0x011f_4ae0;
/// `NiPSysData::INVALID_PARTICLE` (the `u16` at `010be248`).
const INVALID_PARTICLE: u32 = 0x010b_e248;
/// `NiParticleSystem::InitializeNewParticle` (`00c1aee0`, `thiscall`, the
/// particle index).
const INITIALIZE_NEW_PARTICLE: u32 = 0x00c1_aee0;
/// `NiPSysModifier::LoadBinary` (`00c318b0`) and `NiColorA::LoadBinary`
/// (`00a69800`, `thiscall` on the colour, the stream).
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;
/// From file version 10.3.0.2 the radius variation is stored.
const VERSION_RADIUS_VAR: u32 = 0x0a03_0002;
/// `NiParticleSystem`'s model data (+0xb8, the `NiPSysData`) as the PC code
/// reads it.
const SYSTEM_DATA: u32 = 0xb8;

/// `rand() / 32767.0` rounded to a `float` (`FILD`, `FDIV qword`, `FSTP
/// dword`).
fn random_unit(e: &mut Engine) -> u32 {
    let r = e.call(RAND, &args![]).i32();
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    x87_store32(x87_div(f64::from(r), rand_max))
}

/// `(2 * rand()) / 32767.0 - 1.0` rounded to a `float` (`FILD`, `FADD
/// ST0,ST0`, `FDIV qword`, `FSUB qword`, `FSTP dword`).
fn random_signed(e: &mut Engine) -> u32 {
    let r = f64::from(e.call(RAND, &args![]).i32());
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    let one: f64 = e.global(ONE);
    x87_store32(x87_sub(x87_div(x87_add(r, r), rand_max), one))
}

// Translated from 00c220c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::EmitParticles` (Xbox PDB): `thiscall`, `RET 0xc`: the
/// current time (`float`), the number of particles (`u16` in a word) and
/// the address of their ages (`float`s). For each particle: the life span
/// is `(rand()/32767 - 0.5) * m_fLifeSpanVar + m_fLifeSpan`; when the life
/// span is not below the age (a NaN goes on), a particle is added to the
/// target's data (vtable slot +0xa8 of the data; at `INVALID_PARTICLE` the
/// function returns) and set up: speed, declination and planar angle from
/// their variations, the direction from the two tables at the angles'
/// indices (`ftol(angle * 512/2π) & 0x1ff`), velocity = direction * speed,
/// age, life span, generation 0; the emitter's position-and-velocity
/// (vtable slot +0xa8, with the vertex and the info); the initial colour,
/// radius `(m_fRadiusVar * r + m_fInitialRadius) * m_fScale`, size 1 and a
/// random texture index (`rand() % m_ucSubTextureOffsetCount`) when the
/// data has those arrays; the last update time is `time - age`; then
/// `NiParticleSystem::InitializeNewParticle`. The x87 arithmetic is done
/// instruction by instruction in the exe's order.
pub fn ni_p_sys_emitter_emit_particles(
    e: &mut Engine,
    this: Ptr<NiPSysEmitter>,
    time: f32,
    count: u16,
    ages: Ptr,
) {
    let target = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
    let data = Ptr::<NiPSysData>::new(e.mem.u32(target.wrapping_add(SYSTEM_DATA)));
    let vertices = e.get(data, NiPSysData::m_pkVertex);
    let colors = e.get(data, NiPSysData::m_pkColor);
    let radii = e.get(data, NiPSysData::m_pfRadii);
    let sizes = e.get(data, NiPSysData::m_pfSizes);
    let texture_indices = e.get(data, NiPSysData::m_pucTextureIndices);
    let sub_texture_count = e.get(data, NiPSysData::m_ucSubTextureOffsetCount);
    // A field as a `float` memory operand, and as loaded by `FLD dword`.
    let field = |e: &Engine, f| x87_m32(e.get(this, f));
    let loaded = |e: &Engine, f| x87_load32(e.get(this, f));

    let mut i: u16 = 0;
    while i < count {
        let age_bits = x87_quiet_bits(e.mem.u32(ages.addr() + 4 * u32::from(i)));
        let unit = random_unit(e);
        let half: f64 = e.global(HALF);
        let life_span = x87_store32(x87_add(
            x87_mul(
                x87_sub(x87_load32(unit), half),
                field(e, NiPSysEmitter::m_fLifeSpanVar),
            ),
            field(e, NiPSysEmitter::m_fLifeSpan),
        ));
        // `FLD age; FLD lifespan; FCOMPP`: skipped when the life span is
        // below the age.
        if x87_load32(life_span) < x87_load32(age_bits) {
            i = i.wrapping_add(1);
            continue;
        }
        let index = e.vcall(data.addr(), 0xa8, &args![]).u16();
        if index == e.global::<u16>(INVALID_PARTICLE) {
            return;
        }
        let k = u32::from(index);
        let info = e.get(data, NiPSysData::m_pkParticleInfo) + k * 0x1c;

        let unit = random_unit(e);
        let half: f64 = e.global(HALF);
        let speed = x87_store32(x87_add(
            x87_mul(
                x87_sub(x87_load32(unit), half),
                field(e, NiPSysEmitter::m_fSpeedVar),
            ),
            field(e, NiPSysEmitter::m_fSpeed),
        ));
        let r = random_signed(e);
        let declination = x87_store32(x87_add(
            x87_mul(loaded(e, NiPSysEmitter::m_fDeclinationVar), x87_m32(r)),
            field(e, NiPSysEmitter::m_fDeclination),
        ));
        let r = random_signed(e);
        let planar = x87_store32(x87_add(
            x87_mul(loaded(e, NiPSysEmitter::m_fPlanarAngleVar), x87_m32(r)),
            field(e, NiPSysEmitter::m_fPlanarAngle),
        ));
        let table_size: f64 = e.global(TABLE_SIZE);
        let scale = x87_div(table_size, x87_load32(e.global::<u32>(TWO_PI)));
        let declination_index = e
            .call(FTOL, &args![x87_mul(x87_load32(declination), scale)])
            .u32()
            & 0x1ff;
        let a_declination = x87_load32(e.global::<u32>(TABLE_A + 4 * declination_index));
        let a_declination_bits = x87_store32(a_declination);
        let planar_index = e.call(FTOL, &args![x87_mul(scale, x87_m32(planar))]).u32() & 0x1ff;
        let b_planar = x87_load32(e.global::<u32>(TABLE_B + 4 * planar_index));
        let a_declination = x87_load32(a_declination_bits);
        let x_direction = x87_store32(x87_mul(b_planar, a_declination));
        let y_direction = x87_store32(x87_mul(
            a_declination,
            x87_m32(e.global::<u32>(TABLE_A + 4 * planar_index)),
        ));
        let z_direction = x87_store32(x87_load32(e.global::<u32>(TABLE_B + 4 * declination_index)));
        let speed = x87_load32(speed);
        let vx = x87_store32(x87_mul(x87_load32(x_direction), speed));
        e.mem.set_u32(info, vx);
        let vy = x87_store32(x87_mul(x87_load32(y_direction), speed));
        e.mem.set_u32(info + 4, vy);
        let vz = x87_store32(x87_mul(speed, x87_m32(z_direction)));
        e.mem.set_u32(info + 8, vz);
        e.mem.set_u32(info + 0xc, age_bits);
        e.mem.set_u16(info + 0x18, 0);
        e.mem.set_u32(info + 0x10, x87_quiet_bits(life_span));

        e.vcall(this.addr(), 0xa8, &args![vertices + k * 0xc, info]);
        if colors != 0 {
            let color = colors + (k << 4);
            for word in 0..4 {
                let value = e
                    .mem
                    .u32(this.addr() + NiPSysEmitter::m_kInitialColor.off + 4 * word);
                e.mem.set_u32(color + 4 * word, value);
            }
        }
        if radii != 0 {
            let r = random_signed(e);
            let radius = x87_store32(x87_mul(
                x87_add(
                    x87_mul(loaded(e, NiPSysEmitter::m_fRadiusVar), x87_m32(r)),
                    field(e, NiPSysEmitter::m_fInitialRadius),
                ),
                field(e, NiPSysEmitter::m_fScale),
            ));
            e.mem.set_u32(radii + 4 * k, radius);
        }
        if sizes != 0 {
            e.mem.set_u32(sizes + 4 * k, 1.0f32.to_bits());
        }
        if texture_indices != 0 {
            let r = e.call(RAND, &args![]).i32();
            let index = r % i32::from(sub_texture_count);
            e.mem.set_u8(k + texture_indices, index as u8);
        }
        let last_update = x87_store32(x87_sub(
            x87_load32(time.to_bits()),
            x87_m32(e.mem.u32(info + 0xc)),
        ));
        e.mem.set_u32(info + 0x14, last_update);
        e.call(INITIALIZE_NEW_PARTICLE, &args![target, k]);
        i = i.wrapping_add(1);
    }
}

// Translated from 00c223b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::LoadBinary` (Xbox PDB): `NiPSysModifier::LoadBinary`,
/// then 4 bytes each of speed, speed variation, declination, declination
/// variation, planar angle and its variation, the initial colour
/// (`NiColorA::LoadBinary`), the initial radius, the radius variation
/// (from file version 10.3.0.2), the life span and its variation. The scale
/// is not read. The component-size word is the stream argument's stack
/// word.
pub fn ni_p_sys_emitter_load_binary(e: &mut Engine, this: Ptr<NiPSysEmitter>, stream: Ptr) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let read = |e: &mut Engine, off: u32| stream_read(e, stream, this.addr() + off, 4, 4, slot);
        read(e, NiPSysEmitter::m_fSpeed.off);
        read(e, NiPSysEmitter::m_fSpeedVar.off);
        read(e, NiPSysEmitter::m_fDeclination.off);
        read(e, NiPSysEmitter::m_fDeclinationVar.off);
        read(e, NiPSysEmitter::m_fPlanarAngle.off);
        read(e, NiPSysEmitter::m_fPlanarAngleVar.off);
        e.call(
            COLOR_LOAD_BINARY,
            &args![this.addr() + NiPSysEmitter::m_kInitialColor.off, stream],
        );
        read(e, NiPSysEmitter::m_fInitialRadius.off);
        if e.mem.u32(stream + STREAM_VERSION) >= VERSION_RADIUS_VAR {
            read(e, NiPSysEmitter::m_fRadiusVar.off);
        }
        read(e, NiPSysEmitter::m_fLifeSpan.off);
        read(e, NiPSysEmitter::m_fLifeSpanVar.off);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_20c0,
            ni_p_sys_emitter_emit_particles(Ptr<NiPSysEmitter>, f32, u16, Ptr)
        ),
        entry!(
            0x00c2_23b0,
            ni_p_sys_emitter_load_binary(Ptr<NiPSysEmitter>, Ptr)
        ),
        entry!(0x00c21fd0, ni_p_sys_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c220a0, ni_p_sys_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00c220a0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c220a0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x54]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_1fd0, &args![]).u32(), 0x0120_2540);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    fn log_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const ADD_PARTICLE_FN: u32 = 0x0f40_0000;
    const POSITION_FN: u32 = 0x0f40_0010;

    /// An emitter over a particle system whose data has every optional
    /// array, the constants the code reads, and tables whose entry 0 is
    /// 0.5 (`011f52e0`) and 0.25 (`011f4ae0`). `rand` always answers 7.
    fn emitter(e: &mut Engine) -> (Ptr<NiPSysEmitter>, Ptr<NiPSysData>) {
        e.map(0x0101_1000, 0x2_0000);
        e.set_global(RAND_MAX_DOUBLE, 32767.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(TABLE_SIZE, 512.0f64);
        e.map(TWO_PI & !0xfff, 0x1000);
        e.set_global(TWO_PI, std::f32::consts::TAU);
        e.map(INVALID_PARTICLE & !0xfff, 0x1000);
        e.set_global(INVALID_PARTICLE, 0xffffu16);
        e.map(TABLE_B, 0x1000);
        e.set_global(TABLE_A, 0.5f32);
        e.set_global(TABLE_B, 0.25f32);
        e.register(RAND, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register(INITIALIZE_NEW_PARTICLE, |_, _| Ret::default());
        e.register(POSITION_FN, |_, _| Ret::default());
        // The data hands out particles 0, 1, ... from a counter at +0x7c.
        e.register(ADD_PARTICLE_FN, |e, a| {
            let next = e.mem.u32(a[0] + 0x7c);
            e.mem.set_u32(a[0] + 0x7c, next + 1);
            Ret {
                eax: next,
                ..Ret::default()
            }
        });
        let data = Ptr::<NiPSysData>::new(e.mem.alloc(0x80));
        let data_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data_vtable + 0xa8, ADD_PARTICLE_FN);
        e.mem.set_u32(data.addr(), data_vtable);
        let arrays = [
            (NiPSysData::m_pkVertex, 0xc * 4),
            (NiPSysData::m_pkColor, 0x10 * 4),
            (NiPSysData::m_pfRadii, 4 * 4),
            (NiPSysData::m_pfSizes, 4 * 4),
            (NiPSysData::m_pucTextureIndices, 4),
            (NiPSysData::m_pkParticleInfo, 0x1c * 4),
        ];
        for (field, size) in arrays {
            let block = e.mem.alloc(size);
            e.set(data, field, block);
        }
        e.set(data, NiPSysData::m_ucSubTextureOffsetCount, 4);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(system + SYSTEM_DATA, data.addr());
        let this = e.new_object::<NiPSysEmitter>();
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xa8, POSITION_FN);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(this.addr() + 0x10, system);
        e.set(this, NiPSysEmitter::m_fLifeSpan, 10.0f32.to_bits());
        e.set(this, NiPSysEmitter::m_fSpeed, 2.0f32.to_bits());
        e.set(this, NiPSysEmitter::m_fInitialRadius, 3.0f32.to_bits());
        e.set(this, NiPSysEmitter::m_fScale, 2.0f32.to_bits());
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.mem
                .set_u32(this.addr() + 0x30 + 4 * i as u32, value.to_bits());
        }
        (this, data)
    }

    #[test]
    fn emit_particles_sets_up_each_living_particle() {
        let mut e = Engine::new();
        let (this, data) = emitter(&mut e);
        let ages = e.mem.alloc(8);
        e.mem.set_f32(ages, 0.0);
        e.mem.set_f32(ages + 4, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_20c0, &args![this, 5.0f32, 2u32, ages]);
        // The second particle's age is beyond its life span: one particle.
        assert_eq!(log_of(&e, ADD_PARTICLE_FN).len(), 1);
        let info = e.get(data, NiPSysData::m_pkParticleInfo);
        assert_eq!(e.mem.f32(info), 0.25);
        assert_eq!(e.mem.f32(info + 4), 0.5);
        assert_eq!(e.mem.f32(info + 8), 0.5);
        assert_eq!(e.mem.f32(info + 0xc), 0.0);
        assert_eq!(e.mem.f32(info + 0x10), 10.0);
        assert_eq!(e.mem.f32(info + 0x14), 5.0);
        assert_eq!(e.mem.u16(info + 0x18), 0);
        let vertices = e.get(data, NiPSysData::m_pkVertex);
        assert_eq!(
            log_of(&e, POSITION_FN),
            vec![vec![this.addr(), vertices, info]]
        );
        let colors = e.get(data, NiPSysData::m_pkColor);
        assert_eq!(e.mem.f32(colors + 0xc), 0.4);
        assert_eq!(e.mem.f32(e.get(data, NiPSysData::m_pfRadii)), 6.0);
        assert_eq!(e.mem.f32(e.get(data, NiPSysData::m_pfSizes)), 1.0);
        assert_eq!(e.mem.u8(e.get(data, NiPSysData::m_pucTextureIndices)), 3);
        let system = e.mem.u32(this.addr() + 0x10);
        assert_eq!(log_of(&e, INITIALIZE_NEW_PARTICLE), vec![vec![system, 0]]);
    }

    #[test]
    fn emit_particles_stops_when_no_particle_is_free() {
        let mut e = Engine::new();
        let (this, data) = emitter(&mut e);
        e.mem.set_u32(data.addr() + 0x7c, 0xffff);
        let ages = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00c2_20c0, &args![this, 5.0f32, 2u32, ages]);
        assert_eq!(log_of(&e, ADD_PARTICLE_FN).len(), 1);
        assert!(log_of(&e, POSITION_FN).is_empty());
        assert!(log_of(&e, INITIALIZE_NEW_PARTICLE).is_empty());
    }

    #[test]
    fn load_binary_reads_the_emitter_fields() {
        for (version, radius_var) in [(0x0a03_0002u32, 9.0f32), (0x0a03_0001, 0.0)] {
            let mut e = Engine::new();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
            let values: Vec<f32> = if radius_var == 0.0 {
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 10.0, 11.0]
            } else {
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 11.0]
            };
            let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
            let stream = test_stream::new(&mut e, version, &bytes);
            let this = e.new_object::<NiPSysEmitter>();
            e.call_log = Some(vec![]);
            e.call(0x00c2_23b0, &args![this, stream]);
            assert_eq!(
                log_of(&e, COLOR_LOAD_BINARY),
                vec![vec![this.addr() + 0x30, stream]]
            );
            let f = |e: &Engine, field| f32::from_bits(e.get(this, field));
            assert_eq!(f(&e, NiPSysEmitter::m_fSpeed), 1.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fPlanarAngleVar), 6.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fInitialRadius), 7.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fRadiusVar), radius_var);
            assert_eq!(f(&e, NiPSysEmitter::m_fLifeSpan), 10.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fLifeSpanVar), 11.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fScale), 0.0);
        }
    }
}
