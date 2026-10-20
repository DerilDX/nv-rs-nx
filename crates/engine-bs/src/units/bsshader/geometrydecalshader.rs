//! `BSShader/geometrydecalshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The two `NiPointer`s that keep this shader's declarations (`012021e0`, `012021e4`).
const DECLARATION_POINTER_012021E0: u32 = 0x0120_21e0;
const DECLARATION_POINTER_012021E4: u32 = 0x0120_21e4;
/// `GeometryDecalShader::GeometryDecalShader` (`00bcbf00`) and
/// `GeometryDecalShader::LoadShaders` (`00bcb4d0`) (engine map; `thiscall`, no words).
const CONSTRUCT: u32 = 0x00bc_bf00;
const LOAD_SHADERS: u32 = 0x00bc_b4d0;
/// `00e804d0` (`cdecl`, one word: the address of a word of the shader). The engine map names
/// it `GeometryDecalShader::LoadStagesAndPasses`; the call here does not fit that name, so it
/// is described by address.
const CALL_00E804D0: u32 = 0x00e8_04d0;

/// The `NiPointer` assignment of a global the code inlines: nothing when it already holds
/// `object`; else the old object is released (count at +4 through `InterlockedDecrement`,
/// deleted through vtable slot +4 at 0) when not null, the global set and the new object
/// held (`InterlockedIncrement` on its +4) when not null.
fn assign_pointer(e: &mut Engine, global: u32, object: u32) {
    let old = e.global::<u32>(global);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.set_global(global, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00bcc0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x10. No arguments. Makes two declarations
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`: 2 entries, then 4, 1
/// stream each), each assigned at once to its `NiPointer` (`012021e0`, `012021e4`). Through
/// those pointers (read again for each call): the first's entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0)) and slot +0xb4, the second's
/// ((0, 0, 0, 2, 0), (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2, 0)) and slot +0xb4. Then
/// allocates 0x90 bytes and runs the constructor (`00bcbf00`; a null allocation is used as it
/// is), calls the shader's slot +0x4c, `LoadShaders` (`00bcb4d0`), `00e804d0` on the
/// shader's +0x6c and +0x70, slots +0x13c and +0x11c, then +0xc0 with the first declaration
/// (read from `012021e0`), and answers the shader.
// Translation incomplete: 00bcc0a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn geometry_decal_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let first = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    assign_pointer(e, DECLARATION_POINTER_012021E0, first);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let second = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    assign_pointer(e, DECLARATION_POINTER_012021E4, second);
    for entry in [[0u32, 0, 0, 2, 0], [1, 3, 3, 2, 0]] {
        let declaration = e.global::<u32>(DECLARATION_POINTER_012021E0);
        e.vcall(declaration, 0x8c, &entry);
    }
    let declaration = e.global::<u32>(DECLARATION_POINTER_012021E0);
    e.vcall(declaration, 0xb4, &args![]);
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 1, 1, 3, 0],
        [2, 2, 2, 4, 0],
        [3, 3, 3, 2, 0],
    ] {
        let declaration = e.global::<u32>(DECLARATION_POINTER_012021E4);
        e.vcall(declaration, 0x8c, &entry);
    }
    let declaration = e.global::<u32>(DECLARATION_POINTER_012021E4);
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.call(LOAD_SHADERS, &args![shader]);
    for word in [0x6cu32, 0x70] {
        e.call(CALL_00E804D0, &args![shader.wrapping_add(word)]);
    }
    e.vcall(shader, 0x13c, &args![]);
    e.vcall(shader, 0x11c, &args![]);
    let declaration = e.global::<u32>(DECLARATION_POINTER_012021E0);
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// Code the compiler expands in place in the shader units' constructors and loaders
/// (`NiPointer` assignment and release, the reference counts of passes and texture stages,
/// the `eh vector constructor iterator` call for `NiPointer` arrays). The shader units of this
/// crate call these helpers where their functions contain that code.
pub(crate) mod shader_inline {
    use crate::prelude::*;

    /// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
    pub(crate) const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
    pub(crate) const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
    /// `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element size, count,
    /// constructor, destructor).
    pub(crate) const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
    /// The element constructor every `NiPointer` array of the shaders passes (`00b54620`).
    pub(crate) const NI_POINTER_CONSTRUCT_00B54620: u32 = 0x00b5_4620;
    /// The two element destructors the arrays pass (`00b79930`, `00b66f20`).
    pub(crate) const DESTRUCT_00B79930: u32 = 0x00b7_9930;
    pub(crate) const DESTRUCT_00B66F20: u32 = 0x00b6_6f20;
    /// `BSShader::BSShader` (engine map, `00be2200`, `thiscall`, no stack words).
    pub(crate) const BS_SHADER_CONSTRUCT: u32 = 0x00be_2200;
    /// The pass delete `00e80190` (unit `nid3dpass.cpp`, unnamed; `thiscall`, no stack words)
    /// and the texture stage delete `00e7ecd0` (unnamed; `thiscall`, no stack words).
    pub(crate) const PASS_DELETE_00E80190: u32 = 0x00e8_0190;
    pub(crate) const STAGE_DELETE_00E7ECD0: u32 = 0x00e7_ecd0;
    /// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`,
    /// size).
    pub(crate) const MEMORY_MANAGER: u32 = 0x011f_6238;
    pub(crate) const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

    /// A reference-counted object released: `InterlockedDecrement` on its count at +4, and
    /// deleted through its vtable slot +4 when that answers 0.
    pub(crate) fn release_reference(e: &mut Engine, object: u32) {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(object, 4, &args![]);
        }
    }

    /// `NiPointer<T>::operator=(T *)` expanded: nothing when the slot already holds `object`;
    /// else the old object is released when not null, the slot set and the new object's
    /// count at +4 incremented when it is not null.
    pub(crate) fn ni_pointer_assign(e: &mut Engine, slot: u32, object: u32) {
        let old = e.mem.u32(slot);
        if old == object {
            return;
        }
        if old != 0 {
            release_reference(e, old);
        }
        e.mem.set_u32(slot, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }

    /// `NiPointer<T>::operator=(0)` expanded as the shaders have it: when the slot holds an
    /// object, the object is released and the slot cleared.
    pub(crate) fn ni_pointer_clear(e: &mut Engine, slot: u32) {
        let old = e.mem.u32(slot);
        if old != 0 {
            release_reference(e, old);
            e.mem.set_u32(slot, 0);
        }
    }

    /// A pass pointer cleared: when the slot holds a pass, its count at +0x64 goes down by one
    /// (the pass deleted through `00e80190` at 0) and the slot is cleared.
    pub(crate) fn pass_pointer_clear(e: &mut Engine, slot: u32) {
        let pass = e.mem.u32(slot);
        if pass != 0 {
            let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
            e.mem.set_u32(pass + 0x64, left);
            if left == 0 {
                e.call(PASS_DELETE_00E80190, &args![pass]);
            }
            e.mem.set_u32(slot, 0);
        }
    }

    /// A texture stage reference dropped: when the stage is not null its count at +0x60 goes
    /// down by one and the stage is deleted through `00e7ecd0` at 0.
    pub(crate) fn stage_release(e: &mut Engine, stage: u32) {
        if stage != 0 {
            let left = e.mem.u32(stage + 0x60).wrapping_sub(1);
            e.mem.set_u32(stage + 0x60, left);
            if left == 0 {
                e.call(STAGE_DELETE_00E7ECD0, &args![stage]);
            }
        }
    }

    /// An array of `count` `NiPointer`s at `array` constructed by the `eh vector constructor
    /// iterator` (element size 4, constructor `00b54620`, the given destructor).
    pub(crate) fn construct_pointer_array(e: &mut Engine, array: u32, count: u32, destructor: u32) {
        e.call(
            VECTOR_CONSTRUCT,
            &args![
                array,
                4u32,
                count,
                NI_POINTER_CONSTRUCT_00B54620,
                destructor
            ],
        );
    }

    /// CRT `_memset` (`00ec61c0`, `cdecl`).
    pub(crate) const MEMSET: u32 = 0x00ec_61c0;
    /// `BSShader::MakePath` (engine map, `00be0c00`, `cdecl`: the HLSL file name, the 0x104-byte
    /// path buffer).
    pub(crate) const MAKE_PATH: u32 = 0x00be_0c00;
    /// The formatter `00406d00` (`cdecl`: buffer, size, format, then the format's words); the
    /// loaders pass the compiled file name as the format, without further words.
    pub(crate) const FORMAT_00406D00: u32 = 0x0040_6d00;
    /// `BSShader::CompileVertexShaderHLSL` (`00be0fe0`) and `::CompilePixelShaderHLSL`
    /// (`00be1750`) (engine map; `thiscall` on the shader: path, macros, profile, compiled file
    /// name; they answer the shader object).
    pub(crate) const COMPILE_VERTEX_SHADER: u32 = 0x00be_0fe0;
    pub(crate) const COMPILE_PIXEL_SHADER: u32 = 0x00be_1750;

    /// One source block of a `LoadShaders` stack frame: at `offset` the explicit words (the
    /// HLSL file name, then the macro name/value pairs, then the null name that ends them), then
    /// `cleared` bytes cleared through `memset` (none when 0).
    pub(crate) struct ShaderSource {
        pub(crate) offset: u32,
        pub(crate) words: &'static [u32],
        pub(crate) cleared: u32,
    }

    /// One compile step: the source block (its frame offset), the compiled file name the
    /// formatter copies, the profile string, vertex or pixel, and the `NiPointer` of the
    /// shader (offset from `this`) that receives the result.
    pub(crate) struct ShaderCompile {
        pub(crate) source: u32,
        pub(crate) output: u32,
        pub(crate) profile: u32,
        pub(crate) pixel: bool,
        pub(crate) slot: u32,
    }

    /// The frame offsets of a loader's compiled-name buffer and path buffer (0x104 bytes each).
    pub(crate) struct ShaderBuffers {
        pub(crate) name: u32,
        pub(crate) path: u32,
    }

    /// The source blocks written into the frame, in the exe's order: the explicit words, then
    /// `memset` of the rest of the block.
    pub(crate) fn set_up_sources(e: &mut Engine, frame: u32, sources: &[ShaderSource]) {
        for source in sources {
            let block = frame + source.offset;
            for (index, word) in source.words.iter().enumerate() {
                e.mem.set_u32(block + 4 * index as u32, *word);
            }
            if source.cleared != 0 {
                let rest = block + 4 * source.words.len() as u32;
                e.call(MEMSET, &args![rest, 0u32, source.cleared]);
            }
        }
    }

    /// One compile step: `MakePath` of the block's file name into the path buffer, the
    /// compiled name formatted into the name buffer, the compile (`this`, path buffer, the
    /// block's macros at +4, profile, name buffer); answers the compiled shader.
    pub(crate) fn compile_shader(
        e: &mut Engine,
        this: u32,
        frame: u32,
        buffers: &ShaderBuffers,
        step: &ShaderCompile,
    ) -> u32 {
        let block = frame + step.source;
        let file = e.mem.u32(block);
        let path = frame + buffers.path;
        let name = frame + buffers.name;
        e.call(MAKE_PATH, &args![file, path]);
        e.call(FORMAT_00406D00, &args![name, 0x104u32, step.output]);
        let compile = if step.pixel {
            COMPILE_PIXEL_SHADER
        } else {
            COMPILE_VERTEX_SHADER
        };
        e.call(compile, &args![this, path, block + 4, step.profile, name])
            .u32()
    }

    /// Compile steps run in order, each result assigned to its `NiPointer` of the shader.
    pub(crate) fn compile_shaders(
        e: &mut Engine,
        this: u32,
        frame: u32,
        buffers: &ShaderBuffers,
        steps: &[ShaderCompile],
    ) {
        for step in steps {
            let shader = compile_shader(e, this, frame, buffers, step);
            ni_pointer_assign(e, this + step.slot, shader);
        }
    }

    /// `00e804d0` (`cdecl`, one word: the address of the shader's pass pointer): gives the
    /// pointer a new pass from the pass pool.
    pub(crate) const NEW_PASS_00E804D0: u32 = 0x00e8_04d0;
    /// `NiD3DTextureStage::CreateNewStage` (`00e7edf0`, `cdecl`: the address of a stage
    /// pointer), `BSShader::SetupDefaultStage` (`00be0cf0`, `cdecl`: stage, index, three
    /// words), `NiD3DTextureStage::SetFilterMode` (`00e7def0`, `thiscall`, one word) and
    /// `NiD3DPass::SetStage` (`00e802b0`, `thiscall` on the pass: index, stage) (engine map).
    pub(crate) const CREATE_NEW_STAGE: u32 = 0x00e7_edf0;
    pub(crate) const SETUP_DEFAULT_STAGE: u32 = 0x00be_0cf0;
    pub(crate) const SET_FILTER_MODE: u32 = 0x00e7_def0;
    pub(crate) const PASS_SET_STAGE: u32 = 0x00e8_02b0;
    /// The render-state block maker `00e7f5d0` (unnamed, no arguments) and the render-state
    /// setter `00e7f430` (unnamed, `thiscall` on the block: state, value, a third word).
    pub(crate) const NEW_RENDER_STATES_00E7F5D0: u32 = 0x00e7_f5d0;
    pub(crate) const SET_RENDER_STATE_00E7F430: u32 = 0x00e7_f430;

    /// `count` texture stages added to the pass the slot holds, as the loaders' loops do: for
    /// each index from 0 a stage pointer (on the stack, cleared first) filled by
    /// `CreateNewStage`, `SetupDefaultStage(stage, index, setup)`, `SetFilterMode(filter)`,
    /// `NiD3DPass::SetStage` on the pass (read again through the slot) with the pass's word at
    /// +0x14 and the stage, then the stage reference dropped.
    pub(crate) fn add_default_stages(
        e: &mut Engine,
        pass_slot: u32,
        count: u32,
        setup: [u32; 3],
        filter: u32,
    ) {
        e.with_stack(4, |e, local| {
            let local = local.addr();
            for index in 0..count {
                e.mem.set_u32(local, 0);
                e.call(CREATE_NEW_STAGE, &args![local]);
                let stage = e.mem.u32(local);
                e.call(
                    SETUP_DEFAULT_STAGE,
                    &args![stage, index, setup[0], setup[1], setup[2]],
                );
                let stage = e.mem.u32(local);
                e.call(SET_FILTER_MODE, &args![stage, filter]);
                let pass = e.mem.u32(pass_slot);
                let stage = e.mem.u32(local);
                let position = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE, &args![pass, position, stage]);
                let stage = e.mem.u32(local);
                stage_release(e, stage);
            }
        });
    }

    /// A render state set on a pass: its block at +0x30 made first (`00e7f5d0`) when it is
    /// null, then `00e7f430` on the block with the state, the value and 1.
    pub(crate) fn pass_render_state(e: &mut Engine, pass: u32, state: u32, value: u32) {
        if e.mem.u32(pass + 0x30) == 0 {
            let block = e.call(NEW_RENDER_STATES_00E7F5D0, &args![]).u32();
            e.mem.set_u32(pass + 0x30, block);
        }
        let block = e.mem.u32(pass + 0x30);
        e.call(SET_RENDER_STATE_00E7F430, &args![block, state, value, 1u32]);
    }

    /// `operator new[]` of `count` elements of `size` bytes from the memory manager: the
    /// byte count is all ones when the product overflows (`MUL`, `SETO`, `NEG`, `OR`).
    pub(crate) fn allocate_array(e: &mut Engine, count: u32, size: u32) -> u32 {
        let product = u64::from(count) * u64::from(size);
        let bytes = if product > u64::from(u32::MAX) {
            u32::MAX
        } else {
            product as u32
        };
        e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32()
    }
}

