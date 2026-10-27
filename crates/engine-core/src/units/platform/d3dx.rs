//! The platform's D3DX (`d3dx9_38.dll`), the maintainer's decision (2026-10-22, option b): D3DX
//! is Microsoft's code, a platform layer like the Win32 imports, so it is not translated. Each
//! function the game reaches is written here from its documented meaning and matched bit for
//! bit against what the player's DLL answers: `scripts/d3dx-measure` calls the DLL in a 32-bit
//! process on many inputs (random, edge cases, NaN/Inf/denormals, out and in pointers the same)
//! and records inputs and results; the tests here check against those records. The DLL picks a
//! CPU-specific path, so the records are the measuring machine's (AMD Ryzen AI 9 HX 370,
//! 2026-10-22).
//!
//! Functions reached so far (import slots of FalloutNV.exe 1.4.0.525):
//! - `D3DXMatrixMultiply` (slot `00fdf424`, thunk `00ee6de0`): first reached by
//!   `Do_SetCameraData` `00e6c780` in `NiDX9Renderer::Initialize` `00e72e60`.
//! - `D3DXMatrixMultiplyTranspose` (slot `00fdf410`, thunk `00ee6dfe`): first reached
//!   by `00e83db0` < `00e826d0` < `00bca980` (a shader pass's set-up, in `00b994f0`).
//! - `D3DXMatrixTranspose` (slot `00fdf42c`, thunk `00ee6dd4`): written ahead, from the same
//!   measurement.
//! - `D3DXVec3TransformNormal` (slot `00fdf420`, thunk `00ee6df2`) and `D3DXVec4Transform`
//!   (slot `00fdf3ec`): written ahead, from the same measurement. `D3DXVec3TransformCoord` is
//!   not: its division is not a plain divide (results 1-2 ulp below `x / w`, also for `w` = 2: a
//!   reciprocal estimate and refinement, CPU-specific), not modelled yet.
//! - `D3DXMatrixRotationYawPitchRoll` (slot `00fdf40c`, thunk `00ee6e04`, called from `00bb2fc0`):
//!   the DLL's SSE2 quaternion (its own packed sine and cosine) and x87 matrix, followed
//!   instruction by instruction from a private analysis of the player's DLL (no bytes copied);
//!   52,000 measured calls match.

use crate::{Engine, Ptr};

/// IEEE single-precision words, with the NaN rules of the SSE instructions spelled out (an
/// operation with a NaN operand answers its first NaN operand, quieted; an invalid operation
/// answers the default NaN `ffc00000`), so results do not depend on how the compiler orders
/// commutative operations or on the host's NaN rules.
mod single {
    fn is_nan(x: u32) -> bool {
        x & 0x7fff_ffff > 0x7f80_0000
    }

    fn op(x: u32, y: u32, f: fn(f32, f32) -> f32) -> u32 {
        if is_nan(x) {
            return x | 0x0040_0000;
        }
        if is_nan(y) {
            return y | 0x0040_0000;
        }
        let r = f(f32::from_bits(x), f32::from_bits(y));
        if r.is_nan() {
            0xffc0_0000
        } else {
            r.to_bits()
        }
    }

    pub fn mul(x: u32, y: u32) -> u32 {
        op(x, y, |a, b| a * b)
    }

    pub fn add(x: u32, y: u32) -> u32 {
        op(x, y, |a, b| a + b)
    }

    pub fn sub(x: u32, y: u32) -> u32 {
        op(x, y, |a, b| a - b)
    }
}

/// The 16 words of a `D3DXMATRIX` (row major) at `at`.
fn matrix(e: &Engine, at: u32) -> [u32; 16] {
    let mut m = [0u32; 16];
    for (i, w) in m.iter_mut().enumerate() {
        *w = e.mem.u32(at.wrapping_add(4 * i as u32));
    }
    m
}

/// `M1 * M2` as the measured DLL works it out. Each word is `sum over k of m1[i][k] * m2[k][j]`
/// in single precision, with these orders (found from the 40010 measured calls, which all match
/// bit for bit, NaN payloads included; the x87 control word and pointer aliasing make no
/// difference):
/// - rows 0, 1 and 2 multiply `m1[i][k] * m2[k][j]`; row 3 multiplies `m2[k][j] * m1[i][k]`
///   (which NaN wins when both are NaNs);
/// - rows 0 and 3 add `(p0 + p1) + (p2 + p3)`; rows 1 and 2 add `((p0 + p1) + p2) + p3`.
pub fn multiply(a: &[u32; 16], b: &[u32; 16]) -> [u32; 16] {
    let mut out = [0u32; 16];
    for i in 0..4 {
        for j in 0..4 {
            let p: [u32; 4] = std::array::from_fn(|k| {
                if i == 3 {
                    single::mul(b[4 * k + j], a[4 * i + k])
                } else {
                    single::mul(a[4 * i + k], b[4 * k + j])
                }
            });
            out[4 * i + j] = if i == 0 || i == 3 {
                single::add(single::add(p[0], p[1]), single::add(p[2], p[3]))
            } else {
                single::add(single::add(single::add(p[0], p[1]), p[2]), p[3])
            };
        }
    }
    out
}

// Platform: replaces d3dx9_38.dll D3DXMatrixMultiply (import)
/// `D3DXMatrixMultiply(out, m1, m2)` (stdcall): `out = m1 * m2` (row-vector convention:
/// `out[i][j] = sum over k of m1[i][k] * m2[k][j]`, in [`multiply`]'s order), both read before
/// `out` is written, so `out` may be either input (measured); answers `out`.
pub fn matrix_multiply(e: &mut Engine, out: Ptr, m1: Ptr, m2: Ptr) -> u32 {
    let a = matrix(e, m1.addr());
    let b = matrix(e, m2.addr());
    let r = multiply(&a, &b);
    for (i, w) in r.iter().enumerate() {
        e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), *w);
    }
    out.addr()
}

// Platform: replaces d3dx9_38.dll D3DXMatrixMultiplyTranspose (import)
/// `D3DXMatrixMultiplyTranspose(out, m1, m2)` (stdcall): `out = transpose(m1 * m2)`, the
/// product worked out exactly as [`multiply`] (all 4000 measured calls match bit for bit,
/// `measure/d3dx-functions-2026-10-22.tsv`), both inputs read before `out` is written (measured
/// with `out` = `m1`); answers `out`.
pub fn matrix_multiply_transpose(e: &mut Engine, out: Ptr, m1: Ptr, m2: Ptr) -> u32 {
    let a = matrix(e, m1.addr());
    let b = matrix(e, m2.addr());
    let r = multiply(&a, &b);
    for (i, w) in transpose(&r).iter().enumerate() {
        e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), *w);
    }
    out.addr()
}

/// The transpose of a row-major matrix: word `4 * i + j` from word `4 * j + i`.
fn transpose(m: &[u32; 16]) -> [u32; 16] {
    std::array::from_fn(|n| m[4 * (n % 4) + n / 4])
}

/// The words of a vector of `n` floats at `at`.
fn vector(e: &Engine, at: u32, n: usize) -> Vec<u32> {
    (0..n)
        .map(|i| e.mem.u32(at.wrapping_add(4 * i as u32)))
        .collect()
}

/// `D3DXVec3TransformNormal`'s words for `v` and `m`: column `j` is
/// `(m[1][j]*v1 + m[0][j]*v0) + m[2][j]*v2`, each product with the matrix word first (the only
/// order of the products and sums that reproduces all 4000 measured calls bit for bit,
/// `measure/d3dx-functions-2026-10-22.tsv`).
pub fn transform_normal(v: &[u32], m: &[u32; 16]) -> [u32; 3] {
    std::array::from_fn(|j| {
        let p: [u32; 3] = std::array::from_fn(|k| single::mul(m[4 * k + j], v[k]));
        single::add(single::add(p[1], p[0]), p[2])
    })
}

