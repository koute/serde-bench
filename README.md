This repository contains simple benchmarks of various Rust serialization
libraries.

# Running

On Rust nightly run `cargo bench`.

# Results

```
test deserialize_manual                    ... bench:         154 ns/iter (+/- 22)
test deserialize_manual_foreign_endianness ... bench:         156 ns/iter (+/- 14)
test deserialize_serde_bincode             ... bench:         238 ns/iter (+/- 15)
test deserialize_speedy                    ... bench:         180 ns/iter (+/- 12)
test deserialize_speedy_foreign_endianness ... bench:         182 ns/iter (+/- 14)

test serialize_manual                      ... bench:          77 ns/iter (+/- 3)
test serialize_manual_foreign_endianness   ... bench:          76 ns/iter (+/- 3)
test serialize_prost                       ... bench:         105 ns/iter (+/- 8)
test serialize_rmp                         ... bench:          90 ns/iter (+/- 10)
test serialize_serde_bincode               ... bench:          87 ns/iter (+/- 6)
test serialize_serde_cbor                  ... bench:         162 ns/iter (+/- 21)
test serialize_serde_json                  ... bench:         398 ns/iter (+/- 37)
test serialize_serde_pickle                ... bench:         132 ns/iter (+/- 5)
test serialize_serde_rmp                   ... bench:         170 ns/iter (+/- 12)
test serialize_serde_xdr                   ... bench:         119 ns/iter (+/- 16)
test serialize_speedy                      ... bench:          84 ns/iter (+/- 6)
test serialize_speedy_box                  ... bench:          76 ns/iter (+/- 6)
test serialize_speedy_foreign_endianness   ... bench:          83 ns/iter (+/- 9)
```
