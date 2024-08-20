pub trait CoordinateTrait {
    type Type;
    type FieldType;

    fn new(
        x: Self::FieldType,
        y: Self::FieldType,
        z: Self::FieldType,
    ) -> Self::Type;

    fn splat(n: Self::FieldType) -> Self::Type;
}