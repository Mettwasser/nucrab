# nucrab
A very simple package manager for `nushell`.

Nushell already has a [package manager](https://github.com/nushell/nupm).

However, said package manager unfortunately cannot install from a remote - you need to clone the repo and then run your install script.

`nucrab` (very creative name, I know) changes that!

## Installation
`cargo install nucrab`

## Backwards compatibility
Right now `nucrab` isn't fully backwards compatible with `nupm`.

Right now...
- it only supports installing from a github repo
- it only supports installing modules

However, from what I was able to see, most `nupm` packages use modules anyway. If requested I can add the other types as well.

## Showcase
```nushell
~> nucrab -h
Usage: nucrab.exe <COMMAND>

Commands:
  install    Install a package from a github repo
  uninstall  Uninstall a package
  list       List all installed packages
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


~> nucrab install nushell/nupm
nupm has been installed (███████████████████████████████████████████████████████████████████████████████████████) 100%


~> nucrab list
- nupm

1 packages installed


~> nucrab uninstall nupm
uninstalling nupm...
nupm has been uninstalled


~> nucrab list

no packages installed
```