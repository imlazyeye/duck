use crate::{LintCategory, XLint};

pub struct WithLoop;
impl XLint for WithLoop {
    fn tag(&self) -> &str {
        "with_loop"
    }

    fn display_name(&self) -> &str {
        "Use of `with`"
    }

    fn explanation(&self) -> &str {
        "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec![
            "Use `instance_find` if looping over objects",
            "Use direct dot reference `foo.bar` to manipulate single objects",
        ]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
