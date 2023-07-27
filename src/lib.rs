use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
pub use optional_fields_serde_macro::serde_optional_fields;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Field<T> {
    #[default]
    Missing,
    Present(Option<T>),
}

use Field::*;

impl<T> Field<T> {
    /// Is the value missing?
    ///
    /// # Examples
    /// ```
    /// # use optional_field::Field::{*, self};
    /// assert!(Missing::<u8>.is_missing());
    /// assert!(!Present::<u8>(None).is_missing());
    /// assert!(!Present(Some(1)).is_missing());
    /// ```
    #[inline]
    pub fn is_missing(&self) -> bool {
        matches!(self, Missing)
    }

    /// Is the value present?
    ///
    /// # Examples
    /// ```
    /// # use optional_field::Field::{*, self};
    /// assert!(!Missing::<u8>.is_present());
    /// assert!(Present::<u8>(None).is_present());
    /// assert!(Present(Some(1)).is_present());
    /// ```
    #[inline]
    pub fn is_present(&self) -> bool {
        !self.is_missing()
    }

    /// Is present and the value is not None?
    ///
    /// # Examples
    /// ```
    /// # use optional_field::Field::{*, self};
    /// assert!(!Missing::<u8>.has_value());
    /// assert!(!Present::<u8>(None).has_value());
    /// assert!(Present(Some(1)).has_value());
    /// ```
    #[inline]
    pub fn has_value(&self) -> bool {
        matches!(self, Present(Some(_)))
    }

