//! `fallout/ai/combat/combatactions.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0097b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006815c0` of unit run_006815c0, which answers
/// `this` itself, and returns its result (a pointer). Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. The return
/// type (`Ptr`) is that of `006815c0`'s translated signature `fn_006815c0(Ptr) -> Ptr` (its
/// doc: `mov eax, ecx`).
pub fn fn_0097b940(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this]).ptr()
}

// Translated from 0097c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00979c80`, a destructor with an exception frame
/// that stores the vtable `0108cab8` and runs four list-clearing loops, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00979c80`'s disassembly ends with its exception-frame unlink and
/// returns no value (Ghidra types it void).
pub fn fn_0097c470(e: &mut Engine, this: Ptr) {
    e.call(0x0097_9c80, &args![this]);
}

// The `CombatAction` subclasses' constructors below all have one shape (read
// from each one's disassembly): the base constructor with the word the
// factory passes (`00979bd0`, which keeps it at +0x24; or the constructor of
// the subclass they derive from), the class's own vtable (each named by its
// RTTI locator), then pairs of words added to the action's lists: `0097ade0`
// (list +0x04), `00984390` (+0x0C), `0097adc0` (+0x14) and, through
// `0097bc60`, the list at +0x1C (all through `009aa2b0`); `0097c260`,
// `0097c070` and `0097c240` remove a word from the lists at +0x04, +0x0C and
// +0x14 (`009aa340`). What the words mean is not confirmed from the exe.

/// Adds the pair `(a, b)` to the list at +0x1C (`009aa2b0`, `thiscall(a,
/// b)`).
const LIST_ADD: u32 = 0x009a_a2b0;
/// Removes the entry holding `value` from a list (`009aa340`,
/// `thiscall(value)`).
const LIST_REMOVE: u32 = 0x009a_a340;

// Translated from 0097ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at +0xBC of `this` (unnamed in the engine map; the
/// decompiler's library match names it `SchedulerBase::GetIScheduler`,
/// which is only the shared shape).
pub fn fn_0097ae90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xbc)
}

// Translated from 0097bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `(a, b)` to the list at +0x1C (`009aa2b0`; `thiscall`, `RET 8`).
pub fn fn_0097bc60(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(LIST_ADD, &args![this.addr() + 0x1c, a, b]);
}

// Translated from 0097c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `value` from the list at +0x0C (`009aa340`; `thiscall`, `RET 4`).
pub fn fn_0097c070(e: &mut Engine, this: Ptr, value: u32) {
    e.call(LIST_REMOVE, &args![this.addr() + 0xc, value]);
}

// Translated from 0097c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `value` from the list at +0x14 (`009aa340`; `thiscall`, `RET 4`).
pub fn fn_0097c240(e: &mut Engine, this: Ptr, value: u32) {
    e.call(LIST_REMOVE, &args![this.addr() + 0x14, value]);
}

// Translated from 0097c260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `value` from the list at +0x04 (`009aa340`; `thiscall`, `RET 4`).
pub fn fn_0097c260(e: &mut Engine, this: Ptr, value: u32) {
    e.call(LIST_REMOVE, &args![this.addr() + 0x4, value]);
}

// Translated from 0097aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097aeb0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRangedFromCover` constructor (by the vtable it sets, `0108cb7c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097aeb0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cb7c);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x4u32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x2u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b0b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRangedExplosive` constructor (by the vtable it sets, `0108cb94`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b0b0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cb94);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x5u32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0x6u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b280 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRangedExplosiveFromCover` constructor (by the vtable it sets, `0108cbac`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b280(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbac);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x4u32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x2u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b350 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackMelee` constructor (by the vtable it sets, `0108cbc4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b350(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbc4);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x12u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x2u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x2u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x1u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b4e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackHandToHand` constructor (by the vtable it sets, `0108cbdc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b4e0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbdc);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x12u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x6u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x6u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x1u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b620 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenade` constructor (by the vtable it sets, `0108cbf4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b620(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbf4);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x5u32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x3u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b7a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenadeFlushTarget` constructor (by the vtable it sets, `0108cc0c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b7a0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc0c);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x3u32]);
    e.call(0x0097_adc0, &args![this, 0x9u32, 0x1u32]);
    this
}

