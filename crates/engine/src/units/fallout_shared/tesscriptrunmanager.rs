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

// Translated from 005e1160 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e1160 - compiler SEH exception cleanup.
/// Builds a script's action list through the real parser and EvaluateLine
/// entry. Parser failure clears the script's data size; evaluator failure
/// merely ends this pass. Both normal exits return true after clearing busy.
pub fn runner_init_action_list(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    reference: Ptr,
    locals: Ptr,
) -> bool {
    if script.is_null() || !e.call(0x0040_13e0, &args![script]).bool() {
        return false;
    }
    if !e.get(this, ScriptRunner::m_scriptRunning).is_null() {
        let requested_name = e.vcall(script.addr(), 0x130, &args![]).u32();
        let running = e.get(this, ScriptRunner::m_scriptRunning);
        let running_name = e.vcall(running.addr(), 0x130, &args![]).u32();
        e.call(
            0x005b_5e40,
            &args![0x0103_d498u32, running_name, requested_name],
        );
        return false;
    }
    e.set(this, ScriptRunner::m_scriptRunning, script);
    // CExpression (Xbox PDB) is 0x710 bytes, matching EBP-0x73c..EBP-0x2c.
    // Its constructors initialize stack indices, not the embedded buffers.
    e.with_stack(0x710, |e, expression| {
        e.call(0x0059_3490, &args![expression]);
        let object = if reference.is_null() {
            0
        } else {
            e.call(0x007a_f430, &args![reference]).u32()
        };
        e.mem.set_u32(this.addr() + 4, object);
        e.mem.set_u32(this.addr() + 8, locals.addr());
        e.with_stack(20, |e, work| {
            let opcode = work;
            let cursor = work.byte_add(4);
            let length = work.byte_add(8);
            let target = work.byte_add(12);
            let skip = work.byte_add(16);
            e.mem.set_u32(opcode.addr(), 0xffff);
            for offset in [4, 8, 12, 16] {
                e.mem.set_u32(work.addr() + offset, 0);
            }
            let mut line = 0u32;
            loop {
                let header = e.call(0x0050_0940, &args![script]).u32();
                if e.mem.u32(cursor.addr()) >= e.mem.u32(header + 8) {
                    break;
                }
                line = line.wrapping_add(1);
                e.mem.set_u32(this.addr() + 0x10, 0);
                e.mem.set_u32(length.addr(), 0);
                let skipping = e.mem.u32(skip.addr()) != 0;
                if !e
                    .call(
                        0x005e_13a0,
                        &args![this, script, opcode, cursor, length, target, skipping],
                    )
                    .bool()
                {
                    let header = e.call(0x0050_0940, &args![script]).u32();
                    e.mem.set_u32(header + 8, 0);
                    break;
                }
                let remaining = e.mem.u32(skip.addr());
                if remaining != 0 {
                    e.mem.set_u32(skip.addr(), remaining.wrapping_sub(1));
                } else {
                    if e.mem.u32(target.addr()) == 0 {
                        e.mem.set_u32(target.addr(), reference.addr());
                    }
                    let command = e.mem.u32(opcode.addr());
                    let target_ref = e.mem.u32(target.addr());
                    let position = e.mem.u32(cursor.addr());
                    if !e
                        .call(
                            0x005e_1550,
                            &args![
                                this, script, command, target_ref, length, position, skip, line,
                                0u32, 1u32
                            ],
                        )
                        .bool()
                    {
                        break;
                    }
                }
                let next = e
                    .mem
                    .u32(cursor.addr())
                    .wrapping_add(e.mem.u32(length.addr()));
                e.mem.set_u32(cursor.addr(), next);
            }
        });
        e.set(this, ScriptRunner::m_scriptRunning, Ptr::NULL);
        e.call(0x0048_3710, &args![expression]);
    });
    true
}

fn script_word(e: &mut Engine, script: Ptr, cursor: Ptr) -> i32 {
    let data = e.call(0x0067_1d10, &args![script]).u32();
    e.mem.i16(data.wrapping_add(e.mem.u32(cursor.addr()))) as i32
}

fn advance_word(e: &mut Engine, cursor: Ptr) {
    e.mem
        .set_u32(cursor.addr(), e.mem.u32(cursor.addr()).wrapping_add(2));
}

