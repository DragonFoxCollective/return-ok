#[macro_export]
macro_rules! ok_some {
    ($value:expr) => {
        match $value {
            Ok(Some(value)) => value,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err.into()),
        }
    };
}

#[macro_export]
macro_rules! some_ok {
    ($value:expr) => {
        match $value {
            Some(Ok(value)) => value,
            Some(Err(err)) => return Some(Err(err.into())),
            None => return None,
        }
    };
}

#[macro_export]
macro_rules! option_ok {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(err) => return Some(Err(err.into())),
        }
    };
}

#[macro_export]
macro_rules! some_or_return {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return Default::default(),
        }
    };
}

#[macro_export]
macro_rules! some_or_return_ok {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return Ok(Default::default()),
        }
    };
}

#[macro_export]
macro_rules! some_or_return_err {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return Err(Default::default()),
        }
    };
}

#[macro_export]
macro_rules! some_or_return_some {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return Some(Default::default()),
        }
    };
}

#[macro_export]
macro_rules! some_or_continue {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => continue,
        }
    };
}

#[macro_export]
macro_rules! ok_or_return {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(_) => return Default::default(),
        }
    };
}

#[macro_export]
macro_rules! ok_or_return_ok {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(_) => return Ok(Default::default()),
        }
    };
}

#[macro_export]
macro_rules! ok_or_return_err {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(_) => return Err(Default::default()),
        }
    };
}

#[macro_export]
macro_rules! ok_or_return_some {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(_) => return Some(Default::default()),
        }
    };
}

#[macro_export]
macro_rules! ok_or_continue {
    ($value:expr) => {
        match $value {
            Ok(value) => value,
            Err(_) => continue,
        }
    };
}
