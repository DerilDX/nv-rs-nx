//! `Havok SDK/hkptransformagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkTransform::setMul` (Xbox PDB): `this` is the 64-byte result, then the two operands.
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;

// Translated from 00d431b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMotionState::hkMotionState` (Xbox PDB), the copy constructor: copies the 0xAF bytes of
/// `source` into `this`. The first 0x40 bytes are read as a block before any is written (a
/// 16-byte vector copy through the stack); +0x40..+0x90 are copied word by word; the three
/// `float`s at +0xA0, +0xA4 and +0xA8 go through the x87 stack (signalling NaNs come back
/// quiet) and the bytes at +0xAC, +0xAD and +0xAE are copied last. Returns `this`.
pub fn hkp_motion_state_copy_constructor(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let from = source.addr();
    let to = this.addr();
    let head = e.mem.bytes(from, 0x40);
    e.mem.write(to, &head);
    let body = e.mem.bytes(from + 0x40, 0x50);
    e.mem.write(to + 0x40, &body);
    for offset in [0x90, 0x94, 0x98, 0x9c] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0xa0, 0xa4, 0xa8] {
        let value = x87_quiet(e.mem.f32(from + offset));
        e.mem.set_f32(to + offset, value);
    }
    for offset in [0xac, 0xad, 0xae] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    this
}

// Translated from 00d42cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformAgent::linearCast` (Xbox PDB). `entry` is the collision entry (`bodyA`,
/// the body at +0, an `hkTransform` pointer at +8): the transform of the entry is multiplied
/// by the transform at `body + 0x30` into a 64-byte local; then the agent held at `this + 0xc`
/// gets its virtual method at slot 0x14 called with a 16-byte record on the stack
/// (`word at body + 0x14`, 0, the local transform, `entry`) followed by the four remaining
/// words.
pub fn hkp_transform_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    let body = e.mem.u32(entry.addr());
    e.with_stack(0x50, |e, frame| {
        let record = frame.addr();
        let transform = frame.addr() + 0x10;
        let entry_transform = e.mem.u32(entry.addr() + 8);
        e.call(
            TRANSFORM_SET_MUL,
            &args![transform, entry_transform, body + 0x30],
        );
        let first = e.mem.u32(body + 0x14);
        e.mem.set_u32(record, first);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, transform);
        e.mem.set_u32(record + 12, entry.addr());
        let agent = e.mem.u32(this.addr() + 0xc);
        e.vcall(agent, 0x14, &args![record, a, b, c, d]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d431b0,
            hkp_motion_state_copy_constructor(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d42cf0,
            hkp_transform_agent_linear_cast(Ptr, Ptr, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const LINEAR_CAST_SLOT: u32 = 0x0900_0014;

    #[test]
    fn motion_state_copy_copies_every_field_and_returns_this() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0xb0);
        for offset in 0..0xaf {
            e.mem.set_u8(
                source + offset,
                (offset as u8).wrapping_mul(3).wrapping_add(1),
            );
        }
        // A signalling NaN at +0xa4 comes back quiet; the other floats are unchanged.
        e.mem.set_u32(source + 0xa4, 0x7f80_0001);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u8(this + 0xaf, 0x5a);
        let result = e.call(0x00d431b0, &args![this, source]).u32();
        assert_eq!(result, this);
        for offset in (0..0xaf).filter(|o| !(0xa4..0xa8).contains(o)) {
            assert_eq!(
                e.mem.u8(this + offset),
                e.mem.u8(source + offset),
                "{offset:#x}"
            );
        }
        assert_eq!(e.mem.u32(this + 0xa4), 0x7fc0_0001);
        // The byte after the last copied one is not touched.
        assert_eq!(e.mem.u8(this + 0xaf), 0x5a);
    }

    #[test]
    fn linear_cast_hands_the_agent_the_combined_transform() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(TRANSFORM_SET_MUL, move |e, a| {
            // The product is recorded as the sum of the two operand markers.
            let sum = e.mem.u32(a[1]) + e.mem.u32(a[2]);
            e.mem.set_u32(a[0], sum);
            sink.borrow_mut().push(vec![a[0], a[1], a[2]]);
            Ret::default()
        });
        let record_log = Rc::new(RefCell::new(vec![]));
        let sink = record_log.clone();
        e.register_double(LINEAR_CAST_SLOT, move |e, a| {
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            let combined = e.mem.u32(record[2]);
            sink.borrow_mut().push((a.to_vec(), record, combined));
            Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x14, LINEAR_CAST_SLOT);
        let agent = e.mem.alloc(8);
        e.mem.set_u32(agent, table);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, agent);
        let body = e.mem.alloc(0x80);
        e.mem.set_u32(body + 0x14, 0x777);
        e.mem.set_u32(body + 0x30, 30);
        let entry_transform = e.mem.alloc(0x40);
        e.mem.set_u32(entry_transform, 12);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, body);
        e.mem.set_u32(entry + 8, entry_transform);
        e.call(0x00d42cf0, &args![this, entry, 1u32, 2u32, 3u32, 4u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][1..], [entry_transform, body + 0x30]);
        let logs = record_log.borrow();
        assert_eq!(logs.len(), 1);
        let (words, record, combined) = &logs[0];
        assert_eq!(words[0], agent);
        assert_eq!(words[2..], [1, 2, 3, 4]);
        assert_eq!(record[0], 0x777);
        assert_eq!(record[1], 0);
        assert_eq!(record[2], seen[0][0]);
        assert_eq!(record[3], entry);
        assert_eq!(*combined, 42);
    }
}
