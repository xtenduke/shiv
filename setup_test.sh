#!/bin/bash

rm -rf /tmp/shivr/.testdata
set -ex

mkdir -p /tmp/shivr/.testdata
pushd /tmp/shivr/.testdata
mkdir -p packages/backend packages/client packages/frontend
touch test

git init -b main
git add .
git commit -m "Initial commit"
git branch f/test
git checkout f/test
touch packages/backend/run.sh packages/frontend/run.sh
git add .
git commit -m "feature"

# pop back to root
popd

