//! `NiXenonRenderer/nid3dshaderprogramcreatorhlsl.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1460` (`cdecl`, two words: block, size): the Ni sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `00e89970` (`thiscall`, no stack words): the destructor body `00be0af0` runs (engine map:
/// `NiD3DPixelShader`'s, unconfirmed).
const PIXEL_SHADER_DESTRUCT: u32 = 0x00e8_9970;
/// `00e89b70` (`thiscall`, no stack words): the destructor body `00be0b70` runs (engine map:
/// `NiD3DVertexShader`'s, unconfirmed).
const VERTEX_SHADER_DESTRUCT: u32 = 0x00e8_9b70;

// Translated from 00be0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x1c`
/// (`NiD3DShaderProgram::m_pkCreator` in the Xbox PDB; the body is a one-instruction getter that
/// other classes can share).
pub fn fn_00be0ad0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1c)
}

// Translated from 00be0af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall` with
/// one stack word (the flags): runs the destructor `00e89970`, and when bit 0 of the flags is
/// set frees the object with size 0x30 (`00aa1460`). Answers `this`.
pub fn ni_d3d_pixel_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PIXEL_SHADER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

// Translated from 00be0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x2c`
/// (`NiD3DPixelShader::m_hShader` in the Xbox PDB; a one-instruction getter other classes can
/// share).
pub fn fn_00be0b10(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2c)
}

// Translated from 00be0b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPixelShader::SetShaderHandle` (Xbox PDB, engine map), `thiscall` with one stack word:
/// stores it at `+0x2c` (`m_hShader`).
pub fn ni_d3d_pixel_shader_set_shader_handle(e: &mut Engine, this: Ptr, handle: u32) {
    e.mem.set_u32(this.addr() + 0x2c, handle);
}

// Translated from 00be0b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall` with
/// one stack word (the flags): runs the destructor `00e89b70`, and when bit 0 of the flags is
/// set frees the object with size 0x3c (`00aa1460`). Answers `this`.
pub fn ni_d3d_vertex_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(VERTEX_SHADER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 00be0b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x30`
/// (`NiD3DVertexShader::m_uiUsage` in the Xbox PDB; a one-instruction getter other classes can
/// share).
pub fn fn_00be0b90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x30)
}

// Translated from 00be0bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::SetShaderHandle` (Xbox PDB, engine map), `thiscall` with one stack
/// word: stores it at `+0x34` (`m_hShader`).
pub fn ni_d3d_vertex_shader_set_shader_handle(e: &mut Engine, this: Ptr, handle: u32) {
    e.mem.set_u32(this.addr() + 0x34, handle);
}

// Translated from 00be0bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): answers the word at `+0x38`
/// (`NiD3DVertexShader::m_hDecl` in the Xbox PDB; a one-instruction getter other classes can
/// share).
pub fn fn_00be0bc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x38)
}

// Translated from 00be0be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DVertexShader::GetSoftwareVertexProcessing` (Xbox PDB, engine map), `thiscall`, no
/// stack words: answers the byte at `+0x2c` (`m_bSoftwareVP`) in `AL`.
pub fn ni_d3d_vertex_shader_get_software_vertex_processing(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x2c)
}

// Translated from 00cc0760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiD3DShaderProgram::GetName` in the engine map (from a call graph, so unconfirmed:
/// the body is a one-instruction getter many classes can share), `thiscall`, no stack words:
/// answers the word at `+0xc` (`m_pszName` in `NiD3DShaderProgram`, Xbox PDB).
pub fn fn_00cc0760(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xc)
}

// Translated from 00e8bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLVertexShader::GetEntryPoint` (Xbox PDB, engine map), `thiscall`, no stack words:
/// answers the word at `+0x3c` (`m_pcEntryPoint`).
pub fn ni_d3d_hlsl_vertex_shader_get_entry_point(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x3c)
}

// Translated from 00e95d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DHLSLPixelShader::GetShaderTarget` (Xbox PDB, engine map), `thiscall`, no stack words:
/// answers the word at `+0x34` (`m_pcShaderTarget`).
pub fn ni_d3d_hlsl_pixel_shader_get_shader_target(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x34)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00be_0ad0, fn_00be0ad0(Ptr) -> u32),
        entry!(
            0x00be_0af0,
            ni_d3d_pixel_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00be_0b10, fn_00be0b10(Ptr) -> u32),
        entry!(0x00be_0b20, ni_d3d_pixel_shader_set_shader_handle(Ptr, u32)),
        entry!(
            0x00be_0b70,
            ni_d3d_vertex_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00be_0b90, fn_00be0b90(Ptr) -> u32),
        entry!(
            0x00be_0bb0,
            ni_d3d_vertex_shader_set_shader_handle(Ptr, u32)
        ),
        entry!(0x00be_0bc0, fn_00be0bc0(Ptr) -> u32),
        entry!(
            0x00be_0be0,
            ni_d3d_vertex_shader_get_software_vertex_processing(Ptr) -> u8
        ),
        entry!(0x00cc_0760, fn_00cc0760(Ptr) -> u32),
        entry!(0x00e8_bf90, ni_d3d_hlsl_vertex_shader_get_entry_point(Ptr) -> u32),
        entry!(0x00e9_5d50, ni_d3d_hlsl_pixel_shader_get_shader_target(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x48);
        for offset in (0..0x48u32).step_by(4) {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        this
    }

    #[test]
    fn getters_read_their_words() {
        let mut e = Engine::new();
        let this = object(&mut e);
        for (address, offset) in [
            (0x00be_0ad0u32, 0x1cu32),
            (0x00be_0b10, 0x2c),
            (0x00be_0b90, 0x30),
            (0x00be_0bc0, 0x38),
            (0x00cc_0760, 0xc),
            (0x00e8_bf90, 0x3c),
            (0x00e9_5d50, 0x34),
        ] {
            assert_eq!(e.call(address, &args![this]).u32(), 0x1000 + offset);
        }
        e.mem.set_u8(this + 0x2c, 1);
        assert_eq!(e.call(0x00be_0be0, &args![this]).u32() & 0xff, 1);
        e.mem.set_u8(this + 0x2c, 0);
        assert_eq!(e.call(0x00be_0be0, &args![this]).u32() & 0xff, 0);
    }

    #[test]
    fn set_shader_handle_stores_at_its_offset() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.call(0x00be_0b20, &args![this, 0xabcdu32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0xabcd);
        assert_eq!(e.mem.u32(this + 0x34), 0x1034);
        e.call(0x00be_0bb0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x34), 0x1234);
    }

    fn deleting(address: u32, destruct: u32, size: u32) {
        let mut e = Engine::new();
        e.register(destruct, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let this = object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
        assert_eq!(e.call(address, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let destructs = log.iter().filter(|(x, _)| *x == destruct).count();
        assert_eq!(destructs, 2);
        let deletes: Vec<_> = log.iter().filter(|(x, _)| *x == NI_SIZED_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![this, size]);
    }

    #[test]
    fn pixel_shader_deleting_destructor_frees_on_bit_0() {
        deleting(0x00be_0af0, PIXEL_SHADER_DESTRUCT, 0x30);
    }

    #[test]
    fn vertex_shader_deleting_destructor_frees_on_bit_0() {
        deleting(0x00be_0b70, VERTEX_SHADER_DESTRUCT, 0x3c);
    }
}
