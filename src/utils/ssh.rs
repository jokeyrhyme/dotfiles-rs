use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::BitOr;
use std::path::{Path, PathBuf};

use textwrap;

#[derive(Clone, Debug, Default, PartialEq)]
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

impl BitOr for Config {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        let mut result = self.clone();

        result.Hosts.extend(rhs.Hosts);
        result.Matches.extend(rhs.Matches);

        if rhs.AddKeysToAgent.is_some() {
            result.AddKeysToAgent = rhs.AddKeysToAgent.clone();
        }
        if rhs.AddressFamily.is_some() {
            result.AddressFamily = rhs.AddressFamily.clone();
        }
        if rhs.BatchMode.is_some() {
            result.BatchMode = rhs.BatchMode.clone();
        }
        if rhs.BindAddress.is_some() {
            result.BindAddress = rhs.BindAddress.clone();
        }
        if rhs.BindInterface.is_some() {
            result.BindInterface = rhs.BindInterface.clone();
        }
        if rhs.CanonicalDomains.is_some() {
            result.CanonicalDomains = rhs.CanonicalDomains.clone();
        }
        if rhs.CanonicalizeFallbackLocal.is_some() {
            result.CanonicalizeFallbackLocal = rhs.CanonicalizeFallbackLocal.clone();
        }
        if rhs.CanonicalizeHostname.is_some() {
            result.CanonicalizeHostname = rhs.CanonicalizeHostname.clone();
        }
        if rhs.CanonicalizeMaxDots.is_some() {
            result.CanonicalizeMaxDots = rhs.CanonicalizeMaxDots.clone();
        }
        if rhs.CanonicalisePermittedCNAMEs.is_some() {
            result.CanonicalisePermittedCNAMEs = rhs.CanonicalisePermittedCNAMEs.clone();
        }
        if rhs.CertificateFile.is_some() {
            result.CertificateFile = rhs.CertificateFile.clone();
        }
        if rhs.ChallengeResponseAuthentication.is_some() {
            result.ChallengeResponseAuthentication = rhs.ChallengeResponseAuthentication.clone();
        }
        if rhs.CheckHostIP.is_some() {
            result.CheckHostIP = rhs.CheckHostIP.clone();
        }
        if rhs.Ciphers.is_some() {
            result.Ciphers = rhs.Ciphers.clone();
        }
        if rhs.ClearAllForwardings.is_some() {
            result.ClearAllForwardings = rhs.ClearAllForwardings.clone();
        }
        if rhs.Compression.is_some() {
            result.Compression = rhs.Compression.clone();
        }
        if rhs.ConnectionAttempts.is_some() {
            result.ConnectionAttempts = rhs.ConnectionAttempts.clone();
        }
        if rhs.ConnectTimeout.is_some() {
            result.ConnectTimeout = rhs.ConnectTimeout.clone();
        }
        if rhs.ControlMaster.is_some() {
            result.ControlMaster = rhs.ControlMaster.clone();
        }
        if rhs.ControlPath.is_some() {
            result.ControlPath = rhs.ControlPath.clone();
        }
        if rhs.ControlPersist.is_some() {
            result.ControlPersist = rhs.ControlPersist.clone();
        }
        if rhs.DynamicForward.is_some() {
            result.DynamicForward = rhs.DynamicForward.clone();
        }
        if rhs.EnableSSHKeysing.is_some() {
            result.EnableSSHKeysing = rhs.EnableSSHKeysing.clone();
        }
        if rhs.EscapeChar.is_some() {
            result.EscapeChar = rhs.EscapeChar.clone();
        }
        if rhs.ExitOnForwardFailure.is_some() {
            result.ExitOnForwardFailure = rhs.ExitOnForwardFailure.clone();
        }
        if rhs.FingerprintHash.is_some() {
            result.FingerprintHash = rhs.FingerprintHash.clone();
        }
        if rhs.ForwardAgent.is_some() {
            result.ForwardAgent = rhs.ForwardAgent.clone();
        }
        if rhs.ForwardX11.is_some() {
            result.ForwardX11 = rhs.ForwardX11.clone();
        }
        if rhs.ForwardX11Timeout.is_some() {
            result.ForwardX11Timeout = rhs.ForwardX11Timeout.clone();
        }
        if rhs.ForwardX11Trusted.is_some() {
            result.ForwardX11Trusted = rhs.ForwardX11Trusted.clone();
        }
        if rhs.GatewayPorts.is_some() {
            result.GatewayPorts = rhs.GatewayPorts.clone();
        }
        if rhs.GlobalKnownHostsFile.is_some() {
            result.GlobalKnownHostsFile = rhs.GlobalKnownHostsFile.clone();
        }
        if rhs.GSSAPIAuthentication.is_some() {
            result.GSSAPIAuthentication = rhs.GSSAPIAuthentication.clone();
        }
        if rhs.GSSAPIDelegateCredentials.is_some() {
            result.GSSAPIDelegateCredentials = rhs.GSSAPIDelegateCredentials.clone();
        }
        if rhs.HashKnownHosts.is_some() {
            result.HashKnownHosts = rhs.HashKnownHosts.clone();
        }
        if rhs.HostbasedAuthentication.is_some() {
            result.HostbasedAuthentication = rhs.HostbasedAuthentication.clone();
        }
        if rhs.HostbasedKeyTypes.is_some() {
            result.HostbasedKeyTypes = rhs.HostbasedKeyTypes.clone();
        }
        if rhs.HostKeyAlgorithms.is_some() {
            result.HostKeyAlgorithms = rhs.HostKeyAlgorithms.clone();
        }
        if rhs.HostKeyAlias.is_some() {
            result.HostKeyAlias = rhs.HostKeyAlias.clone();
        }
        if rhs.HostName.is_some() {
            result.HostName = rhs.HostName.clone();
        }
        if rhs.IdentitiesOnly.is_some() {
            result.IdentitiesOnly = rhs.IdentitiesOnly.clone();
        }
        if rhs.IdentityAgent.is_some() {
            result.IdentityAgent = rhs.IdentityAgent.clone();
        }
        if rhs.IdentityFile.is_some() {
            result.IdentityFile = rhs.IdentityFile.clone();
        }
        if rhs.IgnoreUnknown.is_some() {
            result.IgnoreUnknown = rhs.IgnoreUnknown.clone();
        }
        if rhs.Include.is_some() {
            result.Include = rhs.Include.clone();
        }
        if rhs.IPQoS.is_some() {
            result.IPQoS = rhs.IPQoS.clone();
        }
        if rhs.KbdInteractiveAuthentication.is_some() {
            result.KbdInteractiveAuthentication = rhs.KbdInteractiveAuthentication.clone();
        }
        if rhs.KbdInteractiveDevices.is_some() {
            result.KbdInteractiveDevices = rhs.KbdInteractiveDevices.clone();
        }
        if rhs.KexAlgorithms.is_some() {
            result.KexAlgorithms = rhs.KexAlgorithms.clone();
        }
        if rhs.LocalCommand.is_some() {
            result.LocalCommand = rhs.LocalCommand.clone();
        }
        if rhs.LocalForward.is_some() {
            result.LocalForward = rhs.LocalForward.clone();
        }
        if rhs.LogLevel.is_some() {
            result.LogLevel = rhs.LogLevel.clone();
        }
        if rhs.MACs.is_some() {
            result.MACs = rhs.MACs.clone();
        }
        if rhs.NoHostAuthenticationForLocalhost.is_some() {
            result.NoHostAuthenticationForLocalhost = rhs.NoHostAuthenticationForLocalhost.clone();
        }
        if rhs.NumberOfPasswordPrompts.is_some() {
            result.NumberOfPasswordPrompts = rhs.NumberOfPasswordPrompts.clone();
        }
        if rhs.PasswordAuthentication.is_some() {
            result.PasswordAuthentication = rhs.PasswordAuthentication.clone();
        }
        if rhs.PermitLocalCommand.is_some() {
            result.PermitLocalCommand = rhs.PermitLocalCommand.clone();
        }
        if rhs.PKCS11Provider.is_some() {
            result.PKCS11Provider = rhs.PKCS11Provider.clone();
        }
        if rhs.Port.is_some() {
            result.Port = rhs.Port.clone();
        }
        if rhs.PreferredAuthentications.is_some() {
            result.PreferredAuthentications = rhs.PreferredAuthentications.clone();
        }
        if rhs.ProxyCommand.is_some() {
            result.ProxyCommand = rhs.ProxyCommand.clone();
        }
        if rhs.ProxyJump.is_some() {
            result.ProxyJump = rhs.ProxyJump.clone();
        }
        if rhs.ProxyUseFdpass.is_some() {
            result.ProxyUseFdpass = rhs.ProxyUseFdpass.clone();
        }
        if rhs.PubkeyAcceptedKeyTypes.is_some() {
            result.PubkeyAcceptedKeyTypes = rhs.PubkeyAcceptedKeyTypes.clone();
        }
        if rhs.PubkeyAuthentication.is_some() {
            result.PubkeyAuthentication = rhs.PubkeyAuthentication.clone();
        }
        if rhs.RekeyLimit.is_some() {
            result.RekeyLimit = rhs.RekeyLimit.clone();
        }
        if rhs.RemoteCommand.is_some() {
            result.RemoteCommand = rhs.RemoteCommand.clone();
        }
        if rhs.RemoteForward.is_some() {
            result.RemoteForward = rhs.RemoteForward.clone();
        }
        if rhs.RequestTTY.is_some() {
            result.RequestTTY = rhs.RequestTTY.clone();
        }
        if rhs.RevokedHostKeys.is_some() {
            result.RevokedHostKeys = rhs.RevokedHostKeys.clone();
        }
        if rhs.SendEnv.is_some() {
            result.SendEnv = rhs.SendEnv.clone();
        }
        if rhs.ServerAliveCountMax.is_some() {
            result.ServerAliveCountMax = rhs.ServerAliveCountMax.clone();
        }
        if rhs.ServerAliveInterval.is_some() {
            result.ServerAliveInterval = rhs.ServerAliveInterval.clone();
        }
        if rhs.StreamLocalBindMask.is_some() {
            result.StreamLocalBindMask = rhs.StreamLocalBindMask.clone();
        }
        if rhs.StreamLocalBindUnlink.is_some() {
            result.StreamLocalBindUnlink = rhs.StreamLocalBindUnlink.clone();
        }
        if rhs.StrictHostKeyChecking.is_some() {
            result.StrictHostKeyChecking = rhs.StrictHostKeyChecking.clone();
        }
        if rhs.SyslogFacility.is_some() {
            result.SyslogFacility = rhs.SyslogFacility.clone();
        }
        if rhs.TCPKeepAlive.is_some() {
            result.TCPKeepAlive = rhs.TCPKeepAlive.clone();
        }
        if rhs.Tunnel.is_some() {
            result.Tunnel = rhs.Tunnel.clone();
        }
        if rhs.TunnelDevice.is_some() {
            result.TunnelDevice = rhs.TunnelDevice.clone();
        }
        if rhs.UpdateHostKeys.is_some() {
            result.UpdateHostKeys = rhs.UpdateHostKeys.clone();
        }
        if rhs.UsePrivilegedPort.is_some() {
            result.UsePrivilegedPort = rhs.UsePrivilegedPort.clone();
        }
        if rhs.User.is_some() {
            result.User = rhs.User.clone();
        }
        if rhs.UserKnownHostsFile.is_some() {
            result.UserKnownHostsFile = rhs.UserKnownHostsFile.clone();
        }
        if rhs.VerifyHostKeyDNS.is_some() {
            result.VerifyHostKeyDNS = rhs.VerifyHostKeyDNS.clone();
        }
        if rhs.VisualHostKey.is_some() {
            result.VisualHostKey = rhs.VisualHostKey.clone();
        }
        if rhs.XAuthLocation.is_some() {
            result.XAuthLocation = rhs.XAuthLocation.clone();
        }

