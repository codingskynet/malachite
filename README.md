<img width="500" src="docs/assets/logo-and-name.svg" alt="Logo">

An arbitrary-precision arithmetic library for Rust.

Parts of Malachite are derived from [GMP](https://gmplib.org/),
[FLINT](https://www.flintlib.org/), and [MPFR](https://www.mpfr.org/).

Floats (arbitrary-precision floating-point numbers) are in development and are currently
experimental. They are missing many important functions. However, the functions that are currently
implemented are thoroughly tested and documented, with the exception of string conversion
functions. The current string conversions are incomplete and will be changed in the future to
match MPFR's behavior.

<https://www.malachite.rs/>

Copyright © 2023 Mikhail Hogrefe
