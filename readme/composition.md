[← Back to main README](../README.md)

# Interaction Composition

We have implemented a composition procedure for interactions models.

Interaction language is a term-based formalism for the specification of 
distributed systems. Its formal semantics was developed by [Mahe et al.](https://doi.org/10.1016/j.scico.2023.103034).

Our implementation of interactions is based on [HIBOU](https://github.com/erwanM974/hibou_label), 
which offer various manipulation utilities for interactions.

As an introduction to interaction, we propose short [tutorial](interactions.md) on
the syntax of interaction with gates, and their representation as .hsf (HIBOU Signature file)
and .hif(HIBOU interaction file) files.


To compose two interaction models, use the command 'compose' with tree arguments:
- a .hsf file, the signature file,
- two .hif files, which represent the interactions to compose. The two interaction
sharing the same signature.

For example:

    $  generaliser compose sig.hsf i1.hif i2.hif

The program will compute the composition of the two interactions, and draw the 
result in a folder called "Composition Output".


##  Flags 

### Restriction of the anti-unification theory

We can restrict the theory under which the composition will happen.
There are many possibilities. By default the theory considered is
ACU (associativity-commutativity-unit). We can restrict the theory by
providing one of the following flags:

--S for syntactic generalization (no equations)

--A or -A for generalization modulo associativity

--C or -C for generalization modulo commutativity

--U or -U for generalization modulo unit

--AC  for generalization modulo associativity and commutativity

--AU  for generalization modulo associativity and unit

--CU for generalization modulo commutativity and unit

--ACU (optional, the theory is ACU by default) for generalization modulo associativity-commutativity-unit.

Those flags are mutually exclusive.


For example, to compose modulo AU, we can use the following command

    $  generaliser compose sig.hsf i1.hif i2.hif --AU


### To only use the rules of the algorithme of [Alpuente et al(2014)](https://doi.org/10.1016/j.ic.2014.01.006)

Use the flag -a or --alpuente

An example is

    $  generaliser compose sig.hsf i1.hif i2.hif -a

### verbose

The flag verbose print in command line the
computation history of each least general generalisations.

The flag is -v or --verbose

An example is

    $  generaliser compose sig.hsf i1.hif i2.hif -v


# Example 

### todo