/// Test doubles the shader units' tests share: the interlocked counts on memory, the array
/// constructor clearing its elements, the base constructor, the pass and stage deletes and a
/// vtable whose delete slot (+4) records its object.
#[cfg(test)]
pub(crate) mod shader_test_rig {
    use super::shader_inline::*;
    use crate::prelude::*;

    /// The delete slot (+4) of [`REFERENCE_VTABLE`].
    pub(crate) const REFERENCE_DELETE: u32 = 0x0e70_0000;
    /// A vtable for reference-counted test objects: slot +4 is [`REFERENCE_DELETE`].
    pub(crate) const REFERENCE_VTABLE: u32 = 0x0e71_0000;

    /// An engine with the shared doubles registered and the call log on.
    pub(crate) fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(VECTOR_CONSTRUCT, |e, a| {
            for index in 0..a[2] {
                e.mem.set_u32(a[0] + a[1] * index, 0);
            }
            Ret::default()
        });
        e.register(BS_SHADER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PASS_DELETE_00E80190, |_, _| Ret::default());
        e.register(STAGE_DELETE_00E7ECD0, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(REFERENCE_DELETE, |_, _| Ret::default());
        e.put_vtable(REFERENCE_VTABLE, &[0, REFERENCE_DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    /// A reference-counted object (vtable [`REFERENCE_VTABLE`]) with the given count.
    pub(crate) fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x70);
        e.mem.set_u32(object, REFERENCE_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// The argument words of every logged call to `addr`, in order.
    pub(crate) fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The `eh vector constructor iterator` call words for an array of `NiPointer`s.
    pub(crate) fn pointer_array(array: u32, count: u32, destructor: u32) -> Vec<u32> {
        vec![array, 4, count, NI_POINTER_CONSTRUCT_00B54620, destructor]
    }

    /// What a compile double saw: the file name `MakePath` was given, the compiled name the
    /// formatter was given, the profile, whether it was the pixel compile, and the first
    /// eight words of the macros.
    #[derive(Debug, PartialEq)]
    pub(crate) struct Compiled {
        pub(crate) file: u32,
        pub(crate) output: u32,
        pub(crate) profile: u32,
        pub(crate) pixel: bool,
        pub(crate) macros: [u32; 8],
    }

    /// [`rig`] with doubles for the loaders' callees: `memset` clears, `MakePath` and the
    /// formatter leave their string's address in the buffer's first word, and each compile
    /// answers a new reference-counted object (count 0) that keeps what it saw at +8.
    pub(crate) fn loader_rig() -> Engine {
        let mut e = rig();
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(MAKE_PATH, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });
        e.register(FORMAT_00406D00, |e, a| {
            assert_eq!(a[1], 0x104);
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        fn compile(e: &mut Engine, a: &[u32], pixel: bool) -> Ret {
            let shader = object(e, 0);
            let file = e.mem.u32(a[1]);
            e.mem.set_u32(shader + 8, file);
            let output = e.mem.u32(a[4]);
            e.mem.set_u32(shader + 0xc, output);
            e.mem.set_u32(shader + 0x10, a[3]);
            e.mem.set_u32(shader + 0x14, u32::from(pixel));
            for index in 0..8 {
                let word = e.mem.u32(a[2] + 4 * index);
                e.mem.set_u32(shader + 0x18 + 4 * index, word);
            }
            shader.into_ret()
        }
        e.register(COMPILE_VERTEX_SHADER, |e, a| compile(e, a, false));
        e.register(COMPILE_PIXEL_SHADER, |e, a| compile(e, a, true));
        e
    }

    /// What the compile double behind `shader` saw.
    pub(crate) fn compiled(e: &Engine, shader: u32) -> Compiled {
        let mut macros = [0u32; 8];
        for (index, word) in macros.iter_mut().enumerate() {
            *word = e.mem.u32(shader + 0x18 + 4 * index as u32);
        }
        Compiled {
            file: e.mem.u32(shader + 8),
            output: e.mem.u32(shader + 0xc),
            profile: e.mem.u32(shader + 0x10),
            pixel: e.mem.u32(shader + 0x14) != 0,
            macros,
        }
    }

    /// [`loader_rig`] with doubles for the stage and pass makers: a new pass (0x70 bytes)
    /// goes into the slot, a new stage (count 1) into its pointer, `SetStage` counts the
    /// pass's stages at +0x14 and holds the stage, a new render-state block is 0x10 bytes.
    pub(crate) fn pass_rig() -> Engine {
        let mut e = loader_rig();
        e.register(NEW_PASS_00E804D0, |e, a| {
            let pass = e.mem.alloc(0x70);
            e.mem.set_u32(pass + 0x64, 1);
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register(CREATE_NEW_STAGE, |e, a| {
            let stage = e.mem.alloc(0x70);
            e.mem.set_u32(stage + 0x60, 1);
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register(SETUP_DEFAULT_STAGE, |_, _| Ret::default());
        e.register(SET_FILTER_MODE, |_, _| Ret::default());
        e.register(PASS_SET_STAGE, |e, a| {
            let stages = e.mem.u32(a[0] + 0x14) + 1;
            e.mem.set_u32(a[0] + 0x14, stages);
            let held = e.mem.u32(a[2] + 0x60) + 1;
            e.mem.set_u32(a[2] + 0x60, held);
            Ret::default()
        });
        e.register(NEW_RENDER_STATES_00E7F5D0, |e, _| {
            e.mem.alloc(0x10).into_ret()
        });
        e.register(SET_RENDER_STATE_00E7F430, |_, _| Ret::default());
        e
    }

    /// The (state, value) pairs set on the pass's render-state block, in order (each with the
    /// third word 1).
    pub(crate) fn render_states(e: &Engine, pass: u32) -> Vec<(u32, u32)> {
        let block = e.mem.u32(pass + 0x30);
        calls(e, SET_RENDER_STATE_00E7F430)
            .iter()
            .filter(|a| a[0] == block)
            .map(|a| {
                assert_eq!(a[3], 1);
                (a[1], a[2])
            })
            .collect()
    }

    /// The (index, three words) `SetupDefaultStage` got for each stage, and the filter modes,
    /// in order, for the stages `SetStage` put on the pass.
    pub(crate) fn stages(e: &Engine, pass: u32) -> Vec<(u32, [u32; 3], u32)> {
        let set: Vec<u32> = calls(e, PASS_SET_STAGE)
            .iter()
            .filter(|a| a[0] == pass)
            .map(|a| a[2])
            .collect();
        let setups = calls(e, SETUP_DEFAULT_STAGE);
        let filters = calls(e, SET_FILTER_MODE);
        set.iter()
            .map(|stage| {
                let setup = setups.iter().find(|a| a[0] == *stage).unwrap();
                let filter = filters.iter().find(|a| a[0] == *stage).unwrap();
                (setup[1], [setup[2], setup[3], setup[4]], filter[1])
            })
            .collect()
    }

    /// The macro words a compile saw: the pairs, then zeros.
    pub(crate) fn macro_words(pairs: &[(u32, u32)]) -> [u32; 8] {
        let mut words = [0u32; 8];
        for (index, (name, value)) in pairs.iter().enumerate() {
            words[2 * index] = *name;
            words[2 * index + 1] = *value;
        }
        words
    }
}

/// Tests of the shared inline helpers.
#[cfg(test)]
mod shader_inline_tests {
    use super::shader_inline::*;
    use super::shader_test_rig::*;
    use crate::prelude::*;

    #[test]
    fn ni_pointer_assign_releases_the_old_object_and_holds_the_new_one() {
        let mut e = rig();
        let old = object(&mut e, 1);
        let new = object(&mut e, 4);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        ni_pointer_assign(&mut e, slot, new);
        assert_eq!(e.mem.u32(slot), new);
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(e.mem.u32(new + 4), 5);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        // The same object again: nothing happens.
        ni_pointer_assign(&mut e, slot, new);
        assert_eq!(e.mem.u32(new + 4), 5);
        // Null: the object is released, nothing is held.
        ni_pointer_assign(&mut e, slot, 0);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.mem.u32(new + 4), 4);
        assert_eq!(calls(&e, REFERENCE_DELETE).len(), 1);
    }

    #[test]
    fn clears_release_only_set_slots() {
        let mut e = rig();
        let held = object(&mut e, 2);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, held);
        ni_pointer_clear(&mut e, slot);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.mem.u32(held + 4), 1);
        ni_pointer_clear(&mut e, slot);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 1);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(slot, pass);
        pass_pointer_clear(&mut e, slot);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        let stage = e.mem.alloc(0x70);
        e.mem.set_u32(stage + 0x60, 2);
        stage_release(&mut e, stage);
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        stage_release(&mut e, stage);
        stage_release(&mut e, 0);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![stage]]);
    }

    #[test]
    fn allocate_array_saturates_an_overflowing_size() {
        let mut e = rig();
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        allocate_array(&mut e, 0xe4, 0x10);
        allocate_array(&mut e, 0x1000_0000, 0x10);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xe40], vec![MEMORY_MANAGER, u32::MAX]]
        );
    }
}

