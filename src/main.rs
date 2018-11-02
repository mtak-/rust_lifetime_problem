//! # rust programming problem focusing on teaching lifetimes
//!
//! ## Pieces
//!
//! * `SmartIter` - an iterator over `&str` focused on parsing strings.
//! * `Person` - a struct containing a first name and last name.
//!     * it has a `TryFrom<&str>` implementation that uses `SmartIter` to create a `Person`.
//! * `PersonIter` - an iterator that lazily parses a `&str`, yielding `Result<Person,
//!   InvalidPerson>` as it goes.
//!
//! ## Restrictions
//!
//! Don't create `String` objects or `Vec`s or any other container. The solutions should perform no
//! allocations.
//!
//! ## Tests
//!
//! Run the tests to verify your solutions are correct.

#![feature(try_from)]
#![feature(pattern)]
#![allow(unused)]

pub mod smart_iter;
#[cfg(test)]
mod tests;

use self::smart_iter::SmartIter;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub struct Person<'a> {
    first_name: &'a str,
    last_name:  &'a str,
}

impl<'a> TryFrom<&mut SmartIter<'a>> for Person<'a> {
    type Error = InvalidPerson;

    /// Parses a &str into a person.
    ///
    /// # Format
    ///
    /// "last_name, first_name" with arbitrary whitespace in between/before tokens.
    ///
    /// # Help
    ///
    /// Use the `SmartIter` functions to make this easier. The `?` operator can be used to early
    /// return in the case of an error.
    ///
    /// This function is extremely useful for converting an `Option<T>` into a `Result<T, E>`.
    ///
    /// ```
    /// std::Option::ok_or<E>(E) -> Result<T, E>;
    /// ```
    fn try_from(smart_iter: &mut SmartIter<'a>) -> Result<Person<'a>, InvalidPerson> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct PersonIter<'a> {
    iter: SmartIter<'a>,
}

impl<'a> From<&'a str> for PersonIter<'a> {
    /// Converts a `&str` into an iterator over `People`.
    ///
    /// # Help
    ///
    /// This should be a one liner
    fn from(data: &'a str) -> PersonIter<'a> {
        unimplemented!()
    }
}

impl<'a> Iterator for PersonIter<'a> {
    type Item = Result<Person<'a>, InvalidPerson>;

    /// assume people are separated by `;` and some optional whitespace
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

fn main() {}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidPerson;
