#[macro_export]
macro_rules! coordinate_type {
    ($name:ident, $coord_type:ident, $type_documentation:literal) => {
        #[doc=$type_documentation]
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name {
            pub x: $coord_type,
            pub y: $coord_type,
            pub z: $coord_type,
        }
    };
}
