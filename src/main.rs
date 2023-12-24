#![feature(const_type_name)]

use crate::{reify::Reified, uint::alpha::*};

pub mod bool;
pub mod macros;
pub mod reify;
pub mod set;
pub mod uint;
pub mod value;

lambdas! {
    pub,

    id    = ( x -> x );
    true  = ( a -> b -> a );
    false = ( a -> b -> a );
    succ  = ( n -> f -> x -> f (n f x) );
    0     = ( f -> x -> x );
    1     = ( succ 0 );
    2     = ( succ 1 );
    3     = ( succ 2 );
    4     = ( succ 3 );
    +     = ( a -> b -> f -> x -> b f (a f x) );
}

pub const VALUE: Reified = lambda!(+ 3 4 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ #);

fn main() {
    println!("{VALUE}");
}
