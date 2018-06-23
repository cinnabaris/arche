#!/bin/sh

mkdir -pv build
cd build
CC=/usr/bin/clang CXX=/usr/bin/clang++ cmake -DCMAKE_BUILD_TYPE=Release ..
make -j
strip -s arche
