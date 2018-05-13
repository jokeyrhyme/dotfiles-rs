use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct Config {
    // Hosts contains any Host sections found
    pub Hosts: HashMap<String, Config>,
    // Matches contains any Match sections found
    pub Matches: HashMap<String, Config>,

    // all other fields are valid SSH config directives
    pub AddKeysToAgent: Option<String>, // YesNoAsk with "confirm"
    pub AddressFamily: Option<String>,
    pub BatchMode: Option<bool>,
    pub BindAddress: Option<bool>,
    pub BindInterface: Option<bool>,
    pub CanonicalDomains: Option<bool>,
    pub CanonicalizeFallbackLocal: Option<bool>,
    pub CanonicalizeHostname: Option<bool>,
    pub CanonicalizeMaxDots: Option<bool>,
    pub CanonicalisePermittedCNAMEs: Option<bool>,
    pub CertificateFile: Option<PathBuf>,
    pub ChallengeResponseAuthentication: Option<bool>,
    pub CheckHostIP: Option<bool>,
    pub Ciphers: Option<Vec<String>>,
    pub ClearAllForwardings: Option<bool>,
    pub Compression: Option<bool>,
    pub ConnectionAttempts: Option<i32>,
    pub ConnectTimeout: Option<i32>,
    pub ControlMaster: Option<String>, // "auto" | "autoask" | "ask"
    pub ControlPath: Option<PathBuf>,
    pub ControlPersist: Option<i32>,
    pub DynamicForward: Option<String>,
    pub EnableSSHKeysing: Option<bool>,
    pub EscapeChar: Option<char>,
    pub ExitOnForwardFailure: Option<bool>,
    pub FingerprintHash: Option<String>,
    pub ForwardAgent: Option<bool>,
    pub ForwardX11: Option<bool>,
    pub ForwardX11Timeout: Option<i32>,
    pub ForwardX11Trusted: Option<bool>,
    pub GatewayPorts: Option<bool>,
    pub GlobalKnownHostsFile: Option<PathBuf>,
    pub GSSAPIAuthentication: Option<bool>,
    pub GSSAPIDelegateCredentials: Option<bool>,
    pub HashKnownHosts: Option<bool>,
    pub HostbasedAuthentication: Option<bool>,
    pub HostbasedKeyTypes: Option<String>,
    pub HostKeyAlgorithms: Option<String>,
    pub HostKeyAlias: Option<String>,
    pub HostName: Option<String>,
    pub IdentitiesOnly: Option<bool>,
    pub IdentityAgent: Option<String>,
    pub IdentityFile: Option<PathBuf>,
    pub IgnoreUnknown: Option<String>,
    pub Include: Option<String>,
    pub IPQoS: Option<String>,
    pub KbdInteractiveAuthentication: Option<bool>,
    pub KbdInteractiveDevices: Option<String>,
    pub KexAlgorithms: Option<Vec<String>>,
    pub LocalCommand: Option<String>,
    pub LocalForward: Option<String>,
    pub LogLevel: Option<String>,
    pub MACs: Option<Vec<String>>,
    pub NoHostAuthenticationForLocalhost: Option<bool>,
    pub NumberOfPasswordPrompts: Option<i32>,
    pub PasswordAuthentication: Option<bool>,
    pub PermitLocalCommand: Option<bool>,
    pub PKCS11Provider: Option<String>,
    pub Port: Option<i32>,
    pub PreferredAuthentications: Option<String>,
    pub ProxyCommand: Option<String>,
    pub ProxyJump: Option<String>,
    pub ProxyUseFdpass: Option<bool>,
    pub PubkeyAcceptedKeyTypes: Option<String>,
    pub PubkeyAuthentication: Option<bool>,
    pub RekeyLimit: Option<String>,
    pub RemoteCommand: Option<String>,
    pub RemoteForward: Option<String>,
    pub RequestTTY: Option<String>,
    pub RevokedHostKeys: Option<String>,
    pub SendEnv: Option<String>,
    pub ServerAliveCountMax: Option<i32>,
    pub ServerAliveInterval: Option<i32>,
    pub StreamLocalBindMask: Option<String>,
    pub StreamLocalBindUnlink: Option<bool>,
    pub StrictHostKeyChecking: Option<YesNoAsk>,
    pub SyslogFacility: Option<String>,
    pub TCPKeepAlive: Option<bool>,
    pub Tunnel: Option<String>,
    pub TunnelDevice: Option<String>,
    pub UpdateHostKeys: Option<YesNoAsk>,
    pub UsePrivilegedPort: Option<bool>,
    pub User: Option<String>,
    pub UserKnownHostsFile: Option<PathBuf>,
    pub VerifyHostKeyDNS: Option<YesNoAsk>,
    pub VisualHostKey: Option<bool>,
    pub XAuthLocation: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            Hosts: HashMap::<String, Config>::new(),
            Matches: HashMap::<String, Config>::new(),

            AddKeysToAgent: None,
            AddressFamily: None,
            BatchMode: None,
            BindAddress: None,
            BindInterface: None,
            CanonicalDomains: None,
            CanonicalizeFallbackLocal: None,
            CanonicalizeHostname: None,
            CanonicalizeMaxDots: None,
            CanonicalisePermittedCNAMEs: None,
            CertificateFile: None,
            ChallengeResponseAuthentication: None,
            CheckHostIP: None,
            Ciphers: None,
            ClearAllForwardings: None,
            Compression: None,
            ConnectionAttempts: None,
            ConnectTimeout: None,
            ControlMaster: None,
            ControlPath: None,
            ControlPersist: None,
            DynamicForward: None,
            EnableSSHKeysing: None,
            EscapeChar: None,
            ExitOnForwardFailure: None,
            FingerprintHash: None,
            ForwardAgent: None,
            ForwardX11: None,
            ForwardX11Timeout: None,
            ForwardX11Trusted: None,
            GatewayPorts: None,
            GlobalKnownHostsFile: None,
            GSSAPIAuthentication: None,
            GSSAPIDelegateCredentials: None,
            HashKnownHosts: None,
            HostbasedAuthentication: None,
            HostbasedKeyTypes: None,
            HostKeyAlgorithms: None,
            HostKeyAlias: None,
            HostName: None,
            IdentitiesOnly: None,
            IdentityAgent: None,
            IdentityFile: None,
            IgnoreUnknown: None,
            Include: None,
            IPQoS: None,
            KbdInteractiveAuthentication: None,
            KbdInteractiveDevices: None,
            KexAlgorithms: None,
            LocalCommand: None,
            LocalForward: None,
            LogLevel: None,
            MACs: None,
            NoHostAuthenticationForLocalhost: None,
            NumberOfPasswordPrompts: None,
            PasswordAuthentication: None,
            PermitLocalCommand: None,
            PKCS11Provider: None,
            Port: None,
            PreferredAuthentications: None,
            ProxyCommand: None,
            ProxyJump: None,
            ProxyUseFdpass: None,
            PubkeyAcceptedKeyTypes: None,
            PubkeyAuthentication: None,
            RekeyLimit: None,
            RemoteCommand: None,
            RemoteForward: None,
            RequestTTY: None,
            RevokedHostKeys: None,
            SendEnv: None,
            ServerAliveCountMax: None,
            ServerAliveInterval: None,
            StreamLocalBindMask: None,
            StreamLocalBindUnlink: None,
            StrictHostKeyChecking: None,
            SyslogFacility: None,
            TCPKeepAlive: None,
            Tunnel: None,
            TunnelDevice: None,
            UpdateHostKeys: None,
            UsePrivilegedPort: None,
            User: None,
            UserKnownHostsFile: None,
            VerifyHostKeyDNS: None,
            VisualHostKey: None,
            XAuthLocation: None,
        }
    }
}

