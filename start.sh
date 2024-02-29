#!/bin/bash

set -ex

# Variables
build_binary="y"
clear_logs="y"
tmpdir="/tmp/snarkos"
currdir=$(pwd)

if [[ ${build_binary} == "y" ]]; then
	if [[ ! -d ${tmpdir} ]]; then
		mkdir -p "${tmpdir}"
		git clone --branch minimal --depth 1 "https://github.com/leology-org/snarkOS" "${tmpdir}"
	fi
	cd "${tmpdir}" && cargo build --release --features minimal && cd "${currdir}"
fi

SNARKOS="${tmpdir}/target/release/snarkos"

if [[ ${clear_logs} == "y" ]]; then
	"${SNARKOS}" clean --dev 0
fi

log_dir=".logs-$(date +"%Y%m%d%H%M%S")"
mkdir -p "${log_dir}"

log_file="${log_dir}/validator.log"
"${SNARKOS}" start --nodisplay --dev 0 --dev-num-validators 1 --validator --logfile "${log_file}"

rm -rf .ledger-* .logs-*
