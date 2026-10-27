//! The platform's sound mixer: what DirectSound sends to the device for the game's buffers, as
//! measured on the player's machine (`scripts/dsmix-measure`: DirectSound 8 played known buffers
//! with known settings and WASAPI loopback recorded what reached a device without audio
//! enhancements, "Virtual Desktop Audio", 48000 Hz 32-bit float stereo; 2026-10-24,
//! `build43-scratch/measure/dsmix-*`).
//!
//! Exact (every sample bit for bit):
//! - A sample becomes `x / 32768` (16-bit) or `(b - 128) * 256 / 32768` (8-bit) as `f32`; a mono
//!   buffer goes unchanged to both channels; a stereo buffer's channels to their own.
//! - Volume and pan are one `f32` multiply per channel by [`gain`] of the channel's attenuation
//!   in hundredths of a decibel: the volume on both, a negative pan on the right only, a positive
//!   pan on the left only. Volume and pan 0 multiply by exactly 1. All 429 volumes and the 25
//!   pans measured.
//!
//! Where the exact path ends (the maintainer decides how to go on; until then such a buffer
//! stops the run when it would be heard, see `super::dsound`):
//! - A buffer whose frequency is not the device's (Windows resamples it with a long polyphase
//!   filter that the measurements do not pin down yet).
//! - A 3D buffer (Windows' 3D stage turns position into volume and pan in 0.1 dB steps by a rule
//!   the measurements do not pin down yet).
//! - The distortion effect (not analysed).
//!
//! - Several buffers playing at once are added in `f32`, the buffer started last first (each
//!   one's gained sample added to the sum of those started after it): 47996 of 48000 samples
//!   for 3 and 4 voices at mixed volumes and pans, and near full scale, where play order or an
//!   `f64` sum differ (`dsmix-multi-2026-10-24`; buffers made and started in the same order, so
//!   whether "last" means made or started is not separated). The 4 other samples: voices
//!   started 2 frames into a device period lost their last 2 frames (they stop on the period
//!   grid); voices started on a period boundary, as [`render`]'s callers start them, are heard
//!   whole.

/// The amplitude factor for an attenuation of `millibels` (hundredths of a decibel, 0 down to
/// -10000): exactly 1 for 0, else `floor(65536 * 10^(d / 2000))` as `f32` times `f32(1 / 65535)`
/// with `d = min(10 * round((millibels + 10) / 10), -10)` (halves away from zero), 0 at -10000.
pub fn gain(millibels: i32) -> f32 {
    if millibels >= 0 {
        return 1.0;
    }
    if millibels <= -10000 {
        return 0.0;
    }
    let tenth = ((millibels + 10) as f64 / 10.0).round() * 10.0;
    let d = tenth.min(-10.0);
    let k = (65536.0 * 10f64.powf(d / 2000.0)).floor() as f32;
    k * (1.0f32 / 65535.0)
}

/// A sample as the mixer reads it.
pub fn sample(bytes: &[u8], bits: u16, at: usize) -> f32 {
    if bits == 16 {
        let v = i16::from_le_bytes([bytes[2 * at], bytes[2 * at + 1]]);
        f32::from(v) / 32768.0
    } else {
        ((i32::from(bytes[at]) - 128) * 256) as f32 / 32768.0
    }
}

/// The left and right gains for a buffer's volume and pan.
pub fn channel_gains(volume: i32, pan: i32) -> (f32, f32) {
    let left = volume + if pan > 0 { -pan } else { 0 };
    let right = volume + if pan < 0 { pan } else { 0 };
    (gain(left.max(-10000)), gain(right.max(-10000)))
}

/// One buffer as the mixer plays it.
#[derive(Debug, Clone)]
pub struct Voice {
    /// The buffer's samples (a copy kept up to date at each `Unlock`).
    pub data: Vec<u8>,
    pub channels: u16,
    pub bits: u16,
    pub volume: i32,
    pub pan: i32,
    pub looping: bool,
    /// The frame the next output frame reads; `None` when not playing.
    pub position: Option<usize>,
}

impl Voice {
    fn frames(&self) -> usize {
        self.data.len() / (usize::from(self.channels) * usize::from(self.bits) / 8)
    }
}

