use return_ok::some_or_return;

fn some_or_return_some() -> i32 {
    some_or_return!(Some(1))
}

fn some_or_return_none() -> i32 {
    some_or_return!(None);
    unreachable!();
}

fn main() {
    assert_eq!(some_or_return_some(), 1);
    assert_eq!(some_or_return_none(), 0);
}
