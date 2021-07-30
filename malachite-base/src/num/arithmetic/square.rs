use num::arithmetic::traits::{Square, SquareAssign};

macro_rules! impl_square {
    ($t:ident) => {
        impl Square for $t {
            type Output = $t;

            /// Squares `self`.
            ///
            /// $f(x) = x^2$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::square` module.
            #[inline]
            fn square(self) -> $t {
                self * self
            }
        }

        impl SquareAssign for $t {
            /// Replaces `self` with `self` squared.
            ///
            /// $x \gets x^2$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::square` module.
            #[inline]
            fn square_assign(&mut self) {
                *self *= *self;
            }
        }
    };
}
apply_to_primitive_ints!(impl_square);
apply_to_primitive_floats!(impl_square);
