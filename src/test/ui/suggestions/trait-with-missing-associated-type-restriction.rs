// These are all the possible variations of this error I could think of for.
// `trait-with-missing-associated-type-restriction-fixable.rs` contains the subset of these that
// can be fixed with `rustfix`.

trait Trait<T = Self> {
    type A;

    fn func(&self) -> Self::A;
    fn funk(&self, _: Self::A);
    fn funq(&self) -> Self::A {} //~ ERROR mismatched types
}

fn foo(_: impl Trait, x: impl Trait) {
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn bar<T: Trait>(x: T) {
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn foo2(x: impl Trait<i32>) {
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn bar2<T: Trait<i32>>(x: T) {
    x.funk(3); //~ ERROR arguments to this function are incorrect
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn baz<D: std::fmt::Debug, T: Trait<A = D>>(x: T) {
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn bat(x: &mut dyn Trait<(), A = ()>) {
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn ban<T>(x: T) where T: Trait {
    qux(x.func()) //~ ERROR arguments to this function are incorrect
}

fn qux(_: usize) {}

fn main() {}
