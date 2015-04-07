#!/bin/bash

set -u -e

rand_int() {
  shuf -i 1-1000 -n 1
}

generate() {
  out=$1
  size=$2
  test ! -e $out
  dd if=/dev/urandom of=$out count=1 bs=$size 2> /dev/null
}

run_tests() {
  model=test-model-$$.tmp
  input=test-input-$$.tmp
  generate $input $(rand_int)
  generate /dev/stdout $(rand_int) | mkskr > $model
  skr $model < $input | unskr $model | diff $input /dev/stdin
  rm -f $model $input
}

go() {
  export PATH=$PWD:$PATH
  export LD_LIBRARY_PATH=$PWD:${LD_LIBRARY_PATH-$PWD}
  n=200

  for _ in $(seq $n); do
    run_tests
  done
}

go
