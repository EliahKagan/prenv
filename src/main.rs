//! Show all environment variables, or particular ones.
//!
//! With no arguments, this prints all environment variables. If arguments are
//! passed, then they are glob patterns and only environment variables whose
//! names match at least one of the patterns are printed. In either case,
//! variables are printed in lexicographic order and never more than once.
//!
//! No options are recognized. Environment variable names may begin with `-` on
//! some systems, and even such arguments are taken to be patterns. Environment
//! variables are case-sensitive on many systems and at least case-aware on
//! most, but globbing is performed case-insensitively. Because this globbing
//! is for convenience rather than related to filenames, `.` and `/` are not
//! treated specially.

fn main() {

}
