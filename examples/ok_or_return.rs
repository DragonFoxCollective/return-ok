use return_ok::ok_or_return;

fn ok_or_return_some() -> i32 {
    ok_or_return!(Ok::<_, i32>(1))
}

fn ok_or_return_none() -> i32 {
    ok_or_return!(Err(2));
    unreachable!();
}

fn main() {
    assert_eq!(ok_or_return_some(), 1);
    assert_eq!(ok_or_return_none(), 0);
}
