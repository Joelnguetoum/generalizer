### Composition

To compose two interaction models, use the command 'compose' with tree arguments:
- a .hsf file
- a .hif file
- and another .hif file

For example:

    $  generaliser compose sig.hsf i1.hif i2.hif

The program will compute the composition of the two interactions.
###  Flags specific for the composition

We can specify the theory under which the composition will happen.
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