/// Adds `voices` into `out` (interleaved stereo, `out.len() / 2` frames), moving each playing
/// voice on by one frame per output frame (the exact path: the buffers are at the device's rate).
/// A non-looping voice stops at its end. `voices` come newest first (the measured order of the
/// sum, module documentation).
pub fn render(voices: &mut [&mut Voice], out: &mut [f32]) {
    for v in voices.iter_mut() {
        let Some(mut at) = v.position else { continue };
        let (gl, gr) = channel_gains(v.volume, v.pan);
        let frames = v.frames();
        for frame in out.chunks_mut(2) {
            if at >= frames {
                if v.looping && frames > 0 {
                    at = 0;
                } else {
                    break;
                }
            }
            let (l, r) = if v.channels == 1 {
                let s = sample(&v.data, v.bits, at);
                (s, s)
            } else {
                (
                    sample(&v.data, v.bits, 2 * at),
                    sample(&v.data, v.bits, 2 * at + 1),
                )
            };
            frame[0] += if gl == 1.0 { l } else { l * gl };
            frame[1] += if gr == 1.0 { r } else { r * gr };
            at += 1;
        }
        v.position = match (at >= frames, v.looping) {
            (false, _) => Some(at),
            (true, true) => Some(0),
            (true, false) => None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Measured: the attenuation asked for (millibels) and the gain DirectSound applied (its
    /// `f32` bits), from `dsmix-measure`'s volume sweeps.
    const MEASURED: &[(i32, u32)] = &[
        (-9000, 0x38000080),
        (-8000, 0x38c000c0),
        (-6000, 0x3a840084),
        (-5000, 0x3b5100d1),
        (-3000, 0x3d030083),
        (-2997, 0x3d030083),
        (-2990, 0x3d048084),
        (-2983, 0x3d061086),
        (-2976, 0x3d061086),
        (-2969, 0x3d07a088),
        (-2962, 0x3d093089),
        (-2955, 0x3d093089),
        (-2948, 0x3d0ac08b),
        (-2941, 0x3d0c608c),
        (-2934, 0x3d0e008e),
        (-2927, 0x3d0e008e),
        (-2920, 0x3d0fa090),
        (-2913, 0x3d115091),
        (-2906, 0x3d115091),
        (-2899, 0x3d130093),
        (-2892, 0x3d14b095),
        (-2885, 0x3d14b095),
        (-2878, 0x3d167096),
        (-2871, 0x3d182098),
        (-2864, 0x3d19f09a),
        (-2857, 0x3d19f09a),
        (-2850, 0x3d1bb09c),
        (-2843, 0x3d1d809e),
        (-2836, 0x3d1d809e),
        (-2829, 0x3d1f509f),
        (-2822, 0x3d2130a1),
        (-2815, 0x3d2130a1),
        (-2808, 0x3d2310a3),
        (-2801, 0x3d24f0a5),
        (-2794, 0x3d26d0a7),
        (-2787, 0x3d26d0a7),
        (-2780, 0x3d28c0a9),
        (-2773, 0x3d2ab0ab),
        (-2766, 0x3d2ab0ab),
        (-2759, 0x3d2cb0ad),
        (-2752, 0x3d2eb0af),
        (-2745, 0x3d2eb0af),
        (-2738, 0x3d30c0b1),
        (-2731, 0x3d32c0b3),
        (-2724, 0x3d34d0b5),
        (-2717, 0x3d34d0b5),
        (-2710, 0x3d36f0b7),
        (-2703, 0x3d3910b9),
        (-2696, 0x3d3910b9),
        (-2689, 0x3d3b30bb),
        (-2682, 0x3d3d60bd),
        (-2675, 0x3d3d60bd),
        (-2668, 0x3d3f90c0),
        (-2661, 0x3d41c0c2),
        (-2654, 0x3d4400c4),
        (-2647, 0x3d4400c4),
        (-2640, 0x3d4650c6),
        (-2633, 0x3d4890c9),
        (-2626, 0x3d4890c9),
        (-2619, 0x3d4ae0cb),
        (-2612, 0x3d4d40cd),
        (-2605, 0x3d4d40cd),
        (-2598, 0x3d4fa0d0),
        (-2591, 0x3d5210d2),
        (-2584, 0x3d5480d4),
        (-2577, 0x3d5480d4),
        (-2570, 0x3d56f0d7),
        (-2563, 0x3d5970d9),
        (-2556, 0x3d5970d9),
        (-2549, 0x3d5bf0dc),
        (-2542, 0x3d5e80de),
        (-2535, 0x3d5e80de),
        (-2528, 0x3d6110e1),
        (-2521, 0x3d63b0e4),
        (-2514, 0x3d6650e6),
        (-2507, 0x3d6650e6),
        (-2500, 0x3d6900e9),
        (-2493, 0x3d6bb0ec),
        (-2486, 0x3d6bb0ec),
        (-2479, 0x3d6e60ee),
        (-2472, 0x3d7130f1),
        (-2465, 0x3d7130f1),
        (-2458, 0x3d73f0f4),
        (-2451, 0x3d76c0f7),
        (-2444, 0x3d79a0fa),
        (-2437, 0x3d79a0fa),
        (-2430, 0x3d7c80fc),
        (-2423, 0x3d7f70ff),
        (-2416, 0x3d7f70ff),
        (-2409, 0x3d813881),
        (-2402, 0x3d82b083),
        (-2395, 0x3d82b083),
        (-2388, 0x3d843884),
        (-2381, 0x3d85c086),
        (-2374, 0x3d874887),
        (-2367, 0x3d874887),
        (-2360, 0x3d88e089),
        (-2353, 0x3d8a708a),
        (-2346, 0x3d8a708a),
        (-2339, 0x3d8c108c),
        (-2332, 0x3d8da88e),
        (-2325, 0x3d8da88e),
        (-2318, 0x3d8f508f),
        (-2311, 0x3d90f891),
        (-2304, 0x3d92a893),
        (-2297, 0x3d92a893),
        (-2290, 0x3d945894),
        (-2283, 0x3d961096),
        (-2276, 0x3d961096),
        (-2269, 0x3d97d098),
        (-2262, 0x3d99909a),
        (-2255, 0x3d99909a),
        (-2248, 0x3d9b589b),
        (-2241, 0x3d9d209d),
        (-2234, 0x3d9ef89f),
        (-2227, 0x3d9ef89f),
        (-2220, 0x3da0d0a1),
        (-2213, 0x3da2a8a3),
        (-2206, 0x3da2a8a3),
        (-2199, 0x3da488a5),
        (-2192, 0x3da670a6),
        (-2185, 0x3da670a6),
        (-2178, 0x3da860a8),
        (-2171, 0x3daa58aa),
        (-2164, 0x3dac50ac),
        (-2157, 0x3dac50ac),
        (-2150, 0x3dae50ae),
        (-2143, 0x3db050b0),
        (-2136, 0x3db050b0),
        (-2129, 0x3db258b2),
        (-2122, 0x3db470b4),
        (-2115, 0x3db470b4),
        (-2108, 0x3db680b6),
        (-2101, 0x3db8a0b9),
        (-2094, 0x3dbac0bb),
        (-2087, 0x3dbac0bb),
        (-2080, 0x3dbcf0bd),
        (-2073, 0x3dbf20bf),
        (-2066, 0x3dbf20bf),
        (-2059, 0x3dc150c1),
        (-2052, 0x3dc390c4),
        (-2045, 0x3dc390c4),
        (-2038, 0x3dc5d8c6),
        (-2031, 0x3dc820c8),
        (-2024, 0x3dca70ca),
        (-2017, 0x3dca70ca),
        (-2010, 0x3dccc8cd),
        (-2003, 0x3dcf28cf),
        (-2000, 0x3dcf28cf),
        (-1996, 0x3dcf28cf),
        (-1989, 0x3dd190d2),
        (-1982, 0x3dd3f8d4),
        (-1975, 0x3dd3f8d4),
        (-1968, 0x3dd670d6),
        (-1961, 0x3dd8e8d9),
        (-1954, 0x3ddb70db),
        (-1947, 0x3ddb70db),
        (-1940, 0x3dddf8de),
        (-1933, 0x3de088e1),
        (-1926, 0x3de088e1),
        (-1919, 0x3de328e3),
        (-1912, 0x3de5c8e6),
        (-1905, 0x3de5c8e6),
        (-1898, 0x3de870e8),
        (-1891, 0x3deb20eb),
        (-1884, 0x3dedd8ee),
        (-1877, 0x3dedd8ee),
        (-1870, 0x3df098f1),
        (-1863, 0x3df360f3),
        (-1856, 0x3df360f3),
        (-1849, 0x3df638f6),
        (-1842, 0x3df910f9),
        (-1835, 0x3df910f9),
        (-1828, 0x3dfbf0fc),
        (-1821, 0x3dfee0ff),
        (-1814, 0x3e00e881),
        (-1807, 0x3e00e881),
        (-1800, 0x3e026882),
        (-1793, 0x3e03e884),
        (-1786, 0x3e03e884),
        (-1779, 0x3e057085),
        (-1772, 0x3e06fc87),
        (-1765, 0x3e06fc87),
        (-1758, 0x3e088c89),
        (-1751, 0x3e0a208a),
        (-1744, 0x3e0bb88c),
        (-1737, 0x3e0bb88c),
        (-1730, 0x3e0d588d),
        (-1723, 0x3e0efc8f),
        (-1716, 0x3e0efc8f),
        (-1709, 0x3e10a491),
        (-1702, 0x3e125092),
        (-1695, 0x3e125092),
        (-1688, 0x3e140094),
        (-1681, 0x3e15b896),
        (-1674, 0x3e177497),
        (-1667, 0x3e177497),
        (-1660, 0x3e193499),
        (-1653, 0x3e1afc9b),
        (-1646, 0x3e1afc9b),
        (-1639, 0x3e1cc89d),
        (-1632, 0x3e1e989f),
        (-1625, 0x3e1e989f),
        (-1618, 0x3e206ca0),
        (-1611, 0x3e2248a2),
        (-1604, 0x3e242ca4),
        (-1597, 0x3e242ca4),
        (-1590, 0x3e2610a6),
        (-1583, 0x3e27fca8),
        (-1576, 0x3e27fca8),
        (-1569, 0x3e29f0aa),
        (-1562, 0x3e2be8ac),
        (-1555, 0x3e2be8ac),
        (-1548, 0x3e2de4ae),
        (-1541, 0x3e2fe8b0),
        (-1534, 0x3e31f0b2),
        (-1527, 0x3e31f0b2),
        (-1520, 0x3e3400b4),
        (-1513, 0x3e3618b6),
        (-1506, 0x3e3618b6),
        (-1499, 0x3e3834b8),
        (-1492, 0x3e3a54ba),
        (-1485, 0x3e3a54ba),
        (-1478, 0x3e3c7cbc),
        (-1471, 0x3e3eacbf),
        (-1464, 0x3e40e0c1),
        (-1457, 0x3e40e0c1),
        (-1450, 0x3e431cc3),
        (-1443, 0x3e4560c5),
        (-1436, 0x3e4560c5),
        (-1429, 0x3e47a8c8),
        (-1422, 0x3e49f8ca),
        (-1415, 0x3e49f8ca),
        (-1408, 0x3e4c50cc),
        (-1401, 0x3e4eaccf),
        (-1394, 0x3e5110d1),
        (-1387, 0x3e5110d1),
        (-1380, 0x3e537cd3),
        (-1373, 0x3e55f0d6),
        (-1366, 0x3e55f0d6),
        (-1359, 0x3e5868d8),
        (-1352, 0x3e5aecdb),
        (-1345, 0x3e5aecdb),
        (-1338, 0x3e5d74dd),
        (-1331, 0x3e6004e0),
        (-1324, 0x3e629ce3),
        (-1317, 0x3e629ce3),
        (-1310, 0x3e653ce5),
        (-1303, 0x3e67e4e8),
        (-1296, 0x3e67e4e8),
        (-1289, 0x3e6a94eb),
        (-1282, 0x3e6d4ced),
        (-1275, 0x3e6d4ced),
        (-1268, 0x3e700cf0),
        (-1261, 0x3e72d4f3),
        (-1254, 0x3e75a0f6),
        (-1247, 0x3e75a0f6),
        (-1240, 0x3e787cf8),
        (-1234, 0x3e7b5cfb),
        (-1233, 0x3e7b5cfb),
        (-1226, 0x3e7b5cfb),
        (-1219, 0x3e7e44fe),
        (-1212, 0x3e809a81),
        (-1205, 0x3e809a81),
        (-1198, 0x3e821882),
        (-1191, 0x3e839a84),
        (-1184, 0x3e852085),
        (-1177, 0x3e852085),
        (-1170, 0x3e86aa87),
        (-1163, 0x3e883a88),
        (-1156, 0x3e883a88),
        (-1149, 0x3e89ce8a),
        (-1142, 0x3e8b668b),
        (-1135, 0x3e8b668b),
        (-1128, 0x3e8d048d),
        (-1121, 0x3e8ea68f),
        (-1114, 0x3e904c90),
        (-1107, 0x3e904c90),
        (-1100, 0x3e91f892),
        (-1093, 0x3e93a894),
        (-1086, 0x3e93a894),
        (-1079, 0x3e955e95),
        (-1072, 0x3e971a97),
        (-1065, 0x3e971a97),
        (-1058, 0x3e98da99),
        (-1051, 0x3e9a9e9b),
        (-1044, 0x3e9c689c),
        (-1037, 0x3e9c689c),
        (-1030, 0x3e9e389e),
        (-1023, 0x3ea00ea0),
        (-1016, 0x3ea00ea0),
        (-1009, 0x3ea1e8a2),
        (-1002, 0x3ea3c8a4),
        (-1000, 0x3ea3c8a4),
        (-995, 0x3ea3c8a4),
        (-988, 0x3ea5aea6),
        (-981, 0x3ea798a8),
        (-974, 0x3ea98aaa),
        (-967, 0x3ea98aaa),
        (-960, 0x3eab80ac),
        (-953, 0x3ead7cad),
        (-946, 0x3ead7cad),
        (-939, 0x3eaf7eaf),
        (-932, 0x3eb186b2),
        (-925, 0x3eb186b2),
        (-918, 0x3eb394b4),
        (-911, 0x3eb5aab6),
        (-904, 0x3eb7c4b8),
        (-897, 0x3eb7c4b8),
        (-890, 0x3eb9e4ba),
        (-883, 0x3ebc0cbc),
        (-876, 0x3ebc0cbc),
        (-869, 0x3ebe38be),
        (-862, 0x3ec06cc0),
        (-855, 0x3ec06cc0),
        (-848, 0x3ec2a8c3),
        (-841, 0x3ec4e8c5),
        (-834, 0x3ec730c7),
        (-827, 0x3ec730c7),
        (-820, 0x3ec97ec9),
        (-813, 0x3ecbd4cc),
        (-806, 0x3ecbd4cc),
        (-799, 0x3ece30ce),
        (-792, 0x3ed094d1),
        (-785, 0x3ed094d1),
        (-778, 0x3ed2fed3),
        (-771, 0x3ed56ed5),
        (-764, 0x3ed7e8d8),
        (-757, 0x3ed7e8d8),
        (-750, 0x3eda68da),
        (-743, 0x3edcf0dd),
        (-736, 0x3edcf0dd),
        (-729, 0x3edf7edf),
        (-722, 0x3ee214e2),
        (-715, 0x3ee214e2),
        (-708, 0x3ee4b2e5),
        (-701, 0x3ee758e7),
        (-694, 0x3eea06ea),
        (-687, 0x3eea06ea),
        (-680, 0x3eecbced),
        (-673, 0x3eef7aef),
        (-666, 0x3eef7aef),
        (-659, 0x3ef240f2),
        (-652, 0x3ef50ef5),
        (-645, 0x3ef50ef5),
        (-638, 0x3ef7e4f8),
        (-631, 0x3efac4fb),
        (-624, 0x3efdaafe),
        (-617, 0x3efdaafe),
        (-610, 0x3f004d80),
        (-603, 0x3f01ca82),
        (-600, 0x3f01ca82),
        (-596, 0x3f01ca82),
        (-589, 0x3f034a83),
        (-582, 0x3f04d085),
        (-575, 0x3f04d085),
        (-568, 0x3f065986),
        (-561, 0x3f07e888),
        (-555, 0x3f07e888),
        (-554, 0x3f097a89),
        (-547, 0x3f097a89),
        (-540, 0x3f0b128b),
        (-533, 0x3f0cae8d),
        (-526, 0x3f0cae8d),
        (-519, 0x3f0e4f8e),
        (-512, 0x3f0ff590),
        (-505, 0x3f0ff590),
        (-498, 0x3f11a092),
        (-491, 0x3f135093),
        (-484, 0x3f150495),
        (-477, 0x3f150495),
        (-470, 0x3f16be97),
        (-463, 0x3f187d98),
        (-456, 0x3f187d98),
        (-449, 0x3f1a419a),
        (-442, 0x3f1c0a9c),
        (-435, 0x3f1c0a9c),
        (-428, 0x3f1dd99e),
        (-421, 0x3f1fada0),
        (-414, 0x3f2186a2),
        (-407, 0x3f2186a2),
        (-400, 0x3f2365a3),
        (-393, 0x3f2549a5),
        (-386, 0x3f2549a5),
        (-379, 0x3f2733a7),
        (-372, 0x3f2923a9),
        (-365, 0x3f2923a9),
        (-358, 0x3f2b18ab),
        (-351, 0x3f2d13ad),
        (-344, 0x3f2f14af),
        (-337, 0x3f2f14af),
        (-330, 0x3f311bb1),
        (-323, 0x3f3328b3),
        (-316, 0x3f3328b3),
        (-309, 0x3f353bb5),
        (-302, 0x3f3755b7),
        (-295, 0x3f3755b7),
        (-288, 0x3f3974b9),
        (-281, 0x3f3b9abc),
        (-274, 0x3f3dc6be),
        (-267, 0x3f3dc6be),
        (-260, 0x3f3ff9c0),
        (-253, 0x3f4232c2),
        (-246, 0x3f4232c2),
        (-239, 0x3f4471c4),
        (-232, 0x3f46b8c7),
        (-225, 0x3f46b8c7),
        (-218, 0x3f4905c9),
        (-211, 0x3f4b59cb),
        (-204, 0x3f4db3ce),
        (-200, 0x3f4db3ce),
        (-199, 0x3f4db3ce),
        (-197, 0x3f4db3ce),
        (-190, 0x3f5015d0),
        (-183, 0x3f527ed2),
        (-176, 0x3f527ed2),
        (-169, 0x3f54eed5),
        (-162, 0x3f5765d7),
        (-155, 0x3f5765d7),
        (-150, 0x3f59e4da),
        (-148, 0x3f59e4da),
        (-141, 0x3f5c6adc),
        (-134, 0x3f5ef7df),
        (-127, 0x3f5ef7df),
        (-120, 0x3f618ce2),
        (-113, 0x3f6429e4),
        (-106, 0x3f6429e4),
        (-101, 0x3f66cde7),
        (-100, 0x3f66cde7),
        (-99, 0x3f66cde7),
        (-92, 0x3f6979e9),
        (-85, 0x3f6979e9),
        (-78, 0x3f6c2dec),
        (-71, 0x3f6ee9ef),
        (-64, 0x3f71adf2),
        (-57, 0x3f71adf2),
        (-51, 0x3f747af4),
        (-50, 0x3f747af4),
        (-49, 0x3f747af4),
        (-43, 0x3f774ff7),
        (-36, 0x3f774ff7),
        (-29, 0x3f7a2cfa),
        (-22, 0x3f7d11fd),
        (-15, 0x3f7d11fd),
        (-11, 0x3f7d11fd),
        (-10, 0x3f7d11fd),
        (-9, 0x3f7d11fd),
        (-8, 0x3f7d11fd),
        (-5, 0x3f7d11fd),
        (-1, 0x3f7d11fd),
    ];

    #[test]
    fn the_gain_law_gives_the_measured_factors() {
        for &(mb, bits) in MEASURED {
            assert_eq!(gain(mb).to_bits(), bits, "{mb} mB");
        }
        assert_eq!(gain(0), 1.0);
        assert_eq!(gain(-10000), 0.0);
    }

    #[test]
    fn pan_attenuates_the_far_channel_only() {
        let (l, r) = channel_gains(0, -1000);
        assert_eq!((l, r.to_bits()), (1.0, 0x3ea3c8a4));
        let (l, r) = channel_gains(-300, -700);
        assert_eq!(
            (l.to_bits(), r.to_bits()),
            (gain(-300).to_bits(), 0x3ea3c8a4)
        );
    }

    #[test]
    fn samples_and_voices_mix_as_measured() {
        let data: Vec<u8> = [1000i16, -1000, 32767, -32768]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let mut a = Voice {
            data,
            channels: 1,
            bits: 16,
            volume: -600,
            pan: 0,
            looping: false,
            position: Some(0),
        };
        let mut b = Voice {
            data: vec![128, 255],
            channels: 1,
            bits: 8,
            volume: 0,
            pan: 0,
            looping: true,
            position: Some(0),
        };
        let mut out = vec![0f32; 2 * 6];
        render(&mut [&mut a, &mut b], &mut out);
        let g = gain(-600);
        let x = |v: f32| v / 32768.0;
        assert_eq!(out[0], x(1000.0) * g + 0.0);
        assert_eq!(out[2], x(-1000.0) * g + x(127.0 * 256.0));
        assert_eq!(out[1], out[0]);
        // The 16-bit voice ended after 4 frames; the looping one goes round.
        assert_eq!(out[8], x(0.0) + 0.0);
        assert_eq!(out[10], x(127.0 * 256.0));
        assert_eq!(a.position, None);
        assert_eq!(b.position, Some(0));
    }
}
