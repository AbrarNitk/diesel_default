#[macro_use]
extern crate diesel;
extern crate failure;

#[macro_use]
extern crate diesel_mate_derive;
extern crate bigdecimal;

use failure::Fail;

mod db_test;
mod endec;
pub mod tables;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Value error")]
    ValueError,
}

pub type Result<T> = std::result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
