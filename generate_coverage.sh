export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

select_nightly_rust() {
  echo "> Selecting nightly rust";
  rustup default nightly;
}

select_stable_rust() {
  echo "> Selecting stable rust";
  rustup default stable;
}

install_grcov() {
  echo "> Installing grcov";
  cargo install grcov;
}

cleanup_older_coverage() {
  echo "> Cleaning old coverage";
  rm -rf ./target/debug/coverage
}

build_project() {
  echo "> Building project";
  cargo build;
}

run_tests() {
  echo "> Run tests";
  cargo test --tests;
}

run_gcov() {
  echo "> Running gcov";
  # grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --keep-only "src/prices/mod.rs" --keep-only "src/blockchain/mod.rs" --keep-only "src/lightning/mod.rs" --keep-only "tests/*" -o ./target/debug/coverage/;
  grcov . \
    --branch \
    --ignore-not-existing \
    --keep-only "src/prices/mod.rs" \
    --keep-only "src/blockchain/mod.rs" \
    --keep-only "src/lightning/mod.rs" \
    --keep-only "tests/*" \
    --binary-path ./target/debug/ \
    -s . \
    -t html \
    -o ./target/debug/coverage/;
}

generate_coverage() {
  select_nightly_rust
  install_grcov
  cleanup_older_coverage
  build_project
  run_tests
  run_gcov
  select_stable_rust
}
generate_coverage