#[macro_use]
extern crate diesel;
extern crate failure;

#[macro_use]
extern crate diesel_default_derive;
extern crate bigdecimal;

use failure::Fail;

pub mod tables;
mod db_test;

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
