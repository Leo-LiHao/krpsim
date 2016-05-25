# Krpsim

[![travis-badge][]][travis] [![docs-badge][]][docs] [![license-badge][]][license]

[license-badge]: https://img.shields.io/badge/license-GPL_3-green.svg?style=flat-square
[license]: https://github.com/adjivas/krpsim/blob/master/README.md#license
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://adjivas.github.io/krpsim/krpsim
[travis-badge]: https://travis-ci.org/adjivas/krpsim.svg?branch=master&style=flat-square
[travis]: https://travis-ci.org/adjivas/krpsim

#### How to use
Example:
```shell
cargo run --bin krpsim -- --delay 1 --file resources/simple
cargo run --bin krpsim_verif -- --delay 1 --file resources/simple --result trace.krpsim.log
```