        result
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
                }
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
                "CanonicalizeFallbackLocal" => {
                    target.CanonicalizeFallbackLocal = parse_config_bool(value)
                }
                "CanonicalizeHostname" => target.CanonicalizeHostname = parse_config_bool(value),
                "CanonicalizeMaxDots" => target.CanonicalizeMaxDots = parse_config_bool(value),
                "CanonicalisePermittedCNAMEs" => {
                    target.CanonicalisePermittedCNAMEs = parse_config_bool(value)
                }
                "CertificateFile" => target.CertificateFile = parse_config_pathbuf(value),
                "ChallengeResponseAuthentication" => {
                    target.ChallengeResponseAuthentication = parse_config_bool(value)
                }
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
                "GSSAPIDelegateCredentials" => {
                    target.GSSAPIDelegateCredentials = parse_config_bool(value)
                }
                "HashKnownHosts" => target.HashKnownHosts = parse_config_bool(value),
                "HostbasedAuthentication" => {
                    target.HostbasedAuthentication = parse_config_bool(value)
                }
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
                "KbdInteractiveAuthentication" => {
                    target.KbdInteractiveAuthentication = parse_config_bool(value)
                }
                "KbdInteractiveDevices" => target.KbdInteractiveDevices = Some(value),
                "KexAlgorithms" => target.KexAlgorithms = parse_config_strings(value),
                "LocalCommand" => target.LocalCommand = Some(value),
                "LocalForward" => target.LocalForward = Some(value),
                "LogLevel" => target.LogLevel = Some(value),
                "MACs" => target.MACs = parse_config_strings(value),
                "NoHostAuthenticationForLocalhost" => {
                    target.NoHostAuthenticationForLocalhost = parse_config_bool(value)
                }
                "NumberOfPasswordPrompts" => {
                    target.NumberOfPasswordPrompts = parse_config_number(value)
                }
                "PasswordAuthentication" => {
                    target.PasswordAuthentication = parse_config_bool(value)
                }
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
                "StrictHostKeyChecking" => {
                    target.StrictHostKeyChecking = parse_config_yesnoask(value)
                }
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

