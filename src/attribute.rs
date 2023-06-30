use crate::*;

use paste::paste;

macro_rules! attributes {
    ($($name:ident),* $(,)?) => {
        $(

            #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
            #[allow(non_camel_case_types)]
            pub struct $name<T>(pub T);
        )*
        pub mod attribute_traits {
            use super::*;
            $(
                paste! {
                    pub trait [<Has $name>] {
                        fn $name(&self) -> &str;
                        fn [<set_ $name>](&mut self, value: impl Into<String>);
                    }

                    impl<E, T> ElementData<E> for $name<T>
                    where
                        E: [<Has $name>],
                        T: Into<String>,
                    {
                        fn add_to(self, element: &mut E) {
                            element.[<set_ $name>](self.0);
                        }
                    }
                }
            )*
        }
    };
}

attributes!(id, class, style, charset, href,);
