# generaliser


### Sub-commands

This simple tool computes least general generalisation for anti-unification  
and constrained anti-unification problems. For now only the empty theory is supported.

The basic syntax of the command is: 

    $ generaliser lgg file.txt

or 

    $ generaliser clgg file.txt


where file.txt (path to the file actually) contains
the specification of the problem.

The subcommand **lgg** is used to solve anti-unification problems and **clgg**
for constrained anti-unification problems.

### Specification of the problem

In the file given to argument to the program, all the function symbols (including constant symbols)
must be declared. The syntax to do it is: 

    Function: <name> <arity> S?

The name is string that must start with a letter, the arity is an integer.
Constant symbols are of arity 0. The 'S' is added to declare special constants for 
the constrained anti-unification. 

A problem is declared as follows: 

    Problem: x t t'

where t and t' are the terms to generalise. Each of the function symbols 
appearing in t and t' must be declared. 
For examples of problem specification, see the Example folder.

To execute one of the example, we can have: 

    $ generaliser lgg example1.txt

or 

    $ generaliser clgg example1.txt

if the executable is in the same folder that of the input files.

### Composition

To compose two interaction models, use the command 'compose' with tree arguments:
 - a.hsf file
 - a .hif file
 - and another .hif file

For example: 

    $  generaliser compose models/sig.hsf models/i1.hif models/i2.hif

The program will compute the composition of the two interactions.

###  Flags

Thoses flags are valid for the lgg, clgg and compose commands.
However, -d is not available for the compose command.

#### To only use the rules of the algorithme of Alpuente et al (A modular order-sorted equational generalization algorithm,2014)

Use the flag -a or --alpuente

An example is

    $ generaliser lgg example1.txt -a

#### verbose

The flag verbose print in command line the computation history of each least general generalisations.

An example is 

    $ generaliser lgg example1.txt -v

or

    $ generaliser lgg example1.txt --verbose

The same flag remain available for the command clgg.

#### dot

The flag dot creates a .dot file and a png depicting the computation history of the least general generalisations.

An example is

    $ generaliser lgg example1.txt -d

or

    $ generaliser lgg example1.txt --dot

#### verbose and dot can be used together

The flags can obviously be combined.

For example 


    $ generaliser lgg example1.txt -v -d

or

    $ generaliser lgg example1.txt --verbose --dot



# Benchmark FM 26

To run the benchmark, download the folder Benchmark FM26, and add an excecutable of generaliser.

The subcommand to run the benchmark is "benchmark". It takes:

- the name of the subfolder containing the interactions. In the downloadable folder, it is Benchmark.
- the number of mutation per cycle
- the number of cycles of composition.


The basic command is: 

    $ generaliser benchmark Benchmark

We can add flags, -m to have the duration in milliseconds, -g to use the rule greedy fail,
-d to draw the models for visualization.

The command to execute to have the result in the paper is: 

      $ generaliser benchmark Benchmark 5 5 -m -g