/// `D3DXVec4Transform`'s words for `v` and `m`: column `j` is
/// `(m[3][j]*v3 + m[2][j]*v2) + (m[1][j]*v1 + m[0][j]*v0)`, each product with the matrix word
/// first (the only such order that reproduces all 4000 measured calls bit for bit).
pub fn vec4_transform(v: &[u32], m: &[u32; 16]) -> [u32; 4] {
    std::array::from_fn(|j| {
        let p: [u32; 4] = std::array::from_fn(|k| single::mul(m[4 * k + j], v[k]));
        single::add(single::add(p[3], p[2]), single::add(p[1], p[0]))
    })
}

// Platform: replaces d3dx9_38.dll D3DXVec3TransformNormal (import)
/// `D3DXVec3TransformNormal(out, v, m)` (stdcall): [`transform_normal`], the inputs read before
/// `out` is written (measured with `out` = `v`); answers `out`.
pub fn vec3_transform_normal(e: &mut Engine, out: Ptr, v: Ptr, m: Ptr) -> u32 {
    let r = transform_normal(&vector(e, v.addr(), 3), &matrix(e, m.addr()));
    for (i, w) in r.iter().enumerate() {
        e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), *w);
    }
    out.addr()
}

// Platform: replaces d3dx9_38.dll D3DXVec4Transform (import)
/// `D3DXVec4Transform(out, v, m)` (stdcall): [`vec4_transform`], the inputs read before `out`
/// is written (measured with `out` = `v`); answers `out`.
pub fn vec4_transform_import(e: &mut Engine, out: Ptr, v: Ptr, m: Ptr) -> u32 {
    let r = vec4_transform(&vector(e, v.addr(), 4), &matrix(e, m.addr()));
    for (i, w) in r.iter().enumerate() {
        e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), *w);
    }
    out.addr()
}

// Platform: replaces d3dx9_38.dll D3DXMatrixTranspose (import)
/// `D3DXMatrixTranspose(out, m)` (stdcall): `out[i][j] = m[j][i]`, each word copied as a float
/// load and store would copy it (a signalling NaN comes out quiet), except that with `out` the
/// same matrix as `m` the diagonal is not written (it keeps its bits). All 4000 measured calls
/// (`measure/d3dx-functions-2026-10-22.tsv`, both control words, `out` = `m` and not) match
/// this; answers `out`.
pub fn matrix_transpose(e: &mut Engine, out: Ptr, m: Ptr) -> u32 {
    let t = transpose(&matrix(e, m.addr()));
    for (n, w) in t.iter().enumerate() {
        if out == m && n % 5 == 0 {
            continue;
        }
        let quiet = if w & 0x7fff_ffff > 0x7f80_0000 {
            w | 0x0040_0000
        } else {
            *w
        };
        e.mem.set_u32(out.addr().wrapping_add(4 * n as u32), quiet);
    }
    out.addr()
}

/// The packed constants of the DLL's SSE2 sine and cosine (the same word in all four lanes):
/// 2/pi, the three-part pi/2 (1.5703125, 4.837513e-4, 7.5495336e-8, 2.563344e-12), 2^23, the
/// sine's coefficients (-1.9515282e-4, 8.33216e-3, -0.16666655, 1) and the cosine's (2.4433157e-5,
/// -1.3887316e-3, 0.041666646, -0.5, 1).
const TWO_OVER_PI: u32 = 0x3f22_f983;
const PI_OVER_2: [u32; 4] = [0x3fc9_0000, 0x39fd_a000, 0x33a2_2000, 0x2c34_611a];
const TWO_TO_23: u32 = 0x4b00_0000;
const SIN_COEFFICIENTS: [u32; 4] = [0xb94c_a1f0, 0x3c08_839d, 0xbe2a_aaa3, 0x3f80_0000];
const COS_COEFFICIENTS: [u32; 5] = [
    0x37cc_f5ce,
    0xbab6_061a,
    0x3d2a_aaa5,
    0xbf00_0000,
    0x3f80_0000,
];

/// One lane of the DLL's SSE2 sine and cosine of a float (`sin`, `cos` as words), the order of
/// its packed instructions followed one by one: `|x| * 2/pi + 2^23` keeps the quadrant `n` in
/// its low mantissa bits, `k = that - 2^23`, the reduced `r = |x| - k*c1 - k*c2 - k*c3 - k*c4`,
/// `z = r*r`; the sine polynomial `((((s1*z + s2)*z + s3)*z + s4) * r` and the cosine
/// `(((c1*z + c2)*z + c3)*z + c4)*z + c5`; an odd quadrant swaps them; the sine's sign is bit 1
/// of `n - (n & 1)` with the input's sign, the cosine's bit 1 of `n + (n & 1)`. Each operation is
/// the SSE one (first operand's NaN first, the default NaN for an invalid one).
pub fn sse2_sin_cos(x: u32) -> (u32, u32) {
    let abs = x & 0x7fff_ffff;
    let sign = x & 0x8000_0000;
    let biased = single::add(single::mul(abs, TWO_OVER_PI), TWO_TO_23);
    let k = single::sub(biased, TWO_TO_23);
    let mut r = abs;
    for c in PI_OVER_2 {
        r = single::sub(r, single::mul(c, k));
    }
    let z = single::mul(r, r);
    let n = biased & 0x007f_ffff;
    let odd = n & 1;
    let sin_sign = ((n.wrapping_sub(odd) & 2) << 30) ^ sign;
    let cos_sign = (n.wrapping_add(odd) & 2) << 30;
    let swap = odd != 0;
    let mut s = SIN_COEFFICIENTS[0];
    let mut c = COS_COEFFICIENTS[0];
    for i in 1..4 {
        s = single::add(single::mul(s, z), SIN_COEFFICIENTS[i]);
        c = single::add(single::mul(c, z), COS_COEFFICIENTS[i]);
    }
    s = single::mul(s, r);
    c = single::add(single::mul(c, z), COS_COEFFICIENTS[4]);
    if swap {
        (c ^ sin_sign, s ^ cos_sign)
    } else {
        (s ^ sin_sign, c ^ cos_sign)
    }
}

/// `D3DXQuaternionRotationYawPitchRoll`'s SSE2 version in the DLL (the one its start-up picks
/// on a processor with SSE2 and without 3DNow!): the half angles (`0.5 * angle`), their sines
/// and cosines ([`sse2_sin_cos`]), then `x = sy*sr*cp + cy*cr*sp`, `y = -(cy*sr*sp) + sy*cr*cp`,
/// `z = -(sy*cr*sp) + cy*sr*cp`, `w = sy*sr*sp + cy*cr*cp`, each product left to right as the
/// packed multiplies pair the lanes and the negation a sign flip.
pub fn quaternion_rotation_yaw_pitch_roll(yaw: u32, pitch: u32, roll: u32) -> [u32; 4] {
    let half = |a: u32| single::mul(0x3f00_0000, a);
    let (sr, cr) = sse2_sin_cos(half(roll));
    let (sp, cp) = sse2_sin_cos(half(pitch));
    let (sy, cy) = sse2_sin_cos(half(yaw));
    let m = single::mul;
    let first = [
        m(m(cy, cr), sp),
        m(m(sy, cr), cp),
        m(m(cy, sr), cp),
        m(m(cy, cr), cp),
    ];
    let second = [
        m(m(sy, sr), cp),
        m(m(cy, sr), sp) ^ 0x8000_0000,
        m(m(sy, cr), sp) ^ 0x8000_0000,
        m(m(sy, sr), sp),
    ];
    std::array::from_fn(|i| single::add(second[i], first[i]))
}

