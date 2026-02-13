#!/bin/bash


#BINARY="./generalizer"
BINARY="../Executable/generalizer"
OUTPUT_DIR="./Benchmark_Output"


echo "----------------------------------------"
echo "Starting the Step 1 of Benchmark: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"
SECONDS=0
NB_MUTATIONS=7
MAX_NB_PARTITIONS=5
# Run benchmark
$BINARY benchmark_step_1 Benchmark $NB_MUTATIONS $MAX_NB_PARTITIONS -m -d

echo "----------------------------------------"
echo "Execution time: $SECONDS seconds"
echo "Step 1 of the Benchmark finished: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "----------------------------------------"
