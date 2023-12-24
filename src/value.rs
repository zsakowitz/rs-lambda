use crate::{
    bool::{Bool, False, True},
    reify::Reified,
    set::{Empty, Set, With},
    uint::{Succ, Uint, Zero},
};
use std::marker::PhantomData;

pub struct Lambda<ThisName: Uint, ThisBody: Value>(PhantomData<ThisName>, PhantomData<ThisBody>);
pub struct Application<Lhs: Value, Rhs: Value>(PhantomData<Lhs>, PhantomData<Rhs>);

pub trait Value {
    type IsLambda: Bool;
    type CanReduce: Bool;
    type Locals: Set;
    type Reduce: Value;
    type ReduceAlt: Value;
    type IsFree<Name: Uint>: Bool;
    type Rename<OldName: Uint, NewName: Uint>: Value;
    type Replace<Name: Uint, NewNode: Value>: Value;
    type Apply<Rhs: Value>: Value;
    type ApplyAlt<Rhs: Value>: Value;

    const VALUE: Reified;
}

impl Value for Zero {
    type IsLambda = False;
    type CanReduce = False;
    type Locals = With<Zero, Empty>;
    type Reduce = Self;
    type ReduceAlt = Self;
    type IsFree<Name: Uint> = Name::Eq<Zero>;
    type Replace<Name: Uint, NewNode: Value> = <Name::Eq<Zero> as Bool>::ChooseNode<NewNode, Self>;
    type Rename<OldName: Uint, NewName: Uint> =
        <OldName::Eq<Zero> as Bool>::ChooseNode<NewName, Self>;
    type Apply<Rhs: Value> = Application<Self, Rhs::Reduce>;
    type ApplyAlt<Rhs: Value> = Application<Self, Rhs::Reduce>;

    const VALUE: Reified = Reified::Name(0);
}

impl<T: Uint> Value for Succ<T> {
    type IsLambda = False;
    type CanReduce = False;
    type Locals = With<Self, Empty>;
    type Reduce = Self;
    type ReduceAlt = Self;
    type IsFree<Name: Uint> = Name::Eq<Self>;
    type Replace<Name: Uint, NewNode: Value> = <Name::Eq<Self> as Bool>::ChooseNode<NewNode, Self>;
    type Rename<OldName: Uint, NewName: Uint> =
        <OldName::Eq<Self> as Bool>::ChooseNode<NewName, Self>;
    type Apply<Rhs: Value> = Application<Self, Rhs::Reduce>;
    type ApplyAlt<Rhs: Value> = Application<Self, Rhs::Reduce>;

    const VALUE: Reified = Reified::Name(Self::AS_USIZE);
}

impl<ThisName: Uint, ThisBody: Value> Value for Lambda<ThisName, ThisBody> {
    type IsLambda = True;
    type CanReduce = ThisBody::CanReduce;
    type Locals = <ThisBody::Locals as Set>::Add<ThisName>;
    type Reduce = Lambda<ThisName, ThisBody::Reduce>;
    type ReduceAlt = Lambda<ThisName, ThisBody::ReduceAlt>;
    type IsFree<Name: Uint> = <Name::Ne<ThisName> as Bool>::And<ThisBody::IsFree<Name>>;
    type Rename<OldName: Uint, NewName: Uint> = <ThisName::Eq<OldName> as Bool>::ChooseNode<
        Self,
        Lambda<ThisName, ThisBody::Rename<OldName, NewName>>,
    >;
    type Replace<Name: Uint, NewNode: Value> = <Name::Eq<ThisName> as Bool>::ChooseNode<
        Self,
        <NewNode::IsFree<ThisName> as Bool>::ChooseNode<
            Lambda<
                <<Self::Locals as Set>::Merge<NewNode::Locals> as Set>::UniqueName,
                <ThisBody::Rename<
                    ThisName,
                    <<Self::Locals as Set>::Merge<NewNode::Locals> as Set>::UniqueName,
                > as Value>::Replace<Name, NewNode>,
            >,
            Lambda<ThisName, ThisBody::Replace<Name, NewNode>>,
        >,
    >;
    type Apply<Rhs: Value> = ThisBody::Replace<ThisName, Rhs>;
    type ApplyAlt<Rhs: Value> = ThisBody::Replace<ThisName, Rhs>;

    const VALUE: Reified = Reified::Lambda(ThisName::AS_USIZE, &ThisBody::VALUE);
}

impl<Lhs: Value, Rhs: Value> Value for Application<Lhs, Rhs> {
    type IsLambda = False;
    type CanReduce = Lhs::IsLambda;
    type Locals = <Lhs::Locals as Set>::Merge<Rhs::Locals>;
    type Reduce = Lhs::Apply<Rhs>;
    type ReduceAlt = Lhs::ApplyAlt<Rhs>;
    type IsFree<Name: Uint> = <Lhs::IsFree<Name> as Bool>::Or<Rhs::IsFree<Name>>;
    type Rename<OldName: Uint, NewName: Uint> =
        Application<Lhs::Rename<OldName, NewName>, Rhs::Rename<OldName, NewName>>;
    type Replace<Name: Uint, NewNode: Value> =
        Application<Lhs::Replace<Name, NewNode>, Rhs::Replace<Name, NewNode>>;
    type Apply<Rhs2: Value> = <Rhs2::CanReduce as Bool>::ChooseNode<
        Application<Self, Rhs2::Reduce>,
        Application<Self::Reduce, Rhs2>,
    >;
    type ApplyAlt<Rhs2: Value> = <Self::CanReduce as Bool>::ChooseNode<
        Application<Self::Reduce, Rhs2>,
        Application<Self, Rhs2::Reduce>,
    >;

    const VALUE: Reified = Reified::Apply(&Lhs::VALUE, &Rhs::VALUE);
}