/// `D3DXMatrixRotationQuaternion`'s x87 version in the DLL (its only one), instruction by
/// instruction at the running thread's precision ([`crate::x87`]): the doubled components and
/// the nine products stored as floats, then `1 - yy - zz`, `xy + wz`, `xz - wy`, `xy - wz`,
/// `1 - xx - zz`, `yz + wx`, `xz + wy`, `yz - wx`, `1 - xx - yy` and the zero and one words.
pub fn matrix_rotation_quaternion(q: &[u32; 4]) -> [u32; 16] {
    use crate::x87::{x87_add, x87_load32, x87_m32, x87_mul, x87_store32, x87_sub};
    let [x, y, z, w] = q.map(x87_load32);
    let doubled = |v: f64| x87_store32(x87_add(v, v));
    let (x2, y2, z2) = (doubled(x), doubled(y), doubled(z));
    let wx = x87_store32(x87_mul(w, x87_m32(x2)));
    let wy = x87_store32(x87_mul(w, x87_m32(y2)));
    let z2_value = x87_load32(z2);
    let wz = x87_store32(x87_mul(w, z2_value));
    let xx = x87_store32(x87_mul(x, x87_m32(x2)));
    let xy = x87_store32(x87_mul(x, x87_m32(y2)));
    let xz = x87_store32(x87_mul(x, z2_value));
    let yy = x87_store32(x87_mul(y, x87_m32(y2)));
    let yz = x87_store32(x87_mul(y, z2_value));
    let zz = x87_store32(x87_mul(z2_value, x87_m32(q[2])));
    let one = 1.0f64;
    let load = x87_load32;
    let mut out = [0u32; 16];
    out[0] = x87_store32(x87_sub(x87_sub(one, load(yy)), load(zz)));
    out[1] = x87_store32(x87_add(load(xy), x87_m32(wz)));
    out[2] = x87_store32(x87_sub(load(xz), x87_m32(wy)));
    out[4] = x87_store32(x87_sub(load(xy), x87_m32(wz)));
    let one_less_xx = x87_sub(one, load(xx));
    out[5] = x87_store32(x87_sub(one_less_xx, load(zz)));
    out[6] = x87_store32(x87_add(load(yz), x87_m32(wx)));
    out[8] = x87_store32(x87_add(load(xz), x87_m32(wy)));
    out[9] = x87_store32(x87_sub(load(yz), x87_m32(wx)));
    out[10] = x87_store32(x87_sub(one_less_xx, load(yy)));
    out[15] = 0x3f80_0000;
    out
}

// Platform: replaces d3dx9_38.dll D3DXMatrixRotationYawPitchRoll (import)
/// `D3DXMatrixRotationYawPitchRoll(out, yaw, pitch, roll)` (stdcall, the angles as floats): as
/// the DLL's start-up leaves it on a processor with SSE2 and without 3DNow! (read from the
/// player's d3dx9_38.dll in a private analysis): the angles copied through the x87 (a
/// signalling NaN comes out quiet), [`quaternion_rotation_yaw_pitch_roll`]'s SSE2 version, then
/// [`matrix_rotation_quaternion`]. Matches the 2000 measured calls (both control words,
/// `measure/d3dx-functions-2026-10-22.tsv`) bit for bit; answers `out`. A processor with
/// 3DNow! (the DLL takes another version there) is not modelled.
pub fn matrix_rotation_yaw_pitch_roll(
    e: &mut Engine,
    out: Ptr,
    yaw: u32,
    pitch: u32,
    roll: u32,
) -> u32 {
    let q = quaternion_rotation_yaw_pitch_roll(
        crate::x87::x87_quiet_bits(yaw),
        crate::x87::x87_quiet_bits(pitch),
        crate::x87::x87_quiet_bits(roll),
    );
    let m = matrix_rotation_quaternion(&q);
    for (i, w) in m.iter().enumerate() {
        e.mem.set_u32(out.addr().wrapping_add(4 * i as u32), *w);
    }
    out.addr()
}

/// `D3DXERR_INVALIDDATA`.
pub const D3DXERR_INVALIDDATA: u32 = 0x8876_0b59;
/// `D3DRTYPE_TEXTURE`, `D3DXIFF_DDS`.
const D3DRTYPE_TEXTURE: u32 = 3;
const D3DXIFF_DDS: u32 = 4;
/// `DDS_HEADER` fields (after the 4-byte magic): height, width, depth, mip count, the pixel
/// format's flags, FourCC, bit count and the red, green, blue and alpha masks, `dwCaps2`.
const DDS_HEIGHT: usize = 12;
const DDS_WIDTH: usize = 16;
const DDS_MIP_COUNT: usize = 28;
const DDS_PF_FLAGS: usize = 80;
const DDS_PF_FOURCC: usize = 84;
const DDS_PF_BITS: usize = 88;
const DDS_PF_MASKS: usize = 92;
const DDS_CAPS2: usize = 112;
const DDS_HEADER_SIZE: usize = 128;
const DDPF_ALPHAPIXELS: u32 = 0x1;
const DDPF_FOURCC: u32 = 0x4;
const DDPF_RGB: u32 = 0x40;
const DDSCAPS2_CUBEMAP: u32 = 0x200;
/// `DDSCAPS2_CUBEMAP_POSITIVEX` .. `NEGATIVEZ`.
const DDSCAPS2_ALL_FACES: u32 = 0xfc00;
/// `D3DRTYPE_CUBETEXTURE`.
const D3DRTYPE_CUBETEXTURE: u32 = 5;
const DDSCAPS2_VOLUME: u32 = 0x20_0000;

/// What a DDS file describes (`D3DXIMAGE_INFO`'s words) and where its levels are.
#[derive(Debug, Clone, PartialEq)]
pub struct DdsImage {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    pub format: u32,
    /// The side of the format's blocks in pixels (4 for DXT, 1 otherwise).
    pub block: u32,
    /// 1, or 6 for a cube map.
    pub faces: u32,
    /// Each level's bytes: (offset in the file, length), largest first, face by face.
    pub levels: Vec<(usize, usize)>,
}

/// The Direct3D 9 format of a DDS pixel format, for the formats measured with the player's
/// DLL (2026-10-22: DXT1, DXT3, DXT5; 24-bit RGB as `D3DFMT_R8G8B8`, 32-bit ARGB with alpha as
/// `D3DFMT_A8R8G8B8`, 16-bit 4444 as `D3DFMT_A4R4G4B4`), with each one's block side and bytes per
/// block. Others are not provided.
fn dds_format(flags: u32, fourcc: u32, bits: u32, masks: [u32; 4]) -> (u32, u32, u32) {
    if flags & DDPF_FOURCC != 0 {
        return match &fourcc.to_le_bytes() {
            b"DXT1" => (fourcc, 4, 8),
            b"DXT2" | b"DXT3" | b"DXT4" | b"DXT5" => (fourcc, 4, 16),
            _ => panic!("D3DX: a DDS file with FourCC {fourcc:#x} is not provided"),
        };
    }
    let alpha = flags & DDPF_ALPHAPIXELS != 0;
    match (flags & DDPF_RGB != 0, bits, masks, alpha) {
        (true, 24, [0xff_0000, 0xff00, 0xff, 0], false) => (20, 1, 3),
        (true, 32, [0xff_0000, 0xff00, 0xff, 0xff00_0000], true) => (21, 1, 4),
        (true, 16, [0xf00, 0xf0, 0xf, 0xf000], true) => (26, 1, 2),
        _ => panic!(
            "D3DX: a DDS pixel format (flags {flags:#x}, {bits} bits, masks {masks:08x?}) is not provided"
        ),
    }
}