use shader_inline::{NEW_RENDER_STATES_00E7F5D0, PASS_DELETE_00E80190};

/// The pass pool (`0126f72c`): element array at +0, count at +8, growth size at +0x10.
const PASS_POOL_0126F72C: u32 = 0x0126_f72c;
/// The pool's grow `00e80420` (unit `geometrydecalshader.cpp`, unnamed; `thiscall` on the
/// pool, one word: the growth size), the render-state block release `00e7f6b0` (unnamed,
/// `cdecl`, the block).
const POOL_GROW_00E80420: u32 = 0x00e8_0420;
const RELEASE_RENDER_STATES_00E7F6B0: u32 = 0x00e7_f6b0;

// Translated from 00e804d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `GeometryDecalShader::LoadStagesAndPasses` in the engine map, but the body is a pass
/// maker (folded code; described by address), `cdecl` with one word: the address of a pass
/// pointer. When the pass pool (`0126f72c`) is empty it grows it (`00e80420` with its growth
/// size at +0x10) and doubles that size; then takes the pool's first element (the last one
/// moves into its place, the count at +8 goes down), holds it (count at +0x64 up), sets its
/// byte +0x61 when clear, gives it a new render-state block (`00e7f5d0`; the old block at
/// +0x30 released through `00e7f6b0` when it differs) and assigns it to the pointer (the old
/// pass's count dropped, `00e80190` at 0; the new one held); finally the first hold is
/// dropped (`00e80190` when the count reaches 0).
pub fn fn_00e804d0(e: &mut Engine, slot: Ptr) {
    let pool = e.global::<u32>(PASS_POOL_0126F72C);
    if e.mem.u32(pool + 8) == 0 {
        let growth = e.mem.u32(pool + 0x10);
        e.call(POOL_GROW_00E80420, &args![pool, growth]);
        let growth = e.mem.u32(pool + 0x10);
        e.mem.set_u32(pool + 0x10, growth.wrapping_add(growth));
    }
    let elements = e.mem.u32(pool);
    let pass = e.mem.u32(elements);
    let count = e.mem.u32(pool + 8).wrapping_sub(1);
    e.mem.set_u32(pool + 8, count);
    let last = e.mem.u32(elements.wrapping_add(count.wrapping_mul(4)));
    e.mem.set_u32(elements, last);
    if pass != 0 {
        let held = e.mem.u32(pass + 0x64).wrapping_add(1);
        e.mem.set_u32(pass + 0x64, held);
    }
    if e.mem.u8(pass.wrapping_add(0x61)) == 0 {
        e.mem.set_u8(pass.wrapping_add(0x61), 1);
    }
    let block = e.call(NEW_RENDER_STATES_00E7F5D0, &args![]).u32();
    let old_block = e.mem.u32(pass.wrapping_add(0x30));
    if old_block != block {
        e.call(RELEASE_RENDER_STATES_00E7F6B0, &args![old_block]);
    }
    e.mem.set_u32(pass.wrapping_add(0x30), block);
    let old = e.mem.u32(slot.addr());
    if old != pass {
        if old != 0 {
            let left = e.mem.u32(old + 0x64).wrapping_sub(1);
            e.mem.set_u32(old + 0x64, left);
            if left == 0 {
                e.call(PASS_DELETE_00E80190, &args![old]);
            }
        }
        e.mem.set_u32(slot.addr(), pass);
        let held = e.mem.u32(pass.wrapping_add(0x64)).wrapping_add(1);
        e.mem.set_u32(pass.wrapping_add(0x64), held);
    }
    let left = e.mem.u32(pass.wrapping_add(0x64)).wrapping_sub(1);
    e.mem.set_u32(pass.wrapping_add(0x64), left);
    if left == 0 {
        e.call(PASS_DELETE_00E80190, &args![pass]);
    }
}

