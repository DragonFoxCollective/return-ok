use return_ok::some_or_continue;

fn some_or_continue_some() {
    for _ in 0..1 {
        let x = some_or_continue!(Some(1));
        assert_eq!(x, 1);
    }
}

fn some_or_continue_none() {
    for _ in 0..1 {
        some_or_continue!(None);
        unreachable!();
    }
}

fn main() {
    some_or_continue_some();
    some_or_continue_none();
}
