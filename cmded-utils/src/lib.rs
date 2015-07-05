#![feature(core)]

extern crate core;
use self::core::num::ParseIntError;
use self::core::u64;

pub fn ranger(range: &str) -> Result<(u64, u64), ParseIntError> {
  let tokens: Vec<_> = range.split("..").collect();
  assert!(0 < tokens.len());
  let start = try!(tokens[0].parse::<u64>());
  let end =  try!(if 1 < tokens.len() {
    match tokens[1] { "$" => Ok(u64::MAX), _ => tokens[1].parse::<u64>() }
  } else { Ok(start) });
  Ok((start, end))
}

