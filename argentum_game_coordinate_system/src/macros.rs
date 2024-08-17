/// Allows you to generate a new type similar to `Coordinate`.
/// 
/// # Inputs
/// 
/// - `$name` = The name of the new type.
/// - `$coord_type` = The type used for the fields `x`, `y` and `z`. Must be a numeric type provided by Rust.
/// - `$type_documentation` = A string literal that will be used for documenting your type.
/// - `$crate_name` = A workaround for not being allowed to `stringify!` the `$crate` variable within the macro for whatever reason. Provide the name of your crate in plain text, and don't enclose it in quotes. If you know how to fix this issue, feel free to open a PR.
/// 
/// # Example
/// 
/// ```
/// type CoordinateType = i32;
/// generate_coordinate_type!(
///     Coordinate,
///     CoordinateType,
///     "3D Coordinate in absolute space.",
///     argentum_game_coordinate_system
/// );
/// ```
#[macro_export]
macro_rules! generate_coordinate_type {
    ($name:ident, $coord_type:ident, $type_documentation:literal, $crate_name:expr) => {
        #[doc=$type_documentation]
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name {
            pub x: $coord_type,
            pub y: $coord_type,
            pub z: $coord_type,
        }

        impl $name {
            #[doc=concat!("
                Represents the smallest possible coordinate on all axes.

                # Examples

                ```
                use ", stringify!($crate_name), "::{", stringify!($name), ", ", stringify!($coord_type), "};
                assert_eq!(", stringify!($name), "::MIN.x, ", stringify!($coord_type), "::MIN);
                assert_eq!(", stringify!($name), "::MIN.y, ", stringify!($coord_type), "::MIN);
                assert_eq!(", stringify!($name), "::MIN.z, ", stringify!($coord_type), "::MIN);
                ```
            ")]
            pub const MIN: Self = Self {
                x: $coord_type::MIN,
                y: $coord_type::MIN,
                z: $coord_type::MIN,
            };

            #[doc=concat!("
                Represents the smallest possible coordinate on all axes.

                # Examples

                ```
                use ", stringify!($crate_name), "::{", stringify!($name), ", ", stringify!($coord_type), "};
                assert_eq!(", stringify!($name), "::MAX.x, ", stringify!($coord_type), "::MAX);
                assert_eq!(", stringify!($name), "::MAX.y, ", stringify!($coord_type), "::MAX);
                assert_eq!(", stringify!($name), "::MAX.z, ", stringify!($coord_type), "::MAX);
                ```
            ")]
            pub const MAX: Self = Self {
                x: $coord_type::MAX,
                y: $coord_type::MAX,
                z: $coord_type::MAX,
            };

            #[doc=concat!("
                Creates a new Coordinate.

                # Examples

                ```
                use ", stringify!($crate_name), "::", stringify!($name), ";
                let pos = ", stringify!($name), "::new(1, 2, 3);
                assert_eq!(pos.x, 1);
                assert_eq!(pos.y, 2);
                assert_eq!(pos.z, 3);
                ```
            ")]
            pub fn new(x: $coord_type, y: $coord_type, z: $coord_type) -> Self {
                Self { x, y, z }
            }

            #[doc=concat!("
                Creates a new coordinate, assigning all values to the input.

                # Examples

                ```
                use ", stringify!($crate_name), "::", stringify!($name), ";
                let pos = ", stringify!($name), "::splat(7);
                assert_eq!(pos.x, 7);
                assert_eq!(pos.y, 7);
                assert_eq!(pos.z, 7);
                ```
            ")]
            pub fn splat(n: $coord_type) -> Self {
                Self::new(n, n, n)
            }
        }

        use core::fmt::{Display, Formatter, Result};
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "({}: {}, {}, {})", stringify!($name), self.x, self.y, self.z)
            }
        }

        use quickcheck::{Arbitrary, Gen};
        impl Arbitrary for $name {
            fn arbitrary(g: &mut Gen) -> Self {
                Self::new(
                    $coord_type::arbitrary(g),
                    $coord_type::arbitrary(g),
                    $coord_type::arbitrary(g),
                )
            }
        }

        use thiserror::Error;
        #[derive(Debug, Clone, Error)]
        #[error("{0} is experiencing integer overflow.")]
        pub struct CoordinateOverflowError(pub $name);

        use std::ops::*;

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let panic_if_out_of_bounds = || panic!("{}", CoordinateOverflowError(self.to_owned()));
                let x = $coord_type::checked_add(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
                let y = $coord_type::checked_add(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
                let z = $coord_type::checked_add(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
                Self::new(x, y, z)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self = self.to_owned() + rhs;
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                let panic_if_out_of_bounds = || panic!("{}", CoordinateOverflowError(self.to_owned()));
                let x = $coord_type::checked_sub(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
                let y = $coord_type::checked_sub(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
                let z = $coord_type::checked_sub(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
                Self::new(x, y, z)
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.to_owned() - rhs;
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let panic_if_out_of_bounds = || panic!("{}", CoordinateOverflowError(self.to_owned()));
                let x = $coord_type::checked_mul(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
                let y = $coord_type::checked_mul(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
                let z = $coord_type::checked_mul(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
                Self::new(x, y, z)
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                *self = self.to_owned() * rhs;
            }
        }

        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                let panic_if_out_of_bounds = || panic!("{}", CoordinateOverflowError(self.to_owned()));
                let x = $coord_type::checked_div(self.x, rhs.x).unwrap_or_else(panic_if_out_of_bounds);
                let y = $coord_type::checked_div(self.y, rhs.y).unwrap_or_else(panic_if_out_of_bounds);
                let z = $coord_type::checked_div(self.z, rhs.z).unwrap_or_else(panic_if_out_of_bounds);
                Self::new(x, y, z)
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                *self = self.to_owned() / rhs;
            }
        }

        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z)
            }
        }

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::new(self.x & rhs.x, self.y & rhs.y, self.z & rhs.z)
            }
        }

        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = self.to_owned() & rhs;
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::new(self.x | rhs.x, self.y | rhs.y, self.z | rhs.z)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = self.to_owned() | rhs;
            }
        }

        impl BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self::new(self.x ^ rhs.x, self.y ^ rhs.y, self.z ^ rhs.z)
            }
        }

        impl BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = self.to_owned() ^ rhs;
            }
        }
    };
}
