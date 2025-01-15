#[macro_export]
macro_rules! setter {
    ($type_name:ident, $name:ident, $field:ident) => {
        impl $type_name {
            pub fn $name(mut self, value: &str) -> Self{
                self.$field = value.to_string();
                self
            }    
        }
    };
}