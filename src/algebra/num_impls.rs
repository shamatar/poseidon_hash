use super::Element;

use pairing::ff::{
    PrimeField
};

use std::ops::{
    Add, 
    Sub, 
    Mul,
    Div,
    Rem,
    Neg,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign
};

use num_traits::{
    Zero,
    One,
    NumOps,
    Num,
    NumAssignOps,
    FromPrimitive,
    ToPrimitive
};

impl<F: PrimeField> Add for Element<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut tmp = self.0;
        tmp.add_assign(&other.0);

        Element(tmp)
    }
}

impl<F: PrimeField> Sub for Element<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut tmp = self.0;
        tmp.sub_assign(&other.0);

        Element(tmp)
    }
}

impl<F: PrimeField> Mul for Element<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut tmp = self.0;
        tmp.mul_assign(&other.0);

        Element(tmp)
    }
}

impl<F: PrimeField> Div for Element<F> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut tmp = self.0;
        let inv = other.0.inverse().unwrap();
        tmp.mul_assign(&inv);

        Element(tmp)
    }
}

impl<F: PrimeField> Rem for Element<F> {
    type Output = Self;

    fn rem(self, _other: Self) -> Self {
        Element(F::zero())
    }
}

impl<F: PrimeField> Neg for Element<F> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut tmp = self.0;
        tmp.negate();
        Element(tmp)
    }
}

impl<F: PrimeField> Zero for Element<F> {
    fn zero() -> Self {
        Element(F::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn set_zero(&mut self) { 
        self.0 = F::zero();
    }
}

impl<F: PrimeField> One for Element<F> {
    fn one() -> Self {
        Element(F::one())
    }

    fn set_one(&mut self) {
        self.0 = F::one();
    }

    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    { 
        self.0 == F::one()
    }
}

impl<F: PrimeField> AddAssign for Element<F> {
    fn add_assign(&mut self, other: Self) {
        self.0.add_assign(&other.0);
    }
}

impl<F: PrimeField> SubAssign for Element<F> {
    fn sub_assign(&mut self, other: Self) {
        self.0.sub_assign(&other.0);
    }
}

impl<F: PrimeField> MulAssign for Element<F> {
    fn mul_assign(&mut self, other: Self) {
        self.0.mul_assign(&other.0);
    }
}

impl<F: PrimeField> DivAssign for Element<F> {
    fn div_assign(&mut self, other: Self) {
        let inv = other.0.inverse().unwrap();
        self.0.mul_assign(&inv);
    }
}

impl<F: PrimeField> RemAssign for Element<F> {
    fn rem_assign(&mut self, _other: Self) {
        self.0 = F::zero();
    }
}

// impl<F: PrimeField> FromPrimitive for Element<F> {
//     fn from_i64(n: i64) -> Self {
//         if n < 0 {
//             let n = -n as u64;
//             let mut tmp = Self::from_u64(n).unwrap();
//             (tmp.0).negate();

//             tmp
//         } else {
//             Self::from_u64(n as u64).unwrap()
//         }
//     }
//     fn from_i128(n: i128) -> Self {
//         if n < 0 {
//             let n = -n as u128;
//             let mut tmp = Self::from_u128(n).unwrap();
//             (tmp.0).negate();

//             tmp
//         } else {
//             Self::from_u128(n as u128).unwrap()
//         }        
//     }
//     fn from_u64(n: u64) -> Self {
//         let mut repr = F::Repr::default();
//         repr.as_mut()[0] = n;

//         let tmp = F::from_repr(repr).unwrap();

//         Element(tmp)
//     }
//     fn from_u128(n: u128) -> Self {
//         let mut repr = F::Repr::default();
//         repr.as_mut()[0] = n as u64;
//         repr.as_mut()[1] = (n >> 64) as u64;

//         let tmp = F::from_repr(repr).unwrap();

//         Element(tmp)
//     }

//     fn from_f64(n: f64) -> Self {
//         unimplemented!()
//     }
// }

// impl<F: PrimeField> ToPrimitive for Element<F> {
//     fn to_i64(&self) -> Self {unimplemented!()}
//     fn to_i128(&self) -> Self {unimplemented!()}
//     fn to_u64(&self) -> Self {unimplemented!()}
//     fn to_u128(&self) -> Option<Self> {unimplemented!()}
//     fn to_f64(&self) -> Option<Self> {unimplemented!()}
// }