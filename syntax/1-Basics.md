# Basics

## 1.1 Primitive Types

These are the basic primitive types in Boreum:

- Nil
- I32
- I64
- F32
- F64
- Decimal64
- Boolean
- Interned String (Symbol)
- String
- Ref
- FnRef

### 1.1.1 Nil

```
Nil ::= 'nil'
```

Nil represents the absense of a value. In an operation, it is automatically considered
a NO-OP.

### 1.1.2 Numbers
```
Digit   ::= [0-9]
Sign    ::= [+-]
Delim   ::= Digit ( Digit | '_' )*
Exp     ::= 'e' Sign? Delim+

Integer ::= Sign? Delim Exp?

Numeral ::= ( Delim+ |
              Delim+ '.' Delim* |
              Delim* '.' Delim+ ) Exp?

Float   ::= Sign? ( 'inf' | 'infinity' | 'nan' | Numeral )
``` 

[comment]: # (Blatently stolen from Rust std docs lole)
I32 and I64 are 32-bit and 64-bit signed integers respectively.

F32 and F64 are 32-bit and 64-bit IEEE-754 compliant floating point numbers respectively.

Decimal64 is a fixed point integer which has
- 1 bit for signing,
- 32 bits for integer,
- and 31 bits of fraction.

Why separate I32/I64/F32/F64 values? As a WASM targeting language, this fits exposes the core of WASM types better. 

### 1.1.3 Booleans
```
Boolean ::= ('true' | 'false')
```
Booleans consist of either `true` or `false` values.

### 1.1.4 Strings
```
Literal ::= <any valid UTF-8>
Symbol         ::= ':' ( Identifier | '"' Literal '"' )
Format         ::= '${' Expression+ '}'
String         ::= '"' (  Format | Literal )* '"'
```
`Literal`s are interned strings. They can also be created using symbols, and are similar to atoms in Elixir. 
Any string that does not contain `Format` expressions are interned.

`Strings` are strings that can have formatting values inside of them. Their partials (aka the parts in between the formatting)
are also automatically interned into a special place separate from the regular Interned Strings. This also includes String
Literal Constants.

Both `String` values are strictly UTF-8, similar to Rust strings, and track their own length.

### 1.1.5 Functions

```
Visibility ::= 'private'
Constant   ::= 'const'
Mesmer     ::= 'mesmerizing'
Argument   ::= Identifier ('requires' TypeExpression)?
Arguments  ::= '(' Argument (',' Argument)* ')'
Return     ::= ':' (TypeIdentifier | TypeExpression)
Shorthand  ::= '=>' Expression
Body       ::= 'do' '\n' Expression* '\n' 'end'

Function   ::= Visibility? Constant? Mesmer? 'fn' Identifier Arguments Return? (Shorthand | Body)
Closure    ::= 'fn' Arguments (Shorthand | Body)
```

Functions are references to Boreum functions. They are subdivided by arity, return type, etc. More will be discussed later.

They are also secretly tables, containing a function pointer `FnRef`.

Mixins can be done by composing a new function into a function field, or using the before and after fields.

### 1.1.6 Ref and FnRef

Ref are references to external data. These are provided by the embedder.

FnRef are references to external functions. Again, these are provided by the embedder.

The Global Store is an example of a `Ref`.

### 1.1.7 Fibres

Fibres are stackful coroutines with flexible scheduling depending on the type of nursury used to spawn them. The default nursury will spawn
fibres in a M:N mode. They exist only within the runtime.

### 1.1.8 Tables and Aliases
```
Key   ::= Expression
Value ::= Expression
Delim ::= ','

Table ::= '%' ( TypeIdentifier | '<' TypeExpression '>' )? '{' 
            Key '=' Value 
            (Delim Key '=' Value)*
            Delim?
          '}'
```

Tables are the base "object" of Boreum. Tables have a few properties:

- Any value can be used as a `key` (however, use of symbol/interned string is preferred)
- Tables have 2 "subtables"
    - User "KAIWAI" Table - accessible from normal user space, normal fields and values.
    - Compiler/Shadow "FARAWAY" Table - private to the language runtime, can be accessed using the `special` keyword.
      Contains information about compositions, callables, metadata, etc. Think Lua's metatable, but hidden behind a keyword
      instead of dunders.

Tables cannot inherit, instead they are composed using the compose operator (also called "sakana" operator.)

This table construction syntax is also used to create `Construct`s and `Figment`s

#### 1.1.8.1 Aliases

Aliases are special values of a table that tell Boreum to invisibly link another part of a table (or a whole different table entirely)
to the current key. This is used for:
- Preventing copying of a whole table when immutability is invoked, instead aliasing all the other unchanged fields.
- Aliasing to/from "FARAWAY" and "KAIWAI" space.

## 1.2 The Global Store

