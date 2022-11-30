#!/usr/bin/env bash
if ! [[ $1 =~ ^[0-9]{1,2}$ ]] ; then
  echo "Not a valid arg" >&2; exit
fi

day=day$(printf "%02d" "$1")

binary=./src/bin/"${day}".rs

if ! [[ -f "$binary" ]] ; then
 cat <<< "fn main() -> anyhow::Result<()> {
    Ok(())
}" >> "$binary"
fi

touch ./src/"${day}".rs

if ! grep -q "${day}" ./src/lib.rs ; then
  cat <<< "pub mod ${day};" >> ./src/lib.rs
fi


