#![allow(dead_code)]
use icha_lib::{IchaError, IchaResult };

pub fn main() -> IchaResult<()> {
    icha_lib::init()?;
    Ok(())
}
