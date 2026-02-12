#!/bin/bash


#BINARY="./generalizer"
BINARY="../Executable/generalizer"
OUTPUT_DIR="./Benchmark_Output"


echo "----------------------------------------"
echo "Starting the step 2 of the benchmark: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"

NB_MUTATIONS=7
MAX_NB_PARTITIONS=5
TIMOUT_SECS=60
# Run benchmark
$BINARY benchmark_step_2 Benchmark $OUTPUT_DIR $NB_MUTATIONS $MAX_NB_PARTITIONS $TIMOUT_SECS -m -d

echo "----------------------------------------"
echo "Step 2 of the benchmark finished: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "Check the file results_step_2.csv for the computation durations"
echo "----------------------------------------"
