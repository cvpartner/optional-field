use ternary_option::TernaryOption::*;

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
