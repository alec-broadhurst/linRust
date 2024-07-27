pub trait IdentityElement {
    fn zero() -> Self;
    fn one() -> Self;
}

impl IdentityElement for i8 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for i16 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for i128 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for isize {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for u8 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for u16 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for u128 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for usize {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl IdentityElement for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl IdentityElement for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}
