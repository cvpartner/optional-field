use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
pub use serde_optional_fields_macro::serde_optional_fields;

#[derive(Debug, Clone, PartialEq)]
pub enum Field<T> {
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
    /// Converts an `Field<`[`String`]`>` into an `Field<`[`usize`]`>`, preserving the original.
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

    /// Maps an `Field<T>` to `Field<U>` by applying a function to the value contained in
    /// the inner `Option`.
    ///
    /// # Examples
    ///
    /// Converts an `Field<`[`String`]`>` into an `Field<`[`usize`]`>`, consuming the original:
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

    /// Maps an `Field<T>` to `Field<U>` by applying a function to the option contained in `Present`.
    ///
    /// # Examples
    ///
    /// Converts an `Field<`[`String`]`>` into an `Field<`[`usize`]`>`, consuming the original:
    ///
    /// [`String`]: ../../std/string/struct.String.html
    /// ```
    /// # use optional_field::Field::{*, self};
    /// let maybe_some_string = Present(Some(String::from("Hello, World!")));
    /// // `Field::map` takes self *by value*, consuming `maybe_some_string`
    /// let maybe_some_len = maybe_some_string.map_present(|_| None);
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

    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Present(Some(t)) => f(t),
            Present(None) => default(),
            Missing => default(),
        }
    }

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

    pub fn unwrap(self) -> T {
        match self {
            Present(Some(t)) => t,
            Present(None) => {
                panic!("called `Field::unwrap_value()` on a `Present(None)` value")
            }
            Missing => panic!("called `Field::unwrap_value()` on a `Missing` value"),
        }
    }

    pub fn unwrap_present(self) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => panic!("called `Field::unwrap()` on a `Missing` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Present(Some(t)) => t,
            _ => default,
        }
    }

    pub fn unwrap_present_or(self, default: T) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => Some(default),
        }
    }

    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Present(Some(x)) => x,
            _ => f(),
        }
    }

    pub fn unwrap_present_or_else<F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> {
        match self {
            Present(x) => x,
            Missing => f(),
        }
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Present(Some(val)) => val,
            _ => panic!("{}", msg),
        }
    }

    pub fn expect_present(self, msg: &str) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => panic!("{}", msg),
        }
    }

    pub fn flatten(self) -> Option<T> {
        match self {
            Present(opt) => opt,
            Missing => None,
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

impl<T> Default for Field<T> {
    fn default() -> Self {
        Missing
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
