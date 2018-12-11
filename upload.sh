#!/usr/bin/env bash

# ideally in the future this script is deprecated for a cargo tools bin method
# https://matklad.github.io/2018/01/03/make-your-own-make.html
# but were waiting on the ability to run std build scripts in a no_std project
# https://github.com/rust-embedded/wg/issues/256#issuecomment-438483578

# alternatively cargo-make is an option but thats difficult to run from a runner
# and maintain cross platform support, so I offer this sugar for *nix users only

# but in the mean time this hardcodes the llvm find from cargo-binutils
# https://github.com/rust-embedded/cargo-binutils

set -x
set -e

$(find $(rustc --print sysroot) -name llvm-objcopy) -O binary "$@" "$@".bin
cp "$@".bin "$@".dfu
dfu-suffix -v 1209 -p 70b1 -a "$@".dfu
dfu-util --download "$@".dfu
