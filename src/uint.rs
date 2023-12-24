use crate::{
    bool::{Bool, False, True},
    value::Value,
};
use std::marker::PhantomData;

pub struct Zero;
pub struct Succ<T>(PhantomData<T>);

pub trait Uint: Value {
    type Next: Uint;
    type Prev: Uint;

    type Add<T: Uint>: Uint;
    type Sub<T: Uint>: Uint;
    type MultPlus<T: Uint, U: Uint>: Uint;
    type Mult<T: Uint>: Uint;

    type IsZero: Bool;
    type Eq<T: Uint>: Bool;
    type Ne<T: Uint>: Bool;

    const AS_USIZE: usize;
}

impl Uint for Zero {
    type Next = Succ<Self>;
    type Prev = Zero;

    type Add<T: Uint> = T;
    type Sub<T: Uint> = Zero;
    type MultPlus<T: Uint, U: Uint> = U;
    type Mult<T: Uint> = Zero;

    type IsZero = True;
    type Eq<T: Uint> = T::IsZero;
    type Ne<T: Uint> = <T::IsZero as Bool>::Not;

    const AS_USIZE: usize = 0;
}

impl<T: Uint> Uint for Succ<T> {
    type Next = Succ<Self>;
    type Prev = T;

    type Add<U: Uint> = Succ<T::Add<U>>;
    type Sub<U: Uint> = <U::IsZero as Bool>::ChooseUint<Self, T::Sub<U::Prev>>;
    type MultPlus<U: Uint, V: Uint> = T::MultPlus<U, U::Add<V>>;
    type Mult<U: Uint> = Self::MultPlus<U, Zero>;

    type IsZero = False;
    type Eq<U: Uint> = <U::IsZero as Bool>::ChooseBool<False, T::Eq<U::Prev>>;
    type Ne<U: Uint> = <U::IsZero as Bool>::ChooseBool<True, T::Ne<U::Prev>>;

    const AS_USIZE: usize = T::AS_USIZE + 1;
}

#[allow(non_camel_case_types)]
pub mod alpha {
    use super::{Succ, Zero};

    pub type a = Zero;
    pub type b = Succ<a>;
    pub type c = Succ<b>;
    pub type d = Succ<c>;
    pub type e = Succ<d>;
    pub type f = Succ<e>;
    pub type g = Succ<f>;
    pub type h = Succ<g>;
    pub type i = Succ<h>;
    pub type j = Succ<i>;
    pub type k = Succ<j>;
    pub type l = Succ<k>;
    pub type m = Succ<l>;
    pub type n = Succ<m>;
    pub type o = Succ<n>;
    pub type p = Succ<o>;
    pub type q = Succ<p>;
    pub type r = Succ<q>;
    pub type s = Succ<r>;
    pub type t = Succ<s>;
    pub type u = Succ<t>;
    pub type v = Succ<u>;
    pub type w = Succ<v>;
    pub type x = Succ<w>;
    pub type y = Succ<x>;
    pub type z = Succ<y>;
}