// Translated from 0097b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b9e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenadeFromCover` constructor (by the vtable it sets, `0108cc24`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b9e0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc24);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x4u32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x2u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097bbd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMove` constructor (by the vtable it sets, `0108cc3c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097bbd0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc3c);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0xffff_0018u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097bfe0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndSwitchToMelee` constructor (by the vtable it sets, `0108cc54`): the base
/// constructor `0097bbd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097bfe0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    fn_0097bbd0(e, this, kind);
    e.mem.set_u32(this.addr(), 0x0108_cc54);
    fn_0097c070(e, this, 0xau32);
    e.call(0x0097_ade0, &args![this, 0xcu32, 0x2u32]);
    fn_0097bc60(e, this, 0xbu32, 0x2u32);
    fn_0097bc60(e, this, 0xau32, 0x2u32);
    this
}

// Translated from 0097c170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c170 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackRanged` constructor (by the vtable it sets, `0108cc6c`): the base
/// constructor `0097ad10` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c170(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_ad10, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc6c);
    fn_0097c260(e, this, 0x3u32);
    fn_0097c260(e, this, 0x12u32);
    fn_0097c070(e, this, 0x1u32);
    fn_0097c070(e, this, 0x6u32);
    fn_0097c070(e, this, 0x5u32);
    fn_0097c240(e, this, 0x0u32);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c3a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackRangedExplosive` constructor (by the vtable it sets, `0108cc84`): the base
/// constructor `0097b0b0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c3a0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    fn_0097b0b0(e, this, kind);
    e.mem.set_u32(this.addr(), 0x0108_cc84);
    fn_0097c260(e, this, 0x3u32);
    fn_0097c260(e, this, 0x12u32);
    fn_0097c070(e, this, 0x1u32);
    fn_0097c070(e, this, 0x6u32);
    fn_0097c070(e, this, 0x5u32);
    fn_0097c240(e, this, 0x0u32);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097c560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c560 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackGrenade` constructor (by the vtable it sets, `0108cc9c`): the base
/// constructor `0097b620` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c560(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    fn_0097b620(e, this, kind);
    e.mem.set_u32(this.addr(), 0x0108_cc9c);
    fn_0097c260(e, this, 0x3u32);
    fn_0097c070(e, this, 0x1u32);
    fn_0097c070(e, this, 0x6u32);
    fn_0097c070(e, this, 0x5u32);
    fn_0097c240(e, this, 0x0u32);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097c700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c700 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionDrawWeapon` constructor (by the vtable it sets, `0108ccb4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c700(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_ccb4);
    e.call(0x0097_ade0, &args![this, 0xcu32, u32::MAX]);
    e.call(0x0097_ade0, &args![this, 0xau32, 0x7u32]);
    e.call(0x0097_adc0, &args![this, 0xbu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0xau32, u32::MAX]);
    this
}

// Translated from 0097c850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c850 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionSwitchWeapon` constructor (by the vtable it sets, `0108cccc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c850(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cccc);
    e.call(0x0097_ade0, &args![this, 0xcu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0xbu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0xau32, u32::MAX]);
    this
}

// Translated from 0097c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c9d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAvoidThreat` constructor (by the vtable it sets, `0108cce4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c9d0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cce4);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xeu32]);
    e.call(0x0097_adc0, &args![this, 0xeu32, 0x0u32]);
    this
}

// Translated from 0097ca60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097ca60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionSearch` constructor (by the vtable it sets, `0108ccfc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097ca60(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_ccfc);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0xffff_0018u32]);
    e.call(0x0097_adc0, &args![this, 0x3u32, 0x0u32]);
    this
}

// Translated from 0097cb70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cb70 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionInvestigate` constructor (by the vtable it sets, `0108cd14`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cb70(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd14);
    e.call(0x0098_4390, &args![this, 0xau32, 0xffff_0018u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xfu32]);
    e.call(0x0097_adc0, &args![this, 0x3u32, 0x0u32]);
    this
}

// Translated from 0097cc00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cc00 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionDodge` constructor (by the vtable it sets, `0108cd2c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cc00(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd2c);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x3u32]);
    e.call(0x0097_adc0, &args![this, 0x6u32, 0x0u32]);
    this
}

// Translated from 0097cc80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cc80 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionIgnoreBlockedTarget` constructor (by the vtable it sets, `0108cd44`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cc80(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd44);
    e.call(0x0097_adc0, &args![this, 0x6u32, 0x0u32]);
    this
}

// Translated from 0097cd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cd30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionFlee` constructor (by the vtable it sets, `0108cd5c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cd30(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd5c);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xcu32]);
    e.call(0x0097_adc0, &args![this, 0xfu32, 0x0u32]);
    this
}

// Translated from 0097cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cfe0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionUseCombatItem` constructor (by the vtable it sets, `0108cd8c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cfe0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd8c);
    e.call(0x0098_4390, &args![this, 0xdu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0x10u32, u32::MAX]);
    this
}

