use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
pub use serde_ternary_fields_macro::serde_ternary_fields;

#[derive(Debug, Clone, PartialEq)]
pub enum TernaryOption<T> {
    Missing,
    Present(Option<T>),
}

use TernaryOption::*;

impl<T> TernaryOption<T> {
    pub fn is_missing(&self) -> bool {
        matches!(self, Missing)
    }

    pub fn is_present(&self) -> bool {
        !self.is_missing()
    }

    pub fn has_value(&self) -> bool {
        matches!(self, Present(Some(_)))
    }

    pub fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Present(Some(y)) => x == y,
            _ => false,
        }
    }

    pub fn as_ref(&self) -> TernaryOption<&T> {
        match *self {
            Present(Some(ref x)) => Present(Some(x)),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    pub fn as_mut(&mut self) -> TernaryOption<&mut T> {
        match *self {
            Present(Some(ref mut x)) => Present(Some(x)),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    pub fn map<U, F: FnOnce(Option<T>) -> Option<U>>(self, f: F) -> TernaryOption<U> {
        match self {
            Present(x) => Present(f(x)),
            Missing => Missing,
        }
    }

    pub fn map_value<U, F: FnOnce(T) -> U>(self, f: F) -> TernaryOption<U> {
        match self {
            Present(Some(x)) => Present(Some(f(x))),
            Present(None) => Present(None),
            Missing => Missing,
        }
    }

    pub fn map_or<U, F: FnOnce(Option<T>) -> Option<U>>(
        self,
        default: Option<U>,
        f: F,
    ) -> Option<U> {
        match self {
            Present(t) => f(t),
            Missing => default,
        }
    }

    pub fn map_value_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Present(Some(t)) => f(t),
            Present(None) => default,
            Missing => default,
        }
    }

    pub fn map_or_else<U, D: FnOnce() -> Option<U>, F: FnOnce(Option<T>) -> Option<U>>(
        self,
        default: D,
        f: F,
    ) -> Option<U> {
        match self {
            Present(t) => f(t),
            Missing => default(),
        }
    }

    pub fn map_value_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Present(Some(t)) => f(t),
            Present(None) => default(),
            Missing => default(),
        }
    }

    pub fn unwrap(self) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => panic!("called `TernaryOption::unwrap()` on a `Missing` value"),
        }
    }

    pub fn unwrap_value(self) -> T {
        match self {
            Present(Some(t)) => t,
            Present(None) => {
                panic!("called `TernaryOption::unwrap_value()` on a `Present(None)` value")
            }
            Missing => panic!("called `TernaryOption::unwrap_value()` on a `Missing` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => Some(default),
        }
    }

    pub fn unwrap_value_or(self, default: T) -> T {
        match self {
            Present(Some(t)) => t,
            _ => default,
        }
    }

    pub fn unwrap_or_else<F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> {
        match self {
            Present(x) => x,
            Missing => f(),
        }
    }

    pub fn unwrap_value_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Present(Some(x)) => x,
            _ => f(),
        }
    }

    pub fn expect(self, msg: &str) -> Option<T> {
        match self {
            Present(val) => val,
            Missing => panic!("{}", msg),
        }
    }

    pub fn expect_value(self, msg: &str) -> T {
        match self {
            Present(Some(val)) => val,
            _ => panic!("{}", msg),
        }
    }

    pub fn flatten(self) -> Option<T> {
        match self {
            Present(opt) => opt,
            Missing => None,
        }
    }

    pub fn ok_or<E>(self, err: E) -> Result<Option<T>, E> {
        match self {
            Present(v) => Ok(v),
            Missing => Err(err),
        }
    }

    pub fn ok_value_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Present(Some(v)) => Ok(v),
            _ => Err(err),
        }
    }

    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<Option<T>, E> {
        match self {
            Present(v) => Ok(v),
            Missing => Err(err()),
        }
    }

    pub fn ok_value_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Present(Some(v)) => Ok(v),
            _ => Err(err()),
        }
    }
}

impl<T: Default> TernaryOption<T> {
    pub fn unwrap_or_default(self) -> Option<T> {
        match self {
            Present(x) => x,
            Missing => Default::default(),
        }
    }

    pub fn unwrap_value_or_default(self) -> T {
        match self {
            Present(Some(x)) => x,
            _ => Default::default(),
        }
    }
}

impl<T> Default for TernaryOption<T> {
    fn default() -> Self {
        Missing
    }
}

impl<T> From<T> for TernaryOption<T> {
    fn from(val: T) -> TernaryOption<T> {
        Present(Some(val))
    }
}

