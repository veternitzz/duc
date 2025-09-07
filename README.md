# duc - a general purpose luau runtime

duc is a general purpose luau runtime with a rich standard library.

## std

std is the luau require alias and main module in duc. std contains numerous sub modules for manipulation of stdio, the filesystem and the underlying system.
here's a list of sub modules within duc and a short description. any sub module prefixed with an asterick (*) indicates that it is on the roadmap,
but not yet implemented.

- fs: a module which exposes functions for manipulation of the filesystem.
- io: a module which exposes functions for reading stdin and writing to stdout/stderr.
- sys: a module which exposes functions for manipulating the underlying system.
