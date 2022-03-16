# Backtrace error submission crate

# Installation
```
[dependencies]
backtraceio = "0.1"
```

# Usage

## Global error handler

Pass your custom token and upload url from your Backtrace account and a report
modification closure/function to the `backtraceio::register_error_handler`
function.

```
backtraceio::register_error_handler(
    "https://UNIVERSE.sp.backtrace.io:6098",
    "YOURTOKEN",
    closure
);

```

### `Report` modification function
The error handler will pass the `Report` and `std::panic::PanicInfo` objects
back to the user, in case there are additional attributes/annotations to be
defined (described more in detail [here][1]). It should accept `&mut Report`,
`&PanicInfo`, making any changes desired to the report.

# Example

```rust
use backtraceio::Report;

fn main() {
    backtraceio::register_error_handler(
        "https://UNIVERSE.sp.backtrace.io:6098",
        "YOUR_TOKEN",
        |r: &mut Report, _| {
            let cpus = num_cpus::get();
            let cpus = cpus.to_string();
            r.attributes.insert(String::from("cpu.cores"), cpus);
        },
    );

    println!("Hello, world!");
    panic!("{:?}", 69);
}

```

[1]: https://api.backtrace.io/#tag/submit-crash
