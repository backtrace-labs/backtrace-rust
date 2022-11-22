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

### Example

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

## Manual Error Submission

We also offer a per error submission handler for logging
individual errors/exceptions.

This can be invoked by chaining the submit_error() method on any Result type.

You'll need to pass your token and url to an init method first before error 
submission works.

### Report Modification 

You can optionally pass in Option wrapped HashMaps of the attributes
and annotations you wish to ammend to your report. the per error handler
will handle ammending them to the report before submitting it.

### Usage

```rust
use std::collections::HashMap;
use std::fs::File;

use backtraceio::ResultExt;

fn main() {
    
    let attributes: HashMap<String, String> = HashMap::new();
    let cpus = num_cpus::get();
    let cpus = cpus.to_string();
    attributes.insert(String::from("cpu.cores"), cpus);

    backtraceio::init("<token>", "<url>", None, Some(attributes.clone()));

    match std::fs::File::open("./this_file_does_not_exist").submit_error()
    {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }

}
```

[1]: https://api.backtrace.io/#tag/submit-crash