/// Reads a DDS file the way the measured DLL does (2026-10-22, 983 of the game's textures and
/// variants of 99 of them): the magic `DDS ` and a 128-byte header; the mip count is the
/// header's field whatever `DDSD_MIPMAPCOUNT` says, 0 counting as 1; the file must hold every
/// level, or the answer is `D3DXERR_INVALIDDATA` (a file one byte short, the header alone, a
/// mip count beyond the data, a broken magic). Not provided (the run stops): other image file
/// formats, volume textures and cube maps without all six faces, more levels than the full chain, and the pixel formats
/// [`dds_format`] doesn't list.
pub fn read_dds(data: &[u8]) -> Result<DdsImage, u32> {
    // Measured: a DDS file with its first byte broken, or cut short of its header, is refused.
    // Anything else that is not a DDS file may be another format the DLL reads: not provided.
    if data.len() < 4 || &data[1..4] != b"DS " {
        panic!("D3DX: image files other than DDS are not provided");
    }
    if data[0] != b'D' || data.len() < DDS_HEADER_SIZE {
        return Err(D3DXERR_INVALIDDATA);
    }
    let word = |at: usize| u32::from_le_bytes(data[at..at + 4].try_into().unwrap());
    let caps2 = word(DDS_CAPS2);
    if caps2 & DDSCAPS2_VOLUME != 0 {
        panic!("D3DX: volume DDS files are not provided");
    }
    // A cube map with all six faces (measured: the game's six cube maps, 2026-10-23); one with
    // some faces only is not provided.
    let faces = if caps2 & DDSCAPS2_CUBEMAP != 0 {
        if caps2 & DDSCAPS2_ALL_FACES != DDSCAPS2_ALL_FACES {
            panic!("D3DX: a cube map DDS file without all six faces is not provided");
        }
        6
    } else {
        1
    };
    let (width, height) = (word(DDS_WIDTH), word(DDS_HEIGHT));
    let mip_levels = word(DDS_MIP_COUNT).max(1);
    let full = 32 - width.max(height).max(1).leading_zeros();
    if mip_levels > full {
        panic!("D3DX: a DDS file with {mip_levels} levels for {width}x{height} is not provided");
    }
    let masks = [0, 1, 2, 3].map(|i| word(DDS_PF_MASKS + 4 * i));
    let (format, block, bytes) = dds_format(
        word(DDS_PF_FLAGS),
        word(DDS_PF_FOURCC),
        word(DDS_PF_BITS),
        masks,
    );
    let mut levels = Vec::new();
    let mut at = DDS_HEADER_SIZE;
    for _face in 0..faces {
        for level in 0..mip_levels {
            let (w, h) = ((width >> level).max(1), (height >> level).max(1));
            let size = (w.div_ceil(block) * h.div_ceil(block) * bytes) as usize;
            levels.push((at, size));
            at += size;
        }
    }
    if at > data.len() {
        return Err(D3DXERR_INVALIDDATA);
    }
    Ok(DdsImage {
        width,
        height,
        mip_levels,
        format,
        block,
        faces,
        levels,
    })
}

// Platform: replaces d3dx9_38.dll D3DXGetImageInfoFromFileInMemory (import)
/// `D3DXGetImageInfoFromFileInMemory(data, size, info)` (stdcall): for a DDS file
/// ([`read_dds`]) `D3D_OK` and the `D3DXIMAGE_INFO` (width, height, depth 1, mip levels,
/// format, `D3DRTYPE_TEXTURE` or for a cube map `D3DRTYPE_CUBETEXTURE`, `D3DXIFF_DDS`); a file it
/// refuses answers its error and writes
/// nothing (measured). A null info is not provided.
pub fn get_image_info_from_file_in_memory(e: &mut Engine, data: Ptr, size: u32, info: Ptr) -> u32 {
    assert!(
        !info.is_null(),
        "D3DXGetImageInfoFromFileInMemory without an info is not provided"
    );
    let bytes = e.mem.bytes(data.addr(), size);
    match read_dds(&bytes) {
        Ok(image) => {
            let words = [
                image.width,
                image.height,
                1,
                image.mip_levels,
                image.format,
                if image.faces == 6 {
                    D3DRTYPE_CUBETEXTURE
                } else {
                    D3DRTYPE_TEXTURE
                },
                D3DXIFF_DDS,
            ];
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(info.addr() + 4 * i as u32, *w);
            }
            0
        }
        Err(hr) => hr,
    }
}

/// Gives the layer the machine-local texture cache folder (`scripts/d3dx-texture-capture`).
pub fn set_texture_cache(e: &mut Engine, dir: std::path::PathBuf) {
    e.kernel.graphics.d3dx_texture_cache = Some(dir);
}

/// How to fill the texture cache, for the stop when an entry is missing.
pub const TEXTURE_CACHE_HINT: &str =
    "run `scripts/d3dx-texture-capture` on this machine (it reads \
     the game's textures and the player's d3dx9_38.dll and writes the cache NV_D3DX_CACHE names, \
     default %LOCALAPPDATA%/nv-rs/d3dx-cache)";

/// FNV-1a, 64 bits.
fn fnv(seed: u64, b: &[u8]) -> u64 {
    b.iter().fold(seed, |h, &x| {
        (h ^ u64::from(x)).wrapping_mul(0x100_0000_01b3)
    })
}

/// The texture cache's key of one call (the same as the capture tool's `key`): the call's
/// name, the data's length and the FNV-1a hash of the name then the data.
pub fn texture_key(call: &str, data: &[u8]) -> String {
    let h = fnv(fnv(0xcbf2_9ce4_8422_2325, call.as_bytes()), data);
    format!("{call}-{:x}-{h:016x}", data.len())
}

/// One surface of a cache entry: format, width, height, and the bytes D3DX made (`None` when
/// it left the file's level as stored).
#[derive(Debug, Clone, PartialEq)]
pub struct CachedSurface {
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub bytes: Option<Vec<u8>>,
}

/// A cache entry (`NVTX0001`): the call's HRESULT, its level and face counts and surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct CachedTexture {
    pub hr: u32,
    pub levels: u32,
    pub faces: u32,
    pub surfaces: Vec<CachedSurface>,
}

/// Reads a cache entry; `None` when it is not one.
pub fn parse_cached_texture(b: &[u8]) -> Option<CachedTexture> {
    if b.get(..8)? != b"NVTX0001" {
        return None;
    }
    let mut at = 8usize;
    fn take(b: &[u8], at: &mut usize) -> Option<u32> {
        let w = u32::from_le_bytes(b.get(*at..*at + 4)?.try_into().ok()?);
        *at += 4;
        Some(w)
    }
    let word = |at: &mut usize| take(b, at);
    let (_kind, hr, levels, faces) = (
        word(&mut at)?,
        word(&mut at)?,
        word(&mut at)?,
        word(&mut at)?,
    );
    let mut surfaces = Vec::new();
    for _ in 0..levels * faces {
        let (format, width, height, kept, len) = (
            word(&mut at)?,
            word(&mut at)?,
            word(&mut at)?,
            word(&mut at)?,
            word(&mut at)?,
        );
        let bytes = if kept != 0 {
            let start = at;
            at += len as usize;
            Some(b.get(start..at)?.to_vec())
        } else {
            None
        };
        surfaces.push(CachedSurface {
            format,
            width,
            height,
            bytes,
        });
    }
    Some(CachedTexture {
        hr,
        levels,
        faces,
        surfaces,
    })
}

