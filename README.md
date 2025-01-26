# open-cypher

Parse [openCypher](https://opencypher.org/) and [ISO/IEC 39075:2024 (GQL)](https://www.iso.org/standard/76120.html) queries using Rust.

This crate uses the [pest](https://pest.rs/) library to parse queries using the grammars. For Cypher, the grammar `src/cypher.pest` is based on he openCypher EBNF file ([link](https://opencypher.org/resources/) or [file](assets/cypher.ebnf)). For GQL, the grammar `src/ISO_IEC_39075_2024_GQL.pest` is derived from the EBNF file ([link](https://github.com/zmajeed/ebnfparser/blob/main/docs/gqlgrammar.quotedliterals.txt) or [file](assets/gqlgrammar.quotedliterals.txt)).


## Project Status

The library is still at a pre-alpha stage. The generated Cypher parser is somewhat tested with queries harvested from the [TCK](https://github.com/opencypher/openCypher/tree/main/tck). The GQL parser is not tested.

I've just started working on a unifying AST between the two, expect what's there to change quite a bit.

The long-term goal for this project is to be a full-fledged parser for Cypher as well as GQL that can be combined with a graph database implementation.

## Contributing

Contributions of any size are more than welcome! Please feel free to submit issues or PRs.

## Licensing
This crate is licensed under the Rust-typical dual license of MIT or Apache2, at your choice.

The following files have particular licenses:
- [assets/gqlgrammar.quotedliterals.txt]: MIT only license
- [assets/cypher.ebnf]: Apache2 only license
- [src/cypher.pest]: MIT only license
