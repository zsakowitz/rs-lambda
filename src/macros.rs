#[macro_export]
macro_rules! lambda {
    [&]   => { __And };
    [&=]  => { __AndEq };
    [&&]  => { __AndAnd };
    [^]   => { __Caret };
    [@]   => { __At };
    [:]   => { __Colon };
    [^=]  => { __CaretEq };
    [$]   => { __Dollar };
    [,]   => { __Comma };
    [..]  => { __DotDot };
    [.]   => { __Dot };
    [..=] => { __DotDotEq };
    [...] => { __DotDotDot };
    [==]  => { __EqEq };
    [=]   => { __Eq };
    [>=]  => { __Ge };
    [=>]  => { __FatArrow };
    [<-]  => { __LArrow };
    [>]   => { __Gt };
    [<]   => { __Lt };
    [<=]  => { __Le };
    [-=]  => { __MinusEq };
    [-]   => { __Minus };
    [!]   => { __Not };
    [!=]  => { __Ne };
    [|=]  => { __OrEq };
    [|]   => { __Or };
    [::]  => { __PathSep };
    [||]  => { __OrOr };
    [%=]  => { __PercentEq };
    [%]   => { __Percent };
    [+=]  => { __PlusEq };
    [+]   => { __Plus };
    [?]   => { __Question };
    [#]   => { __Pound };
    [;]   => { __Semi };
    [<<=] => { __ShlEq };
    [<<]  => { __Shl };
    [>>=] => { __ShrEq };
    [>>]  => { __Shr };
    [/=]  => { __SlashEq };
    [/]   => { __Slash };
    [*=]  => { __StarEq };
    [*]   => { __Star };
    [~]   => { __Tilde };

    ($x:literal) => { ::paste::paste! { [< __ $x >] } };

    (($($x:tt)+)) => {
        $crate::lambda!($($x)+)
    };

    ({ $x:ty }) => {
        $x
    };

    ($name:ident) => {
        $name
    };

    ($x:tt @ $($rest:tt)*) => {
        $crate::lambda!({ <$crate::lambda!($x) as $crate::value::Value>::Reduce } $($rest)*)
    };

    ($x:tt | $($rest:tt)*) => {
        $crate::lambda!({ <$crate::lambda!($x) as $crate::value::Value>::ReduceAlt } $($rest)*)
    };

    ($x:tt # $($rest:tt)*) => {
        <$crate::lambda!($x) as $crate::value::Value>::VALUE
    };

    ($param:ident -> $($body:tt)+) => {
        $crate::value::Lambda<$param, $crate::lambda!($($body)+)>
    };

    ($lhs:tt $rhs:tt $($rest:tt)*) => {
        $crate::lambda!(
            { $crate::value::Application<$crate::lambda!($lhs), $crate::lambda!($rhs)> }
            $($rest)*
        )
    };
}

#[macro_export]
macro_rules! single_lambda {
    ($vis:vis type & = $x:tt)   => { $vis type __And = $crate::lambda!($x); };
    ($vis:vis type &= = $x:tt)  => { $vis type __AndEq = $crate::lambda!($x); };
    ($vis:vis type && = $x:tt)  => { $vis type __AndAnd = $crate::lambda!($x); };
    ($vis:vis type ^ = $x:tt)   => { $vis type __Caret = $crate::lambda!($x); };
    ($vis:vis type @ = $x:tt)   => { $vis type __At = $crate::lambda!($x); };
    ($vis:vis type : = $x:tt)   => { $vis type __Colon = $crate::lambda!($x); };
    ($vis:vis type ^= = $x:tt)  => { $vis type __CaretEq = $crate::lambda!($x); };
    ($vis:vis type , = $x:tt)   => { $vis type __Comma = $crate::lambda!($x); };
    ($vis:vis type .. = $x:tt)  => { $vis type __DotDot = $crate::lambda!($x); };
    ($vis:vis type . = $x:tt)   => { $vis type __Dot = $crate::lambda!($x); };
    ($vis:vis type ..= = $x:tt) => { $vis type __DotDotEq = $crate::lambda!($x); };
    ($vis:vis type ... = $x:tt) => { $vis type __DotDotDot = $crate::lambda!($x); };
    ($vis:vis type == = $x:tt)  => { $vis type __EqEq = $crate::lambda!($x); };
    ($vis:vis type = = $x:tt)   => { $vis type __Eq = $crate::lambda!($x); };
    ($vis:vis type >= = $x:tt)  => { $vis type __Ge = $crate::lambda!($x); };
    ($vis:vis type => = $x:tt)  => { $vis type __FatArrow = $crate::lambda!($x); };
    ($vis:vis type <- = $x:tt)  => { $vis type __LArrow = $crate::lambda!($x); };
    ($vis:vis type > = $x:tt)   => { $vis type __Gt = $crate::lambda!($x); };
    ($vis:vis type < = $x:tt)   => { $vis type __Lt = $crate::lambda!($x); };
    ($vis:vis type <= = $x:tt)  => { $vis type __Le = $crate::lambda!($x); };
    ($vis:vis type -= = $x:tt)  => { $vis type __MinusEq = $crate::lambda!($x); };
    ($vis:vis type - = $x:tt)   => { $vis type __Minus = $crate::lambda!($x); };
    ($vis:vis type ! = $x:tt)   => { $vis type __Not = $crate::lambda!($x); };
    ($vis:vis type != = $x:tt)  => { $vis type __Ne = $crate::lambda!($x); };
    ($vis:vis type |= = $x:tt)  => { $vis type __OrEq = $crate::lambda!($x); };
    ($vis:vis type | = $x:tt)   => { $vis type __Or = $crate::lambda!($x); };
    ($vis:vis type :: = $x:tt)  => { $vis type __PathSep = $crate::lambda!($x); };
    ($vis:vis type || = $x:tt)  => { $vis type __OrOr = $crate::lambda!($x); };
    ($vis:vis type %= = $x:tt)  => { $vis type __PercentEq = $crate::lambda!($x); };
    ($vis:vis type % = $x:tt)   => { $vis type __Percent = $crate::lambda!($x); };
    ($vis:vis type += = $x:tt)  => { $vis type __PlusEq = $crate::lambda!($x); };
    ($vis:vis type + = $x:tt)   => { $vis type __Plus = $crate::lambda!($x); };
    ($vis:vis type ? = $x:tt)   => { $vis type __Question = $crate::lambda!($x); };
    ($vis:vis type # = $x:tt)   => { $vis type __Pound = $crate::lambda!($x); };
    ($vis:vis type ; = $x:tt)   => { $vis type __Semi = $crate::lambda!($x); };
    ($vis:vis type <<= = $x:tt) => { $vis type __ShlEq = $crate::lambda!($x); };
    ($vis:vis type << = $x:tt)  => { $vis type __Shl = $crate::lambda!($x); };
    ($vis:vis type >>= = $x:tt) => { $vis type __ShrEq = $crate::lambda!($x); };
    ($vis:vis type >> = $x:tt)  => { $vis type __Shr = $crate::lambda!($x); };
    ($vis:vis type /= = $x:tt)  => { $vis type __SlashEq = $crate::lambda!($x); };
    ($vis:vis type / = $x:tt)   => { $vis type __Slash = $crate::lambda!($x); };
    ($vis:vis type *= = $x:tt)  => { $vis type __StarEq = $crate::lambda!($x); };
    ($vis:vis type * = $x:tt)   => { $vis type __Star = $crate::lambda!($x); };
    ($vis:vis type ~ = $x:tt)   => { $vis type __Tilde = $crate::lambda!($x); };

    ($vis:vis type $name:literal = $x:tt) => {
        ::paste::paste! {
            #[allow(non_camel_case_types)]
            $vis type [< __ $name >] = $crate::lambda!($x);
        }
    };

    ($vis:vis type $name:ident = $x:tt) => {
        #[allow(non_camel_case_types)]
        $vis type $name = $crate::lambda!($x);
    };
}

#[macro_export]
macro_rules! lambdas {
    ($vis:vis, $($name:tt = $x:tt;)+) => {
        $($crate::single_lambda!($vis type $name = $x);)+
    };

    ($($name:tt = $x:tt;)+) => {
        $($crate::single_lambda!(type $name = $x);)+
    };
}
