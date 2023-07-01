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
    ($(($name:ident, $fn_name:ident, $tag:literal $(,$attr:ident)* $(,)?)),* $(,)*) => {
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
                #[doc = "A `<"]
                #[doc = $tag]
                #[doc = ">` element"]
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
                        let tag = $tag;
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
            #[doc = "Make a `<"]
            #[doc = $tag]
            #[doc = ">` element"]
            pub fn $fn_name(elem_data: impl ElementData<element_structs::$name>) -> element_structs::$name {
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
        "a",
        download,
        href,
        hreflang,
        ping,
        referrerpolicy,
        rel,
        target,
        type_,
    ),
    (Abbr, abbr, "abbr"),
    (
        Area,
        area,
        "area",
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
    (Audio, audio, "audio", autoplay, controls, loop_, muted, preload, src),
    (B, b, "b"),
    (Base, base, "base", href, target),
    (Bdi, bdi, "bdi", dir),
    (Bdo, bdo, "bdo", dir),
    (Blockquote, blockquote, "blockquote", cite),
    (
        Body,
        body,
        "body",
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
    (Br, br, "br", clear),
    (
        Button,
        button,
        "button",
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
    (Canvas, canvas, "canvas", height, width),
    (Caption, caption, "caption"),
    (Cite, cite, "cite"),
    (Code, code, "code", type_),
    (Col, col, "col", span),
    (Colgroup, colgroup, "colgroup", span),
    (Dd, dd, "dd", type_),
    (Del, del, "del", cite, datetime),
    (Details, details, "details", open),
    (Dfn, dfn, "dfn"),
    (Div, div, "div"),
    (Dl, dl, "dl", type_),
    (Dt, dt, "dt", type_),
    (Em, em, "em", type_),
    (Embed, embed, "embed", height, src, type_, width),
    (Fieldset, fieldset, "fieldset", disabled, form, name),
    (
        Form,
        form,
        "form",
        accept_charset,
        action,
        autocomplete,
        enctype,
        method,
        name,
        novalidate,
        target
    ),
    (H1, h1, "h1"),
    (H2, h2, "h2"),
    (H3, h3, "h3"),
    (H4, h4, "h4"),
    (H5, h5, "h5"),
    (H6, h6, "h6"),
    (Head, head, "head", profile),
    (Hr, hr, "hr", align, color, noshade, size, width),
    (Html, html, "html", manifest, xmlns),
    (I, i, "i"),
    (
        Iframe,
        iframe,
        "iframe",
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
        "img",
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
        "input",
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
    (Ins, ins, "ins", cite, datetime),
    (Kbd, kbd, "kbd"),
    (Label, label, "label", for_),
    (Legend, legend, "legend"),
    (Li, li, "li", value),
    (
        Link,
        link,
        "link",
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
    (Map, map, "map", name),
    (Mark, mark, "mark"),
    (Menu, menu, "menu", type_, label),
    (
        Menuitem, menuitem, "menuitem", checked, command, default, disabled, icon, label,
        radiogroup, type_
    ),
    (Meta, meta, "meta", charset, http_equiv, name),
    (Meter, meter, "meter", high, low, max, min, optimum, value),
    (Noscript, noscript, "noscript"),
    (Object, object, "object", data, form, height, name, type_, usemap, width),
    (Ol, ol, "ol", reversed, start, type_),
    (Option, option, "option", disabled, label, selected, value),
    (Output, output, "output", for_, form, name),
    (P, p, "p"),
    (Param, param, "param", name, value),
    (Progress, progress, "progress", max, value),
    (Q, q, "q", cite),
    (Rp, rp, "rp"),
    (Rt, rt, "rt"),
    (Samp, samp, "samp"),
    (
        Script,
        script,
        "script",
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
    (Select, select, "select", disabled, form, multiple, name, required, size),
    (Slot, slot, "slot", name),
    (Small, small, "small"),
    (Source, source, "source", media, sizes, src, srcset, type_),
    (Span, span, "span"),
    (Strong, strong, "strong"),
    (Style, style_elem, "style", media, nonce, type_),
    (Sub, sub, "sub"),
    (Summary, summary, "summary"),
    (Sup, sup, "sup"),
    (Table, table, "table"),
    (Tbody, tbody, "tbody"),
    (Td, td, "td", colspan, headers, rowspan),
    (Template, template, "template"),
    (
        Textarea,
        textarea,
        "textarea",
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
    (Tfoot, tfoot, "tfoot"),
    (Th, th, "th", colspan, headers, rowspan, scope),
    (Thead, thead, "thead"),
    (Time, time, "time", datetime),
    (Title, title, "title"),
    (Tr, tr, "tr"),
    (Track, track, "track", default, kind, label, src, srclang),
    (Ul, ul, "ul"),
    (Var, var, "var"),
    (
        Video,
        video,
        "video",
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
    (Wbr, wbr, "wbr"),
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

/// An iterator of element data
///
/// #example
/// ```rust
///
/// let number_list = {
///     use hotman::*;
///     ul(Iter((1..=5).map(|i| li(i.to_string()))))
/// };
///
/// let expected = "\
/// <ul>
///     <li>1</li>
///     <li>2</li>
///     <li>3</li>
///     <li>4</li>
///     <li>5</li>
/// </ul>
/// ";
///
/// assert_eq!(number_list.to_string(), expected);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Iter<I>(pub I);

impl<I, E> ElementData<E> for Iter<I>
where
    I: IntoIterator,
    I::Item: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self.0 {
            child.add_to(elem);
        }
    }
}
