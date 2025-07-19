#!/bin/bash

git stash
cargo bench --package comparison > old.txt
# cargo bench >> old.txt

git stash pop
cargo bench --package comparison > new.txt
# cargo bench >> new.txt

diff --side-by-side old.txt new.txt
