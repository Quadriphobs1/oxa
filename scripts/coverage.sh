#!/bin/sh
set -eu


main() {
  echo "## Running rust build"
  cargo build

  echo "## Running rust test"
  cargo test

  ## grcov code scripts
  echo "## Running grcov"
  grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

  zip -r oxa_cov.zip ./target/debug/coverage

  # TODO: Fix zip: not found
  # TODO: Export the generated coverage outside of the docker to local machine
}

main "$@"
