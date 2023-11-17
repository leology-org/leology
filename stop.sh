#!/bin/bash

total_validators="4"
validator_indices=($(seq 0 $((total_validators - 1))))
tmux kill-session -t devnet || echo
for validator_index in "${validator_indices[@]}"; do
  tmux kill-window -t "devnet:$validator_index" || echo
done
