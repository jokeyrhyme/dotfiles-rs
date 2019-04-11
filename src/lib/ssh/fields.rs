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
    S: AsRef<str>,
{
    fn from(source: S) -> Self {
        let s = source.as_ref();
        match i32::from_str_radix(&s, 10) {
            Ok(n) => Duration::Seconds(n),
            Err(_) => {
                let re = regex::Regex::new(r"^(\d+[smhdwSMHDW])+$").unwrap();
                if re.is_match(&s) {
                    Duration::Time(String::from(s))
                } else {
                    Duration::Time(String::new())
                }
            }
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
    S: AsRef<str>,
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
    S: AsRef<str>,
{
    fn from(source: S) -> Self {
        let s = source.as_ref();
        match s {
            YES | NO => YesNoAsk::YesNo(YesNo::from(s)),
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
    S: AsRef<str>,
{
    fn from(source: S) -> Self {
        let s = source.as_ref();
        match s {
            YES | NO | ASK => YesNoAskAutoAutoAsk::YesNoAsk(YesNoAsk::from(s)),
            AUTO => YesNoAskAutoAutoAsk::Auto,
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
    S: AsRef<str>,
{
    fn from(source: S) -> Self {
        let s = source.as_ref();
        match s {
            YES | NO => YesNoDuration::YesNo(YesNo::from(s)),
            _ => YesNoDuration::Duration(Duration::from(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_to_from_str() {
        let cases = &[
            ("", Duration::Time(String::new())),
            ("30", Duration::Seconds(30)), // invalid, wrong token order
            ("s30", Duration::Time(String::new())), // invalid, unknown unit token
            ("30x", Duration::Time(String::new())),
            ("30s", Duration::Time(String::from("30s"))),
            ("1m30s", Duration::Time(String::from("1m30s"))),
        ];
        for c in cases {
            let got = Duration::from(c.0);
            assert_eq!(c.1, got);
            let reverse = format!("{}", got);
            if !c.0.is_empty() && !reverse.is_empty() {
                assert_eq!(c.0, reverse);
            }
        }
    }

    #[test]
    fn yesno_to_from_str() {
        let cases = &[("", YesNo::No), ("no", YesNo::No), ("yes", YesNo::Yes)];
        for c in cases {
            let got = YesNo::from(c.0);
            assert_eq!(c.1, got);
            let reverse = format!("{}", got);
            if !c.0.is_empty() && !reverse.is_empty() {
                assert_eq!(c.0, reverse);
            }
        }
    }

    #[test]
    fn yesnoask_to_from_str() {
        let cases = &[
            ("", YesNoAsk::Ask),
            ("no", YesNoAsk::YesNo(YesNo::No)),
            ("yes", YesNoAsk::YesNo(YesNo::Yes)),
            ("ask", YesNoAsk::Ask),
        ];
        for c in cases {
            let got = YesNoAsk::from(c.0);
            assert_eq!(c.1, got);
            let reverse = format!("{}", got);
            if !c.0.is_empty() && !reverse.is_empty() {
                assert_eq!(c.0, reverse);
            }
        }
    }

    #[test]
    fn yesnoaskautoautoask_to_from_str() {
        let cases = &[
            ("", YesNoAskAutoAutoAsk::AutoAsk),
            (
                "no",
                YesNoAskAutoAutoAsk::YesNoAsk(YesNoAsk::YesNo(YesNo::No)),
            ),
            (
                "yes",
                YesNoAskAutoAutoAsk::YesNoAsk(YesNoAsk::YesNo(YesNo::Yes)),
            ),
            ("ask", YesNoAskAutoAutoAsk::YesNoAsk(YesNoAsk::Ask)),
            ("auto", YesNoAskAutoAutoAsk::Auto),
            ("autoask", YesNoAskAutoAutoAsk::AutoAsk),
        ];
        for c in cases {
            let got = YesNoAskAutoAutoAsk::from(c.0);
            assert_eq!(c.1, got);
            let reverse = format!("{}", got);
            if !c.0.is_empty() && !reverse.is_empty() {
                assert_eq!(c.0, reverse);
            }
        }
    }

    #[test]
    fn yesnoduration_to_from_str() {
        let cases = &[
            ("", YesNoDuration::Duration(Duration::Time(String::new()))),
            ("no", YesNoDuration::YesNo(YesNo::No)),
            ("yes", YesNoDuration::YesNo(YesNo::Yes)),
            ("0", YesNoDuration::Duration(Duration::Seconds(0))),
            ("1", YesNoDuration::Duration(Duration::Seconds(1))),
            ("30", YesNoDuration::Duration(Duration::Seconds(30))),
            (
                "30m",
                YesNoDuration::Duration(Duration::Time(String::from("30m"))),
            ),
        ];
        for c in cases {
            let got = YesNoDuration::from(c.0);
            assert_eq!(c.1, got);
            let reverse = format!("{}", got);
            if !c.0.is_empty() && !reverse.is_empty() {
                assert_eq!(c.0, reverse);
            }
        }
    }

}