/// The copy rule (measured 2026-10-23 on 983 of the game's textures): D3DX leaves a level as
/// the file stores it when the file has that level, the texture keeps the file's format and
/// size, and the level is at least one block on each side; it makes every other level itself
/// (sub-block DXT levels, the rest of the chain when the file stores fewer levels, converted
/// formats).
pub fn copies_level(image: &DdsImage, surface: &CachedSurface, level: u32) -> bool {
    let (w, h) = (
        (image.width >> level).max(1),
        (image.height >> level).max(1),
    );
    level < image.mip_levels
        && surface.format == image.format
        && (surface.width, surface.height) == (w, h)
        && w >= image.block
        && h >= image.block
}

/// `IDirect3DDevice9::CreateTexture`, `IDirect3DTexture9::LockRect`/`UnlockRect` (vtable
/// offsets) and `D3DPOOL_MANAGED`.
const DEVICE_CREATE_TEXTURE: u32 = 4 * 23;
const TEXTURE_LOCK_RECT: u32 = 4 * 19;
const TEXTURE_UNLOCK_RECT: u32 = 4 * 20;
const D3DPOOL_MANAGED: u32 = 1;

// Platform: replaces d3dx9_38.dll D3DXCreateTextureFromFileInMemory (import)
/// `D3DXCreateTextureFromFileInMemory(device, data, size, out)` (stdcall): the texture D3DX
/// makes with its defaults (size, levels and format from the file, `D3DPOOL_MANAGED`). The
/// levels D3DX leaves as stored ([`copies_level`]) are copied from the file; the others come
/// from the machine-local texture cache, which also gives the call's answer and the texture's
/// level count and formats; a level the rule copies must be one the cache marks as left as stored. The texture
/// is made through the device's own `CreateTexture` and filled through `LockRect`. Not provided
/// (the run stops): a file the cache does not have (its bytes are saved beside the cache as
/// `missing-<key>.dds` so the capture can be run for it), cube and volume entries, a rule
/// the cache contradicts.
pub fn create_texture_from_file_in_memory(
    e: &mut Engine,
    device: Ptr,
    data: Ptr,
    size: u32,
    out: Ptr,
) -> u32 {
    create_from_file(e, device, data, size, out, false)
}

// Platform: replaces d3dx9_38.dll D3DXCreateCubeTextureFromFileInMemory (import)
/// `D3DXCreateCubeTextureFromFileInMemory(device, data, size, out)` (stdcall): as
/// [`create_texture_from_file_in_memory`] for a cube map, through the device's
/// `CreateCubeTexture`, face by face (the DDS order: +X, -X, +Y, -Y, +Z, -Z).
pub fn create_cube_texture_from_file_in_memory(
    e: &mut Engine,
    device: Ptr,
    data: Ptr,
    size: u32,
    out: Ptr,
) -> u32 {
    create_from_file(e, device, data, size, out, true)
}

/// `IDirect3DDevice9::CreateCubeTexture`, `IDirect3DCubeTexture9::LockRect`/`UnlockRect`.
const DEVICE_CREATE_CUBE_TEXTURE: u32 = 4 * 25;

