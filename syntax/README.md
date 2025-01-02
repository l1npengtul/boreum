# Boreum Language Syntax

Boreum (보름) is a prototype based language 
inspired by Lua, Elixir, Clojure, and Rust.

It can be described as a prototype composing, functional, imperative, duck typed language.
It is designed to run on the `wasmtime` WASM virtual machine. 


This manual/syntax spec was inspired by the structure of the Lua 5.4 manual.

This manual/syntax assumes familiarity with programming as a prerequisite.

## Terms 

- Embedder - The Runtime. Usually will be the standard wasmtime + Boreum Runtime. May include extras
  based on what extra features are required by the user.

## Keywords 

These keywords are **reserved**. 

### Definitions

- let
- fn
- construct
- figment
- union
- mod
- private 

### Expression Keywords

- and 
- or
- not
- xor
- in
- as
- when
- requires
- special

### Block Keywords

- do
- end
- catch
- resque
- raise
- after
- else
- return

### Exports and Imports

- extern
- import
- export

### Symbols

- `><>`: Composition
- `/`: Namespace Separator
- `@`: Global Store Access
- `~`: Destructure
- `=>`: Arrow/Wrapping Function Shorthand
- `>>`: Block/Map
- `*`: Annotation
- `#`: Comment
- `'`: Label

## Typing

Boreum is a dynamically strong typed language. Boreum features optional 
static types, made of type composition expressions. When the compiler is able
to reason about the types at compile time, static types are also checked as well. (Pure functions only)

For static types, Hindley Milner type inference is used.

Generic type parameters are allowed, however are monomorphised at compile time (erased).
They are indicated with `<...>`

Type expressions allow the user to constrict what types are allowed to be used. It can be used with the `requires` keyword.
e.g. `x requires and(or(Type1 Type2 Type3) not(Type4 Type5) +++Type6)` 
becomes "X must have(composed within) either Type 1, Type2, or Type3, cannot have composed within Type4 or Type5, and must have composed within at least 3 Type6"

We call this concept "Composed Typing".

### Primitives

Primitives are stack allocated structs that cannot be extended
at runtime. Boreum features a few:

- Int
- Float
- Int64
- Double
- Array
- String
- Symbol
- Char
- FnPtr
- Any other combination of these, formed by using the `figment` 
  keyword.

### Tables

Tables are the base "object" of Boreum. Tables have a few properties:

- Any value can be used as a `key` (however, use of symbol/interned string is preferred)
- Tables have 2 "subtables"
  - User Table - accessible from normal user space
  - Shadow Compiler Table - private to the language runtime, can be accessed using the `special` keyword.
    Contains information about compositions, callables, metadata, etc.

Tables cannot inherit, instead they are composed using the compose operator (`><>` also called "sakana" operator.)

### Functions and Closures

Functions are first class in Boreum. Unlike in Rust, functions are 
considiered the same as long as they have the same signature and type requirements.

The compiler will also automatically tag functions as "pure" if it can prove that it does 
not cause any side effects (it only calls other pure functions). Note that this does not color
a function - it only causes additional optimizations and inference to be availible to the compiler.
To force a function as pure, you can use the `Pure` annotation.

Functions are defined with this grammar:
```
<annotation>:?
<visibility>:? fn name:? <(let <type_name> = <type_expr>)+>:? ( (<arg_name> <requires <type_expr>?>,)+ )<: <type_expr>>? ( => expr | do (expr)+ end )
```

e.g. 
```elixir
private fn turtle<let T = +++Animal>(self requires Table, a requires T, b requires T): T do
    <stuff>
end
```

Closures are similar, just omit the function name and return type expression. 

### 
