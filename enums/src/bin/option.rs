fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let _absent_char: Option<i32> = None;

    assert_eq!(some_number.is_some(), true);
    assert_eq!(some_char.is_some(), true);
}