impl<T> From<Option<T>> for TernaryOption<T> {
    fn from(opt: Option<T>) -> TernaryOption<T> {
        Present(opt)
    }
}

impl<T> From<Option<Option<T>>> for TernaryOption<T> {
    fn from(opt: Option<Option<T>>) -> TernaryOption<T> {
        match opt {
            Some(inner_opt) => Present(inner_opt),
            None => Missing,
        }
    }
}

impl<T: Copy> TernaryOption<&T> {
    /// Maps a `TernaryOption<&T>` to a `TernaryOption<T>` by copying the contents of the
    /// option.
    ///
    /// # Examples
    ///
    /// ```
    /// use ternary_option::TernaryOption::*;
    /// let x = 12;
    /// let opt_x = Some(&x);
    /// assert_eq!(opt_x, Some(&12));
    /// let copied = opt_x.copied();
    /// assert_eq!(copied, Some(12));
    /// ```
    pub fn copied(self) -> TernaryOption<T> {
        self.map_value(|&t| t)
    }
}

impl<T: Copy> TernaryOption<&mut T> {
    /// Maps a `TernaryOption<&mut T>` to an `TernaryOption<T>` by copying the contents of the
    /// option.
    ///
    /// # Examples
    ///
    /// ```
    /// use ternary_option::TernaryOption::*;
    /// let mut x = 12;
    /// let opt_x = Present(Some(&x));
    /// assert_eq!(opt_x, Present(Some(&x)));
    /// let copied = opt_x.copied();
    /// assert_eq!(copied, Present(Some(12)));
    /// ```
    pub fn copied(self) -> TernaryOption<T> {
        self.map_value(|&mut t| t)
    }
}

impl<T: Clone> TernaryOption<&T> {
    /// Maps a `TernaryOption<&T>` to a `TernaryOption<T>` by cloning the contents .
    ///
    /// # Examples
    ///
    /// ```
    /// use ternary_option::TernaryOption::*;
    /// let x = 12;
    /// let opt_x = Present(Some(&x));
    /// assert_eq!(opt_x, Present(Some(&x)));
    /// let cloned = opt_x.cloned();
    /// assert_eq!(cloned, Present(Some(12)));
    /// ```
    pub fn cloned(self) -> TernaryOption<T> {
        self.map(|t| t.cloned())
    }
}

impl<T: Clone> TernaryOption<&mut T> {
    /// Maps a `TernaryOption<&mut T>` to a `TernaryOption<T>` by cloning the contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use ternary_option::TernaryOption::*;
    /// let mut x = 12;
    /// let opt_x = Present(Some(&mut x));
    /// assert_eq!(opt_x, Present(Some(&mut 12)));
    /// let cloned = opt_x.cloned();
    /// assert_eq!(cloned, Present(Some(12)));
    /// ```
    pub fn cloned(self) -> TernaryOption<T> {
        self.map(|t| t.cloned())
    }
}

impl<T: Deref> TernaryOption<T> {
    /// Converts from `TernaryOption<T>` (or `&TernaryOption<T>`) to `TernaryOption<&T::Target>`.
    ///
    /// Leaves the original TernaryOption in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via [`Deref`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ternary_option::TernaryOption::{self, *};
    /// let x: TernaryOption<String> = Present(Some("hey".to_owned()));
    /// assert_eq!(x.as_deref(), Present(Some("hey")));
    ///
    /// let x: TernaryOption<String> = Present(None);
    /// assert_eq!(x.as_deref(), Present(None));
    /// ```
    pub fn as_deref(&self) -> TernaryOption<&T::Target> {
        self.as_ref().map_value(|t| t.deref())
    }
}

impl<T: DerefMut> TernaryOption<T> {
    /// Converts from `TernaryOption<T>` (or `&mut TernaryOption<T>`) to `TernaryOption<&mut T::Target>`.
    ///
    /// Leaves the original `TernaryOption` in-place, creating a new one containing a mutable reference to
    /// the inner type's `Deref::Target` type.
    ///
    /// # Examples
    ///
    /// ```
    /// use ternary_option::TernaryOption::{self, *};
    /// let mut x: TernaryOption<String> = Present(Some("hey".to_owned()));
    /// assert_eq!(x.as_deref_mut().map_value(|x| {
    ///     x.make_ascii_uppercase();
    ///     x
    /// }), Present(Some("HEY".to_owned().as_mut_str())));
    /// ```
    pub fn as_deref_mut(&mut self) -> TernaryOption<&mut T::Target> {
        self.as_mut().map_value(|t| t.deref_mut())
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for TernaryOption<T>
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
impl<T> Serialize for TernaryOption<T>
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
