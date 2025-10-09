# return-ok

Macros for dealing with Options and Results where you shouldn't

## Macros

- `ok_some`: `Result<Option<T>>` -> `T` or `return Result<Option<T>>`

```rs
match $value {
	Ok(Some(value)) => value,
	Ok(None) => return Ok(None),
	Err(err) => return Err(err.into()),
}
```

- `some_ok`: `Option<Result<T>>` -> `T` or `return Option<Result<T>>`

```rs
match $value {
	Some(Ok(value)) => value,
	Some(Err(err)) => return Some(Err(err.into())),
	None => return None,
}
```

- `option_ok`: `Result<Option<T>>` -> `Option<T>` or `return Result<Option<T>>`

```rs
match $value {
	Ok(value) => value,
	Err(err) => return Some(Err(err.into())),
}
```

- `some_or_return`: `Option<T>` -> `T` or `return default`

```rs
match $value {
	Some(value) => value,
	None => return Default::default(),
}
```

- `some_or_return_ok`: `Option<T>` -> `T` or `return Result<default>`

```rs
match $value {
	Some(value) => value,
	None => return Ok(Default::default()),
}
```

- `some_or_return_err`: `Option<T>` -> `T` or `return Result<default>`

```rs
match $value {
	Some(value) => value,
	None => return Err(Default::default()),
}
```

- `some_or_return_some`: `Option<T>` -> `T` or `return Option<default>`

```rs
match $value {
	Some(value) => value,
	None => return Some(Default::default()),
}
```

- `some_or_continue`: `Option<T>` -> `T` or `continue`

```rs
match $value {
	Some(value) => value,
	None => continue,
}
```

- `ok_or_return`: `Result<T>` -> `T` or `return default`

```rs
match $value {
	Ok(value) => value,
	Err(_) => return Default::default(),
}
```

- `ok_or_return_ok`: `Result<T>` -> `T` or `return Result<default>`

```rs
match $value {
	Ok(value) => value,
	Err(_) => return Ok(Default::default()),
}
```

- `ok_or_return_err`: `Result<T>` -> `T` or `return Result<default>`

```rs
match $value {
	Ok(value) => value,
	Err(_) => return Err(Default::default()),
}
```

- `ok_or_return_some`: `Result<T>` -> `T` or `return Option<default>`

```rs
match $value {
	Ok(value) => value,
	Err(_) => return Some(Default::default()),
}
```

- `ok_or_continue`: `Result<T>` -> `T` or `continue`

```rs
match $value {
	Ok(value) => value,
	Err(_) => continue,
}
```

