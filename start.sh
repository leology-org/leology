#!/bin/bash

set -ex

# Ask the user if they want to run 'cargo install --path .' or use a pre-installed binary
#read -p "Do you want to run 'cargo install --path .' to build the binary? (y/n, default: y): " build_binary
#build_binary=${build_binary:-y}
build_binary="y"

# Ask the user whether to clear the existing ledger logs
#read -p "Do you want to clear the existing ledger logs? (y/n, default: n): " clear_logs
#clear_logs=${clear_logs:-n}
clear_logs="y"
tmpdir="/tmp/snarkos"
currdir=$(pwd)

if [[ $build_binary == "y" ]]; then
  # Build the binary using 'cargo install --path .'
  if [ ! -d "$tmpdir" ]; then
      # Directory doesn't exist, clone the snarkOS repository
      mkdir -p "$tmpdir"
      git clone --branch minimal --depth 1 "https://github.com/leology-org/snarkOS" "$tmpdir"
  fi
  cd "$tmpdir" && cargo build --release --features minimal && cd "$currdir"
fi

SNARKOS="$tmpdir/target/release/snarkos"

# Clear the ledger logs for each validator if the user chooses to clear logs
if [[ $clear_logs == "y" ]]; then
  $SNARKOS clean --dev 0
fi

# Create a timestamp-based directory for log files
log_dir=".logs-$(date +"%Y%m%d%H%M%S")"
mkdir -p "$log_dir"

# Launch the validator
log_file="$log_dir/validator.log"
$SNARKOS start --nodisplay --dev 0 --dev-num-validators 1 --validator --logfile "$log_file"

rm -rf .ledger-* .logs-*