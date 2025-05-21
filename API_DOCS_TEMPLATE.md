The Koto API documentation system and conventions allow tools such as Language Server to access markdown docs for Koto modules, functions and values whoch are made available vie Rust API.

The central point is the directory which contains the markdown API docs, which can be (but don't have to be) structured hierarchally reflecting the nested module structure. The directory also might contain at root level a `DEFAULT_IMPORTS` and/or a `IGNORE_CORE_LIBS` files.

Directrory name: .koto-api-docs
Directory location: where LSP gets started (usually identical to project cargo workspace root)

`DEFAULT_IMPORTS` 
All entries in this file represent modules, functions or values which are imported via Rust at Koto VM start and made available at the toplevel scope.
```
# Example:
my_module_a
my_module_b.my_function
my_module_b.my_value
```

`IGNORE_CORE_LIBS` 
In case some core libs are removed from the prelude default import, they can be listed here.
```
# Example
io.print # the other io functions are still availble for API docs
test     # all module unavailable for API docs
```

Template / Example for Koto API doc markdown file:

# my_module

Lorem ipsum...

`my_module` contains the [`MyType`](#my_type) type, which are bindings to...

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