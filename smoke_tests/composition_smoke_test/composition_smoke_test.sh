#!/bin/bash


#BINARY="./generalizer"
BINARY="../../Executable/generalizer"
OUTPUT_DIR="./Composition Output"

echo "----------------------------------------"
echo "Composition smoke test with a simple example"
echo "This script uses the executable at the location:  generalizer/target/release/generalizer"
echo "Starting composition: $(date)"
echo "Results will be written to: ${OUTPUT_DIR}"
echo "----------------------------------------"


# Run benchmark
$BINARY compose signature.hsf i.hif j.hif -f

echo "----------------------------------------"
echo "Composition smoke test: $(date)"
echo "Check results inside: ${OUTPUT_DIR}"
echo "----------------------------------------"
