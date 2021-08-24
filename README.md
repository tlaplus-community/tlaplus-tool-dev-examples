# Overview
The purpose of this repo is to provide a set of simple examples to get you up & running writing your own development tools for TLA+.
Currently, this repo only contains examples for consuming the [tree-sitter-tlaplus](https://github.com/tlaplus-community/tree-sitter-tlaplus) project; see the [tree-sitter](tree-sitter) directory.

## Contributing
If you would like to contribute your own example, open a pull request adding it to the relevant `$project_name/$language_name` directory and modify the `.github/workflows/ci.yml` file with instructions to compile & run it.
The CI must pass on all supported systems before your example can be merged.
