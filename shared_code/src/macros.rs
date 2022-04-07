#[macro_export]
macro_rules! impl_component {
    ($ty:ty, $enumvalue:ident) => {
        impl Into<Component> for $ty {
            fn into(self) -> Component {
                Component::$enumvalue(Box::new(self))
            }
        }
    }
}