#!/bin/bash

# this can be used to run a test that only prints the output of
# the generator and the output of the generated code, in src/test/printer.rs

cargo test printer -- --nocapture
