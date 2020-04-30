use alga::general::{
    Identity,
    AbstractMagma,
    AbstractSemigroup,
    Multiplicative,
    TwoSidedInverse,
    AbstractQuasigroup,
    AbstractMonoid,
    AbstractGroup,
    AbstractLoop,
    AbstractGroupAbelian,
};

use approx::{
    RelativeEq
};

use pairing::ff::{
    PrimeField
};

use super::Element;

impl<F: PrimeField> Identity<Multiplicative> for Element<F> {
    fn identity() -> Self {
        Element(F::one())
    }
}

impl<F: PrimeField> AbstractMagma<Multiplicative> for Element<F> {
    fn operate(&self, right: &Self) -> Self {
        let mut tmp = self.0;
        tmp.mul_assign(&right.0);

        Element(tmp)
    }
}

impl<F: PrimeField> AbstractSemigroup<Multiplicative> for Element<F> {
    fn prop_is_associative_approx(_args: (Self, Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }

    fn prop_is_associative(_args: (Self, Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}

impl<F: PrimeField> TwoSidedInverse<Multiplicative> for Element<F> {
    fn two_sided_inverse(&self) -> Self {
        let inv = self.0.inverse().unwrap();

        Element(inv)
    }

    fn two_sided_inverse_mut(&mut self) { 
        let inv = self.0.inverse().unwrap();

        self.0 = inv;
    }
}

impl<F: PrimeField> AbstractQuasigroup<Multiplicative> for Element<F> {
    fn prop_inv_is_latin_square_approx(_args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }
    fn prop_inv_is_latin_square(_args: (Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}

impl<F: PrimeField> AbstractMonoid<Multiplicative> for Element<F> {
    fn prop_operating_identity_element_is_noop_approx(args: (Self,)) -> bool
    where
        Self: RelativeEq,
    { 
        let one = F::one();
        (args.0).0 == one
    }
    fn prop_operating_identity_element_is_noop(args: (Self,)) -> bool
    where
        Self: Eq,
    {
        let one = F::one();
        (args.0).0 == one
    }
}

impl<F: PrimeField> AbstractLoop<Multiplicative> for Element<F> {}

impl<F: PrimeField> AbstractGroup<Multiplicative> for Element<F> {}

impl<F: PrimeField> AbstractGroupAbelian<Multiplicative> for Element<F> {
    fn prop_is_commutative_approx(_args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }
    fn prop_is_commutative(_args: (Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}