impl From<Config> for String {
    fn from(source: Config) -> String {
        let mut result = String::from("");

        result.push_str(&format_config_string(
            &"AddKeysToAgent",
            source.AddKeysToAgent,
        ));
        result.push_str(&format_config_string(
            &"AddressFamily",
            source.AddressFamily,
        ));
        result.push_str(&format_config_bool(&"BatchMode", source.BatchMode));
        result.push_str(&format_config_bool(&"BindAddress", source.BindAddress));
        result.push_str(&format_config_bool(&"BindInterface", source.BindInterface));
        result.push_str(&format_config_bool(
            &"CanonicalDomains",
            source.CanonicalDomains,
        ));
        result.push_str(&format_config_bool(
            &"CanonicalizeFallbackLocal",
            source.CanonicalizeFallbackLocal,
        ));
        result.push_str(&format_config_bool(
            &"CanonicalizeHostname",
            source.CanonicalizeHostname,
        ));
        result.push_str(&format_config_bool(
            &"CanonicalizeMaxDots",
            source.CanonicalizeMaxDots,
        ));
        result.push_str(&format_config_bool(
            &"CanonicalisePermittedCNAMEs",
            source.CanonicalisePermittedCNAMEs,
        ));
        result.push_str(&format_config_pathbuf(
            &"CertificateFile",
            source.CertificateFile,
        ));
        result.push_str(&format_config_bool(
            &"ChallengeResponseAuthentication",
            source.ChallengeResponseAuthentication,
        ));
        result.push_str(&format_config_bool(&"CheckHostIP", source.CheckHostIP));
        result.push_str(&format_config_strings(&"Ciphers", source.Ciphers));
        result.push_str(&format_config_bool(
            &"ClearAllForwardings",
            source.ClearAllForwardings,
        ));
        result.push_str(&format_config_bool(&"Compression", source.Compression));
        result.push_str(&format_config_number(
            &"ConnectionAttempts",
            source.ConnectionAttempts,
        ));
        result.push_str(&format_config_number(
            &"ConnectTimeout",
            source.ConnectTimeout,
        ));
        result.push_str(&format_config_string(
            &"ControlMaster",
            source.ControlMaster,
        ));
        result.push_str(&format_config_pathbuf(&"ControlPath", source.ControlPath));
        result.push_str(&format_config_number(
            &"ControlPersist",
            source.ControlPersist,
        ));
        result.push_str(&format_config_string(
            &"DynamicForward",
            source.DynamicForward,
        ));
        result.push_str(&format_config_bool(
            &"EnableSSHKeysing",
            source.EnableSSHKeysing,
        ));
        result.push_str(&format_config_char(&"EscapeChar", source.EscapeChar));
        result.push_str(&format_config_bool(
            &"ExitOnForwardFailure",
            source.ExitOnForwardFailure,
        ));
        result.push_str(&format_config_string(
            &"FingerprintHash",
            source.FingerprintHash,
        ));
        result.push_str(&format_config_bool(&"ForwardAgent", source.ForwardAgent));
        result.push_str(&format_config_bool(&"ForwardX11", source.ForwardX11));
        result.push_str(&format_config_number(
            &"ForwardX11Timeout",
            source.ForwardX11Timeout,
        ));
        result.push_str(&format_config_bool(
            &"ForwardX11Trusted",
            source.ForwardX11Trusted,
        ));
        result.push_str(&format_config_bool(&"GatewayPorts", source.GatewayPorts));
        result.push_str(&format_config_pathbuf(
            &"GlobalKnownHostsFile",
            source.GlobalKnownHostsFile,
        ));
        result.push_str(&format_config_bool(
            &"GSSAPIAuthentication",
            source.GSSAPIAuthentication,
        ));
        result.push_str(&format_config_bool(
            &"GSSAPIDelegateCredentials",
            source.GSSAPIDelegateCredentials,
        ));
        result.push_str(&format_config_bool(
            &"HashKnownHosts",
            source.HashKnownHosts,
        ));
        result.push_str(&format_config_bool(
            &"HostbasedAuthentication",
            source.HostbasedAuthentication,
        ));
        result.push_str(&format_config_string(
            &"HostbasedKeyTypes",
            source.HostbasedKeyTypes,
        ));
        result.push_str(&format_config_string(
            &"HostKeyAlgorithms",
            source.HostKeyAlgorithms,
        ));
        result.push_str(&format_config_string(&"HostKeyAlias", source.HostKeyAlias));
        result.push_str(&format_config_string(&"HostName", source.HostName));
        result.push_str(&format_config_bool(
            &"IdentitiesOnly",
            source.IdentitiesOnly,
        ));
        result.push_str(&format_config_string(
            &"IdentityAgent",
            source.IdentityAgent,
        ));
        result.push_str(&format_config_pathbuf(&"IdentityFile", source.IdentityFile));
        result.push_str(&format_config_string(
            &"IgnoreUnknown",
            source.IgnoreUnknown,
        ));
        result.push_str(&format_config_string(&"Include", source.Include));
        result.push_str(&format_config_string(&"IPQoS", source.IPQoS));
        result.push_str(&format_config_bool(
            &"KbdInteractiveAuthentication",
            source.KbdInteractiveAuthentication,
        ));
        result.push_str(&format_config_string(
            &"KbdInteractiveDevices",
            source.KbdInteractiveDevices,
        ));
        result.push_str(&format_config_strings(
            &"KexAlgorithms",
            source.KexAlgorithms,
        ));
        result.push_str(&format_config_string(&"LocalCommand", source.LocalCommand));
        result.push_str(&format_config_string(&"LocalForward", source.LocalForward));
        result.push_str(&format_config_string(&"LogLevel", source.LogLevel));
        result.push_str(&format_config_strings(&"MACs", source.MACs));
        result.push_str(&format_config_bool(
            &"NoHostAuthenticationForLocalhost",
            source.NoHostAuthenticationForLocalhost,
        ));
        result.push_str(&format_config_number(
            &"NumberOfPasswordPrompts",
            source.NumberOfPasswordPrompts,
        ));
        result.push_str(&format_config_bool(
            &"PasswordAuthentication",
            source.PasswordAuthentication,
        ));
        result.push_str(&format_config_bool(
            &"PermitLocalCommand",
            source.PermitLocalCommand,
        ));
        result.push_str(&format_config_string(
            &"PKCS11Provider",
            source.PKCS11Provider,
        ));
        result.push_str(&format_config_number(&"Port", source.Port));
        result.push_str(&format_config_string(
            &"PreferredAuthentications",
            source.PreferredAuthentications,
        ));
        result.push_str(&format_config_string(&"ProxyCommand", source.ProxyCommand));
        result.push_str(&format_config_string(&"ProxyJump", source.ProxyJump));
        result.push_str(&format_config_bool(
            &"ProxyUseFdpass",
            source.ProxyUseFdpass,
        ));
        result.push_str(&format_config_string(
            &"PubkeyAcceptedKeyTypes",
            source.PubkeyAcceptedKeyTypes,
        ));
        result.push_str(&format_config_bool(
            &"PubkeyAuthentication",
            source.PubkeyAuthentication,
        ));
        result.push_str(&format_config_string(&"RekeyLimit", source.RekeyLimit));
        result.push_str(&format_config_string(
            &"RemoteCommand",
            source.RemoteCommand,
        ));
        result.push_str(&format_config_string(
            &"RemoteForward",
            source.RemoteForward,
        ));
        result.push_str(&format_config_string(&"RequestTTY", source.RequestTTY));
        result.push_str(&format_config_string(
            &"RevokedHostKeys",
            source.RevokedHostKeys,
        ));
        result.push_str(&format_config_string(&"SendEnv", source.SendEnv));
        result.push_str(&format_config_number(
            &"ServerAliveCountMax",
            source.ServerAliveCountMax,
        ));
        result.push_str(&format_config_number(
            &"ServerAliveInterval",
            source.ServerAliveInterval,
        ));
        result.push_str(&format_config_string(
            &"StreamLocalBindMask",
            source.StreamLocalBindMask,
        ));
        result.push_str(&format_config_bool(
            &"StreamLocalBindUnlink",
            source.StreamLocalBindUnlink,
        ));
        result.push_str(&format_config_yesnoask(
            &"StrictHostKeyChecking",
            source.StrictHostKeyChecking,
        ));
        result.push_str(&format_config_string(
            &"SyslogFacility",
            source.SyslogFacility,
        ));
        result.push_str(&format_config_bool(&"TCPKeepAlive", source.TCPKeepAlive));
        result.push_str(&format_config_string(&"Tunnel", source.Tunnel));
        result.push_str(&format_config_string(&"TunnelDevice", source.TunnelDevice));
        result.push_str(&format_config_yesnoask(
            &"UpdateHostKeys",
            source.UpdateHostKeys,
        ));
        result.push_str(&format_config_bool(
            &"UsePrivilegedPort",
            source.UsePrivilegedPort,
        ));
        result.push_str(&format_config_string(&"User", source.User));
        result.push_str(&format_config_pathbuf(
            &"UserKnownHostsFile",
            source.UserKnownHostsFile,
        ));
        result.push_str(&format_config_yesnoask(
            &"VerifyHostKeyDNS",
            source.VerifyHostKeyDNS,
        ));
        result.push_str(&format_config_bool(&"VisualHostKey", source.VisualHostKey));
        result.push_str(&format_config_pathbuf(
            &"XAuthLocation",
            source.XAuthLocation,
        ));

        for (h, h_config) in source.Hosts {
            result.push_str(&format!("\nHost {}\n", h));
            result.push_str(
                textwrap::indent(String::from(h_config).as_str(), &"  ").as_str(),
            );
        }
        for (m, m_config) in source.Matches {
            result.push_str(&format!("\nMatch {}\n", m));
            result.push_str(
                textwrap::indent(String::from(m_config).as_str(), &"  ").as_str(),
            );
        }

        result
    }
}

