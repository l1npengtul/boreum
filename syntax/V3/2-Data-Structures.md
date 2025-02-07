# 2. Data Structures

## 1. Figment

```
figment Moon {
    .brightness: Float64
}
```

Figments are value types that are treated as if a new "base" type. They do not have any special abilities, and serve
as a product type, similar to `struct` in Rust. 


## 2. Union

```
union Shape {
    .point |
    .line (Int64) |
    .square: Square |
}
```

Unions are a tagged enum type, similar to Rust `enum`s. They can contain additional data in the form of a tuple or figment
or construct.

## 3. Effect / Capability

Reserved.

## 4. Construct

```
construct Fish {
    .eyes: Int64
    .water: Water
    
    field 6: Body
    
    <>< Animal
    
    function swim() do
        ...
    end
}
```

Constructs represent a composable record type in Boreum. They are the primary data structure, and are indexed key-value.
Keys may be anything, but should be symbols. The `field` keyword allows for any Boreum item to be used as a field, 
such as type expression, number, operator, etc. 

Sakana expressions are allowed within constructs, this signals that a type must be composed with the construct during
construction. 

A function defined within a construct is said to be "strictly related". This means for any other type to be structurally
equivalent, it must also have a strictly related function by the same name. The content of the function is irrelevant, as 
long as the type of the function is equivalent. Note that multi-methods are considered individual methods.

As with other types, constructs are structural.

### 1. Compositional Types

Boreum's constructs feature compositional types - that is when another object is sakana'd into the table, it is added to
the list of remembered "composed" types, and can be freely subtyped (note: if you don't use rows this does lose information)

The construct consumes the composed type's properties but not its behavior without special syntax. 
```
fish.animal_stuff() # no
fish::<Animal>.animal_stuff() # yes
```

### 2. The KAIWAI and FARAWAY tables

There are 2 tables within a construct, the KAIWAI and the FARAWAY table.

The KAIWAI contains the actual data of the table, while the FARAWAY table contains the runtime
data for the data. 

FARAWAY space cannot be written to, except for the case of function mixins (.before, .after, .replace. around)

The FARAWAY space also serves as a place for the vtable. 

The FARAWAY space can be read by indexing into the table with the `far` keyword. 
```
table[far .turtle] # 
```

## 5. Protocols

Protocols are structural interfaces that define common behaviours (methods) and common properties (fields).
They can be implemented using `assoc`

## 6. Implement and Associate

`impl` blocks are similar to rust, define methods for a type.

`assoc` blocks let you implement protocols for a type.