use shader_inline::{
    compile_shaders, construct_pointer_array, ni_pointer_clear, pass_pointer_clear,
    release_reference, set_up_sources, ShaderBuffers, ShaderCompile, ShaderSource,
    BS_SHADER_CONSTRUCT, DESTRUCT_00B66F20, DESTRUCT_00B79930,
};

/// The geometry decal shader's vtable (`010bb588`; no RTTI locator before it).
const GEOMETRY_DECAL_VTABLE_010BB588: u32 = 0x010b_b588;

// Translated from 00bcbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader::GeometryDecalShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010bb588`, constructs the `NiPointer`
/// arrays at +0x6c (2, destructor `00b79930`), +0x74 and +0x7c (2 each, destructor
/// `00b66f20`), clears the words +0x84, +0x88, +0x8c, sets +0x68 to 0xc and the byte +0x24 to
/// 1, clears (releasing what they hold) the `NiPointer`s at +0x84, +0x88 and +0x8c and the two
/// pass pointers at +0x6c (their counts at +0x64 dropped, `00e80190` at 0). Answers `this`.
// Translation incomplete: 00bcbf00 - the C++ exception frame (FS:[0] chain, unwind states 0 to 6 for the base and the arrays) is not translated.
pub fn geometry_decal_shader_geometry_decal_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, GEOMETRY_DECAL_VTABLE_010BB588);
    construct_pointer_array(e, base + 0x6c, 2, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x74, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x7c, 2, DESTRUCT_00B66F20);
    for offset in [0x84u32, 0x88, 0x8c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 0xc);
    e.mem.set_u8(base + 0x24, 1);
    for offset in [0x84u32, 0x88, 0x8c] {
        ni_pointer_clear(e, base + offset);
    }
    for index in 0..2u32 {
        pass_pointer_clear(e, base + 0x6c + 4 * index);
    }
    this
}

