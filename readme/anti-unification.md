[← Back to main README](../README.md)


The subcommands for the anti-unification are 'lgg' for regular anti-unification
and 'sclgg' for special constant preserving anti-unification.

Our algorithm is inspired from the work of [Alpuente et al(2014)](https://doi.org/10.1016/j.ic.2014.01.006).

### Sub-commands

The basic syntax of the command is:

    $ generalizer lgg file.txt

for unrestricted anti-unification or

    $ generalizer sclgg file.txt

for special constant-preserving anti-unification.

The file.txt (path to the file actually) contains
the specification of the problem.

### Specification of the problem in the file given as argument

In the file given to argument to the program, all the function symbols (including constant symbols)
must be declared. The syntax to do it is:

    Function: <name> <arity> <Axioms>?

The name is string that must start with a letter, the arity is an integer.
Constant symbols are of arity 0.

For example to declare a function symbol 'f' 
of arity 2 and a constant symbol 'a', we write:

    Function: f 2 
    Function: a 0

The 'Axiom' field is an optional string that is specified as follows:

 - Either with a string with letters in {A,C,U} to specify the axioms of a function symbol.
For example to declare a function symbol 'f' with is associative an of arity 2, we do:


        Function: f 2 A
An for a function 'g' that is associative, commutative with unit, we write:

        Function: g 2 ACU
Other combinations like AU (associative-unit) or CU(commutative-unit) are also 
possible. The orther of the charaters A,C or U do not matter.
 - Or with a 'S' that is added to declare special constants for
   the special constant-preserving anti-unification.
For example, to declare a spacial constant 'a', we write:


        Function: a 0 S


A problem is declared as follows:

    Problem: x t t'

where t and t' are the terms to generalize. Each of the function symbols
appearing in t and t' must be declared.
For examples of problem specification, see the Example folder.

To execute , we have:
The file the file [example6.txt](../Examples/Anti-unification/exemple6.txt) in the [Example/Anti-unification](../Examples/Anti-unification) folder
contains the following problem:

~~~
Function: f 2 AC
Function: g 2 U
Function: a 0 S
Function: b 0 S
Function: c 0
Function: d 0
Problem: x f(a,g(b,c)) f(b,a)
~~~

The signature contains an associative-commutative function symbol 'f',
a symbol 'g' with unit element, two constants 'c' and 'd'. In addition there are two
special constants 'a' and 'b'. 



    $ generalizer lgg example6.txt

or

    $ generalizer sclgg example6.txt

if the executable is in the same folder that of the input files.

If special constants are declared, the 'lgg' subcommand will ignore the 

The program will return a list of generalizations. Redundant generalizations
are eliminated through a brute-force AC-matching algorithm. However, our matching
algorithm do not include unit element yet. So redundant generalization might
appear in the results if the problem contain a function symbol with unit element.

> :memo: The sclgg command might return a failure verdict in the case no special 
> constant-constant preserving generalization is found.

###  Flags

Flags might be provided to configure the anti-unification commands.


#### To only use the rules of the algorithme of [Alpuente et al(2014)](https://doi.org/10.1016/j.ic.2014.01.006)

Use the flag -a or --alpuente

An example is

    $ generalizer lgg example1.txt -a

#### verbose

The flag verbose print in command line the computation history of each least general generalisations.

An example is

    $ generalizer lgg example1.txt -v

or

    $ generalizer lgg example1.txt --verbose

The same flag remain available for the command clgg.

#### Rule Fail for the computation sclgg

To use the rule $\textsf{Fail}$ for the anti-unification,
use the flag -f.




