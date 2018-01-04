use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Converts a `Natural` to a `u32`, returning `None` if the `Natural` is too large.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(123u32).to_u32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Natural::trillion().to_u32()), "None");
    /// ```
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            Small(small) => Some(small),
            Large(_) => None,
        }
    }

    /// Converts a `Natural` to a `u32`, wrapping mod 2^(32).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).to_u32_wrapping().to_string(), "123");
    /// assert_eq!(Natural::trillion().to_u32_wrapping().to_string(), "3567587328");
    /// ```
    pub fn to_u32_wrapping(&self) -> u32 {
        match *self {
            Small(small) => small,
            Large(ref large) => unsafe { gmp::mpz_get_ui(large) as u32 },
        }
    }
}
