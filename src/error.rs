use std::collections::HashMap;
use std::sync::Arc;

use arc_swap::ArcSwapOption;
use crossbeam_channel::select;
use lazy_static::lazy_static;

use crate::sender::submit;
use crate::Report;
use crate::SubmissionTarget;

lazy_static! {
    static ref SENDER: ArcSwapOption<crossbeam_channel::Sender<ErrorInfo>> =
        ArcSwapOption::from(None);
}

struct ErrorInfo {
    backtrace: backtrace::Backtrace,
    error: String,
}

pub fn init(
    token: &str,
    url: &str,
    annotations: Option<HashMap<String, String>>,
    attributes: Option<HashMap<String, String>>,
) {
    let (sender, receiver) = crossbeam_channel::bounded(124);
    SENDER.store(Some(Arc::new(sender)));

    let target = SubmissionTarget {
        token: String::from(token),
        url: String::from(url),
    };
    let mut report = Report {
        ..Default::default()
    };

    if let Some(annotations) = &annotations {
        report
            .annotations
            .extend(annotations.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    if let Some(attributes) = &attributes {
        report
            .attributes
            .extend(attributes.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    std::thread::spawn(move || {
        let recv = &receiver;
        loop {
            select! {
                recv(recv) -> error_info => {
                    match error_info {
                        Ok(error_info) => {
                            let mut report = report.clone();
                            let target = target.clone();

                            report
                                .attributes
                                .insert(String::from("error.message"), error_info.error.to_string());

                            submit(&target, &mut report, error_info.backtrace);
                        },
                        Err(e) => {
                            eprintln!("Failed to recv error info: {}", e);
                        }
                    }
                }
            }
        }
    });
}

pub trait ResultExt<T, E: std::fmt::Display> {
    fn submit_error(self) -> Result<T, E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn submit_error(self) -> Result<T, E> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                if let Some(sender) = SENDER.load_full() {
                    let error_info = ErrorInfo {
                        backtrace: backtrace::Backtrace::new(),
                        error: e.to_string(),
                    };

                    if sender.try_send(error_info).is_err() {
                        eprintln!("Failed to send data to channel");
                    }
                }
                Err(e)
            }
        }
    }
}
