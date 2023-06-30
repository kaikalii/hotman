#![warn(missing_docs)]

/*!
This crate provides a simple way to generate HTML elements in pure Rust.
*/

mod attribute;

use std::fmt;

use paste::paste;

pub use attribute::*;
use attribute_traits::*;

/// Trait for types of elements
pub trait Element: Sized {
    /// Get the children of this element
    fn children(&self) -> &[Node];
    /// Get the mutable children of this element
    fn children_mut(&mut self) -> &mut Vec<Node>;
}

macro_rules! impl_global_attrs {
    ($name:ident, $($attr:ident),* $(,)?) => {
        $(
            paste! {
                impl [<Has $attr>] for $name {
                    fn $attr(&self) -> &str {
                        &self.$attr
                    }
                    fn [<set_ $attr>](&mut self, value: impl Into<String>) {
                        self.$attr = value.into();
                    }
                }
            }
        )*
    }
}

macro_rules! elements {
        ($(($name:ident, $tag:ident $(,$attr:ident)?)),* $(,)*) => {
            /// An HTML node
            #[derive(Debug, Clone)]
            pub enum Node {
                $(#[allow(missing_docs)] $name(element_structs::$name),)*
                /// A text element
                Text(String),
            }

            impl fmt::Display for Node {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        $(Node::$name(element) => write!(f, "{element}"),)*
                        Node::Text(text) => write!(f, "{text}"),
                    }
                }
            }

            pub mod element_structs {
                //! Structs that represent HTML elements

                use super::*;
                $(
                    #[derive(Debug, Clone, Default)]
                    #[doc = "A `"]
                    #[doc = stringify!($tag)]
                    #[doc = "` element"]
                    pub struct $name {
                        /// The `id` attribute
                        pub id: String,
                        /// The `class` attribute
                        pub class: String,
                        /// The `style` attribute
                        pub style: String,
                        $(
                            #[doc = "The `"]
                            #[doc = stringify!($attr)]
                            #[doc = "` attribute"]
                            pub $attr: String,
                        )*
                        /// The children of this element
                        pub children: Vec<Node>,
                    }

                    impl fmt::Display for $name {
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                            let tag = stringify!($tag);
                            write!(f, "<{tag}")?;
                            $(
                                if !self.$attr.is_empty() {
                                    write!(f, " {}=\"{}\"", stringify!($attr), self.$attr)?;
                                }
                            )*
                            write!(f, ">")?;
                            for child in &self.children {
                                write!(f, "{child}")?;
                            }
                            write!(f, "</{tag}>")?;
                            Ok(())
                        }
                    }

                    impl From<$name> for Node {
                        fn from(element: $name) -> Self {
                            Node::$name(element)
                        }
                    }

                    impl Element for $name {
                        fn children(&self) -> &[Node] {
                            &self.children
                        }
                        fn children_mut(&mut self) -> &mut Vec<Node> {
                            &mut self.children
                        }
                    }

                    impl_global_attrs!($name, id, class, style);

                    $(
                        paste! {
                            impl [<Has $attr>] for $name {
                                fn $attr(&self) -> &str {
                                    &self.$attr
                                }
                                fn [<set_ $attr>](&mut self, value: impl Into<String>) {
                                    self.$attr = value.into();
                                }
                            }
                        }
                    )*
                )*
            }

            $(
                #[must_use]
                #[doc = "Make a `"]
                #[doc = stringify!($tag)]
                #[doc = "` element"]
                pub fn $tag(data: impl ElementData<element_structs::$name>) -> element_structs::$name {
                    let mut elem = element_structs::$name::default();
                    data.add_to(&mut elem);
                    elem
                }
            )*
        };
    }

impl From<String> for Node {
    fn from(text: String) -> Self {
        Node::Text(text)
    }
}

impl From<&str> for Node {
    fn from(text: &str) -> Self {
        Node::Text(text.to_string())
    }
}

impl From<&String> for Node {
    fn from(text: &String) -> Self {
        Node::Text(text.to_string())
    }
}

