#![allow(unused)]

use std::str::pattern::Pattern;

#[derive(Clone, Debug)]
pub struct SmartIter<'a> {
    data: &'a str,
}

impl<'a> From<&'a str> for SmartIter<'a> {
    /// Converts a `&str` into a `SmartIter`.
    ///
    /// # Help
    ///
    /// Should be a one liner
    fn from(data: &'a str) -> Self {
        unimplemented!()
    }
}

impl<'a> Iterator for SmartIter<'a> {
    type Item = char;

    /// Implements the `next` function on `SmartIter`.
    ///
    /// # Help
    ///
    /// There are many different ways to implement this function. _Having said that_ ;), the
    /// functions below might be helpful.
    ///
    /// ```
    /// std::str::chars(&self) -> Chars;
    /// std::str::Chars::next(&self) -> Option<char>;
    /// std::str::Chars::as_str(&self) -> &str;
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'a> SmartIter<'a> {
    /// Returns the portion of the `str` that `SmartIter` has yet to consume.
    ///
    /// # Help
    ///
    /// Should be a one liner.
    pub fn as_str(&self) -> &'a str {
        unimplemented!()
    }

    /// Gets a the portion of the `str` that occurs before the pattern `pat`.
    ///
    /// # Help
    ///
    /// This function _find_s the location where some `Pattern` occurs, returns everything that
    /// occurs before that pattern, and modifies `self` to refer to the remainder of the `&str` -
    /// starting with the part of the `str` matching the pattern. If the pattern is not found, it
    /// returns `None`.
    ///
    /// The `std::str::split` family of functions will be helpful.
    pub fn take_until<'b, P>(&'b mut self, pat: P) -> Option<&'a str>
    where
        P: Pattern<'b>,
    {
        unimplemented!()
    }

    /// Same as `take_until` except it consumes (and returns) the entire string if the pattern is
    /// not found.
    ///
    /// # Help
    ///
    /// This is intentionally very difficult, and probably best to attempt last, although many
    /// functions depend upon it.
    ///
    /// This function should behave the same as `take_until`, except if the pattern is not found, it
    /// consumes the entire `str`, leaving `self` with the empty `str`.
    ///
    /// You must implement this using `SmartIter::take_until` and cannot mention the `'static`
    /// lifetime. Other than that, you are free to implement this however you want - including
    /// changing anything to do with lifetimes in the function signature.
    pub fn take_until_or_end<'b, P>(&'b mut self, pat: P) -> &'a str
    where
        P: Pattern<'b>,
    {
        unimplemented!()
    }

    /// Takes the next alphabetic substr off the underlying `str`.
    ///
    /// If the underlying `str` does not start with an alphabetic character, or is empty, it returns
    /// `None`.
    ///
    /// # Help
    ///
    /// Implement this using `take_until_or_end`.
    pub fn take_word(&mut self) -> Option<&'a str> {
        unimplemented!()
    }

    /// Skips all whitespace at the start of the underlying `str`.
    ///
    /// # Help
    ///
    /// Implement this using `take_until_or_end`.
    pub fn skip_whitespace(&mut self) {
        unimplemented!()
    }
}
