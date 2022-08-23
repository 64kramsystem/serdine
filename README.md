![Logo](/images/serdine.jpg?raw=true)

# Serdine

Serdine is a tiny serialization library for storing types in a raw (but safe), memcpy-like, format.

This is convenient for example, when interfacing with data files belonging to a C program, whose internal structure are `memcpy`'d from the in-memory instances.

## Status

This project is currently used in another project of mine ([Catacomb II-64k](https://github.com/64kramsystem/catacomb_ii-64k)); it's in the process of being prepared for general use, then it will be released as crate.
