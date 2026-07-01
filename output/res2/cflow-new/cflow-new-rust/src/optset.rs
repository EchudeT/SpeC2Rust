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

    pub fn incr(value: &mut i32) {
        *value += 1;
    }

    pub fn string_copy(slot: &mut Option<String>, arg: Option<String>) -> bool {
        match arg {
            Some(text) => {
                *slot = Some(text);
                true
            }
            None => false,
        }
    }

    pub fn string(slot: &mut Option<String>, arg: Option<&str>) -> Result<(), ()> {
        let text = arg.ok_or(())?;
        *slot = Some(text.to_owned());
        Ok(())
    }

    pub fn string_alloc(slot: &mut Option<String>, arg: Option<&str>) -> Result<(), ()> {
        let text = arg.ok_or(())?;
        *slot = Some(text.to_owned());
        Ok(())
    }

    pub fn r#true(value: &mut bool) {
        *value = true;
    }

    pub fn r#false(value: &mut bool) {
        *value = false;
    }

    pub fn bool(value: &mut bool, arg: &str) {
        *value = arg.as_bytes().first().copied() == Some(b'1');
    }
}
