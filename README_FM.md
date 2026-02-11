
# FM26 artifact


# Introduction

This README file describes the artifact related to the paper ["Specializing anti-unification for interaction
models composition via gate connections"] accepted to the ``FM26`` conference.

The paper proposes an approach to the composition of interaction models using
anti-unification. The program, named `generalizer` is developped in Rust.
Docker image provides a built executable under the folder `generalizer/Executable`.



## Set Up

The artefact is wrapped in a docker image available on Zenodo(todo: link).
After downloading the image, it is loaded with the following command:

```bash
$ docker load -i generalizer.tar.gz
```

Alternatively, the image can be built from the root of the repository
with the following command:

```bash
$ docker build -t generalizer .
```

After loading or building the image, running the container is done with the following command:

```bash
$ docker run -it --rm generalizer:latest
```

## Smoke tests

By running the container, `Docker` will open a shell in a container inside a directory named `generalizer`.
The smoke tests are located in the `generalizer/smoke_tests` directory. There are 
two smoke tests: a composition smoke test and a reduced benchmark smoke test.

### Composition smoke test

To check whether the composition of two interactions works, we check that 
with the example in the introduction of the paper. It is located in 
`generalizer/smoke_tests`. The folder contains:
- `signature.hsf`: the signature file of the interactions containing the declaration of lifelines and messages.
- `i.hif`: the first interaction.
- `j.hif`: the second interaction.
- `composition_smoke_test.sh`: the script to run the composition of the interaction models `i` and `j`.

```bash
$ cd smoke_tests/composition_smoke_test
$ ./composition_smoke_test.sh
```

If successful, the success message will be printed in the terminal.
The result will be put in the folder `Composition_output` which contains a folder 
`result` containing the files `result.hif`(interaction file) and `result.png`(visual representation of the result). 

### Reduced benchmark smoke test

To quickly check wheher the benchmark runs successfully, we provide a reduced version of the benchmark.
It is located in `generalizer/smoke_tests/reduced_benchmark_smoke_test`. 
The folder contains the script `reduced_benchmark_smoke_test.sh` to run the small benchmark.

```bash
$ cd smoke_tests/reduced_benchmark_smoke_test
$ ./reduced_benchmark_smoke_test.sh
```
The result will be put in the folder `Benchmark_Output`. It containts a csv file `result.csv` containing 
a table akin the exprerimental 

# Artifact structure


## Composition Examples

## Anti-unification Examples

## Benchmark

