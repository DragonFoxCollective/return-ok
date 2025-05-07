use return_ok::ok_some;

fn ok_some_ok_some() -> Result<Option<i32>, i32> {
    let x = ok_some!(Ok::<_, i32>(Some(1)));
    Ok(Some(x))
}

fn ok_some_ok_none() -> Result<Option<i32>, i32> {
    let x = ok_some!(Ok::<_, i32>(None));
    Ok(Some(x))
}

fn ok_some_err() -> Result<Option<i32>, i32> {
    let x = ok_some!(Err(3));
    Ok(Some(x))
}

fn main() {
    assert_eq!(ok_some_ok_some(), Ok(Some(1)));
    assert_eq!(ok_some_ok_none(), Ok(None));
    assert_eq!(ok_some_err(), Err(3));
}
