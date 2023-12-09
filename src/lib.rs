/*
const fn sample() -> &'static str {
    include_str!(concat!(
        concat!("../samples/", env!("CARGO_BIN_NAME")),
        ".txt"
    ))
}
*/
#[macro_export]
macro_rules! sample {
    () => {
        include_str!(concat!(
            concat!("../samples/", env!("CARGO_BIN_NAME")),
            ".txt"
        ))
    };
}
#[macro_export]
macro_rules! input {
    () => {
        include_str!(concat!(
            concat!("../inputs/", env!("CARGO_BIN_NAME")),
            ".txt"
        ))
    };
}

pub fn space_indent(recursion_level: u8, max: u8) {
    (0..(max - recursion_level)).for_each(|_| print! {" "});
}

pub fn ints<T>(s: &str) -> impl Iterator<Item = T> + Clone + '_
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split(|c: char| !(c.is_ascii_digit() || c == '-'))
        .filter(|x| !x.is_empty())
        .filter(|x| *x != "-")
        .map(|x| x.parse::<T>().expect("an int"))
}
#[test]
fn ints_test() {
    assert_eq!(
        ints("Hello 1: 42,3874 384|81  1").collect::<Vec<u16>>(),
        vec![1, 42, 3874, 384, 81, 1],
    );
    assert_eq!(
        ints("Hello 1: 42 -3874 - 384|81  -1").collect::<Vec<i16>>(),
        vec![1, 42, -3874, 384, 81, -1],
    );
}
