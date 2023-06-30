use std::fmt;

use paste::paste;

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Attributes<T = ()> {
    pub id: String,
    pub class: String,
    pub style: String,
    pub title: String,
    pub lang: String,
    pub dir: String,
    pub other: T,
}

impl<T> fmt::Display for Attributes<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.id.is_empty() {
            write!(f, " id={:?}", self.id)?;
        }
        if !self.class.is_empty() {
            write!(f, " class={:?}", self.class)?;
        }
        if !self.style.is_empty() {
            write!(f, " style={:?}", self.style)?;
        }
        if !self.title.is_empty() {
            write!(f, " title={:?}", self.title)?;
        }
        if !self.lang.is_empty() {
            write!(f, " lang={:?}", self.lang)?;
        }
        if !self.dir.is_empty() {
            write!(f, " dir={:?}", self.dir)?;
        }
        self.other.fmt(f)
    }
}

macro_rules! attributes {
    ($($name:ident),* $(,)*) => {
        $(
            #[derive(Debug, Clone, Default)]
            #[allow(non_camel_case_types)]
            pub struct $name<T = ()>(pub T);

            impl<A, T> ElementData<Attributes<A>> for $name<T>
            where T: Into<String> {
                fn add_to(self, attributes: &mut Attributes<A>, _children: &mut Children) {
                    attributes.$name = self.0.into();
                }
            }
        )*
    };
}

attributes!(id, class, style, title, lang, dir);

macro_rules! attr {
    ($name:ident, $lower:ident, $([$(($parent_name:ident, $parent_ty:ty))*],)? $($attr:ident),* $(,)?) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            $(pub $attr: String,)*
            $(pub parent_name: $parent_ty,)*
        }

        $(
            #[derive(Debug, Clone, Default)]
            #[allow(non_camel_case_types)]
            pub struct $attr<T = ()>(pub T);

            impl<T> ElementData<Attributes<$name>> for $attr<T>
            where T: Into<String> {
                fn add_to(self, attributes: &mut Attributes<$name>, _children: &mut Children) {
                    attributes.other.$attr = self.0.into();
                }
            }
        )*

        paste! {
            pub trait [<Has $name>] {
                fn $lower(&self) -> &$name;
                fn [<$lower _mut>](&mut self) -> &mut $name;
            }

            impl<E> [<Has $name>] for E where E: ElementTrait<Attr = $name> {
                fn $lower(&self) -> &$name {
                    &self.attributes().other
                }
                fn [<$lower _mut>](&mut self) -> &mut $name {
                    &mut self.attributes_mut().other
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                $(
                    if !self.$attr.is_empty() {
                        write!(f, " {}={:?}", stringify!($attr), self.$attr)?;
                    }
                )*
                Ok(())
            }
        }
    };
}

attr!(MetaAttr, meta, charset, content, http_equiv);
