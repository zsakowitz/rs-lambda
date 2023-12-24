#![feature(const_type_name)]
#![recursion_limit = "4096"]

use crate::{reify::Reified, uint::alpha::*};

pub mod bool;
pub mod macros;
pub mod reify;
pub mod set;
pub mod uint;
pub mod value;

lambdas! {
    pub,

    id      = ( x -> x                             );
    true    = ( a -> b -> a                        );
    false   = ( a -> b -> b                        );
    succ    = ( n -> f -> x -> f (n f x)           );
    0       = ( f -> x -> x                        );
    1       = ( succ 0                             );
    2       = ( succ 1                             );
    3       = ( succ 2                             );
    4       = ( succ 3                             );
    +       = ( a -> b -> f -> x -> b f (a f x)    );
    *       = ( a -> b -> f -> x -> a (x->b f x) x );
    Y       = ( f -> (x -> f (x x)) (x -> f (x x)) );
    pair    = ( a -> b -> f -> f a b               );
    head    = ( p -> p true                        );
    tail    = ( p -> p false                       );
    prev    = ( n -> n
        (p -> pair (tail p) (succ (tail p)))
        (pair 0 0)
        true
    );
    is_zero = ( n -> n (q -> false) true           );
    M       = ( x -> x x                           );
    fact    = ( Y (f -> p -> n -> is_zero n p (f (* n p) (prev n)) ) 1 );
}

pub const VALUE: Reified = lambda!(
    fact 3
    @|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|@|
    #
);

fn main() {
    println!("{VALUE}");
}
