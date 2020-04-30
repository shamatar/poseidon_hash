pub use super::Element;

mod mul_impl;
mod add_impl;

use pairing::ff::{
    PrimeField
};

use mathru::algebra::abstr::{
    Zero,
    One,
    Ring,
    CommutativeRing,
    Sign,
    Field,
    Scalar,
    cast::{ToPrimitive, FromPrimitive, NumCast},
};

use mathru::elementary::{
    Power
};

impl<F: PrimeField> Ring for Element<F> {}

impl<F: PrimeField> CommutativeRing for Element<F> {}

impl<F: PrimeField> Field for Element<F> {}

impl<F: PrimeField> FromPrimitive for Element<F> {
    fn from_i64(n: i64) -> Self {
        if n < 0 {
            let n = -n as u64;
            let mut tmp = Self::from_u64(n);
            (tmp.0).negate();

            tmp
        } else {
            Self::from_u64(n as u64)
        }
    }
    fn from_i128(n: i128) -> Self {
        if n < 0 {
            let n = -n as u128;
            let mut tmp = Self::from_u128(n);
            (tmp.0).negate();

            tmp
        } else {
            Self::from_u128(n as u128)
        }        
    }
    fn from_u64(n: u64) -> Self {
        let mut repr = F::Repr::default();
        repr.as_mut()[0] = n;

        let tmp = F::from_repr(repr).unwrap();

        Element(tmp)
    }
    fn from_u128(n: u128) -> Self {
        let mut repr = F::Repr::default();
        repr.as_mut()[0] = n as u64;
        repr.as_mut()[1] = (n >> 64) as u64;

        let tmp = F::from_repr(repr).unwrap();

        Element(tmp)
    }

    fn from_f64(_n: f64) -> Self {
        unimplemented!()
    }
}

impl<F: PrimeField> ToPrimitive for Element<F> {
    fn to_i64(&self) -> i64 {unimplemented!()}
    fn to_i128(&self) -> i128 {unimplemented!()}
    fn to_u64(&self) -> u64 {unimplemented!()}
    fn to_u128(&self) -> u128 {unimplemented!()}
    fn to_f64(&self) -> f64 {unimplemented!()}
}

impl<F: PrimeField> NumCast for Element<F> {
    fn from<T: ToPrimitive>(n: T) -> Self {
        let as_u128 = n.to_u128();

        Self::from_u128(as_u128)
    }
}

impl<F: PrimeField> Sign for Element<F> {
    fn sign(&self) -> Self {
        if self.0.is_zero() {
            Self::zero()
        } else {
            let repr = self.0.into_repr();
            let mut negated = self.0;
            negated.negate();
    
            let negated = negated.into_repr();
            if repr > negated {
                let mut tmp = F::one();
                tmp.negate();

                Element(tmp)
            } else {
                Self::one()
            }
        }
    }

    fn abs(&self) -> Self {
        *self
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}

impl<F: PrimeField> Scalar for Element<F> {
    fn epsilon() -> Self { 
        Self::zero()
    }
}

// impl<F: PrimeField> Power for Element<F> {
//     fn pow(&self, exp: &Self) -> Self {
//         let tmp = self.0.pow(&exp.0.into_repr());

//         Element(tmp)
//     }
//     fn root(&self, root: &Self) -> Self {
//         unimplemented!()
//     }
//     fn sqrt(&self) -> Self {
//         unimplemented!()
//     }
// }


#[cfg(test)]
mod test {
    use super::*;

    use mathru::algebra::linear::{
        Matrix,
    };

    use mathru::algebra::linear::matrix::{
        Inverse,
    };

    #[test]
    fn test_construction() {
        use crate::pairing::bn256::Fr;
        use crate::pairing::ff::Field;
        use crate::bn256::Bn256PoseidonParams;

        let params = Bn256PoseidonParams::new_checked_2_into_1();

        let mut matrix = vec![];
        for el in params.mds_matrix.iter() {
            let el = Element(*el);
            matrix.push(el);
        }

        let matrix = Matrix::new(3, 3, matrix);

        let inv = matrix.inv();

        assert!(inv.is_some());

        let inv = inv.unwrap();
        let may_be_identity = matrix.clone() * inv;
        println!("{}", may_be_identity);

        assert!(may_be_identity == Matrix::one(3));
    }

    #[test]
    fn test_over_float() {
        let matrix = Matrix::new(3, 3, vec![1f64; 9]);

        let inv = matrix.inv();

        assert!(inv.is_some());
    }
}