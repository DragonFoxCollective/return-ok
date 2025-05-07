use return_ok::option_ok;

fn option_ok_ok() -> Option<Result<i32, i32>> {
    let x = option_ok!(Ok::<_, i32>(1));
    Some(Ok(x))
}

fn option_ok_err() -> Option<Result<i32, i32>> {
    let x = option_ok!(Err(2));
    Some(Ok(x))
}

fn main() {
    assert_eq!(option_ok_ok(), Some(Ok(1)));
    assert_eq!(option_ok_err(), Some(Err(2)));
}
