[← Back to main README](../README.md)

# Benchmark 

We propose a simple experiment to validate our implementation of interaction composition.

As an introduction to interaction, we propose a short [tutorial](interactions.md) on the syntax of interaction with gates, and their representation as .hsf (HIBOU Signature file)
and .hif(HIBOU interaction file) files.

## Protocol

We use the interactions in the folder [Benchmark](../Benchmark%20Composition/Benchmark) 
as our starting global models. For each global interaction $k$, we extract 
at most $N_p$ partitions of its set of lifelines $L$ into a pair of subsets each of size at least $\lfloor L/2 \rfloor$.  

For each partition $(L_1,L_2)$ of a set of lifelines of a global interaction $r$:

- project $r$ onto $L_1$ and $L_2$ to obtain local interactions $i$ and $j$;
- we normalize $i$ and $j$ using HIBOU to obtain $i_\text{norm}$ and $j_\text{norm}$
respectively.
- we apply mutation operations to $i$ and $i$, with consists of successively
applying $N_m$ times one of the following rewrite operation selected uniformly
at random: $\textsf{alt}(x,y) \rightarrow \textsf{alt}(y,x)$ and $\textsf{par}(x,y) \rightarrow \textsf{par}(y,x)$.
We obtain the interactions $i_\text{mut}$ and $j_\text{mut}$ from $s$ and $t$ respectively.
The mutations are done with [Maude](https://maude.cs.illinois.edu/).
- We compose the pairs $(i_\text{norm},j_\text{norm})$ and $(i_\text{mut},j_\text{mut})$.
The result of the composition is normalized with HIBOU an compared to the normal form of 
the starting interaction $k$.

The table below summarizes experiments 
conducted on interactions adapted from the 
literature. Each interaction corresponds to a row in the table.
The second column indicates the size of each interaction,
while the third column shows the range of the number of gates
in local interactions with respect to partitions of lifelines.
The last four columns represent the average composition duration
across partitions, with and without the rule $\textsf{Fail}$.
In particular, the fourth and fifth columns report the average duration
for the composition of normalized local interactions,
and the last two columns report the average duration for the mutated
local interactions. A success ($\checkmark$) confirms
that the normal form  of the result
of each composition across partitions matches with
the normal form of the original interaction before projections. We 
did $N_m = 7$ local mutation, extracted at most $N_p = 5$ partitions of lifelines and 
a computation timout of $60$ seconds.

![benchmark_table](images/benchmark/benchmark_table.png "Benchmark table")



## How to run the benchmark

To run the benchmark, download the folder [Benchmark Composition](../Benchmark%20Composition), and add an executable of the project.

The subcommand to run the benchmark is `benchmark`. It takes as arguments:

- the name of the subfolder containing the interactions. In the downloadable folder, it is Benchmark.
- the number of mutation  per partition
- the maximal number of random partitions extracted by global interaction.
- Timout in seconds


We can add flags, `-m` to have the duration in milliseconds,
`-d` to draw the models for visualization.

The theory for the composition is ACU by default. We can restrict the theory  with
the same flags as the composition: `--A`, `--C`, `--U`, `--AC`, `--AU`, `--CU`, `--S`.

The flag `-g` is not valid for the benchmark, since both composition with and
without the rule Fail are evaluated.

The command to execute to have the result in the table above is:

```
generaliser benchmark Benchmark 7 5 60 -m 
```

It means:

For each global interaction, at most 5 partitions of its lifelines will be extracted;
after projection onto the partitions, 7 random mutations are operated
in the local interactions. The timout threshold is of 60s. the flag -m means that
in the output csv file, the duration will be given in milliseconds.
The theory for the composition is ACU (all the rules are used).

To draw the interactions involved in the process, we can use the flag `-d`.


## Interactions of the benchmark

We present in the following table sequence diagram
representation of the interactions of the benchmark, 
which files are in the folder [Benchmark](../Benchmark%20Composition/Benchmark).
Those interaction were adapted from examples and experiments from 
the literature.



| Name                     | Interaction graphical representation                             | Reference                                        |
|--------------------------|------------------------------------------------------------------|--------------------------------------------------|
| Alternating3Bit Protocol | ![alt3bit](images/benchmark/BO/Alt3bit/input_global_interaction/Alt3bit.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| Filter collaboration     | ![filter](images/benchmark/BO/FilterCo/input_global_interaction/FilterCo.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| Game                     | ![game](images/benchmark/BO/Game/input_global_interaction/Game.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| Health System            | ![HS](images/benchmark/BO/HealthSys/input_global_interaction/HealthSys.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| Logistic                 | ![Log](images/benchmark/BO/Logistic/input_global_interaction/Logistic.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| Professor Online         | ![prof](images/benchmark/BO/ProfOnline/input_global_interaction/ProfOnline.png) | [Rocha et al.](https://doi.org/10.1007/s11219-020-09531-0) |
| Sanitary Agency          | ![San](images/benchmark/BO/Sanitary/input_global_interaction/Sanitary.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| TPM Contract v2          | ![TPM](images/benchmark/BO/TPM/input_global_interaction/TPM.png) | [Lange et al.](https://doi.org/10.1145/2676726.2676964) |
| Travel                   | ![Trav](images/benchmark/BO/Travel/input_global_interaction/Travel.png) | [Bouma et al.](https://doi.org/10.1007/978-3-031-30820-8_3) |
| Two Buyers protocol      | ![Two](images/benchmark/BO/TwoBuyers/input_global_interaction/TwoBuyers.png) | [Honda et al.](https://doi.org/10.1145/2827695)  |
| ATM                      | ![ATM](images/benchmark/BO/ATM/input_global_interaction/ATM.png) | [Edixhoven et al.](https://doi.org/10.1016/j.jlamp.2023.100919) |
| Distributed Voting       | ![DistVot](images/benchmark/BO/DistVoting/input_global_interaction/DistVoting.png) |  [Edixhoven et al.](https://doi.org/10.1016/j.jlamp.2023.100919)|


## An example of the workflow for the interaction Game

The following figure illustrates our protocol with the 
Game global interaction, with only the mutation scenario.

![workflow](images/benchmark/workflow_example.png)
