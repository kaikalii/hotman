pub mod attribute;

use std::fmt;

pub use attribute::*;

pub trait ElementTrait: Sized {
    type Attr;
    fn attributes(&self) -> &Attributes<Self::Attr>;
    fn attributes_mut(&mut self) -> &mut Attributes<Self::Attr>;
    fn children(&self) -> &[Element];
    fn children_mut(&mut self) -> &mut Vec<Element>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[doc(hidden)]
pub struct Unit;
impl fmt::Display for Unit {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
type DefaultUnit<T = Unit> = T;

macro_rules! elements {
        ($(($name:ident, $tag:ident $(,$attr:ty)?)),* $(,)*) => {
            #[derive(Debug, Clone)]
            pub enum Element {
                $($name(element::$name),)*
                Text(String),
            }

            impl fmt::Display for Element {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        $(Element::$name(element) => write!(f, "{element}"),)*
                        Element::Text(text) => write!(f, "{text}"),
                    }
                }
            }

            pub mod element {
                use super::*;
                $(
                    #[derive(Debug, Clone, Default)]
                    pub struct $name {
                        pub attributes: Attributes<DefaultUnit<$($attr)?>>,
                        pub children: Vec<Element>,
                    }

                    impl $name {
                        pub fn new() -> Self {
                            Self::default()
                        }
                    }

                    impl fmt::Display for $name {
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                            let tag = stringify!($tag);
                            write!(f, "<{tag}")?;
                            write!(f, "{}", self.attributes)?;
                            write!(f, ">")?;
                            for child in &self.children {
                                write!(f, "{child}")?;
                            }
                            write!(f, "</{tag}>")?;
                            Ok(())
                        }
                    }

                    impl From<$name> for Element {
                        fn from(element: $name) -> Self {
                            Element::$name(element)
                        }
                    }

                    impl ElementTrait for $name {
                        type Attr = DefaultUnit<$($attr)?>;
                        fn attributes(&self) -> &Attributes<Self::Attr> {
                            &self.attributes
                        }
                        fn attributes_mut(&mut self) -> &mut Attributes<Self::Attr> {
                            &mut self.attributes
                        }
                        fn children(&self) -> &[Element] {
                            &self.children
                        }
                        fn children_mut(&mut self) -> &mut Vec<Element> {
                            &mut self.children
                        }
                    }
                )*
            }

            $(
                pub fn $tag(data: impl ElementData<Attributes<DefaultUnit<$($attr)?>>>) -> element::$name {
                    let mut elem = element::$name::new();
                    data.add_to(&mut elem.attributes, &mut elem.children);
                    elem
                }
            )*
        };
    }

impl From<String> for Element {
    fn from(text: String) -> Self {
        Element::Text(text)
    }
}

impl From<&str> for Element {
    fn from(text: &str) -> Self {
        Element::Text(text.to_string())
    }
}

impl From<&String> for Element {
    fn from(text: &String) -> Self {
        Element::Text(text.to_string())
    }
}

elements!(
    (A, a),
    (Abbr, abbr),
    (Area, area),
    (Audio, audio),
    (Base, base),
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
    (Link, link),
    (Map, map),
    (Mark, mark),
    (Menuitem, menuitem),
    (Meta, meta, MetaAttr),
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

type Children = Vec<Element>;

pub trait ElementData<A> {
    fn add_to(self, attributes: &mut A, children: &mut Children);
}

impl<A> ElementData<A> for () {
    fn add_to(self, _attributes: &mut A, _children: &mut Children) {}
}

impl<A, E> ElementData<A> for E
where
    E: Into<Element>,
{
    fn add_to(self, _attributes: &mut A, children: &mut Children) {
        children.push(self.into());
    }
}

impl<A, E> ElementData<A> for Vec<E>
where
    E: ElementData<A>,
{
    fn add_to(self, attributes: &mut A, children: &mut Children) {
        for child in self {
            child.add_to(attributes, children);
        }
    }
}

impl<A, E, const N: usize> ElementData<A> for [E; N]
where
    E: ElementData<A>,
{
    fn add_to(self, attributes: &mut A, children: &mut Children) {
        for child in self {
            child.add_to(attributes, children);
        }
    }
}

impl<A, E> ElementData<A> for Option<E>
where
    E: ElementData<A>,
{
    fn add_to(self, attributes: &mut A, children: &mut Children) {
        if let Some(child) = self {
            child.add_to(attributes, children);
        }
    }
}

macro_rules! tuple_element_data {
    ($($T:ident),*) => {
        impl<Attributes, $($T),*> ElementData<Attributes> for ($($T,)*)
        where
            $($T: ElementData<Attributes>),*
        {
            #[allow(non_snake_case)]
            fn add_to(self, attributes: &mut Attributes, children: &mut Children) {
                let ($($T,)*) = self;
                $($T.add_to(attributes, children);)*
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
