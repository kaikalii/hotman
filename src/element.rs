use std::{borrow::Cow, fmt};

use paste::paste;

use crate::{attribute, attribute_traits, format::*, ElementData};

/// Trait for types of elements
pub trait Element<'a> {
    /// Get the children of this element
    fn children(&self) -> &[Node<'a>];
    /// Get the mutable children of this element
    fn children_mut(&mut self) -> &mut Vec<Node<'a>>;
}

macro_rules! impl_global_attrs {
    ($name:ident, $($attr:ident),* $(,)?) => {
        $(
            paste! {
                impl<'a> attribute_traits::[<Has $attr:camel>]<'a> for $name<'a> {
                    fn [<get_ $attr>](&self) -> attribute::[<$attr _ref_t>] {
                        attribute::[<$attr _take_ref>](&self.$attr)
                    }
                    fn [<set_ $attr>](&mut self, val: impl Into<attribute::[<$attr _t>]<'a>>) {
                        self.$attr = val.into();
                    }
                }
            }
        )*
    }
}

macro_rules! write_attr {
    ($this:expr, $f:expr, $attr:ident) => {
        paste!(attribute::[<$attr _write>](&$this.$attr, $f.f)?);
    };
}

macro_rules! elements {
    ($(($name:ident $(,$attr:ident)* $(,)?)),* $(,)*) => {
        /// An HTML node
        #[derive(Debug, Clone)]
        pub enum Node<'a> {
            /// A text element
            Text(Cow<'a, str>),
            /// A comment,
            Comment(Cow<'a, str>),
            $(#[allow(missing_docs)] $name(element_structs::$name<'a>),)*
        }

        impl<'a> fmt::Display for Node<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Node::$name(element) => write!(f, "{element}"),)*
                    Node::Text(text) => write!(f, "{text}"),
                    Node::Comment(comment) => write!(f, "<!--{comment}-->"),
                }
            }
        }

        impl<'a> IndentFormat for Node<'a> {
            fn indent_fmt(&self, f: &mut IndentFormatter) -> fmt::Result {
                match self {
                    $(Node::$name(element) => element.indent_fmt(f),)*
                    Node::Text(text) => f.write(text),
                    Node::Comment(comment) => f.write(format_args!("<!--{comment}-->")),
                }
            }
        }

        pub mod element_structs {
            //! Structs that represent HTML elements

            use super::*;
            $(
                paste! {
                    #[derive(Debug, Clone, Default)]
                    #[doc = "A `<" [<$name:lower>] ">` element"]
                    pub struct $name<'a> {
                        /// The `id` attribute
                        pub id: Cow<'a, str>,
                        /// The `class` attribute
                        pub class: Cow<'a, str>,
                        /// The `style` attribute
                        pub style: Cow<'a, str>,
                        /// The `title` attribute
                        pub title: Cow<'a, str>,
                        /// The `autofocus` attribute
                        pub autofocus: bool,
                        /// The `itemscope` attribute
                        pub itemscope: bool,
                        $(
                            #[doc = "The `" $attr "` attribute"]
                            pub $attr: attribute::[<$attr _t>]<'a>,
                        )*
                        /// The children of this element
                        pub children: Vec<Node<'a>>,
                    }
                }

                impl<'a> IndentFormat for $name<'a> {
                    fn indent_fmt(&self, f: &mut IndentFormatter) -> fmt::Result {
                        let tag = paste!(stringify!([<$name:lower>]));
                        f.write(format_args!("<{tag}"))?;
                        write_attr!(self, f, id);
                        write_attr!(self, f, class);
                        write_attr!(self, f, style);
                        write_attr!(self, f, title);
                        write_attr!(self, f, autofocus);
                        write_attr!(self, f, itemscope);
                        $(write_attr!(self, f, $attr);)*
                        if self.children.is_empty() {
                            f.write(format_args!(" />"))?;
                            return Ok(());
                        }
                        f.write(format_args!(">"))?;
                        let single_line = self.children.len() == 1 || self.children.iter().any(|node| matches!(node, Node::Text(_)));
                        if single_line {
                            for child in &self.children {
                                child.indent_fmt(f)?;
                            }
                            f.write(format_args!("</{tag}>"))?;
                            return Ok(());
                        }
                        f.writeln("")?;
                        f.indent();
                        for child in &self.children {
                            child.indent_fmt(f)?;
                            f.writeln("")?;
                        }
                        f.dedent();
                        f.write(format_args!("</{tag}>"))?;
                        Ok(())
                    }
                }

                impl<'a> fmt::Display for $name<'a> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        self.indent_fmt(&mut IndentFormatter::from(f))
                    }
                }

                impl<'a> From<$name<'a>> for Node<'a> {
                    fn from(element: $name<'a>) -> Self {
                        Node::$name(element)
                    }
                }

                impl<'a> Element<'a> for $name<'a> {
                    fn children(&self) -> &[Node<'a>] {
                        &self.children
                    }
                    fn children_mut(&mut self) -> &mut Vec<Node<'a>> {
                        &mut self.children
                    }
                }

                impl_global_attrs!($name, id, class, style, title, autofocus, itemscope);

                $(
                    paste! {
                        impl<'a> attribute_traits::[<Has $attr:camel>]<'a> for $name<'a> {
                            fn [<get_ $attr>](&self) -> attribute::[<$attr _ref_t>] {
                                attribute::[<$attr _take_ref>](&self.$attr)
                            }
                            fn [<set_ $attr>](&mut self, val: impl Into<attribute::[<$attr _t>]<'a>>) {
                                self.$attr = val.into();
                            }
                        }
                    }
                )*
            )*
        }

        $(paste! {
            #[must_use]
            #[doc = "Make a `<" [<$name:lower>] ">` element"]
            pub fn [<$name:lower>]<'a>(elem_data: impl ElementData<element_structs::$name<'a>>) -> element_structs::$name<'a> {
                let mut elem = Default::default();
                elem_data.add_to(&mut elem);
                elem
            }
        })*
    };
}

