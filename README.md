# Krpsim

[![travis-badge][]][travis] [![license-badge][]][license] [![docs-badge][]][docs] 

[license-badge]: https://img.shields.io/badge/license-GPL_3-green.svg?style=flat-square
[license]: https://github.com/adjivas/krpsim/blob/master/README.md#license
[travis-badge]: https://travis-ci.org/adjivas/krpsim.svg?branch=master&style=flat-square
[travis]: https://travis-ci.org/adjivas/krpsim
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://adjivas.github.io/krpsim/krpsim

#### How to use
Example:
```shell
cargo run --bin krpsim -- --delay 1 --file resources/simple
cargo run --bin krpsim_verif -- --delay 1 --file resources/simple --result_to_test resources/simple.krpsim.log
```
