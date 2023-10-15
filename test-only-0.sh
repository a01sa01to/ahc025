#!/bin/bash

export RUST_BACKTRACE=1
cargo build
if [ $? -ne 0 ]; then
  echo "Build failed..."
  exit 1
fi

cd tools
echo Running Test 0...
cargo run --release --bin tester ../target/debug/a < in/0.txt 1> out/0.txt 2> err/0.txt
cat err/0.txt
cd ..
