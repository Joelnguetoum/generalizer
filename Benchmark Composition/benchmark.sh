#!/bin/bash 


BINARY="./generalizer"
OUTPUT_DIR="./Benchmark output"

echo "----------------------------------------"
echo "Starting benchmark: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"


# Run benchmark
./generalizer benchmark Benchmark 7 5 60 -m 

echo "----------------------------------------"
echo "Benchmark finished: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "----------------------------------------"
