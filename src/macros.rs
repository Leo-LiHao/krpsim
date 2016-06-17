// @lecorref - github.com/lecorref, @geam - github@geam,
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/krpsim
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The default module `Macros` adds new macro of krpsim's library.

/// The `println_stderr` macro writes an error message.

#[macro_export]
macro_rules! println_stderr (
    ($($arg: tt)*) => {{
        writeln!(
            &mut std::io::stderr(),
            $($arg)*,
        ).expect("failed printing to stderr");
    }}
);

/// The `parse_error` macro returns a formated error.

#[macro_export]
macro_rules! from_error {
    ($msg: expr) => ({Err(
        std::io::Error::new(std::io::ErrorKind::InvalidData, $msg)
    )});
    ($msg: expr, $extra: expr) => ({
        from_error!($msg.replace("{}", $extra))
    });
}
