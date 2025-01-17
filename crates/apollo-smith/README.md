 <div align="center">
   <h1><code>apollo-smith</code></h1>

   <p>
     <strong>A test case generator for GraphQL language.</strong>
   </p>
   <p>
     <a href="https://crates.io/crates/apollo-smith">
         <img src="https://img.shields.io/crates/v/apollo-smith.svg?style=flat-square" alt="Crates.io" />
     </a>
     <a href="https://crates.io/crates/apollo-smith">
         <img src="https://img.shields.io/crates/d/apollo-smith.svg?style=flat-square" alt="Download" />
     </a>
     <a href="https://docs.rs/apollo-smith/">
         <img src="https://img.shields.io/static/v1?label=docs&message=apollo-smith&color=blue&style=flat-square" alt="docs.rs docs" />
     </a>
   </p>
 </div>

## About
The goal of `apollo-smith` is to generate valid GraphQL documents by sampling
from all available possibilities of [GraphQL grammar].

We've written `apollo-smith` to use in fuzzing, but you may wish to use it for
anything that requires GraphQL document generation.

`apollo-smith` is inspired by bytecodealliance's [`wasm-smith`] crate, and the
[article written by Nick Fitzgerald] on writing test case generators in Rust.

This is still a work in progress, for outstanding issues, checkout out the
[apollo-smith label] in our issue tracker.
## Using `apollo-smith` with `cargo fuzz`

Define a new target with [`cargo fuzz`],

```shell
$ cargo fuzz add my_apollo_smith_fuzz_target
```

and add `apollo-smith` to your Cargo.toml:

```toml
## fuzz/Cargo.toml

[dependencies]
apollo-smith = "0.1.0"
```

It can then be used in a `fuzz_target` along with the [`arbitrary`] crate,

```rust,compile_fail
// fuzz/fuzz_targets/my_apollo_smith_fuzz_target.rs

#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Unstructured;
use apollo_smith::DocumentBuilder;

fuzz_target!(|input: &[u8]| {
    let mut u = Unstructured::new(input);
    let gql_doc = DocumentBuilder::new(&mut u)?;
    let document = gql_doc.finish();
    let document_str = String::from(document);


});
```

and fuzzed with the following command:

```shell
$ cargo +nightly fuzz run my_apollo_smith_fuzz_target
```

## Limitations
- Recursive object type not yet supported (example : `myType { inner: myType }`)

## License
Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

[GraphQL grammar]: https://spec.graphql.org/October2021/#sec-Appendix-Grammar-Summary
[`wasm-smith`]: https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-smith
[article written by Nick Fitzgerald]: https://fitzgeraldnick.com/2020/08/24/writing-a-test-case-generator.html#what-is-a-test-case-generator
[`arbitrary`]: https://docs.rs/arbitrary/latest/arbitrary/
[`cargo fuzz`]: https://github.com/rust-fuzz/cargo-fuzz
[apollo-smith label]: https://github.com/apollographql/apollo-rs/labels/apollo-smith