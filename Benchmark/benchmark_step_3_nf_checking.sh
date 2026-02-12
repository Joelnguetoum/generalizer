#!/bin/bash

BINARY="../Executable/generalizer"
OUTPUT_DIR="./Benchmark_Output"

echo "----------------------------------------"
echo "Starting step 3 benchmark: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"


NB_MUTATIONS=7
MAX_NB_PARTITIONS=5

# Run benchmark
$BINARY benchmark_step_3 Benchmark $OUTPUT_DIR $NB_MUTATIONS $MAX_NB_PARTITIONS -m -d

echo "----------------------------------------"
echo "Benchmark finished: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "Check the file results_step_3.csv for the computation durations"
echo "----------------------------------------"