impl<'a> From<String> for Node<'a> {
    fn from(text: String) -> Self {
        Node::Text(text.into())
    }
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(text: &'a str) -> Self {
        Node::Text(text.into())
    }
}

impl<'a> From<&'a String> for Node<'a> {
    fn from(text: &'a String) -> Self {
        Node::Text(text.as_str().into())
    }
}

/// An HTML comment
#[derive(Debug, Clone)]
pub struct Comment<T>(pub T);

impl<'a, T> From<Comment<T>> for Node<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(comment: Comment<T>) -> Self {
        Node::Comment(comment.0.into())
    }
}

elements!(
    (
        A,
        download,
        href,
        hreflang,
        ping,
        referrerpolicy,
        rel,
        target,
        r#type,
    ),
    (Abbr),
    (
        Area,
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
    (Audio, autoplay, controls, r#loop, muted, preload, src),
    (B),
    (Base, href, target),
    (Bdi, dir),
    (Bdo, dir),
    (Blockquote, cite),
    (
        Body,
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
    (Br, clear),
    (
        Button,
        disabled,
        form,
        formaction,
        formenctype,
        formmethod,
        formnovalidate,
        formtarget,
        name,
        r#type,
        value
    ),
    (Canvas, height, width),
    (Caption),
    (Cite),
    (Code, r#type),
    (Col, span),
    (Colgroup, span),
    (Dd, r#type),
    (Del, cite, datetime),
    (Details, open),
    (Dfn),
    (Div),
    (Dl, r#type),
    (Dt, r#type),
    (Em, r#type),
    (Embed, height, src, r#type, width),
    (Fieldset, disabled, form, name),
    (
        Form,
        accept_charset,
        action,
        autocomplete,
        enctype,
        method,
        name,
        novalidate,
        target
    ),
    (H1),
    (H2),
    (H3),
    (H4),
    (H5),
    (H6),
    (Head, profile),
    (Hr, align, color, noshade, size, width),
    (Html, manifest, xmlns),
    (I),
    (
        Iframe,
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
        r#type,
        value,
        width
    ),
    (Ins, cite, datetime),
    (Kbd),
    (Label, r#for),
    (Legend),
    (Li, value),
    (
        Link,
        href,
        rel,
        media,
        hreflang,
        r#type,
        sizes,
        crossorigin,
        integrity,
        referrerpolicy
    ),
    (Map, name),
    (Mark),
    (Menu, r#type, label),
    (Menuitem, checked, command, default, disabled, icon, label, radiogroup, r#type),
    (Meta, charset, http_equiv, name),
    (Meter, high, low, max, min, optimum, value),
    (Noscript),
    (Object, data, form, height, name, r#type, usemap, width),
    (Ol, reversed, start, r#type),
    (Option, disabled, label, selected, value),
    (Output, r#for, form, name),
    (P),
    (Param, name, value),
    (Progress, max, value),
    (Q, cite),
    (Rp),
    (Rt),
    (Samp),
    (
        Script,
        r#async,
        crossorigin,
        defer,
        integrity,
        nomodule,
        nonce,
        referrerpolicy,
        r#type,
        src
    ),
    (Select, disabled, form, multiple, name, required, size),
    (Slot, name),
    (Small),
    (Source, media, sizes, src, srcset, r#type),
    (Span),
    (Strong),
    (Style, media, nonce, r#type),
    (Sub),
    (Summary),
    (Sup),
    (Table),
    (Tbody),
    (Td, colspan, headers, rowspan),
    (Template),
    (
        Textarea,
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
    (Tfoot),
    (Th, colspan, headers, rowspan, scope),
    (Thead),
    (Time, datetime),
    (Title),
    (Tr),
    (Track, default, kind, label, src, srclang),
    (Ul),
    (Var),
    (
        Video,
        autoplay,
        controls,
        crossorigin,
        height,
        r#loop,
        muted,
        playsinline,
        poster,
        preload,
        src,
        width
    ),
    (Wbr),
);
