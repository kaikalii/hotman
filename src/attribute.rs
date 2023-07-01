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
    };
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
        #[allow(non_camel_case_types)]
        #[doc = "The `"]
        #[doc = stringify!($name)]
        #[doc = "` attribute"]
        pub struct $name<T>(pub T);
    };
}

macro_rules! attribute_trait {
    ($name:ident[bool]) => {
        paste! {
            #[doc = "Trait for elements that have the `"]
            #[doc = stringify!($name)]
            #[doc = "` attribute"]
            #[allow(non_camel_case_types)]
            pub trait [<Has_ $name>] {
                #[doc = "Get the value of the `"]
                #[doc = stringify!($name)]
                #[doc = "` attribute"]
                fn $name(&self) -> bool;
                #[doc = "Set the value of the `"]
                #[doc = stringify!($name)]
                #[doc = "` attribute"]
                fn [<set_ $name>](&mut self, value: bool);
            }

            impl<E> ElementData<E> for $name
            where
                E: [<Has_ $name>],
            {
                fn add_to(self, element: &mut E) {
                    element.[<set_ $name>](true);
                }
            }
        }
    };
    ($name:ident) => {
        paste! {
            #[doc = "Trait for elements that have the `"]
            #[doc = stringify!($name)]
            #[doc = "` attribute"]
            #[allow(non_camel_case_types)]
            pub trait [<Has_ $name>] {
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
                E: [<Has_ $name>],
                T: Into<String>,
            {
                fn add_to(self, element: &mut E) {
                    element.[<set_ $name>](self.0);
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
            $(attribute_trait!($name $([$ty])*);)*
        }
    };
}

attributes!(
    accept_charset,
    accept,
    action,
    align,
    alt,
    autocomplete,
    autofocus[bool],
    autoplay,
    charset,
    checked,
    cite,
    class,
    clear,
    color,
    cols,
    colspan,
    command,
    controls,
    coords,
    crossorigin,
    data,
    datetime,
    decoding,
    default,
    defer,
    dir,
    dirname,
    disabled,
    download,
    enctype,
    form,
    formaction,
    formenctype,
    formmethod,
    formnovalidate,
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
    ismap,
    kind,
    label,
    list,
    loading,
    low,
    manifest,
    max_length,
    max,
    media,
    method,
    min_length,
    min,
    multiple,
    muted,
    name,
    nomodule,
    nonce,
    noshade,
    novalidate,
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
    open,
    optimum,
    pattern,
    ping,
    placeholder,
    playsinline,
    poster,
    preload,
    profile,
    r#async,
    r#for,
    r#loop,
    r#type,
    radiogroup,
    readonly,
    referrerpolicy,
    rel,
    required,
    reversed,
    rows,
    rowspan,
    scope,
    selected,
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