fn create_from_file(
    e: &mut Engine,
    device: Ptr,
    data: Ptr,
    size: u32,
    out: Ptr,
    cube: bool,
) -> u32 {
    let bytes = e.mem.bytes(data.addr(), size);
    let call = if cube {
        "CreateCubeTextureFromFileInMemory"
    } else {
        "CreateTextureFromFileInMemory"
    };
    let key = texture_key(call, &bytes);
    let Some(dir) = e.kernel.graphics.d3dx_texture_cache.clone() else {
        panic!("D3DX{call}: no texture cache is set: {TEXTURE_CACHE_HINT}");
    };
    let cached = std::fs::read(dir.join(format!("{key}.tex"))).ok();
    let Some(entry) = cached.as_deref().and_then(parse_cached_texture) else {
        let saved = dir.join(format!("missing-{key}.dds"));
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(&saved, &bytes);
        panic!("D3DX{call}: {key} is not in the texture cache (input saved to {}): {TEXTURE_CACHE_HINT}", saved.display());
    };
    if entry.hr != 0 {
        return entry.hr;
    }
    let faces = if cube { 6 } else { 1 };
    assert_eq!(
        entry.faces, faces,
        "D3DX{call}: a {}-face entry is not provided",
        entry.faces
    );
    let image = read_dds(&bytes).expect("a file the DLL took");
    assert_eq!(
        image.faces, faces,
        "D3DX{call}: a file with {} faces is not provided",
        image.faces
    );
    // The file's level for (face, level), when it has one.
    let stored = |face: u32, level: u32| -> Option<(usize, usize)> {
        (level < image.mip_levels).then(|| image.levels[(face * image.mip_levels + level) as usize])
    };
    for (i, s) in entry.surfaces.iter().enumerate() {
        let (face, level) = (i as u32 / entry.levels, i as u32 % entry.levels);
        // A level the rule copies must be one D3DX left as stored. The other way round is
        // allowed: D3DX can make a level that comes out the same as the file's (a 1x1 DXT1
        // level re-encoded to the same block, 2026-10-23), and the cache then keeps no bytes
        // for it, so the file's are D3DX's.
        if copies_level(&image, s, level) && s.bytes.is_some() {
            panic!("D3DX{call} {key}: face {face} level {level}: the copy rule copies it, the cache has D3DX's own bytes (not provided)");
        }
        if s.bytes.is_none() && stored(face, level).is_none() {
            panic!("D3DX{call} {key}: face {face} level {level}: the cache says copy, the file has no such level");
        }
    }
    let top = &entry.surfaces[0];
    let made = e.with_stack(4, |e, slot| {
        let hr = if cube {
            e.vcall(
                device.addr(),
                DEVICE_CREATE_CUBE_TEXTURE,
                &[
                    top.width,
                    entry.levels,
                    0,
                    top.format,
                    D3DPOOL_MANAGED,
                    slot.addr(),
                    0,
                ],
            )
        } else {
            e.vcall(
                device.addr(),
                DEVICE_CREATE_TEXTURE,
                &[
                    top.width,
                    top.height,
                    entry.levels,
                    0,
                    top.format,
                    D3DPOOL_MANAGED,
                    slot.addr(),
                    0,
                ],
            )
        }
        .u32();
        (hr, e.mem.u32(slot.addr()))
    });
    if made.0 != 0 {
        panic!(
            "D3DX{call}: the device answered {:#x} (not provided)",
            made.0
        );
    }
    let texture = made.1;
    for (i, s) in entry.surfaces.iter().enumerate() {
        let (face, level) = (i as u32 / entry.levels, i as u32 % entry.levels);
        let level_bytes: &[u8] = match &s.bytes {
            Some(b) => b,
            None => {
                let (at, len) = stored(face, level).expect("checked above");
                &bytes[at..at + len]
            }
        };
        let (pitch, bits) = e.with_stack(8, |e, locked| {
            let hr = if cube {
                e.vcall(
                    texture,
                    TEXTURE_LOCK_RECT,
                    &[face, level, locked.addr(), 0, 0],
                )
            } else {
                e.vcall(texture, TEXTURE_LOCK_RECT, &[level, locked.addr(), 0, 0])
            }
            .u32();
            assert_eq!(hr, 0, "LockRect of a managed level");
            (e.mem.u32(locked.addr()), e.mem.u32(locked.addr() + 4))
        });
        let block = if s.format == image.format {
            image.block
        } else {
            1
        };
        let rows = s.height.div_ceil(block).max(1) as usize;
        let row = level_bytes.len() / rows;
        for r in 0..rows {
            e.mem.write(
                bits + r as u32 * pitch,
                &level_bytes[r * row..(r + 1) * row],
            );
        }
        if cube {
            e.vcall(texture, TEXTURE_UNLOCK_RECT, &[face, level]);
        } else {
            e.vcall(texture, TEXTURE_UNLOCK_RECT, &[level]);
        }
    }
    e.mem.set_u32(out.addr(), texture);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ret;

    /// The measured calls kept as test data (`data/d3dx/matrix_multiply.tsv`, from
    /// `scripts/d3dx-measure`): alias, control word, M1, M2, the DLL's result, returned.
    fn measured() -> Vec<(String, [u32; 16], [u32; 16], [u32; 16])> {
        let text = include_str!("../../../data/d3dx/matrix_multiply.tsv");
        let words = |t: &str| -> [u32; 16] {
            let v: Vec<u32> = t
                .split(' ')
                .map(|w| u32::from_str_radix(w, 16).unwrap())
                .collect();
            v.try_into().unwrap()
        };
        text.lines()
            .filter(|l| l.starts_with("MatrixMultiply"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                assert_eq!(f[6], "1");
                (f[1].to_string(), words(f[3]), words(f[4]), words(f[5]))
            })
            .collect()
    }

    #[test]
    fn multiply_matches_the_measured_dll_bit_for_bit() {
        let cases = measured();
        assert!(cases.len() > 200);
        for (alias, a, b, out) in &cases {
            assert_eq!(&multiply(a, b), out, "{alias} {a:08x?} * {b:08x?}");
        }
    }

    #[test]
    fn the_import_writes_out_and_answers_it_even_when_out_is_an_input() {
        for (alias, a, b, out) in measured() {
            let mut e = Engine::new();
            let m1 = e.mem.alloc(64);
            let m2 = if alias.contains("m1=m2") {
                m1
            } else {
                e.mem.alloc(64)
            };
            let target = match alias.as_str() {
                "out=m1" | "out=m1=m2" => m1,
                "out=m2" => m2,
                _ => e.mem.alloc(64),
            };
            for k in 0..16u32 {
                e.mem.set_u32(m1 + 4 * k, a[k as usize]);
                if m2 != m1 {
                    e.mem.set_u32(m2 + 4 * k, b[k as usize]);
                }
            }
            let answer = matrix_multiply(&mut e, Ptr::new(target), Ptr::new(m1), Ptr::new(m2));
            assert_eq!(answer, target);
            assert_eq!(matrix(&e, target), out, "{alias}");
        }
    }

    type MeasuredCase = (String, [u32; 16], [u32; 16], [u32; 16]);

    fn measured_transpose() -> Vec<MeasuredCase> {
        let text = include_str!("../../../data/d3dx/matrix_multiply_transpose.tsv");
        let words = |t: &str| -> [u32; 16] {
            let v: Vec<u32> = t
                .split(' ')
                .map(|w| u32::from_str_radix(w, 16).unwrap())
                .collect();
            v.try_into().unwrap()
        };
        text.lines()
            .filter(|l| l.starts_with("MatrixMultiplyTranspose"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (f[1].to_string(), words(f[3]), words(f[4]), words(f[5]))
            })
            .collect()
    }

    #[test]
    fn vector_transforms_match_the_measured_dll_bit_for_bit() {
        let text = include_str!("../../../data/d3dx/vector_transforms.tsv");
        let words = |t: &str| -> Vec<u32> {
            t.split(' ')
                .map(|w| u32::from_str_radix(w.trim(), 16).unwrap())
                .collect()
        };
        let mut n = [0; 2];
        for line in text.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<&str> = line.split('\t').collect();
            let (v, m, want) = (words(f[3]), words(f[4]), words(f[5]));
            let mut e = Engine::new();
            let at = e.mem.alloc(16);
            for (i, w) in v.iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, *w);
            }
            let mat = e.mem.alloc(64);
            for (i, w) in m.iter().enumerate() {
                e.mem.set_u32(mat + 4 * i as u32, *w);
            }
            let out = if f[1] == "out=in" {
                at
            } else {
                e.mem.alloc(16)
            };
            let (o, v, m) = (Ptr::new(out), Ptr::new(at), Ptr::new(mat));
            let r = if f[0] == "Vec3TransformNormal" {
                n[0] += 1;
                vec3_transform_normal(&mut e, o, v, m)
            } else {
                n[1] += 1;
                vec4_transform_import(&mut e, o, v, m)
            };
            assert_eq!(r, out);
            assert_eq!(vector(&e, out, want.len()), want, "{line}");
        }
        assert_eq!(n, [250, 250]);
    }

    #[test]
    fn transpose_matches_the_measured_dll_bit_for_bit() {
        let text = include_str!("../../../data/d3dx/matrix_transpose.tsv");
        let words = |t: &str| -> [u32; 16] {
            let v: Vec<u32> = t
                .split(' ')
                .map(|w| u32::from_str_radix(w, 16).unwrap())
                .collect();
            v.try_into().unwrap()
        };
        let mut n = 0;
        for line in text.lines().filter(|l| l.starts_with("MatrixTranspose")) {
            let f: Vec<&str> = line.split('\t').collect();
            let (m, want) = (words(f[3]), words(f[4]));
            let mut e = Engine::new();
            let at = e.mem.alloc(64);
            for (i, w) in m.iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, *w);
            }
            let out = if f[1] == "out=in" {
                at
            } else {
                e.mem.alloc(64)
            };
            assert_eq!(matrix_transpose(&mut e, Ptr::new(out), Ptr::new(at)), out);
            assert_eq!(matrix(&e, out), want, "{} {m:08x?}", f[1]);
            n += 1;
        }
        assert!(n > 300);
    }

    #[test]
    fn multiply_transpose_matches_the_measured_dll_bit_for_bit() {
        let cases = measured_transpose();
        assert!(cases.len() > 250);
        for (alias, a, b, want) in cases {
            let mut e = Engine::new();
            let m1 = e.mem.alloc(64);
            let m2 = e.mem.alloc(64);
            for i in 0..16u32 {
                e.mem.set_u32(m1 + 4 * i, a[i as usize]);
                e.mem.set_u32(m2 + 4 * i, b[i as usize]);
            }
            let out = if alias == "out=in" {
                m1
            } else {
                e.mem.alloc(64)
            };
            let r = matrix_multiply_transpose(&mut e, Ptr::new(out), Ptr::new(m1), Ptr::new(m2));
            assert_eq!(r, out);
            assert_eq!(matrix(&e, out), want, "{alias} {a:08x?} * {b:08x?}");
        }
    }

    #[test]
    fn the_import_slot_is_registered_by_name() {
        assert!(crate::units::platform::imports()
            .iter()
            .any(|(dll, name, _)| {
                *dll == "d3dx9_38.dll" && *name == "D3DXMatrixMultiplyTranspose"
            }));
        let table = crate::units::platform::imports();
        assert!(table
            .iter()
            .any(|(dll, name, _)| *dll == "d3dx9_38.dll" && *name == "D3DXMatrixMultiply"));
    }

    /// A DDS header (TEST DATA, made from the documented layout): `width` x `height` DXT1 with
    /// `mips` levels, followed by `extra` bytes of level data.
    fn dxt1(width: u32, height: u32, mips: u32, data: usize) -> Vec<u8> {
        let mut v = vec![0u8; 128 + data];
        v[..4].copy_from_slice(b"DDS ");
        let mut put = |at: usize, w: u32| v[at..at + 4].copy_from_slice(&w.to_le_bytes());
        put(4, 124);
        put(8, 0x000a_1007);
        put(DDS_HEIGHT, height);
        put(DDS_WIDTH, width);
        put(DDS_MIP_COUNT, mips);
        put(76, 32);
        put(DDS_PF_FLAGS, DDPF_FOURCC);
        put(DDS_PF_FOURCC, u32::from_le_bytes(*b"DXT1"));
        v
    }

    #[test]
    fn dds_info_follows_the_measured_rules() {
        // 8x4 DXT1, 4 levels: 2x1 blocks, then 1x1, 1x1, 1x1: 16 + 8 + 8 + 8 bytes.
        let image = read_dds(&dxt1(8, 4, 4, 40)).unwrap();
        assert_eq!((image.width, image.height, image.mip_levels), (8, 4, 4));
        assert_eq!(image.format, u32::from_le_bytes(*b"DXT1"));
        assert_eq!(image.levels, vec![(128, 16), (144, 8), (152, 8), (160, 8)]);
        assert_eq!(read_dds(&dxt1(8, 4, 4, 39)), Err(D3DXERR_INVALIDDATA));
        assert_eq!(read_dds(&dxt1(8, 4, 0, 16)).unwrap().mip_levels, 1);
        assert_eq!(
            read_dds(&dxt1(8, 4, 1, 16)[..127]),
            Err(D3DXERR_INVALIDDATA)
        );
        let mut broken = dxt1(8, 4, 1, 16);
        broken[0] = b'X';
        assert_eq!(read_dds(&broken), Err(D3DXERR_INVALIDDATA));
    }

    #[test]
    fn image_info_is_written_only_for_a_good_file() {
        let mut e = Engine::new();
        let file = dxt1(8, 4, 4, 40);
        let data = e.mem.alloc(file.len() as u32);
        e.mem.write(data, &file);
        let info = e.mem.alloc(28);
        let hr = get_image_info_from_file_in_memory(
            &mut e,
            Ptr::new(data),
            file.len() as u32,
            Ptr::new(info),
        );
        assert_eq!(hr, 0);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(info + 4 * i)).collect();
        assert_eq!(words, [8, 4, 1, 4, u32::from_le_bytes(*b"DXT1"), 3, 4]);
        e.mem.write(info, &[0x55; 28]);
        let hr = get_image_info_from_file_in_memory(
            &mut e,
            Ptr::new(data),
            file.len() as u32 - 1,
            Ptr::new(info),
        );
        assert_eq!(hr, D3DXERR_INVALIDDATA);
        assert_eq!(e.mem.u32(info), 0x5555_5555);
    }

    /// Checks every measured `D3DXGetImageInfoFromFileInMemory` answer against [`read_dds`]:
    /// `NV_D3DX_IMAGE_INFO=<measure dir>` with `d3dx-imageinfo-*.tsv` and the `dds` copies
    /// (`scripts/d3dx-measure image-info`; the files are the player's, so they stay there).
    #[test]
    #[ignore]
    fn image_info_matches_the_measured_dll() {
        let dir =
            std::path::PathBuf::from(std::env::var("NV_D3DX_IMAGE_INFO").expect("the measure dir"));
        // Every table: `d3dx-imageinfo-cube-*` for the cube maps in `dds-cube`, the others for `dds`.
        let tables: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|d| d.path())
            .filter(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("d3dx-imageinfo-")
            })
            .collect();
        assert!(!tables.is_empty(), "a d3dx-imageinfo table");
        let mut checked = 0;
        let lines: Vec<(String, String)> = tables
            .iter()
            .flat_map(|t| {
                let files = if t.to_string_lossy().contains("-cube-") {
                    "dds-cube"
                } else {
                    "dds"
                };
                std::fs::read_to_string(t)
                    .unwrap()
                    .lines()
                    .map(|l| (files.to_string(), l.to_string()))
                    .collect::<Vec<_>>()
            })
            .collect();
        for (files, line) in &lines {
            let f: Vec<&str> = line.split('\t').collect();
            if f[0] != "ImageInfo" {
                continue;
            }
            let file = std::fs::read(dir.join(files).join(f[1])).unwrap();
            let size = usize::from_str_radix(f[3], 16).unwrap();
            let mut data = file.clone();
            match f[2] {
                "as-is" => {}
                "header-only" | "short-127" | "minus-1" => data.truncate(size),
                "no-mipcount-flag" => {
                    let flags = u32::from_le_bytes(data[8..12].try_into().unwrap()) & !0x2_0000;
                    data[8..12].copy_from_slice(&flags.to_le_bytes());
                }
                "mipcount-0" => data[28..32].copy_from_slice(&0u32.to_le_bytes()),
                "mipcount-99" => continue, // more levels than the chain: not provided
                "bad-magic" => data[0] = b'X',
                other => panic!("variant {other}"),
            }
            let hr = u32::from_str_radix(f[4], 16).unwrap();
            let mut e = Engine::new();
            let at = e.mem.alloc(data.len() as u32 + 1);
            e.mem.write(at, &data);
            let info = e.mem.alloc(28);
            e.mem.write(info, &[0x55; 28]);
            let ours = get_image_info_from_file_in_memory(
                &mut e,
                Ptr::new(at),
                data.len() as u32,
                Ptr::new(info),
            );
            let bytes: String = e
                .mem
                .bytes(info, 28)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            assert_eq!((ours, bytes.as_str()), (hr, f[5]), "{} {}", f[1], f[2]);
            checked += 1;
        }
        assert!(checked > 1000, "{checked}");
    }

    /// A cache entry (TEST DATA) for `dxt1(8, 4, 4, 40)`: 8x4 and 4x2... levels: the first
    /// copied, the three sub-block ones (4x2, 2x1, 1x1) kept with made-up bytes.
    fn entry_for_8x4() -> Vec<u8> {
        let mut e = b"NVTX0001".to_vec();
        for w in [1u32, 0, 4, 1] {
            e.extend_from_slice(&w.to_le_bytes());
        }
        let dxt1 = u32::from_le_bytes(*b"DXT1");
        let surface = |e: &mut Vec<u8>, w: u32, h: u32, bytes: Option<&[u8]>| {
            let kept = bytes.map_or(0u32, |_| 1);
            let len = bytes.map_or(16, |b| b.len() as u32);
            for x in [dxt1, w, h, kept, len] {
                e.extend_from_slice(&x.to_le_bytes());
            }
            if let Some(b) = bytes {
                e.extend_from_slice(b);
            }
        };
        surface(&mut e, 8, 4, None);
        surface(&mut e, 4, 2, Some(&[0xa1; 8]));
        surface(&mut e, 2, 1, Some(&[0xa2; 8]));
        surface(&mut e, 1, 1, Some(&[0xa3; 8]));
        e
    }

    #[test]
    fn cache_entries_parse_and_the_copy_rule_agrees_with_them() {
        let entry = parse_cached_texture(&entry_for_8x4()).unwrap();
        assert_eq!((entry.hr, entry.levels, entry.faces), (0, 4, 1));
        let image = read_dds(&dxt1(8, 4, 4, 40)).unwrap();
        let rule: Vec<bool> = (0..4)
            .map(|l| copies_level(&image, &entry.surfaces[l], l as u32))
            .collect();
        assert_eq!(rule, [true, false, false, false]);
        assert_eq!(texture_key("Call", b"ab"), texture_key("Call", b"ab"));
        assert_ne!(texture_key("Call", b"ab"), texture_key("Call", b"ba"));
    }

    #[test]
    fn a_texture_is_made_through_the_device_from_the_file_and_the_cache() {
        let mut e = Engine::new();
        // The device and texture doubles: CreateTexture records its words, LockRect hands out
        // each level's memory with a pitch of the packed row.
        const CREATE: u32 = 0x7f00_0100;
        const LOCK: u32 = 0x7f00_0200;
        const UNLOCK: u32 = 0x7f00_0204;
        let device_vtable = e.mem.alloc(4 * 30);
        e.mem.set_u32(device_vtable + DEVICE_CREATE_TEXTURE, CREATE);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, device_vtable);
        let texture_vtable = e.mem.alloc(4 * 30);
        e.mem.set_u32(texture_vtable + TEXTURE_LOCK_RECT, LOCK);
        e.mem.set_u32(texture_vtable + TEXTURE_UNLOCK_RECT, UNLOCK);
        let texture = e.mem.alloc(4);
        e.mem.set_u32(texture, texture_vtable);
        let levels = e.mem.alloc(4 * 16);
        e.register_double(CREATE, move |e, a| {
            assert_eq!(&a[1..7], &[8, 4, 4, 0, u32::from_le_bytes(*b"DXT1"), 1]);
            e.mem.set_u32(a[7], texture);
            Ret::default()
        });
        e.register_double(LOCK, move |e, a| {
            let (pitch, bits) = [
                (16, levels),
                (8, levels + 16),
                (8, levels + 24),
                (8, levels + 32),
            ][a[1] as usize];
            e.mem.set_u32(a[2], pitch);
            e.mem.set_u32(a[2] + 4, bits);
            Ret::default()
        });
        e.register(UNLOCK, |_, _| Ret::default());
        let dir = std::env::temp_dir().join(format!("nv-rs-d3dx-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut file = dxt1(8, 4, 4, 40);
        file[128..144].copy_from_slice(&[0x11; 16]);
        let key = texture_key("CreateTextureFromFileInMemory", &file);
        std::fs::write(dir.join(format!("{key}.tex")), entry_for_8x4()).unwrap();
        set_texture_cache(&mut e, dir.clone());
        let data = e.mem.alloc(file.len() as u32);
        e.mem.write(data, &file);
        let out = e.mem.alloc(4);
        let hr = create_texture_from_file_in_memory(
            &mut e,
            Ptr::new(device),
            Ptr::new(data),
            file.len() as u32,
            Ptr::new(out),
        );
        assert_eq!(hr, 0);
        assert_eq!(e.mem.u32(out), texture);
        assert_eq!(
            e.mem.bytes(levels, 16),
            vec![0x11; 16],
            "the stored level, copied"
        );
        assert_eq!(
            e.mem.bytes(levels + 16, 8),
            vec![0xa1; 8],
            "the made ones, from the cache"
        );
        assert_eq!(e.mem.bytes(levels + 32, 8), vec![0xa3; 8]);
        // A file the cache does not have stops the run with its bytes saved.
        e.mem.set_u8(data + 128, 0x12);
        let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            create_texture_from_file_in_memory(
                &mut e,
                Ptr::new(device),
                Ptr::new(data),
                file.len() as u32,
                Ptr::new(out),
            )
        }));
        assert!(stop.is_err());
        assert!(std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .any(|d| d.file_name().to_string_lossy().starts_with("missing-")));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

