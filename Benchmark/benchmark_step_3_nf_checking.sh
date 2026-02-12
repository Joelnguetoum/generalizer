#!/bin/bash


#BINARY="./generalizer"
BINARY="../Executable/generalizer"
OUTPUT_DIR="./Benchmark_Output"

echo "----------------------------------------"
echo "Starting the step 3 of the benchmark: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"

NB_MUTATIONS=7
MAX_NB_PARTITIONS=5
# Run benchmark
$BINARY benchmark_step_3 Benchmark $OUTPUT_DIR $NB_MUTATIONS $MAX_NB_PARTITIONS -m -d

echo "----------------------------------------"
echo "Step 3 of the benchmark finished: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "Check the file results_step_3.csv for the computation durations"
echo "----------------------------------------"