use return_ok::some_ok;

fn some_ok_some_ok() -> Option<Result<i32, i32>> {
    let x = some_ok!(Some(Ok::<_, i32>(1)));
    Some(Ok(x))
}

fn some_ok_some_err() -> Option<Result<i32, i32>> {
    let x = some_ok!(Some(Err(2)));
    Some(Ok(x))
}

fn some_ok_none() -> Option<Result<i32, i32>> {
    let x = some_ok!(None::<Result<i32, i32>>);
    Some(Ok(x))
}

fn main() {
    assert_eq!(some_ok_some_ok(), Some(Ok(1)));
    assert_eq!(some_ok_some_err(), Some(Err(2)));
    assert_eq!(some_ok_none(), None);
}
