This repository contains simple benchmarks of various Rust serialization
libraries.

# Running

On Rust nightly run `cargo bench`.

# Results

```
test deserialize_manual                       ... bench:          52 ns/iter (+/- 6)
test deserialize_manual_foreign_endianness    ... bench:          52 ns/iter (+/- 1)
test deserialize_speedy                       ... bench:          79 ns/iter (+/- 2)
test deserialize_speedy_foreign_endianness    ... bench:          79 ns/iter (+/- 11)
test deserialize_serde_bincode                ... bench:         105 ns/iter (+/- 15)

test serialize_manual                         ... bench:          24 ns/iter (+/- 0)
test serialize_manual_foreign_endianness      ... bench:          24 ns/iter (+/- 0)
test serialize_speedy                         ... bench:          30 ns/iter (+/- 0)
test serialize_speedy_foreign_endianness      ... bench:          30 ns/iter (+/- 5)
test serialize_rmp                            ... bench:          93 ns/iter (+/- 13)
test serialize_serde_bincode                  ... bench:          95 ns/iter (+/- 3)
test serialize_prost                          ... bench:         105 ns/iter (+/- 2)
test serialize_serde_cbor                     ... bench:         177 ns/iter (+/- 5)
test serialize_serde_xdr                      ... bench:         197 ns/iter (+/- 6)
test serialize_serde_rmp                      ... bench:         223 ns/iter (+/- 7)
test serialize_serde_pickle                   ... bench:         246 ns/iter (+/- 9)
test serialize_serde_json                     ... bench:         462 ns/iter (+/- 14)
```
