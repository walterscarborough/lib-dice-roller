#!/usr/bin/env bash

function set_bash_error_handling() {
  set -o errexit
  set -o errtrace
  set -o nounset
  set -o pipefail
}

function go_to_project_root_directory() {
  local -r script_dir=$(dirname "${BASH_SOURCE[0]}")

  cd "$script_dir/.."
}

function generate_protobufs {
  mkdir -p generated-protobufs
  protoc --swift_out=generated-protobufs --proto_path=../../common/lib-dice-roller/protobuf_schemata roll_request.proto roll_response.proto
}

function create_xcode_generated_directory {
  mkdir -p "DiceRoller/DiceRollRepository/Generated LibDiceRoll"
}

function main {
  set_bash_error_handling
  go_to_project_root_directory

  create_xcode_generated_directory
  generate_protobufs
}

main
