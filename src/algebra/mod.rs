use alga::general::{
    Additive,
    Multiplicative,
    AbstractRing,
    AbstractRingCommutative,
    AbstractField
};

use approx::{
    RelativeEq
};

use pairing::ff::{
    PrimeField
};

mod multiplicative_impl;
mod additive_impls;
mod num_impls;
mod mathru;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Element<F: PrimeField>(F);

impl<F: PrimeField> std::fmt::Display for Element<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<F: PrimeField> std::cmp::PartialOrd for Element<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.into_repr().cmp(&other.0.into_repr()))
    }
}

impl<F: PrimeField> std::cmp::Ord for Element<F> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.into_repr().cmp(&other.0.into_repr())
    }
}

impl<F: PrimeField> AbstractRing<Additive, Multiplicative> for Element<F> {
    fn prop_mul_and_add_are_distributive_approx(
        _args: (Self, Self, Self)
    ) -> bool
    where
        Self: RelativeEq,
    { true }
    fn prop_mul_and_add_are_distributive(_args: (Self, Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}

impl<F: PrimeField> AbstractRingCommutative<Additive, Multiplicative> for Element<F> {
    fn prop_mul_is_commutative_approx(_args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }
    fn prop_mul_is_commutative(_args: (Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}

impl<F: PrimeField> AbstractField<Additive, Multiplicative> for Element<F> {}


// impl<F: PrimeField> Num for Element<F> {}



#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_construction() {
    //     use crate::pairing::bn256::Fr;

    //     let one = Element(Fr::one());
        
    //     let matrix = 

    //     let matrix = Matrix3::new(Fr::one(), Fr::one(), Fr::one(), Fr::one(), Fr::one(), Fr::one(), Fr::one(), Fr::one(), Fr::one());
    //     let invertible = matrix.is_invertible();
    // }
}