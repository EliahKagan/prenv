# prenv - Print environment variables

This is like `printenv` but with a few modifications:

- It also runs on Windows, natively.
- It sorts the environment variables case-sensitively by name on all platforms.
- It shows the environment variables as `NAME=value` pairs both when listing all of them (as `printenv` does) and when passing specific values.

This can be useful when comparing environment variables across platforms. I also find printing `NAME=value` to be more readable even if I passed the names.

This does not support options. In particular, this is mainly or interactive use rather than scripting, since it does not support `-0` and it will not output just the value of a variable (it always shows the name).

## License

[0BSD](LICENSE)