#[cfg(test)]
mod tests_yaw_pitch_roll {
    use super::*;

    /// Calls of the player's d3dx9_38.dll (TEST DATA from `scripts/d3dx-measure functions`,
    /// 2026-10-22 and 2026-10-27; all 52,000 measured calls match): control word, yaw, pitch,
    /// roll, the 16 words answered.
    const MEASURED: &[(u16, [u32; 3], &str)] = &[
        (0x027f, [0x3502_59ac, 0xbf80_0000, 0xaa41_20f1], "3f800000 a9e00000 b50259a9 00000000 b4db5f32 3f0a5140 bf576aa5 00000000 348cdb6b 3f576aa5 3f0a5140 00000000 00000000 00000000 00000000 3f800000"),
        (0x007f, [0xb19a_5166, 0xbf80_0000, 0x2e3d_00a7], "3f800000 2dcc3cc0 31991352 00000000 318060a4 3f0a5140 bf576aa5 00000000 b126c1b7 3f576aa5 3f0a5140 00000000 00000000 00000000 00000000 3f800000"),
        // At 24-bit precision the diagonal's two subtractions round each (53-bit would give
        // 3eed1081 first).
        (0x007f, [0xbf80_0000, 0xb8a0_cc29, 0x3f0a_a4ca], "3eed1080 3f03f704 3f389596 00000000 be8e92d6 3f5b5d97 bede1b83 00000000 bf576aa4 38a0d000 3f0a5140 00000000 00000000 00000000 00000000 3f800000"),
        (0x027f, [0x7fc1_2345, 0x4080_0000, 0x03d3_e5e5], "7fc12345 7fc12345 7fc12345 00000000 7fc12345 7fc12345 7fc12345 00000000 7fc12345 7fc12345 7fc12345 00000000 00000000 00000000 00000000 3f800000"),
        // A signalling NaN comes out quiet; an infinity gives the positive default NaN.
        (0x027f, [0x7f80_0001, 0x3eaa_aaab, 0x3400_0000], "7fc00001 7fc00001 7fc00001 00000000 7fc00001 7fc00001 7fc00001 00000000 7fc00001 7fc00001 7fc00001 00000000 00000000 00000000 00000000 3f800000"),
        (0x027f, [0x7f80_0000, 0x3eaa_aaab, 0x8080_0000], "7fc00000 7fc00000 7fc00000 00000000 7fc00000 7fc00000 7fc00000 00000000 7fc00000 7fc00000 7fc00000 00000000 00000000 00000000 00000000 3f800000"),
        // Angles too big for the reduction: NaNs of both signs.
        (0x007f, [0x5f80_0000, 0xff80_0000, 0xdc65_9399], "ffc00000 7fc00000 7fc00000 00000000 7fc00000 ffc00000 7fc00000 00000000 7fc00000 7fc00000 ffc00000 00000000 00000000 00000000 00000000 3f800000"),
        (0x007f, [0, 0, 0], "3f800000 00000000 00000000 00000000 00000000 3f800000 00000000 00000000 00000000 00000000 3f800000 00000000 00000000 00000000 00000000 3f800000"),
    ];