/// Strings of `LoadShaders` (read in Ghidra): `"lighting\\2x\\v\\decal.v.hlsl"` (`010bb728`),
/// `"lighting\\2x\\p\\decal.p.hlsl"` (`010bb6d4`), the macros `"DECAL"` (`010bb720`),
/// `"GEOMDECAL"` (`010bb714`) with `""` (`01011584`), `"MAXDECALS"` (`010bb708`) with `"1"`
/// (`01063f48`) and `"SKIN"` (`010aec10`) with `""`, the compiled names `"GDECAL.vso"`,
/// `"GDECALS.vso"`, `"GDECAL.pso"` and the vertex profile `"vs_1_1"` (`010a93ec`).
const DECAL_VERTEX_SOURCE_010BB728: u32 = 0x010b_b728;
const DECAL_PIXEL_SOURCE_010BB6D4: u32 = 0x010b_b6d4;
const MACRO_DECAL_010BB720: u32 = 0x010b_b720;
const MACRO_GEOMDECAL_010BB714: u32 = 0x010b_b714;
const MACRO_MAXDECALS_010BB708: u32 = 0x010b_b708;
const MACRO_SKIN_010AEC10: u32 = 0x010a_ec10;
const DECAL_EMPTY_STRING_01011584: u32 = 0x0101_1584;
const DECAL_ONE_STRING_01063F48: u32 = 0x0106_3f48;
const OUTPUT_GDECAL_VSO_010BB6FC: u32 = 0x010b_b6fc;
const OUTPUT_GDECALS_VSO_010BB6F0: u32 = 0x010b_b6f0;
const OUTPUT_GDECAL_PSO_010BB6C8: u32 = 0x010b_b6c8;
const DECAL_PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
/// The pixel profile chooser `00b4f380` (unit `bsshadermanager.cpp`, unnamed; `cdecl`, a
/// byte).
const DECAL_PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;

