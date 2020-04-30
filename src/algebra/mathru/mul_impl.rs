use super::Element;

use pairing::ff::{
    PrimeField
};

use mathru::algebra::abstr::{
    One,
    Identity,
    Multiplication,
    Monoid,
    MonoidMul,
    Semigroup,
    SemigroupMul,
    MagmaMul,
    Magma,
    Quasigroup,
    Loop,
    Group,
    GroupMul,
    AbelianGroup,
    AbelianGroupMul,
};

impl<F: PrimeField> One for Element<F> {
    fn one() -> Self {
        Element(F::one())
    }
}

impl<F: PrimeField> Identity<Multiplication> for Element<F> {
    fn id() -> Self {
        Element(F::one())
    }
}

impl<F: PrimeField> Magma<Multiplication> for Element<F> {
    fn operate(self, right: Self) -> Self {
        let mut tmp = self.0;
        tmp.mul_assign(&right.0);

        Element(tmp)
    }
}

impl<F: PrimeField> MagmaMul for Element<F> {}

impl<F: PrimeField> Semigroup<Multiplication> for Element<F> {
    fn is_associative(self, _y: Self, _z: Self) -> bool { true }
}

impl<F: PrimeField> SemigroupMul for Element<F> {}

impl<F: PrimeField> Monoid<Multiplication> for Element<F> {}

impl<F: PrimeField> MonoidMul for Element<F> {}

impl<F: PrimeField> Quasigroup<Multiplication> for Element<F> {}

impl<F: PrimeField> Loop<Multiplication> for Element<F> {}

impl<F: PrimeField> Group<Multiplication> for Element<F> {}

impl<F: PrimeField> GroupMul for Element<F> {}

impl<F: PrimeField> AbelianGroup<Multiplication> for Element<F> {}

impl<F: PrimeField> AbelianGroupMul for Element<F> {}