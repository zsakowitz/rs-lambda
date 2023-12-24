use crate::{
    bool::{Bool, False},
    uint::{Uint, Zero},
};
use std::marker::PhantomData;

pub struct Empty;
pub struct With<A, B>(PhantomData<A>, PhantomData<B>);

pub trait Set {
    type Has<T: Uint>: Bool;
    type Add<T: Uint>: Set;
    type Remove<T: Uint>: Set;
    type Merge<T: Set>: Set;
    type UniqueNameStartingAt<T: Uint>: Uint;
    type UniqueName: Uint;
}

impl Set for Empty {
    type Has<T: Uint> = False;
    type Add<T: Uint> = With<T, Empty>;
    type Remove<T: Uint> = Empty;
    type Merge<T: Set> = T;
    type UniqueNameStartingAt<T: Uint> = Zero;
    type UniqueName = Zero;
}

impl<A: Uint, B: Set> Set for With<A, B> {
    type Has<T: Uint> = <A::Eq<T> as Bool>::Or<B::Has<T>>;
    type Add<T: Uint> = <A::Eq<T> as Bool>::ChooseSet<Self, With<A, B::Add<T>>>;
    type Remove<T: Uint> = <A::Eq<T> as Bool>::ChooseSet<B::Remove<T>, With<A, B::Remove<T>>>;
    type Merge<T: Set> = With<A, B::Merge<T>>;
    type UniqueNameStartingAt<T: Uint> = <Self::Has<T> as Bool>::UniqueName<T, Self>;
    type UniqueName = Self::UniqueNameStartingAt<Zero>;
}
