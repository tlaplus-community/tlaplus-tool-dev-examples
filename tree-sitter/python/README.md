# Using tree-sitter-tlaplus from Python

## How to build & run this example

NOTE: this example will not work on linux until the tree-sitter pip package is updated to include [this fix](https://github.com/tree-sitter/py-tree-sitter/commit/204e41f1f1e316792dbb99b46fef6b7edec6862b).
As a workaround, you can uninstall libc++ before building & running the example (as is done in this repo's CI).
You can discuss & check the status of the release in [this](https://github.com/tree-sitter/py-tree-sitter/issues/69) issue.

1. Check out this repo with the `--recurse submodules` parameter, or if you've already cloned the repo run `git submodule init` and `git submodule update`
1. Install [Python 3](https://www.python.org/downloads/)
1. Ensure you have a C/C++ compiler installed
1. Run `pip install -r requirements.txt`
1. Run `python main.py`

## Adapting to your project

1. Clone the [tree-sitter-tlaplus](https://github.com/tlaplus-community/tree-sitter-tlaplus) repo to a well-known location (or use as a submodule), as the Python bindings consume the repo itself rather than a package
1. Look at the [py-tree-sitter](https://github.com/tree-sitter/py-tree-sitter) project for more info about the Python bindings
1. Look at the [Using Parsers](https://tree-sitter.github.io/tree-sitter/using-parsers) section of tree-sitter's documentation for more info on tree-sitter's API
