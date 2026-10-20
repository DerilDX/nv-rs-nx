//! `(unplaced)/run_00a29580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Loads the base part of the same object (`00a3da20`, `RET 4`: `this`,
/// stream).
const LOAD_BASE: u32 = 0x00a3_da20;
/// `NiColorA::LoadBinary` (engine map), `00a69800` (`this`, stream).
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;

/// Reads `size` bytes from the stream's input through the read function
/// pointer at +8 of the stream object at `NiStream` +0x24C (`cdecl`:
/// input, buffer, size, &count, 1) and adds its result to the input's
/// counter at +4. `count` is the exe's local, set to `size` first.
fn stream_read(e: &mut Engine, stream: Ptr, buffer: u32, size: u32) {
    // NiStream +0x24C: the stream's input object (read function at +8,
    // bytes-read counter at +4).
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        let n = e.call(read, &args![input, buffer, size, count, 1u32]).u32();
        let total = e.mem.u32(input + 4).wrapping_add(n);
        e.mem.set_u32(input + 4, total);
    });
}

// Translated from 00a29580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a 4-byte value and a flag byte from the stream (no Xbox PDB name):
/// four bytes into `this` +0, then one byte into the exe's argument slot,
/// and `this` +4 becomes whether that byte is nonzero.
pub fn fn_00a29580(e: &mut Engine, this: Ptr, stream: Ptr) {
    stream_read(e, stream, this.addr(), 4);
    e.with_stack(4, |e, flag| {
        e.mem.set_u32(flag.addr(), stream.addr());
        stream_read(e, stream, flag.addr(), 1);
        let set = e.mem.u8(flag.addr()) != 0;
        e.mem.set_u8(this.addr() + 4, set as u8);
    });
}

// Translated from 00a29640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads an object holding an `NiColorA` at +4 (no Xbox PDB name):
/// `00a3da20` on `this`, then `NiColorA::LoadBinary` (`00a69800`) on
/// `this` +4, both with the stream.
pub fn fn_00a29640(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BASE, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + 4, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a29580, fn_00a29580(Ptr, Ptr)),
        entry!(0x00a29640, fn_00a29640(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 0x00de_ad00;

    /// A stream whose input object reads from `bytes` through a double at
    /// `READ`, recording each request.
    fn stream(e: &mut Engine, bytes: Vec<u8>) -> Ptr {
        let stream = Ptr::new(e.mem.alloc(0x250));
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream.addr() + 0x24c, input);
        e.mem.set_u32(input + 8, READ);
        e.mem.set_u32(input + 4, 7);
        let mut at = 0usize;
        e.register_double(READ, move |e, a| {
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), a[2]);
            let n = a[2] as usize;
            e.mem.write(a[1], &bytes[at..at + n]);
            at += n;
            Ret {
                eax: n as u32,
                ..Ret::default()
            }
        });
        stream
    }

    #[test]
    fn load_reads_the_value_then_the_flag() {
        let mut e = Engine::new();
        let s = stream(&mut e, vec![0x78, 0x56, 0x34, 0x12, 0x02, 0, 0, 0, 0, 0]);
        let this: Ptr = Ptr::new(e.mem.alloc(8));
        e.call(0x00a2_9580, &args![this, s]);
        assert_eq!(e.mem.u32(this.addr()), 0x1234_5678);
        assert_eq!(e.mem.u8(this.addr() + 4), 1);
        let input = e.mem.u32(s.addr() + 0x24c);
        assert_eq!(e.mem.u32(input + 4), 7 + 5);
        e.call(0x00a2_9580, &args![this, s]);
        assert_eq!(e.mem.u32(this.addr()), 0);
        assert_eq!(e.mem.u8(this.addr() + 4), 0);
    }

    #[test]
    fn color_holder_loads_its_base_then_the_color() {
        let mut e = Engine::new();
        e.register(LOAD_BASE, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_9640, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == LOAD_BASE || *a == COLOR_LOAD_BINARY)
            .cloned()
            .collect();
        assert_eq!(
            calls,
            vec![
                (LOAD_BASE, vec![0x1000, 0x2000]),
                (COLOR_LOAD_BINARY, vec![0x1004, 0x2000]),
            ]
        );
    }
}
