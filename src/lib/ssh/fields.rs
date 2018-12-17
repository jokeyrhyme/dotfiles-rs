use std::{
    fmt::{self, Display},
    i32,
    path::PathBuf,
};

use regex;

const ANY: &str = "any";
const ASK: &str = "ask";
const AUTO: &str = "auto";
const AUTOASK: &str = "autoask";
const CONFIRM: &str = "confirm";
const INET: &str = "inet";
const INET6: &str = "inet6";
const MD5: &str = "md5";
const NO: &str = "no";
const SHA256: &str = "sha256";
const YES: &str = "yes";

#[derive(Clone, Debug, PartialEq)]
pub enum AnyInetInet6 {
    Any,
    Inet,
    Inet6,
}

impl Display for AnyInetInet6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AnyInetInet6::Any => write!(f, "{}", ANY),
            AnyInetInet6::Inet => write!(f, "{}", INET),
            AnyInetInet6::Inet6 => write!(f, "{}", INET6),
        }
    }
}

impl<S> From<S> for AnyInetInet6
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            INET => AnyInetInet6::Inet,
            INET6 => AnyInetInet6::Inet6,
            _ => AnyInetInet6::Any,
        }
    }
}

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

pub enum Field {
    AddKeysToAgent(YesNoAskConfirm),
    AddressFamily(AnyInetInet6),
    BatchMode(YesNo),
    BindAddress(YesNo),
    BindInterface(YesNo),
    CanonicalDomains(YesNo),
    CanonicalizeFallbackLocal(YesNo),
    CanonicalizeHostname(YesNo),
    CanonicalizeMaxDots(YesNo),
    CanonicalisePermittedCNAMEs(String),
    CertificateFile(PathBuf),
    ChallengeResponseAuthentication(YesNo),
    CheckHostIP(YesNo),
    Ciphers(Vec<String>),
    ClearAllForwardings(YesNo),
    Compression(YesNo),
    ConnectionAttempts(i32),
    ConnectTimeout(i32),
    ControlMaster(YesNoAskAutoAutoAsk),
    ControlPath(PathBuf),
    ControlPersist(YesNoDuration),
    DynamicForward(String),
    EnableSSHKeysing(YesNo),
    EscapeChar(String), // character
    ExitOnForwardFailure(YesNo),
    FingerprintHash(Md5Sha256),
    ForwardAgent(YesNo),
    ForwardX11(YesNo),
    ForwardX11Timeout(Duration),
    ForwardX11Trusted(YesNo),
    GatewayPorts(YesNo),
    GlobalKnownHostsFile(PathBuf),
    GSSAPIAuthentication(YesNo),
    GSSAPIDelegateCredentials(YesNo),
    HashKnownHosts(YesNo),
    HostbasedAuthentication(YesNo),
    HostbasedKeyTypes(Vec<String>),
    HostKeyAlgorithms(Vec<String>),
    HostKeyAlias(String),
    Hostname(String),
    IdentitiesOnly(YesNo),
    IdentityAgent(PathBuf),
    IdentityFile(PathBuf),
    IgnoreUnknown(String),
    Include(Vec<PathBuf>),
    IPQoS(String),
    KbdInteractiveAuthentication(YesNo),
    KbdInteractiveDevices(Vec<String>),
    KexAlgorithms(Vec<String>),
    LocalCommand(String),
    LocalForward(String),
    LogLevel(String),
    MACs(Vec<String>),
    NoHostAuthenticationForLocalhost(YesNo),
    NumberOfPasswordPrompts(i32),
    PasswordAuthentication(YesNo),
    PermitLocalCommand(YesNo),
    PKCS11Provider(String),
    Port(i32),
    PreferredAuthentications(Vec<String>),
    ProxyCommand(String),
    ProxyJump(String),
    ProxyUseFdpass(YesNo),
    PubkeyAcceptedKeyTypes(Vec<String>),
    PubkeyAuthentication(YesNo),
    RekeyLimit(String), // fmt: size [duration]
    RemoteCommand(String),
    RemoteForward(String),
    RequestTTY(String), // enum: no, yes, force, auto
    RevokedHostKeys(PathBuf),
    RhostsRSAAuthentication(YesNo),
    RSAAuthentication(YesNo),
    SendEnv(String),
    ServerAliveCountMax(i32),
    ServerAliveInterval(i32),
    StreamLocalBindMask(YesNo),
    StreamLocalBindUnlink(YesNo),
    StrictHostKeyChecking(YesNoAsk),
    SyslogFacility(String),
    TCPKeepAlive(YesNo),
    Tunnel(String), // enum: yes, point-to-point, ethernet, no
    TunnelDevice(String),
    UpdateHostKeys(YesNoAsk),
    UsePrivilegedPort(YesNo),
    User(String),
    UserKnownHostsFile(PathBuf),
    VerifyHostKeyDNS(YesNoAsk),
    VisualHostKey(YesNo),
    XAuthLocation(PathBuf),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Md5Sha256 {
    Md5,
    Sha256,
}

impl Display for Md5Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Md5Sha256::Md5 => write!(f, "{}", MD5),
            Md5Sha256::Sha256 => write!(f, "{}", SHA256),
        }
    }
}

impl<S> From<S> for Md5Sha256
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            MD5 => Md5Sha256::Md5,
            _ => Md5Sha256::Sha256,
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
pub enum YesNoAskConfirm {
    YesNoAsk(YesNoAsk),
    Confirm,
}

impl Display for YesNoAskConfirm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            YesNoAskConfirm::YesNoAsk(ynac) => write!(f, "{}", ynac),
            YesNoAskConfirm::Confirm => write!(f, "{}", CONFIRM),
        }
    }
}

impl<S> From<S> for YesNoAskConfirm
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        match source.as_ref() {
            YES | NO | ASK => YesNoAskConfirm::YesNoAsk(YesNoAsk::from(source)),
            _ => YesNoAskConfirm::Confirm,
        }
    }
}

impl<'a> From<&'a YesNoAskConfirm> for String {
    fn from(source: &YesNoAskConfirm) -> String {
        match source {
            YesNoAskConfirm::YesNoAsk(ynaaa) => String::from(ynaaa),
            YesNoAskConfirm::Confirm => String::from(CONFIRM),
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
