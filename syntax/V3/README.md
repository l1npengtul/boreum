# The Boreum Programming Language, Design Doc V3

Boreum is a language that is a cursed marriage of Lua, Rust, and Elixir. It is designed to
target WASM/WASI(wasmtime specifically), and to be easily interoperable with other WASM languages and Rust.

Boreum is a static, inferred, row polymorphically typed language. 

Boreum is funktional, prototype-composing, and imperative.

Boreum comes from a shortening of the Korean word `보름달`(boreumdal), which means "full moon".

File Extensions:
- `.br` (if you're boring)
- `.brm` (still boring~~ ¯\\_(ツ)_/¯)
- `.boreum` (if you like reading loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong things)
- `.보름` (if you have a korean IME installed)
- `.🌕` (if you're obnoxious)
- `.月魚` (if you have a kanji/hanzi/hanja IME installed (chinese character readers will tell you lies about "how it's not even close to the original meaning" they are lying to you shhhhhhhh))
- `.<><` (is this even valid?)
- `.><>` (im just wasting your time at this point)

i kinda hate RR korean... consider yale or MCR korean
- Porŭm
- Polum

## example boreum syntax

```
# comments are done using #
## two of these indicate a documentation comment
##! this is a package level comment

import std::bar

# funktions are defined using the `funk` keyword.
funk parallel_map(collection: I<A>, action: funk(A) => B) => I<B> where 
  I: Iterable
do
  # Structured Concurrency - In Boreum, the only way to do concurrency
  # is via the nursery block. There is no other way to do concurrency,
  # nor is there a way to escape the nursury. This ensures that
  # A) The lifetime of every fibre can be reasoned by the compiler.
  # B) Ensures that every fibre is cleaned and properly dealt with.
  # C) Ensures a single "hierarchy" and lineage tree for every fibre that is spawned.
  # The runtime deals with scheduling - the spawning of a fibre is cheap as it is a 
  # userland construct.
  nursery 
    bind halo = NURSERY::FAIR do # Optional binding of nursery to a identifier. If nothing else, 
                                      # we use the default identifier "foo"
    collection.for_each()
    >> nursery.start_soon(action(foo)) # A >> is similar to |> in elixir
    >> collect # () is unneeded in a >> call
  end # Nursery blocks act as expressions
end

# Row Polymorphic Referece Record Type
construct Fish {
  # Optional default parameters with = 
  .name: String = ""
  # Composition Operator: Require composition of another construct at init
  # Composition is cascading - newer values shadow older ones
  ><> Water
}

protocol LivingBeing {
  # associated field
  .species: String = ""
  type T
  # the special self pointer
  funk say(self) -> String
}

# Row Polymorphic Referece Algebraic Sum Type
enum Area {
  | Sea
  | Land
  | Air
  | Other(Integer)
}

# Nominal Value Product Type
figment Point {
  .x: Float
  .y: Float
}

# Nominal Value Sum Type (similar to Rust enums)
union ShapeKind {
  | Circle(f64)
  | Square(f64)
}

# Example of a row-polymorphic funktion.
# Row types are polymorphic by default. You can constrict them by using !
funk print_name(item: {.x: String}) -> {.x: String} do
  println(item.x)
  item
end

# this has a constricted row type
funk constrcting(item: !{.x: String}) -> !{.x: String} do
  ...
end

# Span functions
# They have a special way of being called :D
span funk bold(weight: Float = 0.0, span: Span) -> Span {}

funk main() do
  # Boreum supports ultimate conditional syntax
  when x is
    ShapeKind::Circle(y) then
    end
    ShapeKind::Square(s) then
    end
  end
  
  when some_number
    == 0 then "value",
    >> square
      > 100 then "value2",
      < 50 then "value3",
      else "value4",
  end
  
  let bar = 0;
  do 
    # Shadowing is allowed, but must be explicitly done.
    shadow let bar = 1;
  end
end

```

## Things to figure out
- Memory Management
  - Currently, I'm thinking of implementing either Capture Separation Calculus or tempered domination to prevent data races
  - Otherwise, this language will be GC by default, with regions that allow people to use a different allocation strategy for e.g. video games. 
- Rust/WASI Interop
  - Currently, planning to implement the `extern` keyword, which will define an external interface (.WIT) to allow for dynamic linking and the such.
  - I also plan to support directly calling rust functions (since those crates will also be compiled to WASM) with a simple attribute macro.
- Compile Time Reflection
  - Allow for comptime funktions to read into the far table.


## academic brainrot
- [Universal Conditional Syntax](https://dl.acm.org/doi/10.1145/3689746)
- https://arxiv.org/pdf/2308.07474v2 static and effect capability based (Capture Separation
  Calculus) also read https://arxiv.org/pdf/2306.06496 (capture calculus) and this talk https://2024.splashcon.org/details/iwaco-2024-papers/5/Modular-Borrowing-Without-Ownership-or-Linear-Types
    - making box inference better for capture calculus https://arxiv.org/pdf/2306.06496
- https://dl.acm.org/doi/pdf/10.1145/3219753.3219757 kappa (also capability based)
- dala
- https://dl.acm.org/doi/pdf/10.1145/3519939.3523443 kinda like dala but type based and also needs a runtime check sometimes, tempered domination (similar to vale lang) https://news.ycombinator.com/item?id=35917717
- structural typing but uncringed (decidable): https://dl.acm.org/doi/pdf/10.1145/3632932
