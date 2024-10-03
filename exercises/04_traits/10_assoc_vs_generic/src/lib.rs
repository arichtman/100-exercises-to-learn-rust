// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

/*
I'm not clear on this one really.
*/

// Ok so generic trait over type T...
pub trait Power<T> {
    fn power(&self, exponent: T) -> Self;
}

// We're... allowed... to implement the trait for a type defined elsewhere because... we define the trait, I think.
impl Power<u32> for u32 {
    fn power(&self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

// And we need a different implementation for every type
impl Power<u16> for u32  {
    fn power(&self, exponent: u16) -> Self {
        // ... but `pow` requires a u32, and there's no deref trait linking u16 to u32 cause that's a big assumption
        self.pow(exponent.into())
    }
}
// but we need yet _another_ implementation for references passed.
// ... because how you'd work with an owned type is different
impl Power<&u32> for u32 {
    fn power(&self, exponent: &u32) -> Self {
        self.pow(*exponent)
    }
}

// Ok I think i get that a little better.
// Still not sure why they return owned Self but take &self as parameters

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
