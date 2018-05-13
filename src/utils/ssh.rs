use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, PartialEq)]
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
        let config: Config = Default::default();
        config
    }
}

impl<'a> From<&'a str> for Config {
    fn from(source: &str) -> Self {
        let mut config = Config::new();

        let mut section_key: String = String::from("");
        let mut section_type: Section = Section::Global;

        for line in source.lines() {
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
                "AddKeysToAgent" => target.AddKeysToAgent = Some(value),
                "AddressFamily" => target.AddressFamily = Some(value),
                "BatchMode" => target.BatchMode = parse_config_bool(value),
                "BindAddress" => target.BindAddress = parse_config_bool(value),
                "BindInterface" => target.BindInterface = parse_config_bool(value),
                "CanonicalDomains" => target.CanonicalDomains = parse_config_bool(value),
                "CanonicalizeFallbackLocal" => target.CanonicalizeFallbackLocal = parse_config_bool(value),
                "CanonicalizeHostname" => target.CanonicalizeHostname = parse_config_bool(value),
                "CanonicalizeMaxDots" => target.CanonicalizeMaxDots = parse_config_bool(value),
                "CanonicalisePermittedCNAMEs" => target.CanonicalisePermittedCNAMEs = parse_config_bool(value),
                "CertificateFile" => target.CertificateFile = parse_config_pathbuf(value),
                "ChallengeResponseAuthentication" => target.ChallengeResponseAuthentication = parse_config_bool(value),
                "CheckHostIP" => target.CheckHostIP = parse_config_bool(value),
                "Ciphers" => target.Ciphers = parse_config_strings(value),
                "ClearAllForwardings" => target.ClearAllForwardings = parse_config_bool(value),
                "Compression" => target.Compression = parse_config_bool(value),
                "ConnectionAttempts" => target.ConnectionAttempts = parse_config_number(value),
                "ConnectTimeout" => target.ConnectTimeout = parse_config_number(value),
                "ControlMaster" => target.ControlMaster = Some(value),
                "ControlPath" => target.ControlPath = parse_config_pathbuf(value),
                "ControlPersist" => target.ControlPersist = parse_config_number(value),
                "DynamicForward" => target.DynamicForward = Some(value),
                "EnableSSHKeysing" => target.EnableSSHKeysing = parse_config_bool(value),
                "EscapeChar" => target.EscapeChar = parse_config_char(value),
                "ExitOnForwardFailure" => target.ExitOnForwardFailure = parse_config_bool(value),
                "FingerprintHash" => target.FingerprintHash = Some(value),
                "ForwardAgent" => target.ForwardAgent = parse_config_bool(value),
                "ForwardX11" => target.ForwardX11 = parse_config_bool(value),
                "ForwardX11Timeout" => target.ForwardX11Timeout = parse_config_number(value),
                "ForwardX11Trusted" => target.ForwardX11Trusted = parse_config_bool(value),
                "GatewayPorts" => target.GatewayPorts = parse_config_bool(value),
                "GlobalKnownHostsFile" => target.GlobalKnownHostsFile = parse_config_pathbuf(value),
                "GSSAPIAuthentication" => target.GSSAPIAuthentication = parse_config_bool(value),
                "GSSAPIDelegateCredentials" => target.GSSAPIDelegateCredentials = parse_config_bool(value),
                "HashKnownHosts" => target.HashKnownHosts = parse_config_bool(value),
                "HostbasedAuthentication" => target.HostbasedAuthentication = parse_config_bool(value),
                "HostbasedKeyTypes" => target.HostbasedKeyTypes = Some(value),
                "HostKeyAlgorithms" => target.HostKeyAlgorithms = Some(value),
                "HostKeyAlias" => target.HostKeyAlias = Some(value),
                "HostName" => target.HostName = Some(value),
                "IdentitiesOnly" => target.IdentitiesOnly = parse_config_bool(value),
                "IdentityAgent" => target.IdentityAgent = Some(value),
                "IdentityFile" => target.IdentityFile = parse_config_pathbuf(value),
                "IgnoreUnknown" => target.IgnoreUnknown = Some(value),
                "Include" => target.Include = Some(value),
                "IPQoS" => target.IPQoS = Some(value),
                "KbdInteractiveAuthentication" => target.KbdInteractiveAuthentication = parse_config_bool(value),
                "KbdInteractiveDevices" => target.KbdInteractiveDevices = Some(value),
                "KexAlgorithms" => target.KexAlgorithms = parse_config_strings(value),
                "LocalCommand" => target.LocalCommand = Some(value),
                "LocalForward" => target.LocalForward = Some(value),
                "LogLevel" => target.LogLevel = Some(value),
                "MACs" => target.MACs = parse_config_strings(value),
                "NoHostAuthenticationForLocalhost" => target.NoHostAuthenticationForLocalhost = parse_config_bool(value),
                "NumberOfPasswordPrompts" => target.NumberOfPasswordPrompts = parse_config_number(value),
                "PasswordAuthentication" => target.PasswordAuthentication = parse_config_bool(value),
                "PermitLocalCommand" => target.PermitLocalCommand = parse_config_bool(value),
                "PKCS11Provider" => target.PKCS11Provider = Some(value),
                "Port" => target.Port = parse_config_number(value),
                "PreferredAuthentications" => target.PreferredAuthentications = Some(value),
                "ProxyCommand" => target.ProxyCommand = Some(value),
                "ProxyJump" => target.ProxyJump = Some(value),
                "ProxyUseFdpass" => target.ProxyUseFdpass = parse_config_bool(value),
                "PubkeyAcceptedKeyTypes" => target.PubkeyAcceptedKeyTypes = Some(value),
                "PubkeyAuthentication" => target.PubkeyAuthentication = parse_config_bool(value),
                "RekeyLimit" => target.RekeyLimit = Some(value),
                "RemoteCommand" => target.RemoteCommand = Some(value),
                "RemoteForward" => target.RemoteForward = Some(value),
                "RequestTTY" => target.RequestTTY = Some(value),
                "RevokedHostKeys" => target.RevokedHostKeys = Some(value),
                "SendEnv" => target.SendEnv = Some(value),
                "ServerAliveCountMax" => target.ServerAliveCountMax = parse_config_number(value),
                "ServerAliveInterval" => target.ServerAliveInterval = parse_config_number(value),
                "StreamLocalBindMask" => target.StreamLocalBindMask = Some(value),
                "StreamLocalBindUnlink" => target.StreamLocalBindUnlink = parse_config_bool(value),
                "StrictHostKeyChecking" => target.StrictHostKeyChecking = parse_config_yesnoask(value),
                "SyslogFacility" => target.SyslogFacility = Some(value),
                "TCPKeepAlive" => target.TCPKeepAlive = parse_config_bool(value),
                "Tunnel" => target.Tunnel = Some(value),
                "TunnelDevice" => target.TunnelDevice = Some(value),
                "UpdateHostKeys" => target.UpdateHostKeys = parse_config_yesnoask(value),
                "UsePrivilegedPort" => target.UsePrivilegedPort = parse_config_bool(value),
                "User" => target.User = Some(value),
                "UserKnownHostsFile" => target.UserKnownHostsFile = parse_config_pathbuf(value),
                "VerifyHostKeyDNS" => target.VerifyHostKeyDNS = parse_config_yesnoask(value),
                "VisualHostKey" => target.VisualHostKey = parse_config_bool(value),
                "XAuthLocation" => target.XAuthLocation = parse_config_pathbuf(value),
                _ => {}
            }
        }

        config
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

fn parse_config_bool(text: String) -> Option<bool> {
    Some(text == "yes")
}

fn parse_config_char(text: String) -> Option<char> {
    Some(text.chars().next().unwrap_or('~'))
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

        let config = Config::from(got.as_str());

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