// Translated from 0097d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d1c0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAcquireLineOfSight` constructor (by the vtable it sets, `0108cdbc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097d1c0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cdbc);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xdu32]);
    e.call(0x0097_adc0, &args![this, 0x5u32, 0x1u32]);
    this
}

// Translated from 0097d250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d250 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionHide` constructor (by the vtable it sets, `0108cdd4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097d250(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cdd4);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xau32]);
    e.call(0x0097_adc0, &args![this, 0x11u32, u32::MAX]);
    this
}

// Translated from 0097d130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d130 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionApproachTarget` constructor (by the vtable it sets, `0108cda4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097d130(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cda4);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x6u32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x5u32, 0x1u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_b940, fn_0097b940(Ptr) -> Ptr),
        entry!(0x0097_c470, fn_0097c470(Ptr)),
        entry!(0x0097_ae90, fn_0097ae90(Ptr) -> u32),
        entry!(0x0097_bc60, fn_0097bc60(Ptr, u32, u32)),
        entry!(0x0097_c070, fn_0097c070(Ptr, u32)),
        entry!(0x0097_c240, fn_0097c240(Ptr, u32)),
        entry!(0x0097_c260, fn_0097c260(Ptr, u32)),
        entry!(0x0097_aeb0, fn_0097aeb0(Ptr, u32) -> Ptr),
        entry!(0x0097_b0b0, fn_0097b0b0(Ptr, u32) -> Ptr),
        entry!(0x0097_b280, fn_0097b280(Ptr, u32) -> Ptr),
        entry!(0x0097_b350, fn_0097b350(Ptr, u32) -> Ptr),
        entry!(0x0097_b4e0, fn_0097b4e0(Ptr, u32) -> Ptr),
        entry!(0x0097_b620, fn_0097b620(Ptr, u32) -> Ptr),
        entry!(0x0097_b7a0, fn_0097b7a0(Ptr, u32) -> Ptr),
        entry!(0x0097_b9e0, fn_0097b9e0(Ptr, u32) -> Ptr),
        entry!(0x0097_bbd0, fn_0097bbd0(Ptr, u32) -> Ptr),
        entry!(0x0097_bfe0, fn_0097bfe0(Ptr, u32) -> Ptr),
        entry!(0x0097_c170, fn_0097c170(Ptr, u32) -> Ptr),
        entry!(0x0097_c3a0, fn_0097c3a0(Ptr, u32) -> Ptr),
        entry!(0x0097_c560, fn_0097c560(Ptr, u32) -> Ptr),
        entry!(0x0097_c700, fn_0097c700(Ptr, u32) -> Ptr),
        entry!(0x0097_c850, fn_0097c850(Ptr, u32) -> Ptr),
        entry!(0x0097_c9d0, fn_0097c9d0(Ptr, u32) -> Ptr),
        entry!(0x0097_ca60, fn_0097ca60(Ptr, u32) -> Ptr),
        entry!(0x0097_cb70, fn_0097cb70(Ptr, u32) -> Ptr),
        entry!(0x0097_cc00, fn_0097cc00(Ptr, u32) -> Ptr),
        entry!(0x0097_cc80, fn_0097cc80(Ptr, u32) -> Ptr),
        entry!(0x0097_cd30, fn_0097cd30(Ptr, u32) -> Ptr),
        entry!(0x0097_cfe0, fn_0097cfe0(Ptr, u32) -> Ptr),
        entry!(0x0097_d1c0, fn_0097d1c0(Ptr, u32) -> Ptr),
        entry!(0x0097_d250, fn_0097d250(Ptr, u32) -> Ptr),
        entry!(0x0097_d130, fn_0097d130(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_0097b940_returns_the_callees_pointer() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_b940, &args![0x0000_5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, 0x0068_15c0), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_0097c470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0097_9c80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0097_c470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0097_9c80), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;
    type Case = (u32, u32, Calls);

    const DOUBLED: [u32; 6] = [
        0x0097_9bd0,
        0x0097_ad10,
        0x0097_ade0,
        0x0097_adc0,
        0x0098_4390,
        LIST_ADD,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in DOUBLED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e
    }

    #[test]
    fn small_helpers() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0xbc, 0x1234);
        assert_eq!(e.call(0x0097_ae90, &args![this]).u32(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0097_bc60, &args![this, 3u32, 4u32]);
        e.call(0x0097_c070, &args![this, 5u32]);
        e.call(0x0097_c240, &args![this, 6u32]);
        e.call(0x0097_c260, &args![this, 7u32]);
        let log: Calls = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == LIST_ADD || *a == LIST_REMOVE)
            .collect();
        assert_eq!(
            log,
            vec![
                (LIST_ADD, vec![this + 0x1c, 3, 4]),
                (LIST_REMOVE, vec![this + 0xc, 5]),
                (LIST_REMOVE, vec![this + 0x14, 6]),
                (LIST_REMOVE, vec![this + 0x4, 7]),
            ]
        );
    }

    /// Every constructor, its vtable and the calls it makes (helpers of this
    /// file show as the list calls they make; a base constructor of this
    /// file shows as its own calls).
    fn cases(t: u32, k: u32) -> Vec<Case> {
        vec![
            (0x0097_aeb0, 0x0108_cb7c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x4, 0x1]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x1]),
                    (0x0098_4390, vec![t, 0xa, 0x1]),
                    (0x0098_4390, vec![t, 0x1, 0x2]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b0b0, 0x0108_cb94, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0x6, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x0]),
                    (0x0098_4390, vec![t, 0xa, 0x0]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b280, 0x0108_cbac, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x4, 0x1]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x0]),
                    (0x0098_4390, vec![t, 0xa, 0x0]),
                    (0x0098_4390, vec![t, 0x1, 0x2]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b350, 0x0108_cbc4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x12]),
                    (0x0098_4390, vec![t, 0xb, 0x2]),
                    (0x0098_4390, vec![t, 0xa, 0x2]),
                    (0x0098_4390, vec![t, 0x1, 0x1]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b4e0, 0x0108_cbdc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x12]),
                    (0x0098_4390, vec![t, 0xb, 0x6]),
                    (0x0098_4390, vec![t, 0xa, 0x6]),
                    (0x0098_4390, vec![t, 0x1, 0x1]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b620, 0x0108_cbf4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b7a0, 0x0108_cc0c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0097_adc0, vec![t, 0x9, 0x1]),
                ]
            }),
            (0x0097_b9e0, 0x0108_cc24, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x4, 0x1]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0098_4390, vec![t, 0x1, 0x2]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_bbd0, 0x0108_cc3c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_bfe0, 0x0108_cc54, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                    (0x009a_a340, vec![t + 0xc, 0xa]),
                    (0x0097_ade0, vec![t, 0xc, 0x2]),
                    (0x009a_a2b0, vec![t + 0x1c, 0xb, 0x2]),
                    (0x009a_a2b0, vec![t + 0x1c, 0xa, 0x2]),
                ]
            }),
            (0x0097_c170, 0x0108_cc6c, {
                vec![
                    (0x0097_ad10, vec![t, k]),
                    (0x009a_a340, vec![t + 0x4, 0x3]),
                    (0x009a_a340, vec![t + 0x4, 0x12]),
                    (0x009a_a340, vec![t + 0xc, 0x1]),
                    (0x009a_a340, vec![t + 0xc, 0x6]),
                    (0x009a_a340, vec![t + 0xc, 0x5]),
                    (0x009a_a340, vec![t + 0x14, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_c3a0, 0x0108_cc84, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0x6, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x0]),
                    (0x0098_4390, vec![t, 0xa, 0x0]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                    (0x009a_a340, vec![t + 0x4, 0x3]),
                    (0x009a_a340, vec![t + 0x4, 0x12]),
                    (0x009a_a340, vec![t + 0xc, 0x1]),
                    (0x009a_a340, vec![t + 0xc, 0x6]),
                    (0x009a_a340, vec![t + 0xc, 0x5]),
                    (0x009a_a340, vec![t + 0x14, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_c560, 0x0108_cc9c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                    (0x009a_a340, vec![t + 0x4, 0x3]),
                    (0x009a_a340, vec![t + 0xc, 0x1]),
                    (0x009a_a340, vec![t + 0xc, 0x6]),
                    (0x009a_a340, vec![t + 0xc, 0x5]),
                    (0x009a_a340, vec![t + 0x14, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_c700, 0x0108_ccb4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0xc, u32::MAX]),
                    (0x0097_ade0, vec![t, 0xa, 0x7]),
                    (0x0097_adc0, vec![t, 0xb, u32::MAX]),
                    (0x0097_adc0, vec![t, 0xa, u32::MAX]),
                ]
            }),
            (0x0097_c850, 0x0108_cccc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0xc, u32::MAX]),
                    (0x0097_adc0, vec![t, 0xb, u32::MAX]),
                    (0x0097_adc0, vec![t, 0xa, u32::MAX]),
                ]
            }),
            (0x0097_c9d0, 0x0108_cce4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0x1, 0xe]),
                    (0x0097_adc0, vec![t, 0xe, 0x0]),
                ]
            }),
            (0x0097_ca60, 0x0108_ccfc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0097_adc0, vec![t, 0x3, 0x0]),
                ]
            }),
            (0x0097_cb70, 0x0108_cd14, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0098_4390, vec![t, 0x1, 0xf]),
                    (0x0097_adc0, vec![t, 0x3, 0x0]),
                ]
            }),
            (0x0097_cc00, 0x0108_cd2c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0x1, 0x3]),
                    (0x0097_adc0, vec![t, 0x6, 0x0]),
                ]
            }),
            (0x0097_cc80, 0x0108_cd44, {
                vec![(0x0097_9bd0, vec![t, k]), (0x0097_adc0, vec![t, 0x6, 0x0])]
            }),
            (0x0097_cd30, 0x0108_cd5c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0x1, 0xc]),
                    (0x0097_adc0, vec![t, 0xf, 0x0]),
                ]
            }),
            (0x0097_cfe0, 0x0108_cd8c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0xd, u32::MAX]),
                    (0x0097_adc0, vec![t, 0x10, u32::MAX]),
                ]
            }),
            (0x0097_d1c0, 0x0108_cdbc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0098_4390, vec![t, 0x1, 0xd]),
                    (0x0097_adc0, vec![t, 0x5, 0x1]),
                ]
            }),
            (0x0097_d250, 0x0108_cdd4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0x1, 0xa]),
                    (0x0097_adc0, vec![t, 0x11, u32::MAX]),
                ]
            }),
            (0x0097_d130, 0x0108_cda4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0098_4390, vec![t, 0x1, 0x0]),
                    (0x0097_adc0, vec![t, 0x6, 0x0]),
                    (0x0097_adc0, vec![t, 0x5, 0x1]),
                ]
            }),
        ]
    }

    #[test]
    fn each_constructor_builds_its_base_sets_its_vtable_and_fills_its_lists() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        let kind = 0x17;
        let addresses: Vec<u32> = cases(this, kind).iter().map(|c| c.0).collect();
        assert_eq!(addresses.len(), 25);
        for (address, vtable, expected) in cases(this, kind) {
            e.call_log = Some(vec![]);
            let back = e.call(address, &args![this, kind]).u32();
            assert_eq!(back, this, "{address:08x}");
            assert_eq!(e.mem.u32(this), vtable, "{address:08x}");
            let log: Calls = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != address)
                .collect();
            assert_eq!(log, expected, "{address:08x}");
        }
    }
}
