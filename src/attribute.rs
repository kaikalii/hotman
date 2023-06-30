use crate::*;

use paste::paste;

macro_rules! attributes {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
            #[allow(non_camel_case_types)]
            #[doc = "The `"]
            #[doc = stringify!($name)]
            #[doc = "` attribute"]
            pub struct $name<T>(pub T);
        )*
        pub mod attribute_traits {
            //! Traits that mark elements as having attributes

            use super::*;
            $(
                paste! {
                    #[doc = "Trait for elements that have the `"]
                    #[doc = stringify!($name)]
                    #[doc = "` attribute"]
                    pub trait [<Has $name>] {
                        #[doc = "Get the value of the `"]
                        #[doc = stringify!($name)]
                        #[doc = "` attribute"]
                        fn $name(&self) -> &str;
                        #[doc = "Set the value of the `"]
                        #[doc = stringify!($name)]
                        #[doc = "` attribute"]
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

attributes!(id, class, style, charset, href, rel);
