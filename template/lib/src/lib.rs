pub extern crate miette;
pub extern crate thiserror;

pub mod errors;

use errors::*;

pub type Result<T> = std::result::Result<T, AppError>;

pub fn say_hello(name: String) -> Result<()> {
    if name.is_empty() {
        Err(AppError::Unknown("Empty String".into(), "Name cannot be empty".into()))
    } else {
        println!("Hello {}", name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
