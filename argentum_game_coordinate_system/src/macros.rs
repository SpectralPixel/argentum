#[macro_export]
macro_rules! coordinate_type {
    ($name:ident, $coord_type:ident, $type_documentation:literal, $crate_name:expr) => {
        #[doc=$type_documentation]
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name {
            pub x: $coord_type,
            pub y: $coord_type,
            pub z: $coord_type,
        }

        impl Coordinate {
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
    };
}
