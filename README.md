# `Flatten` and `FlatMap` with constant inner iterator size

This Rust package provides `ConstSizeFlatten` and `ConstSizeFlatMap` which make use of the also provided `ConstSizeIntoIterator` to know how many items they will yield.

Note that `core` & `std` already provide this functionality for some types through a hack using specialization. This crate’s contribution is that the trait `ConstSizeIntoIterator` is public and the functionality is therefore extensible.
