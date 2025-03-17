pub mod token;
pub mod password;

#[macro_export]
macro_rules! make_enum {
    ($name:ident, [$($opt:ident),+]) => {
        pub enum $name {
            $(
                $opt,
            )+
        }

        impl $name {
            // Fixed array with commas
            pub const ALL: &'static [Self] = &[$($name::$opt),+];

            pub fn to_string(&self) -> String {
                match self {
                    $(
                        $name::$opt => stringify!($opt).to_string(),
                    )+
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.to_string().as_str())
            }
        }
    };
}


