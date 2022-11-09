#[macro_use]
extern crate serde_json;

mod error;
mod sender;

use std::collections::HashMap;
use std::panic::PanicInfo;

pub use error::init;
pub use error::ResultExt;

#[derive(Debug, Clone)]
pub struct SubmissionTarget {
    token: String,
    url: String,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub annotations: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
}

pub fn register_error_handler<T>(url: &str, token: &str, user_handler: T)
where
    T: Fn(&mut Report, &PanicInfo) -> () + Send + Sync + 'static,
{
    let submission_target = SubmissionTarget {
        token: String::from(token),
        url: String::from(url),
    };

    std::panic::set_hook(Box::new(move |panic_info| {
        let mut r = Report {
            ..Default::default()
        };

        user_handler(&mut r, panic_info);

        let bt = backtrace::Backtrace::new();

        sender::submit(&submission_target, &mut r, bt);
    }));
}
