use return_ok::ok_or_continue;

fn ok_or_continue_some() {
    for _ in 0..1 {
        let x = ok_or_continue!(Ok::<_, i32>(1));
        assert_eq!(x, 1);
    }
}

fn ok_or_continue_none() {
    for _ in 0..1 {
        ok_or_continue!(Err(2));
        unreachable!();
    }
}

fn main() {
    ok_or_continue_some();
    ok_or_continue_none();
}
