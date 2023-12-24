use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Reified {
    Name(usize),
    Apply(&'static Reified, &'static Reified),
    Lambda(usize, &'static Reified),
}

struct PartialString {
    value: String,
    ends_in_lambda: bool,
    is_application: bool,
}

fn to_partial(value: &Reified) -> PartialString {
    match value {
        Reified::Name(value) => PartialString {
            value: value.to_string(),
            ends_in_lambda: false,
            is_application: false,
        },

        Reified::Lambda(name, body) => PartialString {
            value: format!("\\{name} {}", to_partial(body).value),
            ends_in_lambda: true,
            is_application: false,
        },

        Reified::Apply(lhs, rhs) => {
            let lhs = to_partial(lhs);
            let rhs = to_partial(rhs);

            let mut value = if lhs.ends_in_lambda {
                format!("({})", lhs.value)
            } else {
                lhs.value
            };

            value += " ";

            value += &if rhs.is_application {
                format!("({})", rhs.value)
            } else {
                rhs.value
            };

            PartialString {
                value,
                ends_in_lambda: rhs.ends_in_lambda && !rhs.is_application,
                is_application: true,
            }
        }
    }
}

impl Display for Reified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &to_partial(self).value)
    }
}