elements!(
    (A, a, href),
    (Abbr, abbr),
    (Area, area, href),
    (Audio, audio),
    (Base, base, href),
    (Bdi, bdi),
    (Bdo, bdo),
    (Blockquote, blockquote),
    (Body, body),
    (Br, br),
    (Button, button),
    (Canvas, canvas),
    (Caption, caption),
    (Cite, cite),
    (Code, code),
    (Col, col),
    (Colgroup, colgroup),
    (Dd, dd),
    (Del, del),
    (Details, details),
    (Dfn, dfn),
    (Div, div),
    (Dl, dl),
    (Dt, dt),
    (Em, em),
    (Embed, embed),
    (Fieldset, fieldset),
    (H1, h1),
    (H2, h2),
    (H3, h3),
    (H4, h4),
    (H5, h5),
    (H6, h6),
    (Head, head),
    (Hr, hr),
    (I, i),
    (Iframe, iframe),
    (Img, img),
    (Input, input),
    (Ins, ins),
    (Kbd, kbd),
    (Label, label),
    (Legend, legend),
    (Li, li),
    (Link, link, href),
    (Map, map),
    (Mark, mark),
    (Menuitem, menuitem),
    (Meta, meta, charset),
    (Meter, meter),
    (Noscript, noscript),
    (Object, object),
    (Ol, ol),
    (Option, option),
    (Output, output),
    (P, p),
    (Progress, progress),
    (Q, q),
    (Rt, rt),
    (Ruby, ruby),
    (Samp, samp),
    (Script, script),
    (Select, select),
    (Slot, slot),
    (Small, small),
    (Source, source),
    (Span, span),
    (Strong, strong),
    (Style, style),
    (Sub, sub),
    (Summary, summary),
    (Sup, sup),
    (Table, table),
    (Tbody, tbody),
    (Td, td),
    (Template, template),
    (Tfoot, tfoot),
    (Th, th),
    (Thead, thead),
    (Time, time),
    (Title, title),
    (Tr, tr),
    (Track, track),
    (Ul, ul),
    (Var, var),
    (Video, video),
    (Wbr, wbr),
    (B, b),
    (Form, form),
    (Html, html),
    (Menu, menu),
    (Param, param),
    (Rp, rp),
);

/// A piece of data that can be added to an element
///
/// It is usually an attribute or a child element
pub trait ElementData<E> {
    /// Add this data to the given element
    fn add_to(self, element: &mut E);
}

impl<E> ElementData<E> for () {
    fn add_to(self, _: &mut E) {}
}

impl<E, D> ElementData<E> for D
where
    E: Element,
    D: Into<Node>,
{
    fn add_to(self, elem: &mut E) {
        elem.children_mut().push(self.into());
    }
}

impl<E, D> ElementData<E> for Vec<D>
where
    E: Element,
    D: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

impl<E, D, const N: usize> ElementData<E> for [D; N]
where
    E: Element,
    D: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

impl<E, D> ElementData<E> for Option<D>
where
    E: Element,
    D: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        if let Some(child) = self {
            child.add_to(elem);
        }
    }
}

macro_rules! tuple_element_data {
    ($($T:ident),*) => {
        impl<Elem, $($T),*> ElementData<Elem> for ($($T,)*)
        where
            $($T: ElementData<Elem>),*
        {
            #[allow(non_snake_case)]
            fn add_to(self, elem: &mut Elem) {
                let ($($T,)*) = self;
                $($T.add_to(elem);)*
            }
        }
    };
}

tuple_element_data!(A);
tuple_element_data!(A, B);
tuple_element_data!(A, B, C);
tuple_element_data!(A, B, C, D);
tuple_element_data!(A, B, C, D, E);
tuple_element_data!(A, B, C, D, E, F);
tuple_element_data!(A, B, C, D, E, F, G);
tuple_element_data!(A, B, C, D, E, F, G, H);
tuple_element_data!(A, B, C, D, E, F, G, H, I);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
tuple_element_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