    #[test]
    fn rotation_yaw_pitch_roll_matches_the_dll() {
        for (cw, [yaw, pitch, roll], want) in MEASURED {
            let mut e = Engine::new();
            crate::units::crt::set_x87_control_word(&mut e, *cw);
            let out = e.mem.alloc(64);
            assert_eq!(
                matrix_rotation_yaw_pitch_roll(&mut e, Ptr::new(out), *yaw, *pitch, *roll),
                out
            );
            let got: Vec<String> = (0..16)
                .map(|i| format!("{:08x}", e.mem.u32(out + 4 * i)))
                .collect();
            assert_eq!(
                got.join(" "),
                *want,
                "{yaw:08x} {pitch:08x} {roll:08x} at {cw:04x}"
            );
        }
    }

    #[test]
    fn sine_and_cosine_follow_the_quadrants() {
        let f = f32::from_bits;
        for (x, s, c) in [
            (0.0f32, 0.0f32, 1.0f32),
            (std::f32::consts::FRAC_PI_2, 1.0, 0.0),
            (std::f32::consts::PI, 0.0, -1.0),
            (-std::f32::consts::FRAC_PI_2, -1.0, 0.0),
        ] {
            let (sin, cos) = sse2_sin_cos(x.to_bits());
            assert!(
                (f(sin) - s).abs() < 1e-6 && (f(cos) - c).abs() < 1e-6,
                "{x}"
            );
        }
        // The import is registered under its name.
        assert!(crate::units::platform::imports()
            .iter()
            .any(|(dll, name, _)| *dll == "d3dx9_38.dll"
                && *name == "D3DXMatrixRotationYawPitchRoll"));
    }
}
