# 1 - Basics

## 1. Basic Types / Primitives

In Boreum, there are the following primitive types:
- Boolean
- Number
  - Int64
  - Float64
  - Decimal31s32f
- String
- Symbol
- Vector
- Tuple
- Construct
- Fibre

Booleans are true/false values.

Numbers are numbers. They are 64 bit by default. There exist 3 subtypes for numbers: Int64, Float64, and Decimal31s32f.  
Int64 are signed 64-bit integers. Float64 are IEEE-754 compliant 64-bit floating point numbers. Other languages call these "doubles".
Decimal31s32f is a fixed point integer with 1-bit of sign, 31-bits of scalar, and 32-bits of fraction. 

Strings are strings, they must contain valid UTF-8. All other encoding sets are considered HERESY and SIN, and must be burnt
ON SIGHT on a STAKE certified to drive out demons back to their respective circle of hell.

Symbols are first class atomic/interned strings, whose value is its own name. Elixir calls these atoms.

Vectors are a homogenous, contiguously laid out section of memory. Values must be of the same type (thus homogenous), and 
can be of any size(platform-dependant).

Tuples are a heterogeneous type. They can contain any number of any value, with any value of any type. An empty tuple `()`
is also called a unit type.

Constructs are a key-value based heterogeneous type. They are similar to record types, and can be indexed into. More on them later.

There also exist basic types intended for further WASM/Rust Interop
- Limit
- u8, i8, u16, i16, u32, i32, u64, i64 usize, isize, ptr
- FuncRef
- ExternRef
- Result (note: this should be turned into Boreum Table Enums)
- GlobalConst
- GlobalMut
- Slice

## 2. Error Handling

There are 2 types of Errors in Boreum:
- Panic (Exceptions)
- Error (Recoverable Errors, usually via `when`)

While panics can be caught and continued from, it is best practice to use `Result` and `Error` whenever possible.

## 3. Bracing and Keyword Philopshy

Boreum uses both `{}` and `do .. end` \- curly braces are used when dealing with data, and `do .. end` when dealing with code. 

Keywords are their long, uncompressed name - e.g. `function` `isolated` `immutable`.

## 4. Casing Guide

just follow rust's naming conventions lmfao
