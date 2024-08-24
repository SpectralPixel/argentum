pub trait CoordinateTrait {
    type Type;
    type FieldType;

    const MAX: Self::Type;
    const MIN: Self::Type;

    fn new(x: Self::FieldType, y: Self::FieldType, z: Self::FieldType) -> Self::Type;
    fn splat(n: Self::FieldType) -> Self::Type;
}
