# Benchmark FM 26

To run the benchmark, download the folder Benchmark FM26, and add an excecutable of generaliser.

The subcommand to run the benchmark is "benchmark". It takes as arguments:

- the name of the subfolder containing the interactions. In the downloadable folder, it is Benchmark.
- the number of mutation  per partition
- the number of random partitions extracted by global interaction.
- Timout in seconds


We can add flags, -m to have the duration in milliseconds,
-d to draw the models for visualization.

The theory for the composition is ACU by default. We can restrict the theory  with
the same flags as the composition: --A, --C, --U, --AC, --AU, --CU, --S.

The flag -g is not valid for the benchmark, since both composition with and
without the rule Greedy-Fail are evaluated.

The command to execute to have the result in the paper is:

      $ generaliser benchmark Benchmark 7 5 60 -m 

It means:

For each global interaction, 5 partitions of its lifelines will be extracted;
after projection onto the partitions, 7 random mutations are operated
in the local interactions. The timout threshold is of 60s. the flag -m means that
in the output csv file, the duration will be given in milliseconds.
The theory for the composition is ACU (all the rules are used).