    /// Does the value contain the given value?
    ///
    /// # Examples
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = 1;
    /// assert!(!Missing::<u8>.contains(&x));
    /// assert!(!Present::<u8>(None).contains(&x));
    /// assert!(Present(Some(1)).contains(&x));
    /// ```
    #[inline]
    pub fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Present(Some(y)) => x == y,
            _ => false,
        }
    }

    /// Converts from `&Field<T>` to `Field<&T>`.
    ///
    /// # Examples
    ///
    /// Converts a `Field<`[`String`]`>` into an `Field<`[`usize`]`>`, preserving the original.
    /// The [`map`] method takes the `self` argument by value, consuming the original,
    /// so this technique uses `as_ref` to first take an `Field` to a reference
    /// to the value inside the original.
    ///
    /// [`map`]: Field::map
    /// [`String`]: ../../std/string/struct.String.html
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let text: Field<String> = Present(Some("Hello, world!".to_string()));
    /// // First, cast `Field<String>` to `Field<&String>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `text` on the stack.
    /// let text_length: Field<usize> = text.as_ref().map(|s| s.len());
    /// println!("still can print text: {:?}", text);
    /// ```
    #[inline]
    pub fn as_ref(&self) -> Field<&T> {
        match *self {
            Present(Some(ref x)) => Present(Some(x)),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    /// Converts from `&mut Field<T>` to `Field<&mut T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let mut x = Present(Some(2));
    /// match x.as_mut() {
    ///     Present(Some(v)) => *v = 42,
    ///     _ => {},
    /// }
    /// assert_eq!(x, Present(Some(42)));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Field<&mut T> {
        match *self {
            Present(Some(ref mut x)) => Present(Some(x)),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    /// Maps a `Field<T>` to `Field<U>` by applying a function to the value contained in
    /// the inner `Option`.
    ///
    /// # Examples
    ///
    /// Converts a `Field<`[`String`]`>` into an `Field<`[`usize`]`>`, consuming the original:
    ///
    /// [`String`]: ../../std/string/struct.String.html
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let maybe_some_string = Present(Some(String::from("Hello, World!")));
    /// // `Field::map` takes self *by value*, consuming `maybe_some_string`
    /// let maybe_some_len = maybe_some_string.map(|s| s.len());
    ///
    /// assert_eq!(maybe_some_len, Present(Some(13)));
    /// ```
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Field<U> {
        match self {
            Present(Some(x)) => Present(Some(f(x))),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    /// Maps a `Field<T>` to `Field<U>` by applying a function to the option contained in `Present`.
    ///
    /// # Examples
    ///
    /// Converts a `Field<`[`String`]`>` into an `Field<`[`usize`]`>`, consuming the original:
    ///
    /// [`String`]: ../../std/string/struct.String.html
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let maybe_some_string = Present(Some(String::from("Hello, World!")));
    /// // `Field::map_present` takes self *by value*, consuming `maybe_some_string`
    /// let maybe_some_len = maybe_some_string.map_present(|s| None);
    ///
    /// assert_eq!(maybe_some_len, Present::<usize>(None));
    /// ```
    #[inline]
    pub fn map_present<U, F: FnOnce(Option<T>) -> Option<U>>(self, f: F) -> Field<U> {
        match self {
            Present(x) => Present(f(x)),
            Missing => Missing,
        }
    }

    /// Applies a function to the value contained in the inner `Option` (if any),
    /// or returns the provided default (if not).
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`map_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`map_or_else`]: Option::map_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some("foo"));
    /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    ///
    /// let x: Field<&str> = Missing;
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    /// ```
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Present(Some(t)) => f(t),
            Present(None) => default,
            Missing => default,
        }
    }

    /// Applies a function to the value contained in `Present` (if any),
    /// or returns the provided default (if not).
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some("foo"));
    /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    ///
    /// let x: Field<&str> = Missing;
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    /// ```
    #[inline]
    pub fn map_present_or<U, F: FnOnce(Option<T>) -> Option<U>>(
        self,
        default: Option<U>,
        f: F,
    ) -> Option<U> {
        match self {
            Present(t) => f(t),
            Missing => default,
        }
    }

    /// Computes a default function result if the field is Missing or Present(None), or
    /// applies a different function to the contained value (if any).
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let k = 21;
    ///
    /// let x = Present(Some("foo"));
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
    /// ```
    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Present(Some(t)) => f(t),
            Present(None) => default(),
            Missing => default(),
        }
    }

    /// Computes a default function result (if missing), or
    /// applies a different function to the contained value (if any).
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let k = 21;
    ///
    /// let x = Present(Some("foo"));
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
    /// ```
    #[inline]
    pub fn map_present_or_else<U, D: FnOnce() -> Option<U>, F: FnOnce(Option<T>) -> Option<U>>(
        self,
        default: D,
        f: F,
    ) -> Option<U> {
        match self {
            Present(t) => f(t),
            Missing => default(),
        }
    }

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Missing`] or Present(None).
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some("air"));
    /// assert_eq!(x.unwrap(), "air");
    /// ```
    ///
    /// ```should_panic
    /// # use optional_field::Field::{*, self};
    /// let x: Field<&str> = Present(None);
    /// assert_eq!(x.unwrap(), "air"); // fails
    /// ```
    pub fn unwrap(self) -> T {
        match self {
            Present(Some(t)) => t,
            Present(None) => {
                panic!("called `Field::unwrap()` on a `Present(None)` value")
            }
            Missing => panic!("called `Field::unwrap()` on a `Missing` value"),
        }
    }

    /// Returns the contained option, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Missing`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some("air"));
    /// assert_eq!(x.unwrap_present(), Some("air"));
    /// ```
    ///
    /// ```should_panic
    /// # use optional_field::Field::{*, self};
    /// let x: Field<&str> = Missing;
    /// assert_eq!(x.unwrap_present(), Some("air")); // fails
    /// ```
    pub fn unwrap_present(self) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => panic!("called `Field::unwrap_present()` on a `Missing` value"),
        }
    }

    /// Returns a reference to the contained option.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Missing`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some("air"));
    /// assert_eq!(x.unwrap_present_ref(), &Some("air"));
    /// ```
    ///
    /// ```should_panic
    /// # use optional_field::Field::{*, self};
    /// let x: Field<&str> = Missing;
    /// assert_eq!(x.unwrap_present_ref(), &Some("air")); // fails
    /// ```
    pub fn unwrap_present_ref(&self) -> &Option<T> {
        match self {
            Present(ref val) => val,
            Missing => panic!("called `Field::unwrap_present_ref()` on a `Missing` value"),
        }
    }

    /// Returns a mutable reference to the contained option.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Missing`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let mut x = Present(Some("air"));
    /// assert_eq!(x.unwrap_present_mut(), &mut Some("air"));
    /// ```
    ///
    /// ```should_panic
    /// # use optional_field::Field::{*, self};
    /// let mut x: Field<&str> = Missing;
    /// assert_eq!(x.unwrap_present_mut(), &mut Some("air")); // fails
    /// ```
    pub fn unwrap_present_mut(&mut self) -> &mut Option<T> {
        match self {
            Present(ref mut val) => val,
            Missing => panic!("called `Field::unwrap_present_mut()` on a `Missing` value"),
        }
    }

    /// Returns the contained [`Some`] value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: Field::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// assert_eq!(Present(Some("car")).unwrap_or("bike"), "car");
    /// assert_eq!(Present(None).unwrap_or("bike"), "bike");
    /// assert_eq!(Missing.unwrap_or("bike"), "bike");
    /// ```
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Present(Some(t)) => t,
            _ => default,
        }
    }

    /// Returns the contained [`Present`] value or a provided default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// assert_eq!(Present(Some("car")).unwrap_present_or(Some("bike")), Some("car"));
    /// assert_eq!(Present(None).unwrap_present_or(Some("bike")), None);
    /// assert_eq!(Missing.unwrap_present_or(Some("bike")), Some("bike"));
    /// ```
    pub fn unwrap_present_or(self, default: Option<T>) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => default,
        }
    }

    /// Returns the contained [`Some`] value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let k = 10;
    /// assert_eq!(Present(Some(4)).unwrap_or_else(|| 2 * k), 4);
    /// assert_eq!(Present(None).unwrap_or_else(|| 2 * k), 20);
    /// assert_eq!(Missing.unwrap_or_else(|| 2 * k), 20);
    /// ```
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Present(Some(x)) => x,
            _ => f(),
        }
    }

    /// Returns the contained [`Present`] value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// assert_eq!(Present(Some(4)).unwrap_present_or_else(|| Some(10)), Some(4));
    /// assert_eq!(Present(None).unwrap_present_or_else(|| Some(20)), None);
    /// assert_eq!(Missing.unwrap_present_or_else(|| Some(30)), Some(30));
    /// ```
    pub fn unwrap_present_or_else<F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> {
        match self {
            Present(x) => x,
            Missing => f(),
        }
    }

    /// Returns the contained [`Some`] value from within [`Present`], consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Missing`] or ['None'] with a custom panic message provided by
    /// `msg`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some("value"));
    /// assert_eq!(x.expect("fruits are healthy"), "value");
    /// ```
    ///
    /// ```should_panic
    /// # use optional_field::Field::{*, self};
    /// let x: Field<Option<&str>> = Missing;
    /// x.expect("fruits are healthy"); // panics with `fruits are healthy`
    /// ```
    pub fn expect(self, msg: &str) -> T {
        match self {
            Present(Some(val)) => val,
            _ => panic!("{}", msg),
        }
    }

    /// Returns the contained [`Option`] in the [`Present`], consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Missing`] with a custom panic message provided by
    /// `msg`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x: Field<Option<&str>> = Present(None);
    /// assert_eq!(x.expect_present("fruits are healthy"), None);
    /// ```
    ///
    /// ```should_panic
    /// # use optional_field::Field::{*, self};
    /// let x: Field<Option<&str>> = Missing;
    /// x.expect("fruits are healthy"); // panics with `fruits are healthy`
    /// ```
    pub fn expect_present(self, msg: &str) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => panic!("{}", msg),
        }
    }

    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Present(Some(v)) => Ok(v),
            _ => Err(err),
        }
    }

    pub fn ok_present_or<E>(self, err: E) -> Result<Option<T>, E> {
        match self {
            Present(v) => Ok(v),
            Missing => Err(err),
        }
    }

    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Present(Some(v)) => Ok(v),
            _ => Err(err()),
        }
    }

    pub fn ok_present_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<Option<T>, E> {
        match self {
            Present(v) => Ok(v),
            Missing => Err(err()),
        }
    }

    /// Returns [`None`] if the option is [`None`], otherwise returns `fieldb`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some(2));
    /// let y: Field<&str> = Present(None);
    /// assert_eq!(x.and(y), Present(None));
    ///
    /// let x: Field<u32> = Present(None);
    /// let y = Present(Some("foo"));
    /// assert_eq!(x.and(y), Present(None));
    ///
    /// let x = Present(Some(2));
    /// let y = Present(Some("foo"));
    /// assert_eq!(x.and(y), Present(Some("foo")));
    ///
    /// let x: Field<u32> = Present(None);
    /// let y: Field<&str> = Present(None);
    /// assert_eq!(x.and(y), Present(None));

    /// let x: Field<u32> = Missing;
    /// let y: Field<&str> = Missing;
    /// assert_eq!(x.and(y), Missing);
    /// ```
    pub fn and<U>(self, fieldb: Field<U>) -> Field<U> {
        match self {
            Present(Some(_)) => fieldb,
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    /// Returns [`None`] if the option is [`None`], otherwise returns `fieldb`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x = Present(Some(2));
    /// let y: Field<&str> = Present(None);
    /// assert_eq!(x.and(y), Present(None));
    ///
    /// let x: Field<u32> = Present(None);
    /// let y = Present(Some("foo"));
    /// assert_eq!(x.and(y), Present(None));
    ///
    /// let x = Present(Some(2));
    /// let y = Present(Some("foo"));
    /// assert_eq!(x.and(y), Present(Some("foo")));
    ///
    /// let x: Field<u32> = Present(None);
    /// let y: Field<&str> = Present(None);
    /// assert_eq!(x.and(y), Present(None));

    /// let x: Field<u32> = Missing;
    /// let y: Field<&str> = Missing;
    /// assert_eq!(x.and(y), Missing);
    /// ```
    pub fn and_present<U>(self, fieldb: Field<U>) -> Field<U> {
        match self {
            Present(_) => fieldb,
            Missing => Missing,
        }
    }

    /// Returns [`Missing`] if the field is [`Missing`], Present(None) if the field
    /// is Present(None), otherwise calls `f` with the wrapped value and returns the result.
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x: Field<&str> = Present(Some("foo"));
    /// assert_eq!(x.clone().and_then(|_s| Present(Some(1)) ), Present(Some(1)));
    /// assert_eq!(x.clone().and_then(|_s| Missing::<u8> ), Missing::<u8>);
    /// assert_eq!(x.and_then(|_s| Present::<u8>(None) ), Present::<u8>(None));
    /// ```
    pub fn and_then<U, F>(self, f: F) -> Field<U>
    where
        F: FnOnce(T) -> Field<U>,
    {
        match self {
            Present(Some(x)) => f(x),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    /// Returns [`Missing`] if the field is [`Missing`], otherwise calls `f`
    /// with the wrapped value and returns the result.
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let x: Field<&str> = Present(Some("foo"));
    /// assert_eq!(x.clone().and_then_present(|_s| Present(Some(1)) ), Present(Some(1)));
    /// assert_eq!(x.clone().and_then_present(|_s| Missing::<u8> ), Missing::<u8>);
    /// assert_eq!(x.and_then_present(|_s| Present::<u8>(None) ), Present::<u8>(None));
    /// ```
    pub fn and_then_present<U, F>(self, f: F) -> Field<U>
    where
        F: FnOnce(Option<T>) -> Field<U>,
    {
        match self {
            Present(x) => f(x),
            Missing => Missing,
        }
    }

    /// Inserts `value` into the option if it is [`Missing`] or [Present(None)], then
    /// returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let mut x = Missing;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert(5);
    ///     assert_eq!(y, &5);
    ///
    ///     *y = 7;
    /// }
    ///
    /// assert_eq!(x, Present(Some(7)));
    /// ```
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        match *self {
            Missing | Present(None) => {
                *self = Present(Some(value));
            }
            _ => {}
        }

        self.as_mut().unwrap()
    }

    /// Inserts `value` into the option if it is [`Missing`] or [Present(None)], then
    /// returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let mut x = Missing;
    ///
    /// {
    ///     let y: &mut Option<u32> = x.get_or_insert_present(Some(5));
    ///     assert_eq!(y, &Some(5));
    ///
    ///     *y = Some(7);
    /// }
    ///
    /// assert_eq!(x, Present(Some(7)));
    /// ```
    pub fn get_or_insert_present(&mut self, value: Option<T>) -> &mut Option<T> {
        if let Missing = *self {
            *self = Present(value);
        }

        self.unwrap_present_mut()
    }

    /// Inserts a value computed from `f` into the option if it is [`Missing`] or [Present(None)],
    /// then returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let mut x = Missing;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert_with(|| 5);
    ///     assert_eq!(y, &5);
    ///
    ///     *y = 7;
    /// }
    ///
    /// assert_eq!(x, Present(Some(7)));
    /// ```
    pub fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        match self {
            Missing | Present(None) => {
                *self = Present(Some(f()));
            }
            _ => {}
        }

        self.as_mut().unwrap()
    }

    /// Inserts a value computed from `f` into the option if it is [`Missing`] or [Present(None)],
    /// then returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let mut x = Missing;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert_with(|| 5);
    ///     assert_eq!(y, &5);
    ///
    ///     *y = 7;
    /// }
    ///
    /// assert_eq!(x, Present(Some(7)));
    /// ```
    pub fn get_or_insert_with_present<F>(&mut self, f: F) -> &mut Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        if let Missing = self {
            *self = Present(f());
        }

        self.unwrap_present_mut()
    }
}

