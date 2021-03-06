/// Constructs a `phf::Map` at compile time.
///
/// # Examples
///
/// ```rust
/// #![feature(plugin)]
/// #![plugin(phf_macros)]
///
/// extern crate phf;
///
/// static MY_MAP: phf::Map<&'static str, i32> = phf_map! {
///    "hello" => 10,
///    "world" => 11,
/// };
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! phf_map {
    ($($key:expr => $value:expr),*) => {/* ... */}
}

/// Constructs a `phf::Set` at compile time.
///
/// # Examples
///
/// ```rust
/// #![feature(plugin)]
/// #![plugin(phf_macros)]
///
/// extern crate phf;
///
/// static MY_SET: phf::Set<&'static str> = phf_set! {
///    "hello",
///    "world",
/// };
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! phf_set {
    ($($entry:expr),*) => {/* ... */}
}

/// Constructs a `phf::OrderedMap` at compile time.
///
/// # Examples
///
/// ```rust
/// #![feature(plugin)]
/// #![plugin(phf_macros)]
///
/// extern crate phf;
///
/// static MY_MAP: phf::OrderedMap<&'static str, i32> = phf_ordered_map! {
///    "hello" => 10,
///    "world" => 11,
/// };
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! phf_ordered_map {
    ($($key:expr => $value:expr),*) => {/* ... */}
}

/// Constructs a `phf::OrderedSet` at compile time.
///
/// # Examples
///
/// ```rust
/// #![feature(plugin)]
/// #![plugin(phf_macros)]
///
/// extern crate phf;
///
/// static MY_SET: phf::OrderedSet<&'static str> = phf_ordered_set! {
///    "hello",
///    "world",
/// };
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! phf_ordered_set {
    ($($entry:expr),*) => {/* ... */}
}
