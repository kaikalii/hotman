#![warn(missing_docs)]

/*!
~~html~~
<br>
~~hot male~~
<br>
`hotman`.

🥵 Simple HTML generation in pure Rust with no macros 🥵

# Usage

Writing HTML with `hotman` is very similar to writing HTML itself.
All the same words are there, only the punctuation is different.

## Elements

Html elements are constructed using functions with the same name as the tag.

Examples are [`head`], [`body`], [`div`], and [`p`].

## `ElementData`

The [`ElementData`] trait is implemented for any type which adds either attributes or children to an element.

`ElementData` is also implemented for `Option`s, arrays, `Vec`s, some iterators, and tuples of `ElementData`s up to 20.

The element functions all take an `ElementData` as their argument, so you can pass tuples for multiple values.

## Attributes

Attributes are represented by structs with the same name as the attribute. They implement [`ElementData`].

Examples are [`Id`], [`Href`], [`Class`], and [`Style`].

## Events

Individual event handler attributes do not each have their own struct.

Instead, they can be added to elements via the [`On`] struct.

`On` implements [`ElementData`] and consists of an [`Event`] and a string representing the handler.

# Static Example

```rust
use hotman::*;

let dom = html((
    Comment("A simple login page"),
    head((
        meta(Charset("utf-8")),
        title("Login"),
        script(Src("/script.js")),
    )),
    body((
        h1("Login"),
        form((
            (Action("/login"), Method("POST")),
            input((
                Type("text"),
                Name("username"),
                Placeholder("Username"),
                On(Change, "validate_username()"),
                Autofocus,
            )),
            input((
                Type("password"),
                Name("password"),
                Placeholder("Password"),
                On(Change, "validate_password()"),
            )),
            input((Type("submit"), Value("Login"))),
        )),
        BR,
        p((
            "Don't have an account? ",
            a((Href("/register"), "Register")),
        )),
    )),
))
.page();

println!("{dom}");
```

# Iteration

A blanket implementation of [`ElementData`] for any `Iterator` would conflict with the implementaiton for tuples.

As a workaround, `ElementData` is implemented for the [`Map`], [`FilterMap`], and [`FlatMap`] iterators.

Because you usually map data to elements anyway, these implementations are usually more than enough.

```rust
let number_list = {
    use hotman::*;
    ul((1..=5).map(|i| li(i.to_string())))
};

assert_eq!(number_list.to_string(), "\
<ul>
    <li>1</li>
    <li>2</li>
    <li>3</li>
    <li>4</li>
    <li>5</li>
</ul>");
```

## Scoping

To make writing HTML as short as possible, hotman exports every element, attribute, and event in the root of the crate.

This means that there are many potential name conflicts with surrounding code.

It is recommended to scope `user hotman::*` to a small block of code:

```rust
let page_head = {
    use hotman::*;
    head((title("My Page"), meta((Name("Description"), Content("A page")))))
};
```
*/

mod attribute;
mod element;
mod format;

use std::{
    fmt,
    iter::{FilterMap, FlatMap, Map},
};

pub use attribute::*;
pub use element::*;
pub use Event::*;

/// A piece of data that can be added to an element
///
/// It is usually an attribute or a child element
pub trait ElementData<E> {
    /// Add this data to the given element
    fn add_to(self, element: &mut E);
}

impl<'a, E, D> ElementData<E> for D
where
    E: Element<'a>,
    D: Into<Node<'a>>,
{
    fn add_to(self, elem: &mut E) {
        elem.children_mut().push(self.into());
    }
}

impl<'a, E, D> ElementData<E> for Vec<D>
where
    E: Element<'a>,
    D: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

impl<'a, E, D, const N: usize> ElementData<E> for [D; N]
where
    E: Element<'a>,
    D: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

impl<'a, E, D> ElementData<E> for Option<D>
where
    E: Element<'a>,
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
            #[allow(non_snake_case, unused_variables)]
            fn add_to(self, elem: &mut Elem) {
                let ($($T,)*) = self;
                $($T.add_to(elem);)*
            }
        }
    };
}

tuple_element_data!();
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

impl<I, E, F> ElementData<E> for Map<I, F>
where
    Map<I, F>: Iterator,
    <Map<I, F> as Iterator>::Item: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

impl<I, E, F> ElementData<E> for FilterMap<I, F>
where
    FilterMap<I, F>: Iterator,
    <FilterMap<I, F> as Iterator>::Item: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

impl<I, E, U, F> ElementData<E> for FlatMap<I, U, F>
where
    U: IntoIterator,
    FlatMap<I, U, F>: Iterator,
    <FlatMap<I, U, F> as Iterator>::Item: ElementData<E>,
{
    fn add_to(self, elem: &mut E) {
        for child in self {
            child.add_to(elem);
        }
    }
}

/// A full HTML document.
///
/// Automatically adds the `<!DOCTYPE html>` tag.
///
/// Created with [`element_structs::Html::page`].
#[derive(Debug, Clone)]
pub struct Page<'a>(pub element_structs::Html<'a>);

impl<'a> fmt::Display for Page<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "<!DOCTYPE html>")?;
        write!(f, "{}", self.0)
    }
}

impl<'a> element_structs::Html<'a> {
    /// Prefix the HTML element with `<!DOCTYPE html>`
    pub fn page(self) -> Page<'a> {
        Page(self)
    }
}
