# timezone_converter

convert time input to whatever timezone.

## usage

1. `$ cargo build --release`

then, execute `timezone_converter`

```
USAGE:
    timezone_converter [OPTIONS] [TIME]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fromtz <fromtz>
    -t, --totz <totz>

ARGS:
    <TIME>
```

### example

`$ timezone_converter -f "Asia/Tokyo" -t "Europe/London" "2019/12/07 19:31:28"`
