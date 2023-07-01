#!/bin/bash
set -ex
mkdir .testdata
pushd .testdata
mkdir packages
pushd packages
mkdir backend client frontend
popd
popd