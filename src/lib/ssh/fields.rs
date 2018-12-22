use std::{
    fmt::{self, Display},
    i32,
    iter::Iterator,
    path::PathBuf,
};

use regex;

use crate::lib::ssh::consts;

const INDENT: &str = "  ";

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

pub struct Config {
    pub fields: Vec<RootField>,
}

impl<S> From<S> for Config
where
    S: Into<String> + AsRef<str>,
{
    fn from(_s: S) -> Self {
        Config {
            // TODO:
            fields: Vec::<RootField>::new(),
        }
    }
}

impl From<Config> for String {
    fn from(_c: Config) -> Self {
        String::new() // TODO:
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

#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    Comment(String),
    EmptyLine,

    AddKeysToAgent(YesNoAskConfirm),
    AddressFamily(AnyInetInet6),
    BatchMode(YesNo),
    BindAddress(YesNo),
    BindInterface(YesNo),
    CanonicalDomains(YesNo),
    CanonicalizeFallbackLocal(YesNo),
    CanonicalizeHostname(YesNo),
    CanonicalizeMaxDots(YesNo),
    CanonicalizePermittedCNAMEs(String),
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
    EnableSSHKeysign(YesNo),
    EscapeChar(char),
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

impl<S> From<S> for Field
where
    S: Into<String> + AsRef<str>,
{
    // input should be one line, we truncate the rest
    fn from(s: S) -> Self {
        let trimmed = s.as_ref().lines().next().unwrap_or("").trim();
        if trimmed.is_empty() {
            return Field::EmptyLine;
        }

        let mut chars = trimmed.chars();
        if chars.next().unwrap_or('#') == '#' {
            return Field::Comment(String::from(chars.as_str().trim()));
        }

        let mut split = trimmed.splitn(2, ' ');
        let key = split.next().unwrap();
        let value = String::from(split.next().unwrap_or(""));

        match key {
            consts::ADDKEYSTOAGENT => Field::AddKeysToAgent(YesNoAskConfirm::from(value)),
            consts::ADDRESSFAMILY => Field::AddressFamily(AnyInetInet6::from(value)),
            consts::BATCHMODE => Field::BatchMode(YesNo::from(value)),
            consts::BINDADDRESS => Field::BindAddress(YesNo::from(value)),
            consts::BINDINTERFACE => Field::BindInterface(YesNo::from(value)),
            consts::CANONICALDOMAINS => Field::CanonicalDomains(YesNo::from(value)),
            consts::CANONICALIZEFALLBACKLOCAL => {
                Field::CanonicalizeFallbackLocal(YesNo::from(value))
            }
            consts::CANONICALIZEHOSTNAME => Field::CanonicalizeHostname(YesNo::from(value)),
            consts::CANONICALIZEMAXDOTS => Field::CanonicalizeMaxDots(YesNo::from(value)),
            consts::CANONICALIZEPERMITTEDCNAMES => {
                Field::CanonicalizePermittedCNAMEs(String::from(value))
            }
            consts::CERTIFICATEFILE => Field::CertificateFile(PathBuf::from(value)),
            consts::CHALLENGERESPONSEAUTHENTICATION => {
                Field::ChallengeResponseAuthentication(YesNo::from(value))
            }
            consts::CHECKHOSTIP => Field::CheckHostIP(YesNo::from(value)),
            consts::CIPHERS => Field::Ciphers(parse_strings(value)),
            consts::CLEARALLFORWARDINGS => Field::ClearAllForwardings(YesNo::from(value)),
            consts::COMPRESSION => Field::Compression(YesNo::from(value)),
            consts::CONNECTIONATTEMPTS => {
                Field::ConnectionAttempts(parse_number(value).unwrap_or_default())
            }
            consts::CONNECTTIMEOUT => {
                Field::ConnectTimeout(parse_number(value).unwrap_or_default())
            }
            consts::CONTROLMASTER => Field::ControlMaster(YesNoAskAutoAutoAsk::from(value)),
            consts::CONTROLPATH => Field::ControlPath(PathBuf::from(value)),
            consts::CONTROLPERSIST => Field::ControlPersist(YesNoDuration::from(value)),
            consts::DYNAMICFORWARD => Field::DynamicForward(String::from(value)),
            consts::ENABLESSHKEYSIGN => Field::EnableSSHKeysign(YesNo::from(value)),
            consts::ESCAPECHAR => Field::EscapeChar(parse_char(value)),
            consts::EXITONFORWARDFAILURE => Field::ExitOnForwardFailure(YesNo::from(value)),
            consts::FINGERPRINTHASH => Field::FingerprintHash(Md5Sha256::from(value)),
            consts::FORWARDAGENT => Field::ForwardAgent(YesNo::from(value)),
            consts::FORWARDX11 => Field::ForwardX11(YesNo::from(value)),
            consts::FORWARDX11TIMEOUT => Field::ForwardX11Timeout(Duration::from(value)),
            consts::FORWARDX11TRUSTED => Field::ForwardX11Trusted(YesNo::from(value)),
            consts::GATEWAYPORTS => Field::GatewayPorts(YesNo::from(value)),
            consts::GLOBALKNOWNHOSTSFILE => Field::GlobalKnownHostsFile(PathBuf::from(value)),
            consts::GSSAPIAUTHENTICATION => Field::GSSAPIAuthentication(YesNo::from(value)),
            consts::GSSAPIDELEGATECREDENTIALS => {
                Field::GSSAPIDelegateCredentials(YesNo::from(value))
            }
            consts::HASHKNOWNHOSTS => Field::HashKnownHosts(YesNo::from(value)),
            consts::HOSTBASEDAUTHENTICATION => Field::HostbasedAuthentication(YesNo::from(value)),
            consts::HOSTBASEDKEYTYPES => Field::HostbasedKeyTypes(parse_strings(value)),
            consts::HOSTKEYALGORITHMS => Field::HostKeyAlgorithms(parse_strings(value)),
            consts::HOSTKEYALIAS => Field::HostKeyAlias(String::from(value)),
            consts::HOSTNAME => Field::Hostname(String::from(value)),
            consts::IDENTITIESONLY => Field::IdentitiesOnly(YesNo::from(value)),
            consts::IDENTITYAGENT => Field::IdentityAgent(PathBuf::from(value)),
            consts::IDENTITYFILE => Field::IdentityFile(PathBuf::from(value)),
            consts::IGNOREUNKNOWN => Field::IgnoreUnknown(String::from(value)),
            consts::INCLUDE => Field::Include(parse_pathbufs(value)),
            consts::IPQOS => Field::IPQoS(String::from(value)),
            consts::KBDINTERACTIVEAUTHENTICATION => {
                Field::KbdInteractiveAuthentication(YesNo::from(value))
            }
            consts::KBDINTERACTIVEDEVICES => Field::KbdInteractiveDevices(parse_strings(value)),
            consts::KEXALGORITHMS => Field::KexAlgorithms(parse_strings(value)),
            consts::LOCALCOMMAND => Field::LocalCommand(String::from(value)),
            consts::LOCALFORWARD => Field::LocalForward(String::from(value)),
            consts::LOGLEVEL => Field::LogLevel(String::from(value)),
            consts::MACS => Field::MACs(parse_strings(value)),
            consts::NOHOSTAUTHENTICATIONFORLOCALHOST => {
                Field::NoHostAuthenticationForLocalhost(YesNo::from(value))
            }
            consts::NUMBEROFPASSWORDPROMPTS => {
                Field::NumberOfPasswordPrompts(parse_number(value).unwrap_or_default())
            }
            consts::PASSWORDAUTHENTICATION => Field::PasswordAuthentication(YesNo::from(value)),
            consts::PERMITLOCALCOMMAND => Field::PermitLocalCommand(YesNo::from(value)),
            consts::PKCS11PROVIDER => Field::PKCS11Provider(String::from(value)),
            consts::PORT => Field::Port(parse_number(value).unwrap_or_default()),
            consts::PREFERREDAUTHENTICATIONS => {
                Field::PreferredAuthentications(parse_strings(value))
            }
            consts::PROXYCOMMAND => Field::ProxyCommand(String::from(value)),
            consts::PROXYJUMP => Field::ProxyJump(String::from(value)),
            consts::PROXYUSEFDPASS => Field::ProxyUseFdpass(YesNo::from(value)),
            consts::PUBKEYACCEPTEDKEYTYPES => Field::PubkeyAcceptedKeyTypes(parse_strings(value)),
            consts::PUBKEYAUTHENTICATION => Field::PubkeyAuthentication(YesNo::from(value)),
            consts::REKEYLIMIT => Field::RekeyLimit(String::from(value)),
            consts::REMOTECOMMAND => Field::RemoteCommand(String::from(value)),
            consts::REMOTEFORWARD => Field::RemoteForward(String::from(value)),
            consts::REQUESTTTY => Field::RequestTTY(String::from(value)),
            consts::REVOKEDHOSTKEYS => Field::RevokedHostKeys(PathBuf::from(value)),
            consts::RHOSTSRSAAUTHENTICATION => Field::RhostsRSAAuthentication(YesNo::from(value)),
            consts::RSAAUTHENTICATION => Field::RSAAuthentication(YesNo::from(value)),
            consts::SENDENV => Field::SendEnv(String::from(value)),
            consts::SERVERALIVECOUNTMAX => {
                Field::ServerAliveCountMax(parse_number(value).unwrap_or_default())
            }
            consts::SERVERALIVEINTERVAL => {
                Field::ServerAliveInterval(parse_number(value).unwrap_or_default())
            }
            consts::STREAMLOCALBINDMASK => Field::StreamLocalBindMask(YesNo::from(value)),
            consts::STREAMLOCALBINDUNLINK => Field::StreamLocalBindUnlink(YesNo::from(value)),
            consts::STRICTHOSTKEYCHECKING => Field::StrictHostKeyChecking(YesNoAsk::from(value)),
            consts::SYSLOGFACILITY => Field::SyslogFacility(String::from(value)),
            consts::TCPKEEPALIVE => Field::TCPKeepAlive(YesNo::from(value)),
            consts::TUNNEL => Field::Tunnel(String::from(value)),
            consts::TUNNELDEVICE => Field::TunnelDevice(String::from(value)),
            consts::UPDATEHOSTKEYS => Field::UpdateHostKeys(YesNoAsk::from(value)),
            consts::USEPRIVILEGEDPORT => Field::UsePrivilegedPort(YesNo::from(value)),
            consts::USER => Field::User(String::from(value)),
            consts::USERKNOWNHOSTSFILE => Field::UserKnownHostsFile(PathBuf::from(value)),
            consts::VERIFYHOSTKEYDNS => Field::VerifyHostKeyDNS(YesNoAsk::from(value)),
            consts::VISUALHOSTKEY => Field::VisualHostKey(YesNo::from(value)),
            consts::XAUTHLOCATION => Field::XAuthLocation(PathBuf::from(value)),
            _ => Field::EmptyLine,
        }
    }
}

impl From<&Field> for String {
    fn from(f: &Field) -> Self {
        match f {
            Field::Comment(s) => format!("# {}", s),
            Field::AddKeysToAgent(ynac) => format!("{} {}", consts::ADDKEYSTOAGENT, ynac),
            Field::AddressFamily(aii) => format!("{} {}", consts::ADDRESSFAMILY, aii),
            Field::BatchMode(yn) => format!("{} {}", consts::BATCHMODE, yn),
            Field::BindAddress(yn) => format!("{} {}", consts::BINDADDRESS, yn),
            Field::BindInterface(yn) => format!("{} {}", consts::BINDINTERFACE, yn),
            Field::CanonicalDomains(yn) => format!("{} {}", consts::CANONICALDOMAINS, yn),
            Field::CanonicalizeFallbackLocal(yn) => {
                format!("{} {}", consts::CANONICALIZEFALLBACKLOCAL, yn)
            }
            Field::CanonicalizeHostname(yn) => format!("{} {}", consts::CANONICALIZEHOSTNAME, yn),
            Field::CanonicalizeMaxDots(yn) => format!("{} {}", consts::CANONICALIZEMAXDOTS, yn),
            Field::CanonicalizePermittedCNAMEs(s) => {
                format!("{} {}", consts::CANONICALIZEPERMITTEDCNAMES, s)
            }
            Field::CertificateFile(pb) => format!("{} {}", consts::CERTIFICATEFILE, pb.display()),
            Field::ChallengeResponseAuthentication(yn) => {
                format!("{} {}", consts::CHALLENGERESPONSEAUTHENTICATION, yn)
            }
            Field::CheckHostIP(yn) => format!("{} {}", consts::CHECKHOSTIP, yn),
            Field::Ciphers(vs) => format!("{} {}", consts::CIPHERS, vs.join(",")),
            Field::ClearAllForwardings(yn) => format!("{} {}", consts::CLEARALLFORWARDINGS, yn),
            Field::Compression(yn) => format!("{} {}", consts::COMPRESSION, yn),
            Field::ConnectionAttempts(i) => format!("{} {}", consts::CONNECTIONATTEMPTS, i),
            Field::ConnectTimeout(i) => format!("{} {}", consts::CONNECTTIMEOUT, i),
            Field::ControlMaster(ynaaaa) => format!("{} {}", consts::CONTROLMASTER, ynaaaa),
            Field::ControlPath(pb) => format!("{} {}", consts::CONTROLPATH, pb.display()),
            Field::ControlPersist(ynd) => format!("{} {}", consts::CONTROLPERSIST, ynd),
            Field::DynamicForward(s) => format!("{} {}", consts::DYNAMICFORWARD, s),
            Field::EnableSSHKeysign(yn) => format!("{} {}", consts::ENABLESSHKEYSIGN, yn),
            Field::EscapeChar(s) => format!("{} {}", consts::ESCAPECHAR, s),
            Field::ExitOnForwardFailure(yn) => format!("{} {}", consts::EXITONFORWARDFAILURE, yn),
            Field::FingerprintHash(ms) => format!("{} {}", consts::FINGERPRINTHASH, ms),
            Field::ForwardAgent(yn) => format!("{} {}", consts::FORWARDAGENT, yn),
            Field::ForwardX11(yn) => format!("{} {}", consts::FORWARDX11, yn),
            Field::ForwardX11Timeout(d) => format!("{} {}", consts::FORWARDX11TIMEOUT, d),
            Field::ForwardX11Trusted(yn) => format!("{} {}", consts::FORWARDX11TRUSTED, yn),
            Field::GatewayPorts(yn) => format!("{} {}", consts::GATEWAYPORTS, yn),
            Field::GlobalKnownHostsFile(pb) => {
                format!("{} {}", consts::GLOBALKNOWNHOSTSFILE, pb.display())
            }
            Field::GSSAPIAuthentication(yn) => format!("{} {}", consts::GSSAPIAUTHENTICATION, yn),
            Field::GSSAPIDelegateCredentials(yn) => {
                format!("{} {}", consts::GSSAPIDELEGATECREDENTIALS, yn)
            }
            Field::HashKnownHosts(yn) => format!("{} {}", consts::HASHKNOWNHOSTS, yn),
            Field::HostbasedAuthentication(yn) => {
                format!("{} {}", consts::HOSTBASEDAUTHENTICATION, yn)
            }
            Field::HostbasedKeyTypes(vs) => {
                format!("{} {}", consts::HOSTBASEDKEYTYPES, vs.join(","))
            }
            Field::HostKeyAlgorithms(vs) => {
                format!("{} {}", consts::HOSTKEYALGORITHMS, vs.join(","))
            }
            Field::HostKeyAlias(s) => format!("{} {}", consts::HOSTKEYALIAS, s),
            Field::Hostname(s) => format!("{} {}", consts::HOSTNAME, s),
            Field::IdentitiesOnly(yn) => format!("{} {}", consts::IDENTITIESONLY, yn),
            Field::IdentityAgent(pb) => format!("{} {}", consts::IDENTITYAGENT, pb.display()),
            Field::IdentityFile(pb) => format!("{} {}", consts::IDENTITYFILE, pb.display()),
            Field::IgnoreUnknown(s) => format!("{} {}", consts::IGNOREUNKNOWN, s),
            Field::Include(vpb) => format!("{} {}", consts::INCLUDE, {
                let vs: Vec<String> = vpb.iter().map(|pb| format!("{}", pb.display())).collect();
                vs.join(",")
            }),
            Field::IPQoS(s) => format!("{} {}", consts::IPQOS, s),
            Field::KbdInteractiveAuthentication(yn) => {
                format!("{} {}", consts::KBDINTERACTIVEAUTHENTICATION, yn)
            }
            Field::KbdInteractiveDevices(vs) => {
                format!("{} {}", consts::KBDINTERACTIVEDEVICES, vs.join(","))
            }
            Field::KexAlgorithms(vs) => format!("{} {}", consts::KEXALGORITHMS, vs.join(",")),
            Field::LocalCommand(s) => format!("{} {}", consts::LOCALCOMMAND, s),
            Field::LocalForward(s) => format!("{} {}", consts::LOCALFORWARD, s),
            Field::LogLevel(s) => format!("{} {}", consts::LOGLEVEL, s),
            Field::MACs(vs) => format!("{} {}", consts::MACS, vs.join(",")),
            Field::NoHostAuthenticationForLocalhost(yn) => {
                format!("{} {}", consts::NOHOSTAUTHENTICATIONFORLOCALHOST, yn)
            }
            Field::NumberOfPasswordPrompts(i) => {
                format!("{} {}", consts::NUMBEROFPASSWORDPROMPTS, i)
            }
            Field::PasswordAuthentication(yn) => {
                format!("{} {}", consts::PASSWORDAUTHENTICATION, yn)
            }
            Field::PermitLocalCommand(yn) => format!("{} {}", consts::PERMITLOCALCOMMAND, yn),
            Field::PKCS11Provider(s) => format!("{} {}", consts::PKCS11PROVIDER, s),
            Field::Port(i) => format!("{} {}", consts::PORT, i),
            Field::PreferredAuthentications(vs) => {
                format!("{} {}", consts::PREFERREDAUTHENTICATIONS, vs.join(","))
            }
            Field::ProxyCommand(s) => format!("{} {}", consts::PROXYCOMMAND, s),
            Field::ProxyJump(s) => format!("{} {}", consts::PROXYJUMP, s),
            Field::ProxyUseFdpass(yn) => format!("{} {}", consts::PROXYUSEFDPASS, yn),
            Field::PubkeyAcceptedKeyTypes(vs) => {
                format!("{} {}", consts::PUBKEYACCEPTEDKEYTYPES, vs.join(","))
            }
            Field::PubkeyAuthentication(yn) => format!("{} {}", consts::PUBKEYAUTHENTICATION, yn),
            Field::RekeyLimit(s) => format!("{} {}", consts::REKEYLIMIT, s),
            Field::RemoteCommand(s) => format!("{} {}", consts::REMOTECOMMAND, s),
            Field::RemoteForward(s) => format!("{} {}", consts::REMOTEFORWARD, s),
            Field::RequestTTY(s) => format!("{} {}", consts::REQUESTTTY, s),
            Field::RevokedHostKeys(pb) => format!("{} {}", consts::REVOKEDHOSTKEYS, pb.display()),
            Field::RhostsRSAAuthentication(yn) => {
                format!("{} {}", consts::RHOSTSRSAAUTHENTICATION, yn)
            }
            Field::RSAAuthentication(yn) => format!("{} {}", consts::RSAAUTHENTICATION, yn),
            Field::SendEnv(s) => format!("{} {}", consts::SENDENV, s),
            Field::ServerAliveCountMax(i) => format!("{} {}", consts::SERVERALIVECOUNTMAX, i),
            Field::ServerAliveInterval(i) => format!("{} {}", consts::SERVERALIVEINTERVAL, i),
            Field::StreamLocalBindMask(yn) => format!("{} {}", consts::STREAMLOCALBINDMASK, yn),
            Field::StreamLocalBindUnlink(yn) => format!("{} {}", consts::STREAMLOCALBINDUNLINK, yn),
            Field::StrictHostKeyChecking(yna) => {
                format!("{} {}", consts::STRICTHOSTKEYCHECKING, yna)
            }
            Field::SyslogFacility(s) => format!("{} {}", consts::SYSLOGFACILITY, s),
            Field::TCPKeepAlive(yn) => format!("{} {}", consts::TCPKEEPALIVE, yn),
            Field::Tunnel(s) => format!("{} {}", consts::TUNNEL, s),
            Field::TunnelDevice(s) => format!("{} {}", consts::TUNNELDEVICE, s),
            Field::UpdateHostKeys(yna) => format!("{} {}", consts::UPDATEHOSTKEYS, yna),
            Field::UsePrivilegedPort(yn) => format!("{} {}", consts::USEPRIVILEGEDPORT, yn),
            Field::User(s) => format!("{} {}", consts::USER, s),
            Field::UserKnownHostsFile(pb) => {
                format!("{} {}", consts::USERKNOWNHOSTSFILE, pb.display())
            }
            Field::VerifyHostKeyDNS(yna) => format!("{} {}", consts::VERIFYHOSTKEYDNS, yna),
            Field::VisualHostKey(yn) => format!("{} {}", consts::VISUALHOSTKEY, yn),
            Field::XAuthLocation(pb) => format!("{} {}", consts::XAUTHLOCATION, pb.display()),
            _ => String::new(), // EmptyLine, etc
        }
    }
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

#[derive(Clone, Debug)]
pub enum RootField {
    Field(Field),
    Host(String, Vec<Field>),
    Match(String, Vec<Field>),
}

impl PartialEq for RootField {
    fn eq(&self, rhs: &RootField) -> bool {
        match rhs {
            RootField::Field(r) => match self {
                RootField::Field(s) => s.eq(r),
                _ => false,
            },
            RootField::Host(rn, rs) | RootField::Match(rn, rs) => match self {
                RootField::Host(sn, ss) | RootField::Match(sn, ss) => sn == rn && ss == rs,
                _ => false,
            },
        }
    }
}

impl<S> From<S> for RootField
where
    S: Into<String> + AsRef<str>,
{
    // single-line input unless Host|Match section
    fn from(s: S) -> Self {
        if s.as_ref().starts_with(consts::HOST) || s.as_ref().starts_with(consts::MATCH) {
            let mut lines = s.as_ref().lines();
            let first = lines.next().unwrap_or_default();
            let mut split = first.splitn(2, ' ');
            let key = split.next().unwrap_or_default();
            let name = split.last().unwrap_or_default();
            let fields = lines.map(|l| Field::from(l)).collect();
            match key {
                consts::MATCH => RootField::Match(String::from(name), fields),
                _ => RootField::Host(String::from(name), fields),
            }
        } else {
            RootField::Field(Field::from(s))
        }
    }
}

impl From<&RootField> for String {
    fn from(rf: &RootField) -> Self {
        match rf {
            RootField::Field(f) => String::from(f),
            RootField::Host(n, fs) | RootField::Match(n, fs) => {
                let mut lines: Vec<String> = vec![format!(
                    "{} {}",
                    match rf {
                        RootField::Match(_, _) => consts::MATCH,
                        _ => consts::HOST,
                    },
                    n
                )];
                lines.extend(fs.iter().map(|f| format!("{}{}", INDENT, String::from(f))));
                lines.join("\n")
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

#[allow(clippy::needless_pass_by_value)]
fn parse_char<S>(text: S) -> char
where
    S: Into<String> + AsRef<str>,
{
    text.as_ref().chars().next().unwrap_or('~')
}

#[allow(clippy::needless_pass_by_value)]
fn parse_number<S>(text: S) -> Option<i32>
where
    S: Into<String> + AsRef<str>,
{
    match i32::from_str_radix(text.as_ref(), 10) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

#[allow(clippy::needless_pass_by_value)]
fn parse_pathbufs<S>(text: S) -> Vec<PathBuf>
where
    S: Into<String> + AsRef<str>,
{
    text.as_ref()
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(PathBuf::from(s))
            } else {
                None
            }
        })
        .collect()
}

#[allow(clippy::needless_pass_by_value)]
fn parse_strings<S>(text: S) -> Vec<String>
where
    S: Into<String> + AsRef<str>,
{
    text.as_ref()
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(String::from(s))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::env::consts::OS;

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
    fn field_to_from_str() {
        let cases = &[
            ("", Field::EmptyLine),
            ("# comment", Field::Comment(String::from("comment"))),
            (
                "AddKeysToAgent confirm",
                Field::AddKeysToAgent(YesNoAskConfirm::Confirm),
            ),
            (
                "AddressFamily inet6",
                Field::AddressFamily(AnyInetInet6::Inet6),
            ),
            ("BatchMode yes", Field::BatchMode(YesNo::Yes)),
            (
                "CanonicalizePermittedCNAMEs *.foo.com:*.bar.com",
                Field::CanonicalizePermittedCNAMEs(String::from("*.foo.com:*.bar.com")),
            ),
            if OS == "windows" {
                (
                    "CertificateFile c:\\foo",
                    Field::CertificateFile(PathBuf::from("c:\\foo")),
                )
            } else {
                (
                    "CertificateFile /foo",
                    Field::CertificateFile(PathBuf::from("/foo")),
                )
            },
            // TODO: cover one of each data type
        ];
        for c in cases {
            let got = Field::from(c.0);
            assert_eq!(c.1, got);
            let reverse = String::from(&got);
            if !c.0.is_empty() && !reverse.is_empty() {
                assert_eq!(c.0, reverse);
            }
        }
    }

    #[test]
    fn rootfield_to_from_str() {
        let cases = &[
            ("", RootField::Field(Field::EmptyLine)),
            (
                "DynamicForward 8080",
                RootField::Field(Field::DynamicForward(String::from("8080"))),
            ),
            (
                "Match user=\"git\"",
                RootField::Match(String::from("user=\"git\""), Vec::<Field>::new()),
            ),
            (
                "Host foo\n  IPQoS reliability",
                RootField::Host(
                    String::from("foo"),
                    vec![Field::IPQoS(String::from("reliability"))],
                ),
            ),
        ];
        for c in cases {
            let got = RootField::from(c.0);
            assert_eq!(c.1, got);
            let reverse = String::from(&got);
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