impl<T: Default> Field<T> {
    pub fn unwrap_or_default(self) -> T {
        match self {
            Present(Some(x)) => x,
            _ => Default::default(),
        }
    }

    pub fn unwrap_present_or_default(self) -> Option<T> {
        match self {
            Present(x) => x,
            Missing => Default::default(),
        }
    }
}

impl<T> From<T> for Field<T> {
    fn from(val: T) -> Field<T> {
        Present(Some(val))
    }
}

impl<T> From<Option<T>> for Field<T> {
    fn from(opt: Option<T>) -> Field<T> {
        Present(opt)
    }
}

impl<T> From<Option<Option<T>>> for Field<T> {
    fn from(opt: Option<Option<T>>) -> Field<T> {
        match opt {
            Some(inner_opt) => Present(inner_opt),
            None => Missing,
        }
    }
}

impl<T: Copy> Field<&T> {
    /// Maps a `Field<&T>` to a `Field<T>` by copying the contents of the
    /// option.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::*;
    /// let x = 12;
    /// let opt_x = Some(&x);
    /// assert_eq!(opt_x, Some(&12));
    /// let copied = opt_x.copied();
    /// assert_eq!(copied, Some(12));
    /// ```
    pub fn copied(self) -> Field<T> {
        self.map(|&t| t)
    }
}

