use std::fmt;

#[derive(Debug, Clone)]
pub enum Flour {
    AP(f64),
    Bread(f64),
    WW(f64),
}

impl fmt::Display for Flour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (s, x) = match self {
            Flour::AP(x) => ("ap", x),
            Flour::Bread(x) => ("bread", x),
            Flour::WW(x) => ("whole wheat", x),
        };
        write!(f, "{} flour: {} grams", s, x)
    }
}