```
Global = '@' Identifier 
```

The `@` symbol prefixed to an `Identifier` designates a field in the Global Store. 

The global store is provided by the embedder.

The global store
- Does not block readers OR writers
  - Similar to KV stores such as LMDB. (Implementers Note: Recommended to use a LMDB in memory only like store to
    manage the global store.)
- Is a simple Key-Value store, kept in memory, cleared when the program exits. 
- Keys are strings.
- Does NOT feature a "FARAWAY" table.
  - Can NOT be composed into directly.
- **All** values reachable from the global store live for the lifetime of the program. Adding values to the global store 
  dynamically is considered an antipattern (and you'll OOM eventually)


## 1.3 Exceptions, Option, and Result

### 1.3.1 Option and Result

Option and Result are both returned from maps using `Variant`s, mentioned later.
They can be used in a `when` expression for pattern matching. Otherwise, they are used
as they would be in Rust, and is the ideal way of handling errors/missing values. (No unwinding!)

### 1.3.2 Exceptions

Sometimes, there is an error so catastrophic that there is simply no way to continue execution. This is when
exceptions are used. Exceptions can be `raise`d, and they can be caught with `catch`. An exception unhandled will cause the 
current `Fibre` its running on to crash (although this isn't necessarily a bad thing.)

## 1.4 Garbage Collection

GC is handled by the WASM GC Types proposal and are thus handled by `wasmtime`.

## 1.5 Compositional Prototypes and Boreum Tables

`Table`s and their compositions form the basis of the Data Management Model of 
Boreum. 

### 1.5.1 User "KAIWAI" Tables and Compiler/Shadow "FARAWAY" Tables

```
IndexingExpression   ::= 'special'? Expression
TableIndexExpression ::= Identifier '[' IndexingExpression ']'  
```

"KAIWAI" space is the normally accessible space of the `Table`. 
This contains all the user defined fields and methods. Most of the time, users
will spend their time indexing from "KAIWAI" space.

"FARAWAY" space is for the metadata about fields and compositions that the runtime needs
to function. (e.g. Mode, Type Aliases, etc)
Unlike other language that assign a special prefix (e.g. Lua/Python's dunders), Boreum takes 
a page from C++'s syntax by introducing a special `special` keyword that allows a 
user to access the "FARAWAY" space _without_ affecting the normal "KAIWAI" space. This allows for more complex
keys (e.g. TypeIds, Ints, Other Tables) to be inserted into "KAIWAI" space. 

"FARAWAY" space is **Read Only.** Only special runtime functions are blessed by the compiler 
to operate directly on "FARAWAY" space. (see: Reflection API).

### 1.5.2 Composition

```
ForwardSakana         ::= '><' Expression? '>'
ReverseSakana         ::= '<' Expression? '><'
Sakana                ::= (Forward Sakana | ReverseSakana)
Namespace             ::= Expression
Target                ::= Expression
ComposedItem          ::= Expression

CompositionExpression ::= Sakana Target ( ComposedItems )+
```

Composition sits in Boreum as the replacement for Inheritance.
In an inheritance based system, there is a tendancy for objects to recurse 
ad-infinitum. Composition prevents this from happening.

Instead of thinking "This apartment is a type of(inherits from) house", 
we think "This apartment has (insert things that belong in an apartment here e.g. toilet, sink, giant life-size cutout of 
hatsune miku and kasane teto engaged in vocaloid yuri, etc.) with common behavior(protocol) House"

When composition occurs, the fields of the items being composed (lets call them the "composee" from now)
is aliased into the namespace (if there was no namespace provided, it composes into the base table).
If the field already exists, it's cast into its own "type" namespace in the "FARAWAY" (accessible using `special`) then
aliased back to the "KAIWAI" table if the namespace is expressible as a simple `Symbol`, or under the namespace provided
by the `Sakana` operator. Remember that in reality, namespaces create their own "sub-table" so they have their own "FARAWAY"
and "KAIWAI" table.
The type composed is also added to the "FARAWAY" composed types table. There can be multiple of the same type
composed into one table.

#### 1.5.2.1 Forward and Reverse Sakana

The `<><` and `><>` operators, henceforce referred to as the "forward sakana"(former) and "reverse sakana"(latter) (or 
just "forward" and "backward" for simplicity) are the operators used to compose in Boreum. They both compose tables, 
however differ in their internal behavior when a collision occurs. 

When a collision occurs when using "forward sakana" operator, keys are treated with **first come** priority - keys that
already exist are left alone, and new keys are hidden away.

The reverse happens with the "reverse sakana" operator, keys are overwritten with the latest 

#### 1.5.2.2 Sakana is also (secretly) a function

It is possible to use Sakana in a map expression, as the Sakana desugars into a compiler macro. This allows for 
simple chaining of Sakana operations.

#### 1.5.3 A Note on Methods

Methods are equivalent to a key with a Function value in the "FARAWAY" table. Note that to be a _proper, callable_ 
method, (a.k.a. accessible via the `table.function(args)` syntax), the key **must** be a simple string. 

### 1.5.3 Constructs

```
NormlField ::= (Symbol | Identifier) ('requires' TypeExpression)? 
CExprField ::= 'field' ConstExpression? 'requires' TypeExpression 
FishyField ::= Sakana TypeExpression? 
DefaultVal ::= (`=` Expression)?
Field      ::= (NormlField | CExprField | FishyField) DefaultVal?
Construct  ::= 'construct' TypeIdentifier 'do'
                    Field*
               'end'
```

A Construct is similar to `defrecord` in Clojure or `struct` in Elixir. It lets you create a type and creates a 
constructable map. This helps reduce user error, and is registered with the runtime as a valid type, able to be used 
in `TypeExpressions`. Note that `construct` can not be self-referential - whatever self-referential logic you have 
is probably better split out into multiple `construct`s and then composed into a larger `construct`.

You may compose in `construct` definitions, if there is no namespace the construct is merged into the root namespace
of the Table and absorbs all fields, protocol implementations, and methods of the constructed type. It is also added
to the "FARAWAY" table as a composed type.

### 1.5.4 Methods

```
Implement ::= 'impl' TypeIdentifier ('for' TypeIdentifier)? 'do'
                Field*
                Function*
              'end'
```

Associated Functions. All functions are assumed to be static by default, you can create a proper method by using 
the `self` parameter as the first argument of the function.

There is another keyword `Self`, that refers the to the `TypeIdentifier`.

Note that `impl` will overwrite composed from the `Construct` definition, but you cannot create multiple colliding functions
(same arity, same types. See Multiple Dispatch.).

Methods are also internally tracked from the "FARAWAY" table - it is possible to have the field "turtle" ("KAIWAI") _and_ the method
"turtle" ("FARAWAY"), and also to alias "turtle" ("KAIWAI") to "turtle" ("FARAWAY")

### 1.5.5 Variant

```
Tag   ::= (Symbol | Identifier) ('(' Identifier ('requires' TypeExpression)? ('=' Expression)? ')')?
Variant ::= 'variant' TypeIdentifier 'do'
            Tag ('|' Tag)?
          'end'
```

A Tagged Enum, also internally a table. However, you cannot compose into this one (will tell you to fuck off) and 
you cannot add any more fields (will tell you to fuck off).

### 1.5.6 Protocols

```
Marker   ::= 'marker'
Protocol ::= Marker? 'protocol' TypeIdentifier ('requires' TypeExpression)? 'do'
                Field*
                Function*
             'end'
```

Protocols allow for shared behaviour between tables, also called Polymorphism. Functions may have a default implementation
or not, and must be associative (you can not have a static protocol method, outside of special protocol methods blessed
by the compiler.) 

Use of marker enforces a zero-size protocol (one with no fields that only takes up space in "FARAWAY") as a tag,
this is used like "Send" or "Sync" traits in Rust.

### 1.5.7 Figments

```
Figment ::= Marker? 'figment' TypeIdentifier 'do'
                (
                NormlField DefaultVal?
                (',' NormlField DefaultVal?)?
                )?
                
            'end'
```

Figments are not tables. Instead, `figment` allows you to define a struct-like rigid type. In bytecode, this 
leads to Figments being directly translated into `struct` definitions. Using simple primitive types will get translated
into their direct equivalents.

Fields of a figment are by default private, they cannot be indexed into and you must write getters and setters as they
are not tables. 

Figments are stack-allocated when possible.

Marker allows you to define a zero-size Figment.

### 1.5.8 Unions

Unions are the value-type version of `Variant`. This type exists to map onto the WASM data model.

## 1.6 Structured Concurrency Model

```
Nursury   ::= 'nursury'
ConcBlock ::= Nursury Expression 'nurses'
                Expression+
              'end'
              CatchBlock?
              CancelBlock?
              ExitBlock?
```

Structured Concurrency allows the user to make reasonable assumptions of coroutines. Before, a coroutine would be a black
box. Does it outlive the current function? What about error handling? With structured concurrency, all of this is made 
explicit. 

The language and runtime enforce the rule that no concurrency may be done outside of a nursury.
When calling a function containing yields outside of a nursury, it will simply block until completion.

### 1.6.1 Mesmerizing Functions

There is function coloring in Boreum, and that is with `mesmerizer`.
This allows a function task to outlive the function creating the nursury, spawn new tasks, etc.
This is the only type of function allowed to be used inside a nursury that has an argument that takes a nursury.

### 1.6.2 The Wired

Elixir style send-receive. 
