/*
Örneği çalıştırdıktan sonra cargo expand ile içeriğini göstermekte yarar var.
 */
macro_rules! gstruct {
    ($struct_name:ident { $( $field:ident : $field_type:ty ),* }) => {
        #[derive(Debug)]
        struct $struct_name {
            $(
                $field: $field_type,
            )*
        }

        impl $struct_name {
            fn new($($field: $field_type),*) -> Self {
                $struct_name {
                    $(
                        $field,
                    )*
                }
            }

            fn display(&self) {
                println!("{} {{", stringify!($struct_name));
                $(
                    println!("    {}: {:?}", stringify!($field), self.$field);
                )*
                println!("}}");
            }
        }
    };
}

gstruct!(Player {
    name: String,
    age: u32,
    is_active: bool
});

fn main() {
    let prince = Player::new("Prince of Persia".to_string(), 25, false);
    prince.display();
}
