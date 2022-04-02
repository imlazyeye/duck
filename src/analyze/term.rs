use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    Trait(Trait),
}

impl Term {
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Term::App(App::Object(obj)) => Some(obj),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Term::App(App::Object(obj)) => Some(obj),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<Object> {
        match self {
            Term::App(App::Object(obj)) => Some(obj),
            _ => None,
        }
    }
}
