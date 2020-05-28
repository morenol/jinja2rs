use super::Value;
use crate::expression_evaluator::BinaryOperation;
pub struct BinaryMathOperation;

impl BinaryMathOperation {
    pub fn apply(op: &BinaryOperation, left: Value, right: Value) -> Value {
        match op {
            BinaryOperation::Plus => left + right,
            BinaryOperation::Minus => left - right,
            BinaryOperation::Mul => left * right,
            BinaryOperation::Div => left / right,
            BinaryOperation::Modulo => left % right,
            BinaryOperation::DivInteger => {
                let value = left / right;
                if let Value::Double(num) = value {
                    Value::Integer(num as i64)
                } else {
                    todo!()
                }
            }
            BinaryOperation::Pow => left.pow(right),
            BinaryOperation::LogicalEq => Value::Boolean(left == right),
            BinaryOperation::LogicalLe => Value::Boolean(left <= right),
            BinaryOperation::LogicalLt => Value::Boolean(left < right),
            BinaryOperation::LogicalGe => Value::Boolean(left >= right),
            BinaryOperation::LogicalGt => Value::Boolean(left > right),
            BinaryOperation::LogicalNe => Value::Boolean(left != right),
            BinaryOperation::LogicalOr => left | right,
            BinaryOperation::LogicalAnd => left & right,
        }
    }
}
