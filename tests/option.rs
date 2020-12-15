use ternary_option::TernaryOption::*;

#[test]
fn bools() {
    assert!(Present(Some(1)).is_present());
    assert!(!Present(Some(1)).is_missing());
    assert!(Present::<u8>(None).is_present());
    assert!(!Present::<u8>(None).is_missing());
    assert!(Missing::<u8>.is_missing());
    assert!(!Missing::<u8>.is_present());

    assert!(Present(Some(1)).has_value());
    assert!(!Present::<u8>(None).has_value());
    assert!(!Missing::<u8>.has_value());
}

#[test]
fn contains() {
    assert!(Present(Some(1)).contains(&1));
    assert!(!Present(Some(1)).contains(&2));
    assert!(!Present::<u8>(None).contains(&1));
    assert!(!Missing::<u8>.contains(&1));
}

#[test]
fn map_value_with_val() {
    Present(Some(1)).map_value(|x| assert_eq!(1, x)).unwrap();
}

#[test]
#[should_panic]
fn map_value_with_missing() {
    Missing::<u8>.map_value(|_| assert_eq!(1, 2)).unwrap();
}

#[test]
fn unwrap_or_default() {
    assert_eq!(Present(Some(1)).unwrap_or_default(), Some(1));
    assert_eq!(Present::<u8>(None).unwrap_or_default(), None);
    assert_eq!(Missing::<u8>.unwrap_or_default(), None);
}

#[test]
fn unwrap_value_or_default() {
    assert_eq!(Present(Some(1)).unwrap_value_or_default(), 1);
    assert_eq!(Present::<u8>(None).unwrap_value_or_default(), 0);
    assert_eq!(Missing::<u8>.unwrap_value_or_default(), 0);
}