#[derive(Debug)]
enum Section {
    Global,
    Host,
    Match,
}

#[derive(Clone, Debug, PartialEq)]
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

impl From<YesNoAsk> for String {
    fn from(source: YesNoAsk) -> String {
        match source {
            YesNoAsk::Yes => String::from("yes"),
            YesNoAsk::No => String::from("no"),
            YesNoAsk::Ask => String::from("ask"),
        }
    }
}

fn format_config_bool(key: &str, value: Option<bool>) -> String {
    match value {
        Some(v) => {
            let display = if v { "yes" } else { "no" };
            format!("{} {}\n", key, display)
        }
        None => String::from(""),
    }
}

fn format_config_char(key: &str, value: Option<char>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v),
        None => String::from(""),
    }
}

fn format_config_number(key: &str, value: Option<i32>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v),
        None => String::from(""),
    }
}

fn format_config_pathbuf(key: &str, value: Option<PathBuf>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v.display()),
        None => String::from(""),
    }
}

fn format_config_string(key: &str, value: Option<String>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v),
        None => String::from(""),
    }
}

fn format_config_strings(key: &str, value: Option<Vec<String>>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v.join(",")),
        None => String::from(""),
    }
}


fn format_config_yesnoask(key: &str, value: Option<YesNoAsk>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, String::from(v)),
        None => String::from(""),
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
        text.split(",")
            .into_iter()
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.len() > 0 {
                    Some(String::from(s))
                } else {
                    None
                }
            })
            .collect(),
    )
}

