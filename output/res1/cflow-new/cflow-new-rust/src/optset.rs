use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optset;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptsetError {
    InvalidInteger,
    OutOfRange,
    MissingArgument,
}

impl Optset {
    pub fn get_signed_int(
        arg: &str,
        min: i128,
        max: i128,
    ) -> Result<i128, OptsetError> {
        let value = arg
            .parse::<i128>()
            .map_err(|_| OptsetError::InvalidInteger)?;

        if value < min || value > max {
            return Err(OptsetError::OutOfRange);
        }

        Ok(value)
    }

    pub fn get_unsigned_int(arg: &str, max: u128) -> Result<u128, OptsetError> {
        let value = arg
            .parse::<u128>()
            .map_err(|_| OptsetError::InvalidInteger)?;

        if value > max {
            return Err(OptsetError::OutOfRange);
        }

        Ok(value)
    }

    pub fn incr(value: &RefCell<i32>, arg: Option<&str>) -> Result<(), OptsetError> {
        if arg.is_some() {
            return Err(OptsetError::InvalidInteger);
        }
        *value.borrow_mut() += 1;
        Ok(())
    }

    pub fn string_copy(target: &RefCell<Option<String>>, arg: Option<&str>) -> Result<(), OptsetError> {
        *target.borrow_mut() = arg.map(ToOwned::to_owned);
        Ok(())
    }

    pub fn string(target: &RefCell<Option<String>>, arg: Option<&str>) -> Result<(), OptsetError> {
        let arg = arg.ok_or(OptsetError::MissingArgument)?;
        *target.borrow_mut() = Some(arg.to_owned());
        Ok(())
    }

    pub fn string_alloc(
        target: &RefCell<Option<String>>,
        arg: Option<&str>,
    ) -> Result<(), OptsetError> {
        let arg = arg.ok_or(OptsetError::MissingArgument)?;
        *target.borrow_mut() = Some(arg.to_owned());
        Ok(())
    }

    pub fn r#true(target: &RefCell<bool>, _arg: Option<&str>) -> Result<(), OptsetError> {
        *target.borrow_mut() = true;
        Ok(())
    }

    pub fn r#false(target: &RefCell<bool>, _arg: Option<&str>) -> Result<(), OptsetError> {
        *target.borrow_mut() = false;
        Ok(())
    }

    pub fn r#bool(target: &RefCell<bool>, arg: &str) -> Result<(), OptsetError> {
        *target.borrow_mut() = arg.starts_with('1');
        Ok(())
    }
}
