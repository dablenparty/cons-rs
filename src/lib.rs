#[macro_export]
macro_rules! cons {
    ($iter:ident as $hd:ident) => {
        let iter = $iter.into_iter();
        let $hd = iter.collect::<Vec<_>>();
    };
    ($iter:ident as $hd:ident$(::$tl:ident)+) => {
        let mut iter = $iter.into_iter();
        let $hd = iter.next().unwrap_or_else(|| {
            panic!("Iterator exhausted before reaching variable {}", stringify!($hd));
        });
        cons!(iter as $($tl)::+);
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
    #[should_panic(expected = "Iterator exhausted before reaching variable y")]
    #[allow(unused_variables)]
    fn test_iterator_too_short() {
        let v = [1];
        cons!(v as x::y::zs);
    }
}
