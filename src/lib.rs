#![warn(missing_docs)]

/*!
~~html~~
<br>
~~hot male~~
<br>
`hotman`

This crate provides a simple way to generate HTML elements in pure Rust.

# Example

This example looks better with proper language server syntax highlighting
because tags are functions and attributes are structs.
```rust
use hotman::*;

let dom = html((
    head((meta(charset("utf-8")), title("Login"))),
    body((
        h1("Login"),
        form((
            // Attributes can be grouped, but it's not required
            (action("/login"), method("POST")),
            input((
                type_("text"),
                name("username"),
                placeholder("Username"),
                autofocus,
            )),
            input((type_("password"), name("password"), placeholder("Password"))),
            input((type_("submit"), value("Login"))),
            p((
                "Don't have an account? ",
                a((href("/register"), "Register")),
            )),
        )),
    )),
));
println!("{dom}");
```
*/

mod attribute;
mod format;

use std::fmt;

use paste::paste;

pub use attribute::*;
use attribute_traits::*;
use format::*;

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
                impl [<Has_ $attr>] for $name {
                    fn $attr(&self) -> [<$attr _ref_t>] {
                        [<$attr _take_ref>](&self.$attr)
                    }
                    fn [<set_ $attr>](&mut self, val: impl Into<[<$attr _t>]>) {
                        self.$attr = val.into();
                    }
                }
            }
        )*
    }
}

macro_rules! write_attr {
    ($this:expr, $f:expr, $attr:ident) => {
        paste!([<$attr _write>](&$this.$attr, $f.f)?);
    };
}

