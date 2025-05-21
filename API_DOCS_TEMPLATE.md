<!-- Template for koto API doc markdown file
Directory which contains the koto API docs: .koto-api-docs
Directory location: from which LSP get started (usually identical project cargo workspace root)
-->

# my_module

Lorem ipsum...

T`my_module` contains the [`MyType`](#my_type) type, which are bindings to...

## my_function

```kototype
|| -> MyType
```

Lorem ipsum.

### Example

```koto
my_type = my_function()
```

## my_function_2

```kototype
|a: Number| -> Null
|a: String: b: Number| -> Null
```

Lorem ipsum...

### Example

```koto
my_function_2(1)
my_function_2("foo", 2)
```

## MyType

Lorem ipsum...

### Example

```koto
# TODO
```

## MyType.my_function

```kototype
|MyType| -> MyType
```

Lorem ipsum...

### Example

```koto
new_my_type = my_type.my_function()
```