/// The source blocks of `LoadShaders`' frame (0x300 bytes up to the return address).
const DECAL_VERTEX_BLOCK: ShaderSource = ShaderSource {
    offset: 0xa8,
    words: &[
        DECAL_VERTEX_SOURCE_010BB728,
        MACRO_DECAL_010BB720,
        DECAL_EMPTY_STRING_01011584,
        MACRO_GEOMDECAL_010BB714,
        DECAL_EMPTY_STRING_01011584,
        MACRO_MAXDECALS_010BB708,
        DECAL_ONE_STRING_01063F48,
        0,
    ],
    cleared: 0x2c,
};
/// The skinned vertex block: every word stored by `MOV`, the ten after the pairs as zeros.
const DECAL_SKINNED_VERTEX_BLOCK: ShaderSource = ShaderSource {
    offset: 0x10,
    words: &[
        DECAL_VERTEX_SOURCE_010BB728,
        MACRO_DECAL_010BB720,
        DECAL_EMPTY_STRING_01011584,
        MACRO_GEOMDECAL_010BB714,
        DECAL_EMPTY_STRING_01011584,
        MACRO_MAXDECALS_010BB708,
        DECAL_ONE_STRING_01063F48,
        MACRO_SKIN_010AEC10,
        DECAL_EMPTY_STRING_01011584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    cleared: 0,
};
const DECAL_PIXEL_BLOCK: ShaderSource = ShaderSource {
    offset: 0x5c,
    words: &[
        DECAL_PIXEL_SOURCE_010BB6D4,
        MACRO_DECAL_010BB720,
        DECAL_EMPTY_STRING_01011584,
        MACRO_GEOMDECAL_010BB714,
        DECAL_EMPTY_STRING_01011584,
        MACRO_MAXDECALS_010BB708,
        DECAL_ONE_STRING_01063F48,
        0,
    ],
    cleared: 0x2c,
};
const DECAL_LOAD_SHADERS_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xf4,
    path: 0x1f8,
};

