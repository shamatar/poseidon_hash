use super::Element;

use pairing::ff::{
    PrimeField
};

use mathru::algebra::abstr::{
    Zero,
    Identity,
    Addition,
    Monoid,
    MonoidAdd,
    Semigroup,
    SemigroupAdd,
    MagmaAdd,
    Magma,
    Quasigroup,
    Loop,
    Group,
    GroupAdd,
    AbelianGroup,
    AbelianGroupAdd,
};

impl<F: PrimeField> Zero for Element<F> {
    fn zero() -> Self {
        Element(F::zero())
    }
}

impl<F: PrimeField> Identity<Addition> for Element<F> {
    fn id() -> Self {
        Element(F::zero())
    }
}

impl<F: PrimeField> Magma<Addition> for Element<F> {
    fn operate(self, right: Self) -> Self {
        let mut tmp = self.0;
        tmp.add_assign(&right.0);

        Element(tmp)
    }
}

impl<F: PrimeField> MagmaAdd for Element<F> {}

impl<F: PrimeField> Semigroup<Addition> for Element<F> {
    fn is_associative(self, _y: Self, _z: Self) -> bool { true }
}

impl<F: PrimeField> SemigroupAdd for Element<F> {}

impl<F: PrimeField> Monoid<Addition> for Element<F> {}

impl<F: PrimeField> MonoidAdd for Element<F> {}

impl<F: PrimeField> Quasigroup<Addition> for Element<F> {}

impl<F: PrimeField> Loop<Addition> for Element<F> {}

impl<F: PrimeField> Group<Addition> for Element<F> {}

impl<F: PrimeField> GroupAdd for Element<F> {}

impl<F: PrimeField> AbelianGroup<Addition> for Element<F> {}

impl<F: PrimeField> AbelianGroupAdd for Element<F> {}