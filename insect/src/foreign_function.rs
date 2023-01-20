use crate::{quantity::Quantity, number::Number};

pub(crate) struct ForeignFunction {
    pub(crate) name: String,
    pub(crate) arity: usize,
    pub(crate) function: fn(&[Quantity]) -> Quantity,
}

pub(crate) fn abs(args: &[Quantity]) -> Quantity {
    assert!(args.len() == 1);

    let value = args[0].unsafe_value().to_f64();
    Quantity::new(Number::from_f64(value.abs()), args[0].unit().clone())
}

pub(crate) fn sin(args: &[Quantity]) -> Quantity {
    assert!(args.len() == 1);

    let input = args[0].as_scalar().unwrap().to_f64();
    Quantity::from_scalar(input.sin())
}

pub(crate) fn atan2(args: &[Quantity]) -> Quantity {
    assert!(args.len() == 2);

    let input0 = args[0].unsafe_value().to_f64();
    let input1 = args[1].unsafe_value().to_f64();
    Quantity::from_scalar(input0.atan2(input1))
}