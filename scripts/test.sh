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

function generate_flatbuffer_models {
  mkdir -p flatbuffer_generated
  rm -f flatbuffer_generated/*.rs
  rmdir flatbuffer_generated

   flatc --rust -o flatbuffer_generated flatbuffer_schemata/roll_request.fbs flatbuffer_schemata/roll_response.fbs
}

function run_tests {
  cargo test
}

function main {
  set_bash_error_handling
  go_to_project_root_directory
  generate_flatbuffer_models
  run_tests
}

main
