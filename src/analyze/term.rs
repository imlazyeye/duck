use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Error,
    Type(Type),
    Marker(Marker),
    App(App),
    // Trait(Trait),
}