fn parse_config_yesnoask(text: String) -> Option<YesNoAsk> {
    Some(YesNoAsk::from(text.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitor_merges_configs() {
        let mut c1 = Config::new();
        c1.AddKeysToAgent = Some(String::from("confirm"));
        c1.ConnectTimeout = Some(30);
        let mut c2 = Config::new();
        c2.ConnectTimeout = Some(10);

        let c3 = c1 | c2;

        assert_eq!(c3.AddKeysToAgent, Some(String::from("confirm")));
        assert_eq!(c3.ConnectTimeout, Some(10));
    }

    #[test]
    fn config_from_string() {
        let config_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ssh_config.input.txt");

        let extracted = File::open(&config_path).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        let config = Config::from(got.as_str());

        let mut want = Config::new();
        want.AddKeysToAgent = Some(String::from("confirm"));
        want.ConnectTimeout = Some(10);
        want.VisualHostKey = Some(true);

        let mut h1 = Config::new();
        h1.Ciphers = Some(vec![
            String::from("aes128-ctr"),
            String::from("aes192-ctr"),
            String::from("aes256-ctr"),
        ]);
        want.Hosts.insert(String::from("foo"), h1);

        let mut h2 = Config::new();
        h2.IdentityFile = Some(PathBuf::new().join("~").join(".ssh").join("id_rsa"));
        want.Hosts.insert(String::from("bar"), h2);

        let mut m = Config::new();
        m.EscapeChar = Some('%');
        m.StrictHostKeyChecking = Some(YesNoAsk::Ask);
        want.Matches.insert(String::from("exec true"), m);

        assert_eq!(config.Hosts, want.Hosts);
        assert_eq!(config.Matches, want.Matches);
        assert_eq!(config, want);
    }

    #[test]
    fn string_from_config() {
        let config_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ssh_config.output.txt");

        let extracted = File::open(&config_path).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut want = String::new();
        reader.read_to_string(&mut want).unwrap();

        let mut config = Config::new();
        config.AddKeysToAgent = Some(String::from("confirm"));
        config.ConnectTimeout = Some(10);
        config.VisualHostKey = Some(true);

        let mut h1 = Config::new();
        h1.Ciphers = Some(vec![
            String::from("aes128-ctr"),
            String::from("aes192-ctr"),
            String::from("aes256-ctr"),
        ]);
        config.Hosts.insert(String::from("foo"), h1);

        let mut h2 = Config::new();
        h2.IdentityFile = Some(PathBuf::new().join("~").join(".ssh").join("id_rsa"));
        config.Hosts.insert(String::from("bar"), h2);

        let mut m = Config::new();
        m.EscapeChar = Some('%');
        m.StrictHostKeyChecking = Some(YesNoAsk::Ask);
        config.Matches.insert(String::from("exec true"), m);

        assert_eq!(String::from(config), want);
    }
}