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
