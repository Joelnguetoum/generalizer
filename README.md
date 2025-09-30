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