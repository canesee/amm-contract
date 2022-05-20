use uint::construct_uint;

construct_uint! {
    pub struct u256(4);
}

pub fn mul_u128(base: u128, a: u128, b: u128) -> u128 {
    let a = u256::from(a);
    let b = u256::from(b);
    let base = u256::from(base);
    let c0 = a * b;
    let c1 = c0 + (base / 2);
    (c1 / base).as_u128()
}

pub fn div_u128(base: u128, a: u128, b: u128) -> u128 {
    let a = u256::from(a);
    let b = u256::from(b);
    let base = u256::from(base);
    let c0 = a / b;
    let c1 = c0 + (base / 2);
    (c1 / base).as_u128()
}

pub fn mul(base: u128, a: u128, b: u128) -> u128 {
    (u256::from(a) * u256::from(b) / u256::from(base)).as_u128()
}
