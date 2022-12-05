#!/usr/bin/env bash
if ! [[ $1 =~ ^[0-9]{1,2}$ ]] ; then
  echo "Not a valid arg" >&2; exit
fi

day=day$(printf "%02d" "$1")

binary=./src/bin/"${day}".rs

if ! [[ -f "$binary" ]] ; then
    cat <<< "use aoc2022::run;

fn main() -> anyhow::Result<()> {
    run!(${day})
}" >> "$binary"
fi

lib=./src/"${day}".rs

if ! [[ -f "$lib" ]] ; then
    cat <<< "pub fn run(input: &str) -> anyhow::Result<(u32, u32)> { 
    todo!() 
}" >> "$lib"
fi

if ! grep -q "${day}" ./src/lib.rs ; then
  cat <<< "pub mod ${day};" >> ./src/lib.rs
fi

cargo fmt
