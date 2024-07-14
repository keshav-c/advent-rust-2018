Approach

First read input

1. Define a `RawLog` struct which will hold the unprocessed log lines
2. Implement `TryFrom<&str>` for `RawLog` in order to parse the input text data and show errors if any
3. The input is not sorted in chronological order. So I define a wrapper struct which will hold the raw logs in a sorted order.
4. To make a vector of `RawLog`s sortable by their timestamp I need to implement `Ord` for `RawLog`
5. As per the documentation of `std::slice::sort` the type needs to implement `Ord` trait. The instructions for how to do this are in `std::cmp::Ord` documentation.

Then Sort by timestamp
