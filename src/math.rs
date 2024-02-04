use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

pub fn int(n: u128) -> Decimal {
	Decimal::from(n)
}

pub fn float(n: f64) -> Decimal {
	Decimal::from_f64(n).unwrap()
}