// Translated from 005e13a0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Reads signed opcode/length words, optionally resolving a reference prefix.
/// A skipped prefix leaves the reference output unchanged; ordinary commands
/// clear it. The PC body does not add bounds checks to the bytecode reads.
#[allow(clippy::too_many_arguments)] // PC receiver plus six stack arguments.
pub fn runner_parse_line(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    opcode: Ptr,
    cursor: Ptr,
    length: Ptr,
    reference: Ptr,
    skip_reference: bool,
) -> bool {
    e.mem.set_u32(length.addr(), 0);
    let command = script_word(e, script, cursor);
    e.mem.set_i32(opcode.addr(), command);
    advance_word(e, cursor);
    if e.mem.u32(opcode.addr()) == 0x1c {
        let index = script_word(e, script, cursor);
        advance_word(e, cursor);
        let mut variable = false;
        if !skip_reference {
            let locals = e.mem.u32(this.addr() + 8);
            let found = e.call(0x005a_c4f0, &args![script, index, locals]).u32();
            if found != 0 {
                variable = e.mem.u32(found + 0xc) != 0;
                let object = e.mem.u32(found + 8);
                if object != 0 {
                    let target = e
                        .call(
                            0x00ec_43fb,
                            &args![object, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
                        )
                        .u32();
                    e.mem.set_u32(reference.addr(), target);
                }
            }
        }
        let command = script_word(e, script, cursor);
        e.mem.set_i32(opcode.addr(), command);
        advance_word(e, cursor);
        if !skip_reference && e.mem.u32(reference.addr()) == 0 && !variable {
            let command = e.mem.u32(opcode.addr());
            let descriptor = e.call(0x005b_1120, &args![command]).u32();
            let script_name = e.vcall(script.addr(), 0x130, &args![]).u32();
            if descriptor != 0 {
                let command_name = e.mem.u32(descriptor);
                e.call(
                    0x005e_2410,
                    &args![this, script, 0x0103_d538u32, command_name, script_name],
                );
            } else {
                e.call(
                    0x005e_2410,
                    &args![this, script, 0x0103_d4f4u32, script_name],
                );
            }
            return false;
        }
    } else {
        e.mem.set_u32(reference.addr(), 0);
    }
    let size = script_word(e, script, cursor);
    e.mem.set_i32(length.addr(), size);
    advance_word(e, cursor);
    true
}

// Translated from 005e1550 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e1550 - non-BEGIN execution/parse modes without action-list flag, and compiler SEH cleanup.
/// `ScriptRunner::EvaluateLine` (Xbox PDB), BEGIN and action-list paths.
/// Other execution/parse paths remain open rather than reporting success.
#[allow(clippy::too_many_arguments)] // Exact PC receiver and nine stack words.
pub fn runner_evaluate_line(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    command: i32,
    reference: Ptr,
    length: Ptr,
    position: u32,
    _skip: Ptr,
    line: u32,
    parse_only: bool,
    action_list: bool,
) -> bool {
    let data = e.call(0x0067_1d10, &args![script]).u32();
    if command != 0x10 {
        if action_list {
            return true;
        }
        panic!("open execution/parse path 005e1550 for opcode {command:08x}");
    }
    e.with_stack(12, |e, work| {
        let cursor = work;
        let result = work.byte_add(4);
        let kind = e.mem.i16(data.wrapping_add(position)) as i32;
        e.mem.set_u32(cursor.addr(), position.wrapping_add(2));
        if !(0..0x26).contains(&kind) {
            let form_id = e.call(0x0084_e3a0, &args![script]).u32();
            let name = e.vcall(script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d8d0u32, line, name, form_id]);
            return false;
        }
        let span = e.mem.u32(data.wrapping_add(e.mem.u32(cursor.addr())));
        e.mem.set_u32(cursor.addr(), position.wrapping_add(6));
        let entry = 0x0118_e300 + kind as u32 * 0x28;
        let parameters = e.mem.u32(entry + 4);
        let handler = e.mem.u32(entry + 8);
        if parse_only {
            if parameters != 0 && e.mem.u32(length.addr()) != 0 {
                let locals = e.mem.u32(this.addr() + 8);
                let container = e.mem.u32(this.addr());
                e.call(
                    0x005a_ccb0,
                    &args![parameters, data, cursor, reference, container, script, locals],
                );
            }
            return true;
        }
        if e.mem.u8(entry) != 0 && reference.is_null() {
            let name = e.vcall(script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d890u32, name, line]);
            return false;
        }
        let succeeded = if handler == 0 {
            false
        } else {
            let locals = e.mem.u32(this.addr() + 8);
            let container = e.mem.u32(this.addr());
            e.call(
                handler,
                &args![parameters, data, reference, container, script, locals, result, cursor],
            )
            .bool()
        };
        if !succeeded {
            // The PC pushes the cursor and line before calling the name getter.
            let failed_at = e.mem.u32(cursor.addr());
            let name = e.vcall(script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d848u32, name, line, failed_at]);
            return false;
        }
        if e.mem.f64(result.addr()) == 0.0 || action_list {
            e.mem
                .set_u32(length.addr(), e.mem.u32(length.addr()).wrapping_add(span));
        }
        true
    })
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
/// or a temporary runner if the cached one is busy.
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
            0x005e1160,
            runner_init_action_list(Ptr<ScriptRunner>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e13a0,
            runner_parse_line(Ptr<ScriptRunner>, Ptr, Ptr, Ptr, Ptr, Ptr, bool) -> bool
        ),
        entry!(
            0x005e1550,
            runner_evaluate_line(
                Ptr<ScriptRunner>,
                Ptr,
                i32,
                Ptr,
                Ptr,
                u32,
                Ptr,
                u32,
                bool,
                bool,
            ) -> bool
        ),
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

    fn parser_fixture(e: &mut Engine, bytes: &[u8]) -> (Ptr<ScriptRunner>, Ptr, Ptr) {
        let runner = e.new_object::<ScriptRunner>();
        let script = Ptr::new(e.mem.alloc(0x54));
        let code = e.mem.alloc(bytes.len() as u32);
        e.mem.write(code, bytes);
        e.register_double(0x00671d10, move |_, a| {
            assert_eq!(a, [script.addr()]);
            Ret {
                eax: code,
                ..Ret::default()
            }
        });
        let work = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(work.addr(), 0x12345678);
        e.mem.set_u32(work.addr() + 8, 0x87654321);
        e.mem.set_u32(work.addr() + 12, 0xdeadbeef);
        (runner, script, work)
    }

    fn parse(
        e: &mut Engine,
        runner: Ptr<ScriptRunner>,
        script: Ptr,
        work: Ptr,
        skip: bool,
    ) -> bool {
        runner_parse_line(
            e,
            runner,
            script,
            work,
            work.byte_add(4),
            work.byte_add(8),
            work.byte_add(12),
            skip,
        )
    }

    #[test]
    fn evaluator_action_mode_skips_non_begin_opcodes_after_fetching_data() {
        for command in [0x11i32, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1d, 0x1059, -1] {
            let mut e = Engine::new();
            e.register_double(0x00671d10, |_, a| {
                assert_eq!(a, [0x2222]);
                Ret::default()
            });
            assert!(e
                .call(
                    0x005e1550,
                    &args![
                        0x1111u32,
                        0x2222u32,
                        command,
                        0u32,
                        0u32,
                        0xfffffff0u32,
                        0u32,
                        7u32,
                        false,
                        true
                    ]
                )
                .bool());
        }
    }

    fn begin_fixture(e: &mut Engine, kind: i16, span: u32) -> (Ptr<ScriptRunner>, Ptr, Ptr) {
        let mut bytes = Vec::from(kind.to_le_bytes());
        bytes.extend(span.to_le_bytes());
        let (runner, script, work) = parser_fixture(e, &bytes);
        e.mem.set_u32(runner.addr(), 0x4444);
        e.mem.set_u32(runner.addr() + 8, 0x5555);
        e.mem.set_u32(work.addr(), 11);
        let table = e.mem.alloc(0x134);
        e.mem.set_u32(script.addr(), table);
        e.mem.set_u32(table + 0x130, 0x12340000);
        e.register_double(0x12340000, |_, _| Ret {
            eax: 0x6666,
            ..Ret::default()
        });
        if (0..0x26).contains(&kind) {
            let entry = 0x0118e300 + kind as u32 * 0x28;
            e.map(entry, 12);
            e.mem.set_u8(entry, 0);
            e.mem.set_u32(entry + 4, 0x7777);
            e.mem.set_u32(entry + 8, 0x12340001);
        }
        (runner, script, work)
    }

    #[test]
    fn evaluator_begin_uses_eight_callback_arguments_and_live_length() {
        for result in [0.0, -0.0, 1.0, -1.0, f64::NAN] {
            for action_list in [false, true] {
                let mut e = Engine::new();
                let (runner, script, length) = begin_fixture(&mut e, 3, 173);
                e.register_double(0x12340001, move |e, a| {
                    assert_eq!(a.len(), 8);
                    assert_eq!(a[0], 0x7777);
                    assert_eq!(&a[2..6], &[0x3333, 0x4444, script.addr(), 0x5555]);
                    assert_eq!(e.mem.i16(a[1]), 3);
                    assert_eq!(e.mem.u32(a[7]), 6);
                    e.mem.set_f64(a[6], result);
                    e.mem.set_u32(length.addr(), u32::MAX - 1);
                    Ret {
                        eax: 1,
                        ..Ret::default()
                    }
                });
                assert!(runner_evaluate_line(
                    &mut e,
                    runner,
                    script,
                    16,
                    Ptr::new(0x3333),
                    length,
                    0,
                    Ptr::NULL,
                    2,
                    false,
                    action_list
                ));
                assert_eq!(
                    e.mem.u32(length.addr()),
                    if result == 0.0 || action_list {
                        171
                    } else {
                        u32::MAX - 1
                    }
                );
            }
        }
    }

    #[test]
    fn evaluator_begin_invalid_kind_and_required_reference_have_distinct_errors() {
        for kind in [-1, 38, 3] {
            let mut e = Engine::new();
            let (runner, script, length) = begin_fixture(&mut e, kind, 173);
            e.register_double(0x0084e3a0, move |_, a| {
                assert_eq!(a, [script.addr()]);
                Ret {
                    eax: 0x8888,
                    ..Ret::default()
                }
            });
            if kind == 3 {
                e.mem.set_u8(0x0118e300 + 3 * 0x28, 1);
            }
            e.register_double(0x005b5e40, move |_, a| {
                if kind == 3 {
                    assert_eq!(a, [0x0103d890, 0x6666, 2]);
                } else {
                    assert_eq!(a, [0x0103d8d0, 2, 0x6666, 0x8888]);
                }
                Ret::default()
            });
            assert!(!runner_evaluate_line(
                &mut e,
                runner,
                script,
                16,
                Ptr::NULL,
                length,
                0,
                Ptr::NULL,
                2,
                false,
                true
            ));
            assert_eq!(e.mem.u32(length.addr()), 11);
        }
    }

    #[test]
    fn evaluator_begin_failure_reports_cursor_before_script_name_callback() {
        for handler_present in [false, true] {
            let mut e = Engine::new();
            let (runner, script, length) = begin_fixture(&mut e, 3, 173);
            let cursor_slot = std::rc::Rc::new(std::cell::Cell::new(0));
            let slot = cursor_slot.clone();
            if handler_present {
                e.register_double(0x12340001, move |e, a| {
                    e.mem.set_u32(a[7], 19);
                    slot.set(a[7]);
                    Ret::default()
                });
            } else {
                e.mem.set_u32(0x0118e300 + 3 * 0x28 + 8, 0);
            }
            e.register_double(0x12340000, move |e, _| {
                if cursor_slot.get() != 0 {
                    e.mem.set_u32(cursor_slot.get(), 99);
                }
                Ret {
                    eax: 0x6666,
                    ..Ret::default()
                }
            });
            e.register_double(0x005b5e40, move |_, a| {
                assert_eq!(
                    a,
                    [0x0103d848, 0x6666, 2, if handler_present { 19 } else { 6 }]
                );
                Ret::default()
            });
            assert!(!runner_evaluate_line(
                &mut e,
                runner,
                script,
                16,
                Ptr::NULL,
                length,
                0,
                Ptr::NULL,
                2,
                false,
                true
            ));
        }
    }

    #[test]
    fn evaluator_begin_parse_mode_ignores_result_and_requires_parameters_and_length() {
        for (parameters, bytes) in [(0, 11), (0x7777, 0), (0x7777, 11)] {
            let mut e = Engine::new();
            let (runner, script, length) = begin_fixture(&mut e, 3, 173);
            e.mem.set_u8(0x0118e300 + 3 * 0x28, 1);
            e.mem.set_u32(0x0118e300 + 3 * 0x28 + 4, parameters);
            e.mem.set_u32(length.addr(), bytes);
            let calls = std::rc::Rc::new(std::cell::Cell::new(0));
            let count = calls.clone();
            e.register_double(0x005accb0, move |e, a| {
                assert_eq!(a.len(), 7);
                assert_eq!(a[0], parameters);
                assert_eq!(e.mem.u32(a[2]), 6);
                assert_eq!(&a[3..], &[0, 0x4444, script.addr(), 0x5555]);
                count.set(count.get() + 1);
                Ret::default()
            });
            assert!(runner_evaluate_line(
                &mut e,
                runner,
                script,
                16,
                Ptr::NULL,
                length,
                0,
                Ptr::NULL,
                2,
                true,
                true
            ));
            assert_eq!(calls.get(), i32::from(parameters != 0 && bytes != 0));
            assert_eq!(e.mem.u32(length.addr()), bytes);
        }
    }

    #[test]
    #[should_panic(expected = "open execution/parse path 005e1550")]
    fn evaluator_does_not_claim_unsupported_execution_modes_succeed() {
        let mut e = Engine::new();
        e.register_double(0x00671d10, |_, _| Ret::default());
        runner_evaluate_line(
            &mut e,
            Ptr::NULL,
            Ptr::NULL,
            0x15,
            Ptr::NULL,
            Ptr::NULL,
            0,
            Ptr::NULL,
            1,
            false,
            false,
        );
    }

    #[test]
    fn action_list_rejects_null_invalid_and_busy_scripts_before_construction() {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        let runner = e.new_object::<ScriptRunner>();
        let script = Ptr::new(e.mem.alloc(0x54));
        assert!(!runner_init_action_list(
            &mut e,
            runner,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL
        ));
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.register_double(0x004013e0, |_, _| Ret::default());
        assert!(!runner_init_action_list(
            &mut e,
            runner,
            script,
            Ptr::NULL,
            Ptr::NULL
        ));
        assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
        e.register_double(0x004013e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let replacement = e.mem.alloc(4);
        let table = e.mem.alloc(0x134);
        e.mem.set_u32(script.addr(), table);
        e.mem.set_u32(replacement, table);
        e.mem.set_u32(table + 0x130, 0x12340000);
        e.set(runner, ScriptRunner::m_scriptRunning, Ptr::new(0x1111));
        e.register_double(0x12340000, move |e, a| {
            if a[0] == script.addr() {
                e.set(runner, ScriptRunner::m_scriptRunning, Ptr::new(replacement));
                Ret {
                    eax: 0x2222,
                    ..Ret::default()
                }
            } else {
                assert_eq!(a, [replacement]);
                Ret {
                    eax: 0x3333,
                    ..Ret::default()
                }
            }
        });
        e.register_double(0x005b5e40, |_, a| {
            assert_eq!(a, [0x0103d498, 0x3333, 0x2222]);
            Ret::default()
        });
        assert!(!runner_init_action_list(
            &mut e,
            runner,
            script,
            Ptr::NULL,
            Ptr::NULL
        ));
        assert_eq!(
            e.get(runner, ScriptRunner::m_scriptRunning).addr(),
            replacement
        );
    }

    #[test]
    fn action_list_empty_script_matches_native_mutations_and_preserves_context() {
        // Native 005e1160: AL=1, RET 12; only +4/+8 change in these
        // generated runner/script buffers. Null and disabled return AL=0.
        for locals in [0u32, 0x12345678] {
            let mut e = Engine::new();
            let block = e.mem.alloc(0x100);
            let mut expected = vec![0xa5; 0x100];
            expected[0x14..0x18].fill(0);
            expected[0xc4..0xc8].fill(0);
            expected[0xac..0xb0].copy_from_slice(&8u32.to_le_bytes());
            e.mem.write(block, &expected);
            expected[4..8].fill(0);
            expected[8..12].copy_from_slice(&locals.to_le_bytes());
            e.register_double(0x004013e0, |e, a| Ret {
                eax: u32::from(e.mem.u32(a[0] + 8) & 8 != 0),
                ..Ret::default()
            });
            e.register_double(0x00500940, |_, a| Ret {
                eax: a[0] + 0x18,
                ..Ret::default()
            });
            e.register_double(0x00593490, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
            e.register_double(0x00483710, |_, _| Ret::default());
            assert!(e.call(0x005e1160, &[block, block + 0xa4, 0, locals]).bool());
            assert_eq!(e.mem.bytes(block, 0x100), expected);
        }
    }

    #[test]
    fn action_list_parses_three_lines_and_honors_evaluator_skip_and_length() {
        let mut e = Engine::new();
        // First length is replaced by EvaluateLine; second is skipped without
        // resolving its reference prefix; third clears that stale reference.
        let (runner, script, _) = parser_fixture(
            &mut e,
            &[
                16, 0, 99, 0, 0xaa, 0xbb, 28, 0, 7, 0, 17, 0, 0, 0, 18, 0, 0, 0,
            ],
        );
        let header = e.mem.alloc(12);
        e.mem.set_u32(header + 8, 18);
        e.register_double(0x004013e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00593490, move |e, a| {
            assert_eq!(e.get(runner, ScriptRunner::m_scriptRunning), script);
            e.mem.set_u32(a[0] + 0x70c, 0x12345678);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(0x007af430, |_, a| {
            assert_eq!(a, [0x4444]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        e.register_double(0x00500940, move |_, a| {
            assert_eq!(a, [script.addr()]);
            Ret {
                eax: header,
                ..Ret::default()
            }
        });
        let evaluations = std::rc::Rc::new(std::cell::Cell::new(0));
        let count = evaluations.clone();
        e.register_double(0x005e1550, move |e, a| {
            assert_eq!(a.len(), 10);
            assert_eq!(&a[..2], &[runner.addr(), script.addr()]);
            assert_eq!(a[3], 0x4444);
            assert_eq!(&a[8..], &[0, 1]);
            assert_eq!(e.mem.u32(runner.addr() + 4), 0x5555);
            assert_eq!(e.mem.u32(runner.addr() + 8), 0x6666);
            assert_eq!(e.mem.u32(runner.addr() + 0x10), 0);
            if count.get() == 0 {
                assert_eq!((a[2], a[5], a[7]), (16, 4, 1));
                assert_eq!(e.mem.u32(a[4]), 99);
                e.mem.set_u32(a[4], 2);
                e.mem.set_u32(a[6], 1);
                e.mem.set_u32(runner.addr() + 0x10, 0x7777);
            } else {
                assert_eq!((a[2], a[5], a[7]), (18, 18, 3));
                assert_eq!(e.mem.u32(a[6]), 0);
            }
            count.set(count.get() + 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00483710, move |e, a| {
            assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
            assert_eq!(e.mem.u32(a[0] + 0x70c), 0x12345678);
            Ret::default()
        });
        assert!(e
            .call(0x005e1160, &args![runner, script, 0x4444u32, 0x6666u32])
            .bool());
        assert_eq!(evaluations.get(), 2);
        assert_eq!(e.mem.u32(header + 8), 18);
    }

    #[test]
    fn action_list_uses_real_parser_evaluator_and_expression_for_two_event_blocks() {
        let mut e = Engine::new();
        let bytes = [
            29, 0, 0, 0, 16, 0, 6, 0, 3, 0, 4, 0, 0, 0, 17, 0, 0, 0, 16, 0, 6, 0, 6, 0, 0, 0, 0, 0,
        ];
        let (runner, script, _) = parser_fixture(&mut e, &bytes);
        let header = e.mem.alloc(12);
        e.mem.set_u32(header + 8, bytes.len() as u32);
        e.register_double(0x004013e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x007af430, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register_double(0x00500940, move |_, _| Ret {
            eax: header,
            ..Ret::default()
        });
        e.register_double(0x00483710, move |e, a| {
            assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
            assert_eq!(e.mem.u32(a[0]), 0);
            assert_eq!(e.mem.u32(a[0] + 0x504), u32::MAX);
            assert_eq!(e.mem.u32(a[0] + 0x708), u32::MAX);
            Ret::default()
        });
        let hits = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        for (kind, cursor) in [(3u32, 14u32), (6, 28)] {
            let entry = 0x0118e300 + kind * 0x28;
            e.map(entry, 12);
            let handler = 0x12340000 + kind;
            e.mem.set_u8(entry, 1);
            e.mem.set_u32(entry + 4, 0x7777);
            e.mem.set_u32(entry + 8, handler);
            let log = hits.clone();
            e.register_double(handler, move |e, a| {
                assert_eq!(&a[2..6], &[0x3333, 0, script.addr(), 0x5555]);
                assert_eq!(e.mem.u32(a[7]), cursor);
                e.mem.set_f64(a[6], 1.0);
                log.borrow_mut().push(kind);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            });
        }
        assert!(e
            .call(0x005e1160, &args![runner, script, 0x3333u32, 0x5555u32])
            .bool());
        assert_eq!(*hits.borrow(), [3, 6]);
        assert_eq!(e.mem.u32(header + 8), 28);
    }

    #[test]
    fn action_list_failures_return_true_but_only_parse_failure_clears_fresh_header() {
        for parse_success in [false, true] {
            let mut e = Engine::new();
            let runner = e.new_object::<ScriptRunner>();
            let script = Ptr::new(e.mem.alloc(0x54));
            let original = e.mem.alloc(12);
            let replacement = e.mem.alloc(12);
            e.mem.set_u32(original + 8, 4);
            e.mem.set_u32(replacement + 8, 9);
            e.register_double(0x004013e0, |_, _| Ret {
                eax: 1,
                ..Ret::default()
            });
            e.register_double(0x00593490, |_, _| Ret::default());
            let mut reads = 0;
            e.register_double(0x00500940, move |_, _| {
                reads += 1;
                Ret {
                    eax: if reads == 1 { original } else { replacement },
                    ..Ret::default()
                }
            });
            e.register_double(0x005e13a0, move |e, a| {
                assert_eq!(a[6], 0);
                assert_eq!(e.mem.u32(a[2]), 0xffff);
                assert_eq!(e.mem.u32(a[3]), 0);
                e.mem.set_u32(a[3], 4);
                Ret {
                    eax: u32::from(parse_success),
                    ..Ret::default()
                }
            });
            e.register_double(0x005e1550, |_, a| {
                assert_eq!((a[3], a[5], a[7]), (0, 4, 1));
                Ret::default()
            });
            e.register_double(0x00483710, move |e, _| {
                assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
                Ret::default()
            });
            assert!(runner_init_action_list(
                &mut e,
                runner,
                script,
                Ptr::NULL,
                Ptr::NULL
            ));
            assert_eq!(e.mem.u32(original + 8), 4);
            assert_eq!(
                e.mem.u32(replacement + 8),
                if parse_success { 9 } else { 0 }
            );
            assert_eq!(e.mem.u32(runner.addr() + 4), 0);
        }
    }

    #[test]
    fn parser_matches_six_native_signed_word_and_prefix_vectors() {
        // Generated nv-call cases, PC 005e13a0, exe SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        type ParserCase<'a> = (&'a [u8], bool, i32, u32, i32, u32);
        let cases: &[ParserCase<'_>] = &[
            (&[0, 0, 0, 0], false, 0, 4, 0, 0),
            (&[16, 0, 10, 0], false, 16, 4, 10, 0),
            (&[255, 255, 1, 0], false, -1, 4, 1, 0),
            (&[16, 0, 255, 255], false, 16, 4, -1, 0),
            (&[16, 0, 0, 128], true, 16, 4, -32768, 0),
            (&[28, 0, 7, 0, 16, 0, 255, 255], true, 16, 8, -1, 0xdeadbeef),
        ];
        for &(bytes, skip, opcode, cursor, length, reference) in cases {
            let mut e = Engine::new();
            let (runner, script, work) = parser_fixture(&mut e, bytes);
            assert!(e
                .call(
                    0x005e13a0,
                    &args![
                        runner,
                        script,
                        work,
                        work.byte_add(4),
                        work.byte_add(8),
                        work.byte_add(12),
                        skip
                    ]
                )
                .bool());
            assert_eq!(e.mem.i32(work.addr()), opcode);
            assert_eq!(e.mem.u32(work.addr() + 4), cursor);
            assert_eq!(e.mem.i32(work.addr() + 8), length);
            assert_eq!(e.mem.u32(work.addr() + 12), reference);
        }
    }

    #[test]
    fn parser_reloads_code_and_resolves_signed_reference_index() {
        let mut e = Engine::new();
        let (runner, script, work) = parser_fixture(&mut e, &[28, 0, 255, 255, 16, 0, 8, 0]);
        e.mem.set_u32(runner.addr() + 8, 0x7000);
        let mut blocks = Vec::new();
        for bytes in [
            [28, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 255, 255, 0, 0, 0, 0],
            [0, 0, 0, 0, 17, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 255, 255],
        ] {
            let block = e.mem.alloc(8);
            e.mem.write(block, &bytes);
            blocks.push(block);
        }
        let mut read = 0;
        e.register_double(0x00671d10, move |_, _| {
            let result = blocks[read];
            read += 1;
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
        let found = e.mem.alloc(16);
        e.mem.set_u32(found + 8, 0x8000);
        e.register_double(0x005ac4f0, move |_, a| {
            assert_eq!(a, [script.addr(), u32::MAX, 0x7000]);
            Ret {
                eax: found,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec43fb, |_, a| {
            assert_eq!(a, [0x8000, 0, 0x01183028, 0x011841cc, 0]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        assert!(parse(&mut e, runner, script, work, false));
        assert_eq!(e.mem.i32(work.addr()), 17);
        assert_eq!(e.mem.i32(work.addr() + 8), -1);
        assert_eq!(e.mem.u32(work.addr() + 12), 0x9000);
    }

    #[test]
    fn parser_null_variable_reference_is_allowed_but_missing_reference_logs() {
        for (variable, descriptor_present) in [(true, false), (false, false), (false, true)] {
            let mut e = Engine::new();
            let (runner, script, work) = parser_fixture(&mut e, &[28, 0, 1, 0, 16, 0, 9, 0]);
            e.mem.set_u32(work.addr() + 12, 0);
            let found = e.mem.alloc(16);
            e.mem.set_u32(found + 12, u32::from(variable));
            e.register_double(0x005ac4f0, move |_, _| Ret {
                eax: found,
                ..Ret::default()
            });
            let descriptor = e.mem.alloc(4);
            e.mem.set_u32(descriptor, 0x1111);
            e.register_double(0x005b1120, move |_, a| {
                assert_eq!(a, [16]);
                Ret {
                    eax: if descriptor_present { descriptor } else { 0 },
                    ..Ret::default()
                }
            });
            let vtable = e.mem.alloc(0x134);
            e.mem.set_u32(script.addr(), vtable);
            e.mem.set_u32(vtable + 0x130, 0x12340000);
            e.register_double(0x12340000, move |e, _| {
                e.mem.set_u32(descriptor, 0x2222);
                Ret {
                    eax: 0x3333,
                    ..Ret::default()
                }
            });
            e.register_double(0x005e2410, move |_, a| {
                let mut expected = vec![runner.addr(), script.addr()];
                if descriptor_present {
                    expected.extend([0x0103d538, 0x2222, 0x3333]);
                } else {
                    expected.extend([0x0103d4f4, 0x3333]);
                }
                assert_eq!(a, expected);
                Ret::default()
            });
            assert_eq!(parse(&mut e, runner, script, work, false), variable);
            assert_eq!(e.mem.u32(work.addr() + 4), if variable { 8 } else { 6 });
            assert_eq!(e.mem.u32(work.addr() + 8), if variable { 9 } else { 0 });
        }
    }

    #[test]
    fn parser_preserves_store_order_when_opcode_aliases_cursor() {
        let mut e = Engine::new();
        let (runner, script, work) = parser_fixture(&mut e, &[4, 0, 99, 0, 99, 0, 7, 0]);
        e.mem.set_u32(work.addr(), 0);
        assert!(runner_parse_line(
            &mut e,
            runner,
            script,
            work,
            work,
            work.byte_add(8),
            work.byte_add(12),
            false
        ));
        assert_eq!(e.mem.u32(work.addr()), 8);
        assert_eq!(e.mem.u32(work.addr() + 8), 7);
    }

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