#[derive(Debug)]
enum Section {
    Global,
    Host,
    Match,
}

#[derive(Debug, PartialEq)]
pub enum YesNoAsk {
    Yes,
    No,
    Ask,
}

impl<'a> From<&'a str> for YesNoAsk {
    fn from(source: &str) -> Self {
        match source {
            "yes" => YesNoAsk::Yes,
            "no" => YesNoAsk::No,
            _ => YesNoAsk::Ask,
        }
    }
}

impl Into<String> for YesNoAsk {
    fn into(self) -> String {
        match self {
            YesNoAsk::Yes => String::from("yes"),
            YesNoAsk::No => String::from("no"),
            YesNoAsk::Ask => String::from("ask"),
        }
    }
}

fn parse_config(text: String) -> Config {
    let mut config = Config::new();

    let mut section_key: String = String::from("");
    let mut section_type: Section = Section::Global;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.len() == 0 && trimmed.chars().next().unwrap_or('#') == '#' {
            continue; // skip empty lines and comments
            // TODO: one day support keeping comments
        }

        let mut split = trimmed.splitn(2, " ");
        let key = split.next().unwrap();
        let value = String::from(split.next().unwrap_or(""));

        match key {
            "Host" => {
                section_key = value.clone();
                section_type = Section::Host;
                config.Hosts.insert(value, Config::new());
                continue;
            }
            "Match" => {
                section_key = value.clone();
                section_type = Section::Match;
                config.Matches.insert(value, Config::new());
                continue;
            },
            _ => {}
        }

        let target: &mut Config = match section_type {
            Section::Global => &mut config,
            Section::Host => config.Hosts.get_mut(&section_key).unwrap(),
            Section::Match => config.Matches.get_mut(&section_key).unwrap(),
        };

        match key {
            "AddKeysToAgent" => {
                target.AddKeysToAgent = Some(value);
            }
            "Ciphers" => {
                target.Ciphers = parse_config_strings(value);
            },
            "ConnectTimeout" => {
                target.ConnectTimeout = parse_config_number(value);
            }
            "IdentityFile" => {
                target.IdentityFile = parse_config_pathbuf(value);
            },
            "StrictHostKeyChecking" => {
                target.StrictHostKeyChecking = parse_config_yesnoask(value);
            },
            "VisualHostKey" => {
                target.VisualHostKey = parse_config_bool(value);
            },
            _ => {},
        }
    }

    config
}

