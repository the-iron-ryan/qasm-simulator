#!/bin/bash
for file in ./qasm/*
do
    cargo run -- $file
    echo ""
done