impl<T: Copy> Field<&mut T> {
    /// Maps a `Field<&mut T>` to an `Field<T>` by copying the contents of the
    /// option.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::*;
    /// let mut x = 12;
    /// let opt_x = Present(Some(&x));
    /// assert_eq!(opt_x, Present(Some(&x)));
    /// let copied = opt_x.copied();
    /// assert_eq!(copied, Present(Some(12)));
    /// ```
    pub fn copied(self) -> Field<T> {
        self.map(|&mut t| t)
    }
}

impl<T: Clone> Field<&T> {
    /// Maps a `Field<&T>` to a `Field<T>` by cloning the contents .
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::*;
    /// let x = 12;
    /// let opt_x = Present(Some(&x));
    /// assert_eq!(opt_x, Present(Some(&x)));
    /// let cloned = opt_x.cloned();
    /// assert_eq!(cloned, Present(Some(12)));
    /// ```
    pub fn cloned(self) -> Field<T> {
        self.map(|t| t.clone())
    }
}

impl<T: Clone> Field<&mut T> {
    /// Maps a `Field<&mut T>` to a `Field<T>` by cloning the contents.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::*;
    /// let mut x = 12;
    /// let opt_x = Present(Some(&mut x));
    /// assert_eq!(opt_x, Present(Some(&mut 12)));
    /// let cloned = opt_x.cloned();
    /// assert_eq!(cloned, Present(Some(12)));
    /// ```
    pub fn cloned(self) -> Field<T> {
        self.map(|t| t.clone())
    }
}

