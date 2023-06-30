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
            )*
        }
    };
}

attributes!(
    accept_charset,
    accept,
    action,
    alt,
    autocomplete,
    autofocus,
    autoplay,
    charset,
    checked,
    cite,
    class,
    clear,
    cols,
    controls,
    coords,
    datetime,
    dir,
    disabled,
    download,
    enctype,
    form,
    formaction,
    formenctype,
    formmethod,
    formnovalidate,
    formtarget,
    height,
    href,
    hreflang,
    id,
    list,
    max_length,
    max,
    media,
    method,
    min_length,
    min,
    multiple,
    muted,
    name,
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
    pattern,
    ping,
    placeholder,
    preload,
    r#for,
    r#loop,
    r#type,
    readonly,
    referrerpolicy,
    rel,
    required,
    rows,
    size,
    sizes,
    span,
    src,
    srcdoc,
    srcset,
    step,
    style,
    target,
    title,
    usemap,
    value,
    width,
    wrap,
    profile,
    align,
    color,
    noshade,
    manifest,
    xmlns,
    crossorigin,
    decoding,
    importance,
    intrinsicsize,
    ismap,
    loading,
    dirname,
    integrity,
    command,
    default,
    label,
    radiogroup,
    selected,
    shape,
    icon,
    http_equiv,
    high,
    low,
    optimum,
    data,
    reversed,
    start,
    r#async,
    defer,
    nomodule,
    nonce,
    colspan,
    headers,
    rowspan,
    scope,
    kind,
    srclang,
    playsinline,
    poster,
);
