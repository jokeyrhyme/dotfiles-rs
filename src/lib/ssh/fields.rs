use std::{
    fmt::{self, Display},
    i32,
};

use regex;

const ASK: &str = "ask";
const AUTO: &str = "auto";
const AUTOASK: &str = "autoask";
const NO: &str = "no";
const YES: &str = "yes";

// Duration is for "TIME FORMATS" fields, see `man sshd_config(5)`
#[derive(Clone, Debug, PartialEq)]
pub enum Duration {
    Seconds(i32), // 30 => 30 seconds
    Time(String), // "30m" => 30 minutes, etc
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Duration::Seconds(d) => write!(f, "{}", d),
            Duration::Time(d) => write!(f, "{}", d),
        }
    }
}

impl<S> From<S> for Duration
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match i32::from_str_radix(source.as_ref(), 10) {
            Ok(n) => Duration::Seconds(n),
            Err(_) => {
                let re = regex::Regex::new(r"^(\d+[smhdwSMHDW])+$").unwrap();
                if re.is_match(source.as_ref()) {
                    Duration::Time(String::from(source.as_ref()))
                } else {
                    Duration::Time(String::new())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_into_duration() {
        struct TestCase<'a> {
            input: &'a str,
            want: Duration,
        }
        let cases = &[
            TestCase {
                input: "",
                want: Duration::Time(String::new()),
            },
            TestCase {
                input: "30",
                want: Duration::Seconds(30),
            },
            TestCase {
                input: "s30", // invalid, wrong token order
                want: Duration::Time(String::new()),
            },
            TestCase {
                input: "30x", // invalid, unknown unit token
                want: Duration::Time(String::new()),
            },
            TestCase {
                input: "30s",
                want: Duration::Time(String::from("30s")),
            },
            TestCase {
                input: "1m30s",
                want: Duration::Time(String::from("1m30s")),
            },
        ];
        for c in cases {
            let got = Duration::from(c.input);
            assert_eq!(c.want, got);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum YesNo {
    Yes,
    No,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            YesNo::Yes => write!(f, "{}", YES),
            YesNo::No => write!(f, "{}", NO),
        }
    }
}

impl<S> From<S> for YesNo
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            YES | "1" | "true" => YesNo::Yes,
            _ => YesNo::No,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum YesNoAsk {
    YesNo(YesNo),
    Ask,
}

impl Display for YesNoAsk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            YesNoAsk::YesNo(yn) => write!(f, "{}", yn),
            YesNoAsk::Ask => write!(f, "{}", ASK),
        }
    }
}

impl<S> From<S> for YesNoAsk
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            YES | NO => YesNoAsk::YesNo(YesNo::from(source)),
            _ => YesNoAsk::Ask,
        }
    }
}

impl<'a> From<&'a YesNoAsk> for String {
    fn from(source: &YesNoAsk) -> String {
        format!("{}", source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum YesNoAskAutoAutoAsk {
    YesNoAsk(YesNoAsk),
    Auto,
    AutoAsk,
}

impl Display for YesNoAskAutoAutoAsk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            YesNoAskAutoAutoAsk::YesNoAsk(ynaaa) => write!(f, "{}", ynaaa),
            YesNoAskAutoAutoAsk::Auto => write!(f, "{}", AUTO),
            YesNoAskAutoAutoAsk::AutoAsk => write!(f, "{}", AUTOASK),
        }
    }
}

impl<S> From<S> for YesNoAskAutoAutoAsk
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            YES | NO | ASK => YesNoAskAutoAutoAsk::YesNoAsk(YesNoAsk::from(source)),
            _ => YesNoAskAutoAutoAsk::AutoAsk,
        }
    }
}

impl<'a> From<&'a YesNoAskAutoAutoAsk> for String {
    fn from(source: &YesNoAskAutoAutoAsk) -> String {
        match source {
            YesNoAskAutoAutoAsk::YesNoAsk(ynaaa) => String::from(ynaaa),
            YesNoAskAutoAutoAsk::Auto => String::from(AUTO),
            YesNoAskAutoAutoAsk::AutoAsk => String::from(AUTOASK),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum YesNoDuration {
    YesNo(YesNo),
    Duration(Duration),
}

impl Display for YesNoDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            YesNoDuration::YesNo(ynd) => write!(f, "{}", ynd),
            YesNoDuration::Duration(ynd) => write!(f, "{}", ynd),
        }
    }
}

impl<S> From<S> for YesNoDuration
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            YES | NO => YesNoDuration::YesNo(YesNo::from(source)),
            _ => YesNoDuration::Duration(Duration::from(source)),
        }
    }
}
