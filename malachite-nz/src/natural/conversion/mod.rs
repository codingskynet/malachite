use error::ParseIntegerError;
use malachite_base::num::{Assign, DivAssignMod, Zero};
use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

impl Natural {
    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: i32) -> Result<(), ParseIntegerError> {
        assert!(!src.starts_with('-'));
        self.assign(Limb::ZERO);
        for c in src.chars() {
            *self *= radix as Limb;
            if c >= '0' && c <= '9' {
                *self += c as Limb - 48;
            }
        }
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: i32) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::ZERO;
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    //TODO test
    pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
        self.assign_str_radix(src, 10)
    }
}

fn make_string(i: &Natural, radix: i32, to_upper: bool) -> String {
    assert!(!to_upper);
    assert!(radix >= 2 && radix <= 36, "radix out of range");
    if *i == Small(0) {
        return "0".to_string();
    }
    let mut i_cloned = i.clone();
    let mut cs = Vec::new();
    while i_cloned != Natural::ZERO {
        cs.push(
            i_cloned
                .div_assign_mod(10 as Limb)
                .to_string()
                .chars()
                .next()
                .unwrap(),
        );
    }
    cs.into_iter().rev().collect()
}

fn fmt_radix(
    i: &Natural,
    f: &mut Formatter,
    radix: i32,
    to_upper: bool,
    prefix: &str,
) -> fmt::Result {
    f.pad_integral(true, prefix, &make_string(i, radix, to_upper))
}

//TODO test
impl Display for Natural {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

//TODO test
impl Debug for Natural {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

//TODO test
impl FromStr for Natural {
    type Err = ParseIntegerError;

    fn from_str(src: &str) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::ZERO;
        i.assign_str(src)?;
        Ok(i)
    }
}

impl fmt::Binary for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Small(small) => write!(f, "{:b}", small),
            Large(ref limbs) => {
                write!(f, "{:b}", limbs.last().unwrap())?;
                let mut i = limbs.len() - 2;
                loop {
                    #[cfg(feature = "32_bit_limbs")]
                    let result = write!(f, "{:032b}", limbs[i]);
                    #[cfg(feature = "64_bit_limbs")]
                    let result = write!(f, "{:064b}", limbs[i]);
                    if i == 0 {
                        return result;
                    }
                    i -= 1;
                }
            }
        }
    }
}

pub mod assign;
pub mod assign_double_limb;
pub mod assign_limb;
pub mod double_limb_from_natural;
pub mod floating_point_from_natural;
pub mod from_bits;
pub mod from_double_limb;
pub mod from_floating_point;
pub mod from_limb;
pub mod from_limbs;
pub mod limb_count;
pub mod limb_from_natural;
pub mod to_bits;
pub mod to_limbs;
