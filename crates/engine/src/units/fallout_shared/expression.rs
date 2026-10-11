//! `fallout shared/expression.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CExpression` (Xbox PDB), size 0x710 and stack offsets checked
    /// against PC 00593490/00594b60/00594c60 and caller 005e1160.
    pub struct CExpression: 0x710 {
        0 m_ExpressionError: u32,
    }
}

// Translated from 00593490 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `CExpression::CExpression` (Xbox PDB): initializes the two stack indices,
/// then the error word. The infix/postfix and stack storage are untouched.
pub fn expression_construct(e: &mut Engine, this: Ptr<CExpression>) -> Ptr<CExpression> {
    e.call(0x0059_4b60, &args![this.addr() + 0x404]);
    e.call(0x0059_4c60, &args![this.addr() + 0x508]);
    e.set(this, CExpression::m_ExpressionError, 0);
    this
}

// Translated from 00594b60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Initializes the operator stack's index; its 64 data words remain intact.
pub fn operator_stack_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0x100, u32::MAX);
    this
}

// Translated from 00594c60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Initializes the operand stack's index; its 32 operands remain intact.
pub fn operand_stack_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0x200, u32::MAX);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00593490,
            expression_construct(Ptr<CExpression>) -> Ptr<CExpression>
        ),
        entry!(0x00594b60, operator_stack_construct(Ptr) -> Ptr),
        entry!(0x00594c60, operand_stack_construct(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_initializes_only_error_and_indices_in_guarded_storage() {
        for fill in [0xa5, 0xcc, 0xff] {
            let mut e = Engine::new();
            let block = e.mem.alloc(0x718);
            let mut expected = vec![fill; 0x718];
            e.mem.write(block, &expected);
            expected[4..8].fill(0);
            expected[0x508..0x50c].fill(0xff);
            expected[0x70c..0x710].fill(0xff);
            assert_eq!(e.call(0x00593490, &[block + 4]).u32(), block + 4);
            assert_eq!(e.mem.bytes(block, 0x718), expected);
        }
    }

    #[test]
    fn expression_clears_error_after_ordered_stack_callbacks() {
        let mut e = Engine::new();
        let object = e.new_object::<CExpression>();
        e.set(object, CExpression::m_ExpressionError, 7);
        e.register_double(0x00594b60, move |e, a| {
            assert_eq!(a, [object.addr() + 0x404]);
            assert_eq!(e.get(object, CExpression::m_ExpressionError), 7);
            e.set(object, CExpression::m_ExpressionError, 8);
            Ret::default()
        });
        e.register_double(0x00594c60, move |e, a| {
            assert_eq!(a, [object.addr() + 0x508]);
            assert_eq!(e.get(object, CExpression::m_ExpressionError), 8);
            Ret::default()
        });
        assert_eq!(expression_construct(&mut e, object), object);
        assert_eq!(e.get(object, CExpression::m_ExpressionError), 0);
    }

    #[test]
    fn stack_constructors_preserve_data_and_outer_guards() {
        for (address, size) in [(0x00594b60, 0x104), (0x00594c60, 0x204)] {
            let mut e = Engine::new();
            let block = e.mem.alloc(size + 8);
            let mut expected = vec![0xa5; size as usize + 8];
            e.mem.write(block, &expected);
            expected[size as usize..size as usize + 4].fill(0xff);
            assert_eq!(e.call(address, &[block + 4]).u32(), block + 4);
            assert_eq!(e.mem.bytes(block, size + 8), expected);
        }
    }
}
