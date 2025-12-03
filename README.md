# Generalizer

*Generalizer* provides an implementation for special-constant 
preserving anti-unification and  
interaction models composition.

We have implemented anti-unification modulo
any combination of associative commutative and
unit element equations.

## How to build 

You can build the project with cargo using 

```
cargo build --release
```

An executable will be generated in "./target/release".

## Anti-unification

There are two dedicated commands for anti-unification:

- `lgg`: for anti-unification modulo ACU (associativity+commutativity+unit element)

- `sclgg`: for special constant preserving anti-unification modulo ACU.

We provide a section describing the details for those two commands:

[Anti-unification commands](readme/anti-unification.md)

## Interaction composition

The command for the composition of interactions is :
- `compose`

The following section describes this command in details:

[Interaction composition command](readme/composition.md)

We also provide a short tutorial on the syntax of interactions
based on [HIBOU](https://github.com/erwanM974/hibou_label):

[Tutorial on interactions](readme/interactions.md)


## A Benchmark for interaction composition 

We also provide a simple benchmark to validate our 
implementation of interaction composition:

[Execution of the benchmark](readme/benchmark.md)

## License
This project is licensed under the GNU GPL version 3 or later (GPL-3.0-or-later).  
See the LICENSE file for details.








