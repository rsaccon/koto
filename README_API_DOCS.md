The Koto API docs system is designed to provide a standardized way to document Koto libs which are written in Rust. The docs are written in markdown and therefore easy human readable and editable. To make the docs also accessible to tools such as the Koto CLI and Koto Language Server, some conventions need to be followed (see the template/example below) and two optional configuration files allow to customize to specific situations. NOT IMPLEMENTED YET: a tool which downloads and/or extracts the docs from the original lib and makes the docs avaibale locally to other tools.

The central point is the directory which contains the markdown API docs, which can be (but don't have to be) structured hierarchally reflecting the nested module structure. The directory also might contain at root level a `DEFAULT_IMPORTS` and/or a `IGNORE_CORE_LIBS` configuration file.

- Directrory name: **`.koto-api-docs`**
- Directory location: where LSP gets started (usually identical to project cargo workspace root)

**DEFAULT_IMPORTS**
All entries in this file represent functions or values which are imported via Rust at Koto VM start and made available at the toplevel scope. Wildcard `*` is supported to represent all functions or values of a module.
```
# .koto-api-docs/DEFAULT_IMPORTS Example:
my_module_a.*
my_module_b.my_function
my_module_b.my_value
```

**IGNORE_CORE_LIBS**
In case some core libs are removed from the prelude default import, they can be listed here. Supported entries are fully qualified module, functons or value names.
```
# .koto-api-docs/IGNORE_CORE_LIBS Example:
io.print
test
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
