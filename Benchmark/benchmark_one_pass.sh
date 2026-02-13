#!/bin/bash 


#BINARY="./generalizer"
BINARY="../Executable/generalizer"
OUTPUT_DIR="./Benchmark output"


echo "----------------------------------------"
echo "Starting benchmark on one pass: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"
SECONDS=0

# Run benchmark
$BINARY benchmark Benchmark 7 5 60 -m -d

echo "----------------------------------------"
echo "Execution time: $SECONDS seconds"
echo "Benchmark finished: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "Check the file results.csv for the computation durations"
echo "----------------------------------------"
