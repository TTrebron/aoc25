use std::u64;

mod iters;

#[test]
fn test_get_half_point() {
    assert_eq!(iters::get_half_point(0), 0);
    assert_eq!(iters::get_half_point(1), 0);
    assert_eq!(iters::get_half_point(10), 1);
    assert_eq!(iters::get_half_point(100), 1);
    assert_eq!(iters::get_half_point(1000), 2);
}

#[test]
fn test_split_at_digits_count() {
    let (digits, _, _, _, _) = iters::split_at(0, 0);
    assert_eq!(digits, 1);

    let (digits, _, _, _, _) = iters::split_at(1, 0);
    assert_eq!(digits, 1);

    let (digits, _, _, _, _) = iters::split_at(5, 0);
    assert_eq!(digits, 1);

    let (digits, _, _, _, _) = iters::split_at(10, 0);
    assert_eq!(digits, 2);

    let (digits, _, _, _, _) = iters::split_at(99, 0);
    assert_eq!(digits, 2);

    let (digits, _, _, _, _) = iters::split_at(100, 0);
    assert_eq!(digits, 3);
}

#[test]
fn test_split_at() {
    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(0, 0);
    assert_eq!(digits, 1);
    assert_eq!(upper_half_digits, 0);
    assert_eq!(lower_half_digits, 1);
    assert_eq!(upper_half, 0);
    assert_eq!(lower_half, 0);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(0, 1);
    assert_eq!(digits, 1);
    assert_eq!(upper_half_digits, 1);
    assert_eq!(lower_half_digits, 0);
    assert_eq!(upper_half, 0);
    assert_eq!(lower_half, 0);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(1, 0);
    assert_eq!(digits, 1);
    assert_eq!(upper_half_digits, 0);
    assert_eq!(lower_half_digits, 1);
    assert_eq!(upper_half, 0);
    assert_eq!(lower_half, 1);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(1, 1);
    assert_eq!(digits, 1);
    assert_eq!(upper_half_digits, 1);
    assert_eq!(lower_half_digits, 0);
    assert_eq!(upper_half, 1);
    assert_eq!(lower_half, 0);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(1, 2);
    assert_eq!(digits, 1);
    assert_eq!(upper_half_digits, 1);
    assert_eq!(lower_half_digits, 0);
    assert_eq!(upper_half, 1);
    assert_eq!(lower_half, 0);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(1, 10);
    assert_eq!(digits, 1);
    assert_eq!(upper_half_digits, 1);
    assert_eq!(lower_half_digits, 0);
    assert_eq!(upper_half, 1);
    assert_eq!(lower_half, 0);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(23, 0);
    assert_eq!(digits, 2);
    assert_eq!(upper_half_digits, 0);
    assert_eq!(lower_half_digits, 2);
    assert_eq!(upper_half, 0);
    assert_eq!(lower_half, 23);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(23, 1);
    assert_eq!(digits, 2);
    assert_eq!(upper_half_digits, 1);
    assert_eq!(lower_half_digits, 1);
    assert_eq!(upper_half, 2);
    assert_eq!(lower_half, 3);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(23, 2);
    assert_eq!(digits, 2);
    assert_eq!(upper_half_digits, 2);
    assert_eq!(lower_half_digits, 0);
    assert_eq!(upper_half, 23);
    assert_eq!(lower_half, 0);

    let (digits, upper_half_digits, lower_half_digits, upper_half, lower_half) =
        iters::split_at(u64::MAX, 3);
    assert_eq!(digits, 20);
    assert_eq!(upper_half_digits, 3);
    assert_eq!(lower_half_digits, 17);
    assert_eq!(upper_half, 184);
    assert_eq!(lower_half, 46744073709551615);
}

#[test]
fn iterator_test_zero_digits() {
    let mut iter = iters::InvalidIdIterator::new(0, 0, None);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_invalid_min_max() {
    let mut iter = iters::InvalidIdIterator::new(22, 11, None);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_half_1() {
    let mut iter = iters::InvalidIdIterator::new(11, 22, None);
    assert_eq!(iter.next().unwrap(), 11);
    assert_eq!(iter.next().unwrap(), 22);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_half_2() {
    let mut iter = iters::InvalidIdIterator::new(11, 111, None);
    assert_eq!(iter.next().unwrap(), 11);
    assert_eq!(iter.next().unwrap(), 22);
    assert_eq!(iter.next().unwrap(), 33);
    assert_eq!(iter.next().unwrap(), 44);
    assert_eq!(iter.next().unwrap(), 55);
    assert_eq!(iter.next().unwrap(), 66);
    assert_eq!(iter.next().unwrap(), 77);
    assert_eq!(iter.next().unwrap(), 88);
    assert_eq!(iter.next().unwrap(), 99);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_half_3() {
    let mut iter = iters::InvalidIdIterator::new(199, 200, None);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_half_4() {
    let mut iter = iters::InvalidIdIterator::new(199, 1010, None);
    assert_eq!(iter.next().unwrap(), 1010);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_half_5() {
    let mut iter = iters::InvalidIdIterator::new(1920, 2019, None);
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_limit_1() {
    let mut iter = iters::InvalidIdIterator::new(0, 0, Some(1));
    assert!(iter.next().is_none());

    let mut iter = iters::InvalidIdIterator::new(1, 1, Some(1));
    assert!(iter.next().is_none());

    let mut iter = iters::InvalidIdIterator::new(1, 11, Some(1));
    assert_eq!(iter.next().unwrap(), 11);
    assert!(iter.next().is_none());

    let mut iter = iters::InvalidIdIterator::new(1, 1010, Some(1));
    assert_eq!(iter.next().unwrap(), 11);
    assert_eq!(iter.next().unwrap(), 22);
    assert_eq!(iter.next().unwrap(), 33);
    assert_eq!(iter.next().unwrap(), 44);
    assert_eq!(iter.next().unwrap(), 55);
    assert_eq!(iter.next().unwrap(), 66);
    assert_eq!(iter.next().unwrap(), 77);
    assert_eq!(iter.next().unwrap(), 88);
    assert_eq!(iter.next().unwrap(), 99);
    assert!(iter.next().is_none());

    let mut iter = iters::InvalidIdIterator::new(11111, 55555, Some(1));
    assert!(iter.next().is_none());
}

#[test]
fn iterator_test_invalid_limit() {
    let mut iter = iters::InvalidIdIterator::new(0, 0, Some(0));
    assert!(iter.next().is_none());

    let mut iter = iters::InvalidIdIterator::new(0, 0, Some(100));
    assert!(iter.next().is_none());

    let mut iter = iters::InvalidIdIterator::new(11, 22, Some(100));
    assert!(iter.next().is_none());
}
