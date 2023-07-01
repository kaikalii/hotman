use std::fmt;

use crate::*;

use paste::paste;

macro_rules! attribute_struct {
    ($name:ident[bool]) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
        #[allow(non_camel_case_types)]
        #[doc = "The `"]
        #[doc = stringify!($name)]
        #[doc = "` attribute"]
        pub struct $name;
        paste! {
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _t>] = bool;
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
        }
        impl $name {
            fn take(self) -> bool {
                true
            }
        }
    };
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
        #[allow(non_camel_case_types)]
        #[doc = "The `"]
        #[doc = stringify!($name)]
        #[doc = "` attribute"]
        pub struct $name<T = String>(pub T);
        paste! {
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _t>] = String;
            #[allow(non_camel_case_types)]
            pub(crate) type [<$name _ref_t>]<'a> = &'a str;
            #[allow(non_snake_case)]
            pub(crate) fn [<$name _take_ref>](val: &[<$name _t>]) -> [<$name _ref_t>] {
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
        }
        impl<T> $name<T> {
            fn take(self) -> T {
                self.0
            }
        }
    };
}

macro_rules! attribute_trait {
    ($name:ident [bool]) => {
        paste! {
            impl<E> ElementData<E> for $name
            where
                E: [<Has_ $name>]
            {
                fn add_to(self, element: &mut E) {
                    element.[<set_ $name>](self.take());
                }
            }
        }
    };
    ($name:ident) => {
        paste! {
            impl<E, T> ElementData<E> for $name<T>
            where
                E: [<Has_ $name>],
                T: Into<String>,
            {
                fn add_to(self, element: &mut E) {
                    element.[<set_ $name>](self.take());
                }
            }
        }
    };
}

macro_rules! attributes {
    ($($name:ident $([$ty:ident])?),* $(,)?) => {
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
                    pub trait [<Has_ $name>] {
                        #[doc = "Get the value of the `"]
                        #[doc = stringify!($name)]
                        #[doc = "` attribute"]
                        fn $name(&self) -> [<$name _ref_t>];
                        #[doc = "Set the value of the `"]
                        #[doc = stringify!($name)]
                        #[doc = "` attribute"]
                        fn [<set_ $name>](&mut self, value: impl Into<[<$name _t>]>);
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
    onunload,
    open[bool],
    optimum,
    pattern,
    ping,
    placeholder,
    playsinline,
    poster,
    preload,
    profile,
    async_[bool],
    for_,
    loop_[bool],
    type_,
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
