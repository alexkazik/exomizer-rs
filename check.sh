#!/bin/bash

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

set -x

# builds (std+alloc, alloc, nothing) (std implies alloc)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features || exit 1

# clippy (std+alloc, alloc, nothing) (std implies alloc)
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
cargo rdme --force || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

# tests (safe+std+alloc, alloc) (std implies alloc, tests require alloc)
cargo $TOOLCHAIN test --release --all-features || exit 1
cargo $TOOLCHAIN test --release --no-default-features || exit 1