impl<T: Deref> Field<T> {
    /// Converts from `Field<T>` (or `&Field<T>`) to `Field<&T::Target>`.
    ///
    /// Leaves the original Field in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via [`Deref`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{self, *};
    /// let x: Field<String> = Present(Some("hey".to_owned()));
    /// assert_eq!(x.as_deref(), Present(Some("hey")));
    ///
    /// let x: Field<String> = Present(None);
    /// assert_eq!(x.as_deref(), Present(None));
    /// ```
    pub fn as_deref(&self) -> Field<&T::Target> {
        self.as_ref().map(|t| t.deref())
    }
}

impl<T: DerefMut> Field<T> {
    /// Converts from `Field<T>` (or `&mut Field<T>`) to `Field<&mut T::Target>`.
    ///
    /// Leaves the original `Field` in-place, creating a new one containing a mutable reference to
    /// the inner type's `Deref::Target` type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{self, *};
    /// let mut x: Field<String> = Present(Some("hey".to_owned()));
    /// assert_eq!(x.as_deref_mut().map(|x| {
    ///     x.make_ascii_uppercase();
    ///     x
    /// }), Present(Some("HEY".to_owned().as_mut_str())));
    /// ```
    pub fn as_deref_mut(&mut self) -> Field<&mut T::Target> {
        self.as_mut().map(|t| t.deref_mut())
    }
}

impl<T> Field<T>
where
    T: Clone + PartialEq,
{
    /// Returns a new Field<T> which is the difference between
    /// `Self` and `other`.
    ///
    /// Assumes `Self` is the current value and `other` is the "new" value,
    /// it evaluates if the value has changed and if so returns a field
    /// with the new value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use optional_field::Field::{self, *};
    /// let old = Present(Some("oh hai"));
    /// // Values are the same
    /// assert_eq!(Missing, old.delta(&Present(Some("oh hai"))));
    /// // There is no new value to compare
    /// assert_eq!(Missing, old.delta(&Missing));
    /// // The value has changed
    /// assert_eq!(Present(Some("new")), old.delta(&Present(Some("new"))));
    /// ```
    pub fn delta(&self, other: &Field<T>) -> Field<T> {
        if self != other && other.has_value() {
            return other.clone();
        }

        Field::Missing
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Field<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Field<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Present(opt) = self {
            opt.serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}
