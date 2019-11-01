#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# shellcheck disable=SC2155
# this script is intended to run on OSX by default, comment this line out to run on linux
export PATH="$PATH:$(brew --prefix llvm)/bin"

for infile in e2e_test/*.ins; do
  echo "$infile"

  ./insc_jvm "$infile" > /dev/null 2>&1
  ./insc_llvm "$infile" > /dev/null 2>&1

  pushd e2e_test > /dev/null 2>&1

  expected_out="$(basename "$infile" .ins).output"
  jvm_in="$(basename "$infile" .ins)"
  jvm_out="$(basename "$infile" .ins).jvmout"
  llvm_in="$(basename "$infile" .ins).bc"
  llvm_out="$(basename "$infile" .ins).llvmout"

  java "$jvm_in" > "$jvm_out"
  if [[ -n $(diff "$expected_out" "$jvm_out") ]]; then
    echo "JVM invalid result:"
    echo diff "$expected_out" "$jvm_out"
    exit 1
  fi

  lli "$llvm_in" > "$llvm_out"
  if [[ -n $(diff "$expected_out" "$llvm_out") ]]; then
    echo "LLVM invalid result:"
    echo diff "$expected_out" "$llvm_out"
    exit 1
  fi

  popd > /dev/null 2>&1
done

echo "All tests passed!"
