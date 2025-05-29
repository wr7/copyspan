//! A an alternative to `Range<T>` that has a defined memory layout, implements
//! [`std::marker::Copy`], and has some convenience methods.
//!
//! ```
//! use copyspan::Span;
//!
//! let text = "hello world";
//! let s = Span::from(6..11);
//!
//! for i in s {
//!     dbg!(i);
//! }
//!
//! // Because `Span` is copyable, we can reuse it without calling `clone`
//! assert_eq!(&text[s], "world");
//! assert_eq!(&text[s.with_len(2)], "wo");
//!```
//!
//! This is also useful for making copyable datastructures that contain ranges.
//! ```
//! use copyspan::Span;
//! use std::ops::Range;
//!
//! #[derive(Clone, Copy, Default)]
//! struct HoldsSpan {
//!     x: Span<usize>,
//! }
//!
//! fn expects_range(_: Range<usize>) {}
//! fn takes_val(_: HoldsSpan) {}
//!
//! let val = HoldsSpan::default();
//! takes_val(val); // If `HoldSpan` wasn't `Copy`, `val` would be moved into this function
//!
//! expects_range(val.x.range());
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;

mod util;

use core::{
    hash::{Hash, Hasher},
    ops::{Index, IndexMut, Range},
};

use util::RustNumber;

/// An alternative to `Range<T>` that has a defined memory layout and implements
/// [`std::marker::Copy`].
#[repr(C)]
pub struct Span<T: RustNumber = usize> {
    pub start: T,
    pub end: T,
}

impl<T: RustNumber> Clone for Span<T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            end: self.end,
        }
    }
}

impl<T: RustNumber> Copy for Span<T> {}

impl<T: RustNumber> Span<T> {
    /// A zero-width span at the end of a span
    #[must_use]
    pub const fn span_after(self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }

    /// A zero-width span at the start of a span
    #[must_use]
    pub const fn span_at(self) -> Self {
        Self {
            start: self.start,
            end: self.start,
        }
    }

    /// Sets the length of a span without changing its start
    #[must_use]
    pub fn with_len(self, len: T) -> Self {
        Self {
            start: self.start,
            end: self.start + len,
        }
    }

    /// Sets the start of a span without changing its end
    #[must_use]
    pub const fn with_start(self, start: T) -> Self {
        Self {
            start,
            end: self.end,
        }
    }

    /// Sets the end of a span without changing its start
    #[must_use]
    pub const fn with_end(self, end: T) -> Self {
        Self {
            start: self.start,
            end,
        }
    }

    /// Returns the zero-width span at a certain position
    /// ```rust
    /// # use copyspan::Span;
    /// assert_eq!(Span::at(5), Span::from(5..5));
    /// assert_eq!(Span::at(0), Span::from(0..0));
    /// assert_eq!(Span::at(50), Span::from(50..50));
    /// ```
    #[must_use]
    pub const fn at(pos: T) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    #[must_use]
    pub fn contains(&self, elem: T) -> bool {
        self.range().contains(&elem)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        &self.start == &self.end
    }

    /// Checks if this `Span` overlaps with another `Span`.
    /// ```rust
    /// # use copyspan::Span;
    /// let foo = Span::from(0..3);
    /// let bar = Span::from(2..4);
    /// let biz = Span::from(3..6);
    ///
    /// assert!(!foo.overlaps_with(biz));
    /// assert!(foo.overlaps_with(bar));
    /// assert!(bar.overlaps_with(biz));
    ///
    /// assert!(foo.overlaps_with(Span::from(0..0)));
    /// ```
    #[must_use]
    pub fn overlaps_with(&self, other: Self) -> bool {
        self.contains(other.start) || other.contains(self.start)
    }

    #[must_use]
    pub const fn range(self) -> Range<T> {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use crate::util::RustNumber;
    use alloc::fmt::Debug;

    use super::Span;

    impl<T: RustNumber> Debug for Span<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            Debug::fmt(&self.range(), f)
        }
    }
}

impl<T: RustNumber> Hash for Span<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.range(), state)
    }
}

impl<T: RustNumber> PartialEq for Span<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range().eq(&other.range())
    }
}

impl<T: RustNumber> Eq for Span<T> {}

impl<T: RustNumber> From<Range<T>> for Span<T> {
    fn from(value: Range<T>) -> Self {
        Span {
            start: value.start,
            end: value.end,
        }
    }
}

impl<T: RustNumber> From<Span<T>> for Range<T> {
    fn from(value: Span<T>) -> Self {
        value.range()
    }
}

impl<U> Index<Span<usize>> for [U] {
    type Output = [U];

    fn index(&self, index: Span<usize>) -> &Self::Output {
        &self[index.range()]
    }
}

impl<U> IndexMut<Span<usize>> for [U] {
    fn index_mut(&mut self, index: Span<usize>) -> &mut Self::Output {
        &mut self[index.range()]
    }
}

impl Index<Span<usize>> for str {
    type Output = str;

    fn index(&self, index: Span<usize>) -> &Self::Output {
        &self[index.range()]
    }
}

impl IndexMut<Span<usize>> for str {
    fn index_mut(&mut self, index: Span<usize>) -> &mut Self::Output {
        &mut self[index.range()]
    }
}

impl<T: RustNumber> IntoIterator for Span<T>
where
    Range<T>: Iterator<Item = T>,
{
    type Item = T;

    type IntoIter = Range<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl<T: RustNumber> Default for Span<T> {
    fn default() -> Self {
        Self::from(T::default()..T::default())
    }
}
