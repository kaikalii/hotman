use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

use crate::{format::*, *};

use paste::paste;

/// Wrapper around attributes that are common to all elements
///
/// Since many elements don't have any of these attributes, this
/// wrapper keeps the size of the element structs small.
///
/// `Deref`s (and `DerefMut`s) to [`GlobalAttributesInner`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GlobalAttributes<'a>(Option<Box<GlobalAttributesInner<'a>>>);

impl<'a> GlobalAttributes<'a> {
    /// No attributes
    pub const EMPTY: Self = Self(None);
}

/// Attributes that are common to all elements
///
/// Wrapped by [`GlobalAttributes`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GlobalAttributesInner<'a> {
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
}

pub(crate) static DEFAULT_GLOBAL_ATTRIBUTES_INNER: GlobalAttributesInner<'static> =
    GlobalAttributesInner {
        id: Cow::Borrowed(""),
        class: Cow::Borrowed(""),
        style: Cow::Borrowed(""),
        title: Cow::Borrowed(""),
        autofocus: false,
        itemscope: false,
    };

impl<'a> Deref for GlobalAttributes<'a> {
    type Target = GlobalAttributesInner<'a>;
    fn deref(&self) -> &Self::Target {
        self.0
            .as_deref()
            .unwrap_or(&DEFAULT_GLOBAL_ATTRIBUTES_INNER)
    }
}

impl<'a> DerefMut for GlobalAttributes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.get_or_insert_with(std::default::Default::default)
    }
}

impl<'a> IndentFormat for GlobalAttributes<'a> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> fmt::Result {
        id_write(&self.id, f.f)?;
        class_write(&self.class, f.f)?;
        style_write(&self.style, f.f)?;
        title_write(&self.title, f.f)?;
        autofocus_write(&self.autofocus, f.f)?;
        itemscope_write(&self.itemscope, f.f)?;
        Ok(())
    }
}

/// The HTML events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Events<'a>(Vec<(&'static str, Cow<'a, str>)>);

impl<'a> Events<'a> {
    /// No events
    pub const NONE: Self = Self(Vec::new());
    /// Check if the events is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Check if the events contains an event with the given name
    pub fn contains(&self, name: &str) -> bool {
        self.0.iter().any(|(n, _)| n == &name)
    }
    /// Get the value of the event with the given name
    pub fn get(&self, name: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(n, _)| n == &name)
            .map(|(_, v)| v.as_ref())
    }
    /// Insert an event with the given name and value
    pub fn insert(&mut self, name: &'static str, value: impl Into<Cow<'a, str>>) {
        if let Some(i) = self.0.iter().position(|(n, _)| n == &name) {
            self.0[i].1 = value.into();
        } else {
            self.0.push((name, value.into()));
        }
    }
    /// Remove the event with the given name
    pub fn remove(&mut self, name: &str) {
        self.0.retain(|(n, _)| n != &name);
    }
    /// Iterate over the events
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &str)> {
        self.0.iter().map(|(n, v)| (*n, v.as_ref()))
    }
}

/// Add an event handler to an element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct On<V>(pub &'static str, pub V);

impl<'a, E, V> ElementData<E> for On<V>
where
    E: Element<'a>,
    V: Into<Cow<'a, str>>,
{
    fn add_to(self, element: &mut E) {
        element.events_mut().insert(self.0, self.1);
    }
}

macro_rules! attribute_struct {
    ($name:tt[bool]) => {
        paste! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
            #[doc = "The `"]
            #[doc = stringify!($name)]
            #[doc = "` attribute"]
            pub struct [<$name:camel>];
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _t>]<'a> = bool;
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _ref_t>] = bool;
            #[allow(non_snake_case)]
            pub(crate) fn [<$name _take_ref>](val: &[<$name _t>]) -> [<$name _ref_t>] {
                *val
            }
            #[allow(non_snake_case)]
            pub(crate) fn [<$name _write>](b: &bool, f: &mut fmt::Formatter) -> fmt::Result {
                if *b {
                    write!(f, " {}", stringify!($name).trim_end_matches('_'))
                } else {
                    Ok(())
                }
            }
            impl [<$name:camel>] {
                fn take(self) -> bool {
                    true
                }
            }
        }
    };
    ($name:tt) => {
        paste! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
            #[allow(non_camel_case_types)]
            #[doc = "The `"]
            #[doc = stringify!($name)]
            #[doc = "` attribute"]
            pub struct [<$name:camel>]<T = String>(pub T);
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _t>]<'a> = Cow<'a, str>;
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _ref_t>]<'a> = &'a str;
            #[allow(non_snake_case)]
            pub(crate) fn [<$name _take_ref>]<'a>(val: &'a [<$name _t>]) -> [<$name _ref_t>]<'a> {
                val
            }
            #[allow(non_snake_case)]
            pub(crate) fn [<$name _write>](s: &str, f: &mut fmt::Formatter) -> fmt::Result {
                if s.is_empty() {
                    Ok(())
                } else {
                    write!(f, " {}=\"{}\"", stringify!($name).trim_end_matches('_'), s)
                }
            }
            impl<T> [<$name:camel>]<T> {
                fn take(self) -> T {
                    self.0
                }
            }
        }
    };
}

