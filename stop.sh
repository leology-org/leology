#!/bin/bash

total_validators="4"
mapfile -t validator_indices < <(seq 0 $((total_validators - 1))) || true

# Separate the command to avoid masking its return value
if ! tmux kill-session -t devnet; then
	echo "Session kill failed or session did not exist"
fi

for validator_index in "${validator_indices[@]}"; do
	# Use braces around variable and separate command for error handling
	if ! tmux kill-window -t "devnet:${validator_index}"; then
		true
	fi
done
