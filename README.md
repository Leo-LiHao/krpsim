# Krpsim

[![travis-badge][]][travis] [![license-badge][]][license] [![docs-badge][]][docs] 

[license-badge]: https://img.shields.io/badge/license-GPL_3-green.svg?style=flat-square
[license]: https://github.com/adjivas/krpsim/blob/master/README.md#license
[travis-badge]: https://travis-ci.org/adjivas/krpsim.svg?branch=master&style=flat-square
[travis]: https://travis-ci.org/adjivas/krpsim
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://adjivas.github.io/krpsim/krpsim

#### Usage
```
Krpsim
francois LE CORRE <lecorre.f@gmail.com>, geam <geam@users.noreply.github.com>, adjivas <adjivas@users.noreply.github.com>
Krpsim

USAGE:
    krpsim [FLAGS] [OPTIONS] --file <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Defines a output with more informations.

OPTIONS:
    -d, --delay <delay>                      Defines the delay.
    -f, --file <file>                        Defines the input's files who contain the stock, process and optimization.
    -r, --result_to_test <result_to_test>    Defines the log result's of output
```

##### How to use
Example:
```shell
cargo run --bin krpsim -- --delay 100 --file resources/simple
cargo run --bin krpsim_verif -- --delay 100 --file resources/simple --result_to_test resources/simple.krpsim.log
```
#### UML-Language
![Screen Shot](https://raw.githubusercontent.com/adjivas/krpsim/notes/uml.png)

#### Graph-Dependency
Many thanks goes to:
![Screen Shot](https://raw.githubusercontent.com/adjivas/krpsim/notes/cargo.png)

#### License
*Krpsim*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://raw.githubusercontent.com/adjivas/krpsim    /master/LICENSE).
