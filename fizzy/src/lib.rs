use std::ops::Rem;
use std::convert::TryInto;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool + 'static>,
    subs: &'static str,
}

impl<T> Matcher<T> {
    pub fn new(matcher: impl Fn(T) -> bool + 'static, subs: &'static str) -> Matcher<T> {
        Matcher {
            matcher: Box::new(matcher),
            subs,
        }
    }

    pub fn does_match(&self, n: T) -> bool {
        (self.matcher)(n)
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: vec![],
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item=T>) -> impl Iterator<Item=String> {
        iter.map(move |n| {
            let result = self.get_match(n);
            if !result.is_empty() {
                result
            } else {
                n.to_string()
            }
        })
    }

    fn get_match(&self, n: T) -> String {
        self.matchers
            .iter()
            .filter(|&m| m.does_match(n))
            .map(|m| m.subs)
            .collect::<String>()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T:ToString + Copy + From<u8> + Rem<Output=T> + PartialEq>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