// Translated from 00bcb4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader::LoadShaders` (Xbox PDB), `thiscall`, no stack words. Fills each source
/// block on its stack just before its compile (the macros `DECAL`, `GEOMDECAL` = `""` and
/// `MAXDECALS` = `"1"`): the vertex file `decal.v.hlsl` as `"GDECAL.vso"` (`vs_1_1`) into the
/// `NiPointer` at +0x74; the same with `SKIN` = `""` as `"GDECALS.vso"` into +0x78; then, with
/// the pixel profile from `00b4f380(1)`, the pixel file `decal.p.hlsl` as `"GDECAL.pso"` into
/// +0x7c; finally the `NiPointer` at +0x80 is assigned what +0x7c holds (read again after the
/// old object's release). Each step: `MakePath`, the name copied by `00406d00`, the compile,
/// the assignment.
// Translation incomplete: 00bcb4d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn geometry_decal_shader_load_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x300, |e, frame| {
        let frame = frame.addr();
        let buffers = &DECAL_LOAD_SHADERS_BUFFERS;
        set_up_sources(e, frame, &[DECAL_VERTEX_BLOCK]);
        let vertex = ShaderCompile {
            source: 0xa8,
            output: OUTPUT_GDECAL_VSO_010BB6FC,
            profile: DECAL_PROFILE_VS_1_1_010A93EC,
            pixel: false,
            slot: 0x74,
        };
        compile_shaders(e, base, frame, buffers, &[vertex]);
        set_up_sources(e, frame, &[DECAL_SKINNED_VERTEX_BLOCK]);
        let skinned = ShaderCompile {
            source: 0x10,
            output: OUTPUT_GDECALS_VSO_010BB6F0,
            profile: DECAL_PROFILE_VS_1_1_010A93EC,
            pixel: false,
            slot: 0x78,
        };
        compile_shaders(e, base, frame, buffers, &[skinned]);
        let profile = e.call(DECAL_PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        set_up_sources(e, frame, &[DECAL_PIXEL_BLOCK]);
        let pixel = ShaderCompile {
            source: 0x5c,
            output: OUTPUT_GDECAL_PSO_010BB6C8,
            profile,
            pixel: true,
            slot: 0x7c,
        };
        compile_shaders(e, base, frame, buffers, &[pixel]);
    });
    let old = e.mem.u32(base + 0x80);
    if old != e.mem.u32(base + 0x7c) {
        if old != 0 {
            release_reference(e, old);
        }
        let shader = e.mem.u32(base + 0x7c);
        e.mem.set_u32(base + 0x80, shader);
        if shader != 0 {
            e.call(
                shader_inline::IMPORT_INTERLOCKED_INCREMENT,
                &args![shader + 4],
            );
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_bf00, geometry_decal_shader_geometry_decal_shader(Ptr) -> Ptr),
        entry!(0x00bc_b4d0, geometry_decal_shader_load_shaders(Ptr)),
        entry!(0x00e8_04d0, fn_00e804d0(Ptr)),
        entry!(
            0x00bc_c0a0,
            geometry_decal_shader_create_shader() -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    const OLD: u32 = 0x0f90_0100;
    const OLD_DELETE: u32 = 0x0f90_1004;

    /// The rig, with the two pointers mapped (the first holding `OLD`, held once) and the
    /// interlocked imports changing the counts.
    fn decal_rig() -> Engine {
        let mut e = rig(CONSTRUCT, &[LOAD_SHADERS, CALL_00E804D0]);
        e.map(0x0120_2000, 0x1000);
        e.put_vtable(0x0f90_0000, &[0, OLD_DELETE]);
        e.mem.set_u32(OLD, 0x0f90_0000);
        e.mem.set_u32(OLD + 4, 1);
        e.set_global(DECLARATION_POINTER_012021E0, OLD);
        e.register(OLD_DELETE, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e
    }

    #[test]
    fn geometry_decal_shader_create_shader_keeps_both_declarations() {
        let mut e = decal_rig();
        let (shader, log) = run(&mut e, 0x00bc_c0a0);
        let (a, b) = (DECLARATION_0, DECLARATION_1);
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (IMPORT_INTERLOCKED_DECREMENT, vec![OLD + 4]),
                (OLD_DELETE, vec![OLD]),
                (IMPORT_INTERLOCKED_INCREMENT, vec![a + 4]),
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (IMPORT_INTERLOCKED_INCREMENT, vec![b + 4]),
                (declaration_slot(0x8c), vec![a, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![a, 1, 3, 3, 2, 0]),
                (declaration_slot(0xb4), vec![a]),
                (declaration_slot(0x8c), vec![b, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![b, 1, 1, 1, 3, 0]),
                (declaration_slot(0x8c), vec![b, 2, 2, 2, 4, 0]),
                (declaration_slot(0x8c), vec![b, 3, 3, 3, 2, 0]),
                (declaration_slot(0xb4), vec![b]),
                (NI_ALLOCATE, vec![0x90]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (LOAD_SHADERS, vec![SHADER]),
                (CALL_00E804D0, vec![SHADER + 0x6c]),
                (CALL_00E804D0, vec![SHADER + 0x70]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, a]),
            ]
        );
        assert_eq!(e.global::<u32>(DECLARATION_POINTER_012021E0), a);
        assert_eq!(e.global::<u32>(DECLARATION_POINTER_012021E4), b);
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(e.mem.u32(b + 4), 1);
    }

    #[test]
    fn assign_pointer_keeps_the_same_object() {
        let mut e = decal_rig();
        e.call_log = Some(vec![]);
        assign_pointer(&mut e, DECLARATION_POINTER_012021E0, OLD);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.mem.u32(OLD + 4), 1);
        // A null object releases the old one (still held once more here) and holds nothing.
        e.mem.set_u32(OLD + 4, 2);
        assign_pointer(&mut e, DECLARATION_POINTER_012021E0, 0);
        assert_eq!(e.mem.u32(OLD + 4), 1);
        assert_eq!(e.global::<u32>(DECLARATION_POINTER_012021E0), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod new_pass_tests {
    use super::shader_test_rig::*;
    use super::*;

    /// A pool holding `passes` (their counts 0) and a growth size of 4.
    fn pool(e: &mut Engine, passes: &[u32]) -> u32 {
        e.map(0x0126_f000, 0x1000);
        let pool = e.mem.alloc(0x14);
        let elements = e.mem.alloc(0x20);
        for (index, pass) in passes.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *pass);
        }
        e.mem.set_u32(pool, elements);
        e.mem.set_u32(pool + 8, passes.len() as u32);
        e.mem.set_u32(pool + 0x10, 4);
        e.set_global::<u32>(PASS_POOL_0126F72C, pool);
        pool
    }

    fn engine() -> Engine {
        let mut e = rig();
        e.register(NEW_RENDER_STATES_00E7F5D0, |_, _| 0x0e90_0000u32.into_ret());
        e.register(RELEASE_RENDER_STATES_00E7F6B0, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00e804d0_takes_the_first_pass_and_moves_the_last_into_its_place() {
        let mut e = engine();
        let passes = [e.mem.alloc(0x70), e.mem.alloc(0x70), e.mem.alloc(0x70)];
        e.mem.set_u32(passes[0] + 0x30, 0x0e80_0000);
        let pool = pool(&mut e, &passes);
        let old = e.mem.alloc(0x70);
        e.mem.set_u32(old + 0x64, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        e.call(0x00e8_04d0, &args![slot]);
        assert_eq!(e.mem.u32(slot), passes[0]);
        assert_eq!(e.mem.u32(passes[0] + 0x64), 1);
        assert_eq!(e.mem.u8(passes[0] + 0x61), 1);
        assert_eq!(e.mem.u32(passes[0] + 0x30), 0x0e90_0000);
        assert_eq!(
            calls(&e, RELEASE_RENDER_STATES_00E7F6B0),
            vec![vec![0x0e80_0000]]
        );
        assert_eq!(e.mem.u32(pool + 8), 2);
        assert_eq!(e.mem.u32(e.mem.u32(pool)), passes[2]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![old]]);
        assert!(calls(&e, POOL_GROW_00E80420).is_empty());
    }

    #[test]
    fn fn_00e804d0_grows_an_empty_pool_and_keeps_the_same_pass() {
        let mut e = engine();
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x30, 0x0e90_0000);
        e.mem.set_u8(pass + 0x61, 7);
        let pool = pool(&mut e, &[]);
        e.register_double(POOL_GROW_00E80420, move |e, a| {
            let elements = e.mem.u32(a[0]);
            e.mem.set_u32(elements, pass);
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, pass);
        e.mem.set_u32(pass + 0x64, 1);
        e.call(0x00e8_04d0, &args![slot]);
        assert_eq!(calls(&e, POOL_GROW_00E80420), vec![vec![pool, 4]]);
        assert_eq!(e.mem.u32(pool + 0x10), 8);
        assert_eq!(e.mem.u32(pool + 8), 3);
        assert_eq!(e.mem.u8(pass + 0x61), 7);
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        assert!(calls(&e, RELEASE_RENDER_STATES_00E7F6B0).is_empty());
        assert!(calls(&e, PASS_DELETE_00E80190).is_empty());
    }
}

#[cfg(test)]
mod constructor_and_loader_tests {
    use super::shader_inline::*;
    use super::shader_test_rig::*;
    use super::*;

    #[test]
    fn geometry_decal_shader_constructor_builds_the_shader() {
        let mut e = rig();
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcc; 0x90]);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), GEOMETRY_DECAL_VTABLE_010BB588);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 2, DESTRUCT_00B79930),
                pointer_array(this + 0x74, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x7c, 2, DESTRUCT_00B66F20),
            ]
        );
        for offset in [0x6cu32, 0x70, 0x84, 0x88, 0x8c] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0xc);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert!(calls(&e, PASS_DELETE_00E80190).is_empty());
    }

    #[test]
    fn geometry_decal_shader_load_shaders_compiles_three_shaders_and_shares_the_pixel_one() {
        const PROFILE: u32 = 0x0ee0_0000;
        let mut e = loader_rig();
        e.register(DECAL_PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a[0], 1);
            PROFILE.into_ret()
        });
        let this = e.mem.alloc(0x90);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x80, old);
        e.call(LOAD_SHADERS, &args![this]);
        let decal = [
            (MACRO_DECAL_010BB720, DECAL_EMPTY_STRING_01011584),
            (MACRO_GEOMDECAL_010BB714, DECAL_EMPTY_STRING_01011584),
            (MACRO_MAXDECALS_010BB708, DECAL_ONE_STRING_01063F48),
        ];
        let mut skinned = decal.to_vec();
        skinned.push((MACRO_SKIN_010AEC10, DECAL_EMPTY_STRING_01011584));
        let expected = [
            (
                0x74u32,
                DECAL_VERTEX_SOURCE_010BB728,
                OUTPUT_GDECAL_VSO_010BB6FC,
                DECAL_PROFILE_VS_1_1_010A93EC,
                false,
                macro_words(&decal),
            ),
            (
                0x78,
                DECAL_VERTEX_SOURCE_010BB728,
                OUTPUT_GDECALS_VSO_010BB6F0,
                DECAL_PROFILE_VS_1_1_010A93EC,
                false,
                macro_words(&skinned),
            ),
            (
                0x7c,
                DECAL_PIXEL_SOURCE_010BB6D4,
                OUTPUT_GDECAL_PSO_010BB6C8,
                PROFILE,
                true,
                macro_words(&decal),
            ),
        ];
        for (slot, file, output, profile, pixel, macros) in expected {
            let shader = e.mem.u32(this + slot);
            assert_eq!(
                compiled(&e, shader),
                Compiled {
                    file,
                    output,
                    profile,
                    pixel,
                    macros
                }
            );
        }
        let pixel = e.mem.u32(this + 0x7c);
        assert_eq!(e.mem.u32(this + 0x80), pixel);
        assert_eq!(e.mem.u32(pixel + 4), 2);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        let cleared: Vec<u32> = calls(&e, MEMSET).iter().map(|a| a[2]).collect();
        assert_eq!(cleared, vec![0x2c, 0x2c]);
    }
}
