use ternary_option::TernaryOption::*;

#[test]
fn map_present_with_val() {
    Present(Some(1)).map_present(|x| assert_eq!(1, x)).unwrap();
}

#[test]
#[should_panic]
fn map_present_with_missing() {
    Missing::<u8>.map_present(|_| assert_eq!(1, 2)).unwrap();
}
