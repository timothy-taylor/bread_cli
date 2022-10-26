use std::fmt;

#[derive(Debug)]
pub enum Yeast {
    Instant(f64),
    Starter(f64),
}

impl fmt::Display for Yeast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (s, x) = match self {
            Yeast::Instant(x) => ("instant yeast", x),
            Yeast::Starter(x) => ("sourdough starter", x),
        };
        write!(f, "{}: {}", s, x)
    }
}
