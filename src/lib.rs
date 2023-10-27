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
/// cons!(v => x::xs);
/// assert_eq!(x, 1);
/// assert_eq!(xs, vec![2, 3]);
/// ```
///
/// The syntax is not exactly the same as SML's. The reason for this is
/// explained further in the _Limitations_ section of the [crate-level documentation](crate).
/// As a result of this, you can create/use iterators directly in the
/// macro invocation:
///
/// ```rust
/// # use cons::cons;
/// cons!(1..=3 => x::xs);
/// assert_eq!(x, 1);
/// assert_eq!(xs, vec![2, 3]);
/// ```
///
/// You can also exhaust (i.e. bind every element of) the iterator:
///
/// ```rust
/// # use cons::cons;
/// let v = [1, 2, 3, 4, 5];
/// cons!(v => a::b::c::d::e::nil);
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
/// cons!(v => x::y::zs);
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
/// It is important to note that the macro does _not_ support
/// destructuring. For example:
///
/// ```rust,compile_fail
/// # use cons::cons;
/// let v = [(1, 2), (3, 4)];
/// cons!(v => (x, y)::zs); // compile error
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
/// cons!(v => x::y::zs); // panics, iter not long enough
/// ```
///
/// If `nil` is at the end of the pattern and there are still elements
/// left in the iterator, the macro will panic with a message indicating
/// what elements were left:
///
/// ```rust,should_panic
/// # use cons::cons;
/// let v = [1, 2];
/// cons!(v => x::nil); // panics, iter still has [2]
/// ```
#[macro_export]
macro_rules! cons {
    // base case 1
    ($iter:expr => $hd:ident::nil) => {
        let mut iter = $iter.into_iter();
        let $hd = iter.next().unwrap_or_else(|| {
            panic!("Iterator exhausted before reaching variable {}", stringify!($hd));
        });
        {
            let rest = iter.collect::<Vec<_>>();
            assert!(rest.is_empty(), "Found `nil` in cons but iterator is not empty ({rest:?})\nConsider removing `::nil`");
        }
    };
    // base case 2
    ($iter:expr => $hd:ident) => {
        let iter = $iter.into_iter();
        let $hd = iter.collect::<Vec<_>>();
    };
    ($iter:expr => $hd:ident$(::$tl:ident)+) => {
        let mut iter = $iter.into_iter();
        let $hd = iter.next().unwrap_or_else(|| {
            panic!("Iterator exhausted before reaching variable {}", stringify!($hd));
        });
        $crate::cons!(iter => $($tl)::+);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_head() {
        let v = [1, 2, 3];
        let expected = (1, vec![2, 3]);
        cons!(v => x::xs);
        assert_eq!(x, expected.0);
        assert_eq!(xs, expected.1);
    }

    #[test]
    fn test_triple_cons() {
        let v = [1, 2, 3, 4, 5];
        let expected = (1, 2, vec![3, 4, 5]);
        cons!(v => x::y::zs);
        assert_eq!(x, expected.0);
        assert_eq!(y, expected.1);
        assert_eq!(zs, expected.2);
    }

    #[test]
    #[should_panic(expected = "Iterator exhausted before reaching variable y")]
    #[allow(unused_variables)]
    fn test_iterator_too_short() {
        let v = [1];
        cons!(v => x::y::zs);
    }

    #[test]
    #[should_panic(
        expected = "Found `nil` in cons but iterator is not empty ([2])\nConsider removing `::nil`"
    )]
    #[allow(unused_variables)]
    fn test_iterator_too_long() {
        let v = [1, 2];
        cons!(v => x::nil);
    }
}
