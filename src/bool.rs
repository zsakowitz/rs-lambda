use crate::{
    set::Set,
    uint::{Succ, Uint},
    value::Value,
};

pub struct True;
pub struct False;

pub trait Bool {
    type Not: Bool;
    type And<T: Bool>: Bool;
    type Or<T: Bool>: Bool;
    type Xor<T: Bool>: Bool;
    type Eq<T: Bool>: Bool;

    type UniqueName<CurrentMinimum: Uint, AvoidList: Set>: Uint;

    type ChooseBool<A: Bool, B: Bool>: Bool;
    type ChooseUint<A: Uint, B: Uint>: Uint;
    type ChooseNode<A: Value, B: Value>: Value;
    type ChooseSet<A: Set, B: Set>: Set;
}

impl Bool for True {
    type Not = False;
    type And<T: Bool> = T;
    type Or<T: Bool> = True;
    type Xor<T: Bool> = T::Not;
    type Eq<T: Bool> = T;

    type UniqueName<CurrentMinimum: Uint, AvoidList: Set> =
        AvoidList::UniqueNameStartingAt<Succ<CurrentMinimum>>;

    type ChooseBool<A: Bool, B: Bool> = A;
    type ChooseUint<A: Uint, B: Uint> = A;
    type ChooseNode<A: Value, B: Value> = A;
    type ChooseSet<A: Set, B: Set> = A;
}

impl Bool for False {
    type Not = True;
    type And<T: Bool> = False;
    type Or<T: Bool> = T;
    type Xor<T: Bool> = T;
    type Eq<T: Bool> = T::Not;

    type UniqueName<CurrentMinimum: Uint, AvoidList: Set> = CurrentMinimum;

    type ChooseBool<A: Bool, B: Bool> = B;
    type ChooseUint<A: Uint, B: Uint> = B;
    type ChooseNode<A: Value, B: Value> = B;
    type ChooseSet<A: Set, B: Set> = B;
}
