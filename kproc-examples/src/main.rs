use kproc_macros_examples::{derive_impl, RustBuilder};

trait GenTrait {}

#[derive(RustBuilder, Clone)]
// #[build_struct] // FIXME: support the parsing of this too
pub struct Foo {
    #[build]
    attr: String,
    self_ref: u32,
}

#[derive(RustBuilder)]
pub struct Boo {
    attr: String,
    self_ref: u32,
    pub gen: Vec<Foo>,
}

#[derive(RustBuilder)]
pub struct BooLifetime<'a> {
    attr: String,
    self_ref: u32,
    gen: Vec<&'a Foo>,
}

#[derive(RustBuilder)]
pub struct BooLifetimeDyn<'a> {
    attr: String,
    self_ref: u32,
    gen: Vec<&'a dyn GenTrait>,
}

#[derive(RustBuilder)]
pub struct BooComplex {
    pub gen: Vec<Foo>,
    attr: String,
    self_ref: u32,
}

struct ForImplDerive {}

#[derive_impl]
impl ForImplDerive {}

fn main() {
    let obj = Foo {
        attr: "Alibaba".to_string(),
        self_ref: 0,
    };
    assert_eq!(obj.get_attr(), obj.attr);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let obj = crate::Foo {
            attr: "Alibaba".to_string(),
            self_ref: 0,
        };
        assert_eq!(obj.get_attr(), obj.attr);
    }

    #[test]
    fn generics_works() {
        let obj = crate::Boo {
            attr: "Alibaba".to_string(),
            self_ref: 0,
            gen: vec![],
        };
        assert!(obj.gen.is_empty());
    }

    #[test]
    fn generics_with_dyn_works() {
        let obj = crate::BooLifetimeDyn {
            attr: "Alibaba".to_string(),
            self_ref: 0,
            gen: vec![],
        };
        assert!(obj.gen.is_empty());
    }
}
