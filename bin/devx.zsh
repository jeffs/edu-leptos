#!/usr/bin/env -S zsh -euo pipefail
#
# Installation and configuration for improved Leptos development experience.
#
# For VS Code, add the following to in Workspace settings:
#
#   "rust-analyzer.rustfmt.overrideCommand": [
#       "leptosfmt",
#       "--stdin",
#       "--rustfmt"
#   ]
#
# See also: https://github.com/bram209/leptosfmt

cargo install leptosfmt
