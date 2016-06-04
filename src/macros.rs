// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `parse_error` macro returns a formated error.

#[macro_export]
macro_rules! from_error {
    ($msg: expr) => ({
        std::io::Error::new(std::io::ErrorKind::InvalidData, $msg)
    });
    ($msg: expr, $extra: expr) => ({
        from_error!(
            format!("{} - {}",
                $msg,
                $extra.iter()
                      .map(|a| format!("{}", a))
                      .collect::<Vec<String>>()
                      .concat()
            )
        )
    });
}