macro_rules! attribute_trait {
    ($name:tt [bool]) => {
        paste! {
            impl<'a, E> ElementData<E> for [<$name:camel>]
            where
                E: [<Has $name:camel>]<'a>
            {
                fn add_to(self, element: &mut E) {
                    element.[<set_ $name>](self.take());
                }
            }
        }
    };
    ($name:tt) => {
        paste! {
            impl<'a, E, T> ElementData<E> for [<$name:camel>]<T>
            where
                E: [<Has $name:camel>]<'a>,
                T: Into<Cow<'a, str>>,
            {
                fn add_to(self, element: &mut E) {
                    element.[<set_ $name>](self.take());
                }
            }
        }
    };
}

macro_rules! attributes {
    ($($name:tt $([$ty:ident])?),* $(,)?) => {
        $(attribute_struct!($name $([$ty])*);)*
        pub mod attribute_traits {
            //! Traits that mark elements as having attributes
            use super::*;
            $(
                paste! {
                    #[doc = "Trait for elements that have the `"]
                    #[doc = stringify!($name)]
                    #[doc = "` attribute"]
                    #[allow(non_camel_case_types)]
                    pub trait [<Has $name:camel>]<'a> {
                        #[doc = "Get the value of the `"]
                        #[doc = stringify!($name)]
                        #[doc = "` attribute"]
                        fn [<get_ $name>](&self) -> [<$name _ref_t>];
                        #[doc = "Set the value of the `"]
                        #[doc = stringify!($name)]
                        #[doc = "` attribute"]
                        fn [<set_ $name>](&mut self, value: impl Into<[<$name _t>]<'a>>);
                    }
                }
                attribute_trait!($name $([$ty])*);
            )*
        }
    };
}

attributes!(
    accept_charset,
    accept,
    action,
    align,
    allow,
    alt,
    autocomplete,
    autofocus[bool],
    autoplay[bool],
    charset,
    checked[bool],
    cite,
    class,
    clear,
    color,
    cols,
    colspan,
    command,
    controls[bool],
    coords,
    crossorigin,
    data,
    datetime,
    decoding,
    default[bool],
    defer[bool],
    dir,
    dirname,
    disabled[bool],
    download,
    enctype,
    form,
    formaction,
    formenctype,
    formmethod,
    formnovalidate[bool],
    formtarget,
    headers,
    height,
    high,
    href,
    hreflang,
    http_equiv,
    icon,
    id,
    importance,
    integrity,
    intrinsicsize,
    ismap[bool],
    itemscope[bool],
    kind,
    label,
    list,
    loading,
    low,
    manifest,
    max_length,
    max,
    maxlength,
    media,
    method,
    min_length,
    min,
    minlength,
    multiple[bool],
    muted[bool],
    name,
    nomodule[bool],
    nonce,
    noshade,
    novalidate[bool],
    open[bool],
    optimum,
    pattern,
    ping,
    placeholder,
    playsinline,
    poster,
    preload,
    profile,
    async[bool],
    for,
    loop[bool],
    type,
    radiogroup,
    readonly[bool],
    referrerpolicy,
    rel,
    required[bool],
    reversed[bool],
    rows,
    rowspan,
    sandbox,
    scope,
    selected[bool],
    shape,
    size,
    sizes,
    span,
    src,
    srcdoc,
    srclang,
    srcset,
    start,
    step,
    style,
    target,
    title,
    usemap,
    value,
    width,
    wrap,
    xmlns,
);
