pub struct Optset;

impl Optset {
    pub fn get_signed_int(arg: &str, min: i64, max: i64) -> Option<i64> {
        let value = arg.parse::<i64>().ok()?;
        if value < min || value > max {
            None
        } else {
            Some(value)
        }
    }

    pub fn get_unsigned_int(arg: &str, max: u64) -> Option<u64> {
        let value = arg.parse::<u64>().ok()?;
        if value > max {
            None
        } else {
            Some(value)
        }
    }

    pub fn incr(target: &mut i32) {
        *target += 1;
    }

    pub fn string_copy(target: &mut Option<String>, arg: Option<&str>) {
        *target = arg.map(str::to_owned);
    }

    pub fn string(target: &mut Option<String>, arg: Option<&str>) -> Result<(), ()> {
        match arg {
            Some(value) => {
                *target = Some(value.to_owned());
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn string_alloc(target: &mut Option<String>, arg: Option<&str>) -> Result<(), ()> {
        match arg {
            Some(value) => {
                *target = Some(value.to_owned());
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn r#true(target: &mut bool) {
        *target = true;
    }

    pub fn r#false(target: &mut bool) {
        *target = false;
    }

    pub fn bool(target: &mut bool, arg: &str) {
        *target = arg.starts_with('1');
    }
}