macro_rules! elements {
    ($(($name:ident, $tag:ident $(,$attr:ident)* $(,)?)),* $(,)*) => {
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

        impl IndentFormat for Node {
            fn indent_fmt(&self, f: &mut IndentFormatter) -> fmt::Result {
                match self {
                    $(Node::$name(element) => element.indent_fmt(f),)*
                    Node::Text(text) => f.write(text),
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
                    /// The `title` attribute
                    pub title: String,
                    /// The `autofocus` attribute
                    pub autofocus: bool,
                    /// The `itemscope` attribute
                    pub itemscope: bool,
                    $(
                        #[doc = "The `"]
                        #[doc = stringify!($attr)]
                        #[doc = "` attribute"]
                        pub $attr: paste!([<$attr _t>]),
                    )*
                    /// The children of this element
                    pub children: Vec<Node>,
                }

                impl IndentFormat for $name {
                    fn indent_fmt(&self, f: &mut IndentFormatter) -> fmt::Result {
                        let tag = stringify!($tag);
                        f.write(format_args!("<{tag}"))?;
                        write_attr!(self, f, id);
                        write_attr!(self, f, class);
                        write_attr!(self, f, style);
                        write_attr!(self, f, title);
                        write_attr!(self, f, autofocus);
                        write_attr!(self, f, itemscope);
                        $(write_attr!(self, f, $attr);)*
                        if self.children.is_empty() {
                            f.writeln(format_args!(" />"))?;
                            return Ok(());
                        }
                        f.write(format_args!(">"))?;
                        let single_child = self.children.len() == 1 && matches!(self.children[0], Node::Text(_));
                        if single_child {
                            let child = &self.children[0];
                            child.indent_fmt(f)?;
                            f.writeln(format_args!("</{tag}>"))?;
                            return Ok(());
                        }
                        f.writeln("")?;
                        f.indent();
                        for child in &self.children {
                            child.indent_fmt(f)?;
                        }
                        f.dedent();
                        f.write(format_args!("</{tag}>"))?;
                        if !single_child {
                            f.writeln("")?;
                        }
                        Ok(())
                    }
                }

                impl fmt::Display for $name {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        self.indent_fmt(&mut IndentFormatter::from(f))
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

                impl_global_attrs!($name, id, class, style, title, autofocus, itemscope);

                $(
                    paste! {
                        impl [<Has_ $attr>] for $name {
                            fn $attr(&self) -> [<$attr _ref_t>] {
                                [<$attr _take_ref>](&self.$attr)
                            }
                            fn [<set_ $attr>](&mut self, val: impl Into<[<$attr _t>]>) {
                                self.$attr = val.into();
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
            pub fn $tag(elem_data: impl ElementData<element_structs::$name>) -> element_structs::$name {
                let mut elem = Default::default();
                elem_data.add_to(&mut elem);
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
    (
        A,
        a,
        download,
        href,
        hreflang,
        ping,
        referrerpolicy,
        rel,
        target,
        type_,
    ),
    (Abbr, abbr),
    (
        Area,
        area,
        alt,
        coords,
        download,
        href,
        hreflang,
        ping,
        referrerpolicy,
        rel,
        shape,
        target
    ),
    (Audio, audio, autoplay, controls, loop_, muted, preload, src),
    (B, b),
    (Base, base, href, target),
    (Bdi, bdi, dir),
    (Bdo, bdo, dir),
    (Blockquote, blockquote, cite),
    (
        Body,
        body,
        onafterprint,
        onbeforeprint,
        onbeforeunload,
        onhashchange,
        onlanguagechange,
        onmessage,
        onmessageerror,
        onoffline,
        ononline,
        onpagehide,
        onpageshow,
        onpopstate,
        onrejectionhandled,
        onstorage,
        onunhandledrejection,
        onunload
    ),
    (Br, br, clear),
    (
        Button,
        button,
        disabled,
        form,
        formaction,
        formenctype,
        formmethod,
        formnovalidate,
        formtarget,
        name,
        type_,
        value
    ),
    (Canvas, canvas, height, width),
    (Caption, caption),
    (Cite, cite),
    (Code, code, type_),
    (Col, col, span),
    (Colgroup, colgroup, span),
    (Dd, dd, type_),
    (Del, del, cite, datetime),
    (Details, details, open),
    (Dfn, dfn),
    (Div, div),
    (Dl, dl, type_),
    (Dt, dt, type_),
    (Em, em, type_),
    (Embed, embed, height, src, type_, width),
    (Fieldset, fieldset, disabled, form, name),
    (
        Form,
        form,
        accept_charset,
        action,
        autocomplete,
        enctype,
        method,
        name,
        novalidate,
        target
    ),
    (H1, h1),
    (H2, h2),
    (H3, h3),
    (H4, h4),
    (H5, h5),
    (H6, h6),
    (Head, head, profile),
    (Hr, hr, align, color, noshade, size, width),
    (Html, html, manifest, xmlns),
    (I, i),
    (
        Iframe,
        iframe,
        allow,
        height,
        loading,
        name,
        referrerpolicy,
        sandbox,
        src,
        srcdoc,
        width
    ),
    (
        Img,
        img,
        alt,
        crossorigin,
        decoding,
        height,
        importance,
        intrinsicsize,
        ismap,
        loading,
        referrerpolicy,
        sizes,
        src,
        srcset,
        usemap,
        width
    ),
    (
        Input,
        input,
        accept,
        alt,
        autocomplete,
        checked,
        dirname,
        disabled,
        form,
        formaction,
        formenctype,
        formmethod,
        formnovalidate,
        formtarget,
        height,
        list,
        max,
        max_length,
        min,
        min_length,
        multiple,
        name,
        pattern,
        placeholder,
        readonly,
        required,
        size,
        src,
        step,
        type_,
        value,
        width
    ),
    (Ins, ins, cite, datetime),
    (Kbd, kbd),
    (Label, label, for_),
    (Legend, legend),
    (Li, li, value),
    (
        Link,
        link,
        href,
        rel,
        media,
        hreflang,
        type_,
        sizes,
        crossorigin,
        integrity,
        referrerpolicy
    ),
    (Map, map, name),
    (Mark, mark),
    (Menu, menu, type_, label),
    (Menuitem, menuitem, checked, command, default, disabled, icon, label, radiogroup, type_),
    (Meta, meta, charset, http_equiv, name),
    (Meter, meter, high, low, max, min, optimum, value),
    (Noscript, noscript),
    (Object, object, data, form, height, name, type_, usemap, width),
    (Ol, ol, reversed, start, type_),
    (Option, option, disabled, label, selected, value),
    (Output, output, for_, form, name),
    (P, p),
    (Param, param, name, value),
    (Progress, progress, max, value),
    (Q, q, cite),
    (Rp, rp),
    (Rt, rt),
    (Samp, samp),
    (
        Script,
        script,
        async_,
        crossorigin,
        defer,
        integrity,
        nomodule,
        nonce,
        referrerpolicy,
        type_,
        src
    ),
    (Select, select, disabled, form, multiple, name, required, size),
    (Slot, slot, name),
    (Small, small),
    (Source, source, media, sizes, src, srcset, type_),
    (Span, span),
    (Strong, strong),
    (Style, style, media, nonce, type_),
    (Sub, sub),
    (Summary, summary),
    (Sup, sup),
    (Table, table),
    (Tbody, tbody),
    (Td, td, colspan, headers, rowspan),
    (Template, template),
    (
        Textarea,
        textarea,
        autocomplete,
        cols,
        dirname,
        disabled,
        form,
        maxlength,
        minlength,
        name,
        placeholder,
        readonly,
        required,
        rows,
        wrap
    ),
    (Tfoot, tfoot),
    (Th, th, colspan, headers, rowspan, scope),
    (Thead, thead),
    (Time, time, datetime),
    (Title, title),
    (Tr, tr),
    (Track, track, default, kind, label, src, srclang),
    (Ul, ul),
    (Var, var),
    (
        Video,
        video,
        autoplay,
        controls,
        crossorigin,
        height,
        loop_,
        muted,
        playsinline,
        poster,
        preload,
        src,
        width
    ),
    (Wbr, wbr),
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
