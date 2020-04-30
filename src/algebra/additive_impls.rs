use alga::general::{
    Identity,
    Additive,
    AbstractMagma,
    AbstractSemigroup,
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

impl<F: PrimeField> Identity<Additive> for Element<F> {
    fn identity() -> Self {
        Element(F::zero())
    }
}

impl<F: PrimeField> AbstractMagma<Additive> for Element<F> {
    fn operate(&self, right: &Self) -> Self {
        let mut tmp = self.0;
        tmp.add_assign(&right.0);

        Element(tmp)
    }
}

impl<F: PrimeField> AbstractSemigroup<Additive> for Element<F> {
    fn prop_is_associative_approx(_args: (Self, Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }

    fn prop_is_associative(_args: (Self, Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}

impl<F: PrimeField> TwoSidedInverse<Additive> for Element<F> {
    fn two_sided_inverse(&self) -> Self {
        let mut tmp = self.0;
        tmp.negate();

        Element(tmp)
    }

    fn two_sided_inverse_mut(&mut self) { 
        let mut tmp = self.0;
        tmp.negate();

        self.0 = tmp;
    }
}

impl<F: PrimeField> AbstractQuasigroup<Additive> for Element<F> {
    fn prop_inv_is_latin_square_approx(_args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }
    fn prop_inv_is_latin_square(_args: (Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}

impl<F: PrimeField> AbstractMonoid<Additive> for Element<F> {
    fn prop_operating_identity_element_is_noop_approx(args: (Self,)) -> bool
    where
        Self: RelativeEq,
    { 
        (args.0).0.is_zero()
    }
    fn prop_operating_identity_element_is_noop(args: (Self,)) -> bool
    where
        Self: Eq,
    {
        (args.0).0.is_zero()
    }
}

impl<F: PrimeField> AbstractLoop<Additive> for Element<F> {}

impl<F: PrimeField> AbstractGroup<Additive> for Element<F> {}

impl<F: PrimeField> AbstractGroupAbelian<Additive> for Element<F> {
    fn prop_is_commutative_approx(_args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    { true }
    fn prop_is_commutative(_args: (Self, Self)) -> bool
    where
        Self: Eq,
    { true }
}