fn parse_config_bool(text: String) -> Option<bool> {
    Some(text == "yes")
}

fn parse_config_number(text: String) -> Option<i32> {
    match i32::from_str_radix(&text, 10) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn parse_config_pathbuf(text: String) -> Option<PathBuf> {
    Some(PathBuf::new().join(text))
}

fn parse_config_strings(text: String) -> Option<Vec<String>> {
    Some(
        text.split(",").into_iter().filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.len() > 0 {
                Some(String::from(s))
            } else {
                None
            }
        }).collect()
    )
}

fn parse_config_yesnoask(text: String) -> Option<YesNoAsk> {
    Some(YesNoAsk::from(text.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_with_all_types() {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ssh_config");

        let extracted = File::open(&config_path).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        let config = parse_config(got);

        let mut want = Config::new();
        want.AddKeysToAgent = Some(String::from("confirm"));
        want.ConnectTimeout = Some(10);
        want.VisualHostKey = Some(true);

        let mut h = Config::new();
        h.Ciphers = Some(vec![
            String::from("aes128-ctr"),
            String::from("aes192-ctr"),
            String::from("aes256-ctr"),
        ]);
        h.IdentityFile = Some(PathBuf::new().join("~").join(".ssh").join("id_rsa"));
        want.Hosts.insert(String::from("foo"), h);

        let mut m = Config::new();
        m.StrictHostKeyChecking = Some(YesNoAsk::Ask);
        want.Matches.insert(String::from("exec true"), m);

        assert_eq!(config.Hosts, want.Hosts);
        assert_eq!(config.Matches, want.Matches);
        assert_eq!(config, want);
    }
}