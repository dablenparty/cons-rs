/*!
This crate attempts to bring the "cons" feature from SML into Rust.
See the [`cons!`] macro for more information.
*/

/// Brings the "cons" feature from SML into Rust. I'm not the most
/// experienced with SML so this most likely will not be feature
/// complete. This is more of an experiment than anything else.
///
/// # Examples
///
/// This macro does not return anything; rather, it creates variables
/// with the names specified in the macro invocation and binds them
/// in the current scope. For example:
///
/// ```rust
/// # use cons::cons;
/// let v = [1, 2, 3];
/// cons!(v as x::xs);
/// assert_eq!(x, 1);
/// assert_eq!(xs, vec![2, 3]);
/// ```
///
/// You can even use iterators directly in the macro invocation;
/// however, in order to match the syntax to SML as closely as
/// possible, you must wrap the iterator in parentheses:
///
/// ```rust
/// # use cons::cons;
/// cons!((1..=3) as x::xs);
/// assert_eq!(x, 1);
/// assert_eq!(xs, vec![2, 3]);
/// ```
///
/// You can also exhaust (i.e. bind every element of) the iterator:
///
/// ```rust
/// # use cons::cons;
/// let v = [1, 2, 3, 4, 5];
/// cons!(v as a::b::c::d::e::nil);
/// // ...
/// # assert_eq!(a, 1);
/// # assert_eq!(b, 2);
/// # assert_eq!(c, 3);
/// # assert_eq!(d, 4);
/// assert_eq!(e, 5);
/// ```
///
/// Notice the `nil` at the end of the cons pattern. This special
/// case is used to indicate the end of the iterator and bind the
/// last element rather than a vector of the remaining elements.
/// See the _Panics_ section for how this can go wrong.
///
/// So far, the examples have only shown slices with elements that
/// implement `Copy`. This is not a requirement:
///
/// ```rust
/// # use cons::cons;
/// let v = vec![
///    String::from("hello"),
///    String::from("to"),
///    String::from("the"),
///    String::from("world"),
/// ];
/// cons!(v as x::y::zs);
/// assert_eq!(x, "hello");
/// assert_eq!(y, "to");
/// assert_eq!(zs, vec!["the", "world"]);
/// ```
///
/// In these cases, the elements are borrowed from the iterator
/// rather than copied. Notice that `v` is a vector of owned
/// `String`s, but `x` and `y` are `&str` slices and `zs` is
/// a `Vec<&str>`.
///
/// The macro also supports destructuring by wrapping the pattern
/// in parentheses:
///
/// ```rust
/// # use cons::cons;
/// let tuples = [(1, 2), (3, 4), (5, 6)];
/// cons!(tuples as ((x, y))::zs);
/// assert_eq!(x, 1);
/// assert_eq!(y, 2);
/// assert_eq!(zs, vec![(3, 4), (5, 6)]);
///
/// # #[derive(Debug, PartialEq, Eq)]
/// struct Point(i32, i32);
///
/// let points = [Point(1, 2), Point(3, 4), Point(5, 6)];
/// cons!(points as (Point(x, y))::zs);
/// assert_eq!(x, 1);
/// assert_eq!(y, 2);
/// assert_eq!(zs, vec![Point(3, 4), Point(5, 6)]);
/// ```
///
/// # Panics
///
/// If there are not enough elements in the iterator to match the
/// pattern, the macro will panic with a message indicating which
/// variable was not reached:
///
/// ```rust,should_panic
/// # use cons::cons;
/// let v = [1];
/// cons!(v as x::y::zs); // panics, iter not long enough
/// ```
///
/// If `nil` is at the end of the pattern and there are still elements
/// left in the iterator, the macro will panic with a message indicating
/// what elements were left:
///
/// ```rust,should_panic
/// # use cons::cons;
/// let v = [1, 2];
/// cons!(v as x::nil); // panics, iter still has an element
/// ```
#[macro_export]
macro_rules! cons {
    ($iter:ident as $($rest:tt)+) => {
        $crate::cons!(@__ $iter => $($rest)+);
    };
    (($iter:expr) as $($rest:tt)+) => {
        $crate::cons!(@__ $iter => $($rest)+);
    };
    (@__ $iter:expr => $hd:ident :: nil) => {
        $crate::cons!(@__ $iter => ($hd)::nil);
    };
    (@__ $iter:expr => ($hd:pat) :: nil) => {
        let mut iter = $iter.into_iter();
        let $hd = iter.next().unwrap_or_else(|| {
            panic!("Iterator exhausted before reaching variable {}", stringify!($hd));
        });
        {
            let rest = iter.count();
            assert_eq!(rest, 0, "Found `nil` in cons but iterator is not empty ({rest} elements left)\nConsider removing `::nil`");
        }
    };
    (@__ $iter:expr => $hd:ident) => {
        let iter = $iter.into_iter();
        let $hd = iter.collect::<Vec<_>>();
    };
    (@__ $iter:expr => $hd:ident :: $($rest:tt)+) => {
        $crate::cons!(@__ $iter => ($hd) :: $($rest)+);
    };
    (@__ $iter:expr => ($hd:pat) :: $($rest:tt)+) => {
        let mut iter = $iter.into_iter();
        let $hd = iter.next().unwrap_or_else(|| {
            panic!("Iterator exhausted before reaching variable {}", stringify!($hd));
        });
        $crate::cons!(@__ iter => $($rest)+);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_head() {
        let v = [1, 2, 3];
        let expected = (1, vec![2, 3]);
        cons!(v as x::xs);
        assert_eq!(x, expected.0);
        assert_eq!(xs, expected.1);
    }

    #[test]
    fn test_triple_cons() {
        let v = [1, 2, 3, 4, 5];
        let expected = (1, 2, vec![3, 4, 5]);
        cons!(v as x::y::zs);
        assert_eq!(x, expected.0);
        assert_eq!(y, expected.1);
        assert_eq!(zs, expected.2);
    }

    #[test]
    fn test_destructure_in_middle() {
        let v = [(1, 2), (3, 4), (5, 6)];
        cons!(v as w::((x, y))::zs);
        assert_eq!(w, (1, 2));
        assert_eq!(x, 3);
        assert_eq!(y, 4);
        assert_eq!(zs, vec![(5, 6)]);
    }

    #[test]
    #[should_panic(expected = "Iterator exhausted before reaching variable y")]
    #[allow(unused_variables)]
    fn test_iterator_too_short() {
        let v = [1];
        cons!(v as x::y::zs);
    }

    #[test]
    #[should_panic(
        expected = "Found `nil` in cons but iterator is not empty (1 elements left)\nConsider removing `::nil`"
    )]
    #[allow(unused_variables)]
    fn test_iterator_too_long() {
        let v = [1, 2];
        cons!(v as x::nil);
    }
}
