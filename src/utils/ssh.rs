use std::{collections::HashMap, fmt::Display, ops::BitOr, path::PathBuf};

use textwrap;
use which;

use crate::lib::ssh::fields::{Duration, YesNo, YesNoAsk, YesNoAskAutoAutoAsk, YesNoDuration};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Config {
    // Hosts contains any Host sections found
    pub Hosts: HashMap<String, Config>,
    // Matches contains any Match sections found
    pub Matches: HashMap<String, Config>,

    // all other fields are valid SSH config directives
    pub AddKeysToAgent: Option<String>, // YesNoAsk with "confirm"
    pub AddressFamily: Option<String>,
    pub BatchMode: Option<YesNo>,
    pub BindAddress: Option<YesNo>,
    pub BindInterface: Option<YesNo>,
    pub CanonicalDomains: Option<YesNo>,
    pub CanonicalizeFallbackLocal: Option<YesNo>,
    pub CanonicalizeHostname: Option<YesNo>,
    pub CanonicalizeMaxDots: Option<YesNo>,
    pub CanonicalisePermittedCNAMEs: Option<YesNo>,
    pub CertificateFile: Option<PathBuf>,
    pub ChallengeResponseAuthentication: Option<YesNo>,
    pub CheckHostIP: Option<YesNo>,
    pub Ciphers: Option<Vec<String>>,
    pub ClearAllForwardings: Option<YesNo>,
    pub Compression: Option<YesNo>,
    pub ConnectionAttempts: Option<i32>,
    pub ConnectTimeout: Option<i32>,
    pub ControlMaster: Option<YesNoAskAutoAutoAsk>,
    pub ControlPath: Option<PathBuf>,
    pub ControlPersist: Option<YesNoDuration>,
    pub DynamicForward: Option<String>,
    pub EnableSSHKeysing: Option<YesNo>,
    pub EscapeChar: Option<char>,
    pub ExitOnForwardFailure: Option<YesNo>,
    pub FingerprintHash: Option<String>,
    pub ForwardAgent: Option<YesNo>,
    pub ForwardX11: Option<YesNo>,
    pub ForwardX11Timeout: Option<Duration>,
    pub ForwardX11Trusted: Option<YesNo>,
    pub GatewayPorts: Option<YesNo>,
    pub GlobalKnownHostsFile: Option<PathBuf>,
    pub GSSAPIAuthentication: Option<YesNo>,
    pub GSSAPIDelegateCredentials: Option<YesNo>,
    pub HashKnownHosts: Option<YesNo>,
    pub HostbasedAuthentication: Option<YesNo>,
    pub HostbasedKeyTypes: Option<String>,
    pub HostKeyAlgorithms: Option<String>,
    pub HostKeyAlias: Option<String>,
    pub Hostname: Option<String>,
    pub IdentitiesOnly: Option<YesNo>,
    pub IdentityAgent: Option<String>,
    pub IdentityFile: Option<PathBuf>,
    pub IgnoreUnknown: Option<String>,
    pub Include: Option<String>,
    pub IPQoS: Option<String>,
    pub KbdInteractiveAuthentication: Option<YesNo>,
    pub KbdInteractiveDevices: Option<String>,
    pub KexAlgorithms: Option<Vec<String>>,
    pub LocalCommand: Option<String>,
    pub LocalForward: Option<String>,
    pub LogLevel: Option<String>,
    pub MACs: Option<Vec<String>>,
    pub NoHostAuthenticationForLocalhost: Option<YesNo>,
    pub NumberOfPasswordPrompts: Option<i32>,
    pub PasswordAuthentication: Option<YesNo>,
    pub PermitLocalCommand: Option<YesNo>,
    pub PKCS11Provider: Option<String>,
    pub Port: Option<i32>,
    pub PreferredAuthentications: Option<String>,
    pub ProxyCommand: Option<String>,
    pub ProxyJump: Option<String>,
    pub ProxyUseFdpass: Option<YesNo>,
    pub PubkeyAcceptedKeyTypes: Option<String>,
    pub PubkeyAuthentication: Option<YesNo>,
    pub RekeyLimit: Option<String>, // "size [duration]", e.g. "5G", "4K 10m", etc
    pub RemoteCommand: Option<String>,
    pub RemoteForward: Option<String>,
    pub RequestTTY: Option<String>,
    pub RevokedHostKeys: Option<String>,
    pub SendEnv: Option<String>,
    pub ServerAliveCountMax: Option<i32>,
    pub ServerAliveInterval: Option<i32>,
    pub StreamLocalBindMask: Option<String>,
    pub StreamLocalBindUnlink: Option<YesNo>,
    pub StrictHostKeyChecking: Option<YesNoAsk>,
    pub SyslogFacility: Option<String>,
    pub TCPKeepAlive: Option<YesNo>,
    pub Tunnel: Option<String>,
    pub TunnelDevice: Option<String>,
    pub UpdateHostKeys: Option<YesNoAsk>,
    pub UsePrivilegedPort: Option<YesNo>,
    pub User: Option<String>,
    pub UserKnownHostsFile: Option<PathBuf>,
    pub VerifyHostKeyDNS: Option<YesNoAsk>,
    pub VisualHostKey: Option<YesNo>,
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

    #[allow(clippy::cyclomatic_complexity)]
    fn bitor(self, rhs: Self) -> Self {
        let mut result = self.clone();

        result.Hosts.extend(rhs.Hosts);
        result.Matches.extend(rhs.Matches);

        result.AddKeysToAgent = rhs.AddKeysToAgent.or(self.AddKeysToAgent);
        result.AddressFamily = rhs.AddressFamily.or(self.AddressFamily);
        result.BatchMode = rhs.BatchMode.or(self.BatchMode);
        result.BindAddress = rhs.BindAddress.or(self.BindAddress);
        result.BindInterface = rhs.BindInterface.or(self.BindInterface);
        result.CanonicalDomains = rhs.CanonicalDomains.or(self.CanonicalDomains);
        result.CanonicalizeFallbackLocal = rhs
            .CanonicalizeFallbackLocal
            .or(self.CanonicalizeFallbackLocal);
        result.CanonicalizeHostname = rhs.CanonicalizeHostname.or(self.CanonicalizeHostname);
        result.CanonicalizeMaxDots = rhs.CanonicalizeMaxDots.or(self.CanonicalizeMaxDots);
        result.CanonicalisePermittedCNAMEs = rhs
            .CanonicalisePermittedCNAMEs
            .or(self.CanonicalisePermittedCNAMEs);
        result.CertificateFile = rhs.CertificateFile.or(self.CertificateFile);
        result.ChallengeResponseAuthentication = rhs
            .ChallengeResponseAuthentication
            .or(self.ChallengeResponseAuthentication);
        result.CheckHostIP = rhs.CheckHostIP.or(self.CheckHostIP);
        result.Ciphers = rhs.Ciphers.or(self.Ciphers);
        result.ClearAllForwardings = rhs.ClearAllForwardings.or(self.ClearAllForwardings);
        result.Compression = rhs.Compression.or(self.Compression);
        result.ConnectionAttempts = rhs.ConnectionAttempts.or(self.ConnectionAttempts);
        result.ConnectTimeout = rhs.ConnectTimeout.or(self.ConnectTimeout);
        result.ControlMaster = rhs.ControlMaster.or(self.ControlMaster);
        result.ControlPath = rhs.ControlPath.or(self.ControlPath);
        result.ControlPersist = rhs.ControlPersist.or(self.ControlPersist);
        result.DynamicForward = rhs.DynamicForward.or(self.DynamicForward);
        result.EnableSSHKeysing = rhs.EnableSSHKeysing.or(self.EnableSSHKeysing);
        result.EscapeChar = rhs.EscapeChar.or(self.EscapeChar);
        result.ExitOnForwardFailure = rhs.ExitOnForwardFailure.or(self.ExitOnForwardFailure);
        result.FingerprintHash = rhs.FingerprintHash.or(self.FingerprintHash);
        result.ForwardAgent = rhs.ForwardAgent.or(self.ForwardAgent);
        result.ForwardX11 = rhs.ForwardX11.or(self.ForwardX11);
        result.ForwardX11Timeout = rhs.ForwardX11Timeout.or(self.ForwardX11Timeout);
        result.ForwardX11Trusted = rhs.ForwardX11Trusted.or(self.ForwardX11Trusted);
        result.GatewayPorts = rhs.GatewayPorts.or(self.GatewayPorts);
        result.GlobalKnownHostsFile = rhs.GlobalKnownHostsFile.or(self.GlobalKnownHostsFile);
        result.GSSAPIAuthentication = rhs.GSSAPIAuthentication.or(self.GSSAPIAuthentication);
        result.GSSAPIDelegateCredentials = rhs
            .GSSAPIDelegateCredentials
            .or(self.GSSAPIDelegateCredentials);
        result.HashKnownHosts = rhs.HashKnownHosts.or(self.HashKnownHosts);
        result.HostbasedAuthentication =
            rhs.HostbasedAuthentication.or(self.HostbasedAuthentication);
        result.HostbasedKeyTypes = rhs.HostbasedKeyTypes.or(self.HostbasedKeyTypes);
        result.HostKeyAlgorithms = rhs.HostKeyAlgorithms.or(self.HostKeyAlgorithms);
        result.HostKeyAlias = rhs.HostKeyAlias.or(self.HostKeyAlias);
        result.Hostname = rhs.Hostname.or(self.Hostname);
        result.IdentitiesOnly = rhs.IdentitiesOnly.or(self.IdentitiesOnly);
        result.IdentityAgent = rhs.IdentityAgent.or(self.IdentityAgent);
        result.IdentityFile = rhs.IdentityFile.or(self.IdentityFile);
        result.IgnoreUnknown = rhs.IgnoreUnknown.or(self.IgnoreUnknown);
        result.Include = rhs.Include.or(self.Include);
        result.IPQoS = rhs.IPQoS.or(self.IPQoS);
        result.KbdInteractiveAuthentication = rhs
            .KbdInteractiveAuthentication
            .or(self.KbdInteractiveAuthentication);
        result.KbdInteractiveDevices = rhs.KbdInteractiveDevices.or(self.KbdInteractiveDevices);
        result.KexAlgorithms = rhs.KexAlgorithms.or(self.KexAlgorithms);
        result.LocalCommand = rhs.LocalCommand.or(self.LocalCommand);
        result.LocalForward = rhs.LocalForward.or(self.LocalForward);
        result.LogLevel = rhs.LogLevel.or(self.LogLevel);
        result.MACs = rhs.MACs.or(self.MACs);
        result.NoHostAuthenticationForLocalhost = rhs
            .NoHostAuthenticationForLocalhost
            .or(self.NoHostAuthenticationForLocalhost);
        result.NumberOfPasswordPrompts =
            rhs.NumberOfPasswordPrompts.or(self.NumberOfPasswordPrompts);
        result.PasswordAuthentication = rhs.PasswordAuthentication.or(self.PasswordAuthentication);
        result.PermitLocalCommand = rhs.PermitLocalCommand.or(self.PermitLocalCommand);
        result.PKCS11Provider = rhs.PKCS11Provider.or(self.PKCS11Provider);
        result.Port = rhs.Port.or(self.Port);
        result.PreferredAuthentications = rhs
            .PreferredAuthentications
            .or(self.PreferredAuthentications);
        result.ProxyCommand = rhs.ProxyCommand.or(self.ProxyCommand);
        result.ProxyJump = rhs.ProxyJump.or(self.ProxyJump);
        result.ProxyUseFdpass = rhs.ProxyUseFdpass.or(self.ProxyUseFdpass);
        result.PubkeyAcceptedKeyTypes = rhs.PubkeyAcceptedKeyTypes.or(self.PubkeyAcceptedKeyTypes);
        result.PubkeyAuthentication = rhs.PubkeyAuthentication.or(self.PubkeyAuthentication);
        result.RekeyLimit = rhs.RekeyLimit.or(self.RekeyLimit);
        result.RemoteCommand = rhs.RemoteCommand.or(self.RemoteCommand);
        result.RemoteForward = rhs.RemoteForward.or(self.RemoteForward);
        result.RequestTTY = rhs.RequestTTY.or(self.RequestTTY);
        result.RevokedHostKeys = rhs.RevokedHostKeys.or(self.RevokedHostKeys);
        result.SendEnv = rhs.SendEnv.or(self.SendEnv);
        result.ServerAliveCountMax = rhs.ServerAliveCountMax.or(self.ServerAliveCountMax);
        result.ServerAliveInterval = rhs.ServerAliveInterval.or(self.ServerAliveInterval);
        result.StreamLocalBindMask = rhs.StreamLocalBindMask.or(self.StreamLocalBindMask);
        result.StreamLocalBindUnlink = rhs.StreamLocalBindUnlink.or(self.StreamLocalBindUnlink);
        result.StrictHostKeyChecking = rhs.StrictHostKeyChecking.or(self.StrictHostKeyChecking);
        result.SyslogFacility = rhs.SyslogFacility.or(self.SyslogFacility);
        result.TCPKeepAlive = rhs.TCPKeepAlive.or(self.TCPKeepAlive);
        result.Tunnel = rhs.Tunnel.or(self.Tunnel);
        result.TunnelDevice = rhs.TunnelDevice.or(self.TunnelDevice);
        result.UpdateHostKeys = rhs.UpdateHostKeys.or(self.UpdateHostKeys);
        result.UsePrivilegedPort = rhs.UsePrivilegedPort.or(self.UsePrivilegedPort);
        result.User = rhs.User.or(self.User);
        result.UserKnownHostsFile = rhs.UserKnownHostsFile.or(self.UserKnownHostsFile);
        result.VerifyHostKeyDNS = rhs.VerifyHostKeyDNS.or(self.VerifyHostKeyDNS);
        result.VisualHostKey = rhs.VisualHostKey.or(self.VisualHostKey);
        result.XAuthLocation = rhs.XAuthLocation.or(self.XAuthLocation);

        result
    }
}

impl<S> From<S> for Config
where
    S: Into<String> + AsRef<str>,
{
    fn from(source: S) -> Self {
        let mut config = Config::new();

        let mut section_key: String = String::from("");
        let mut section_type: Section = Section::Global;

        for line in source.as_ref().lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.chars().next().unwrap_or('#') == '#' {
                continue; // skip empty lines and comments
                          // TODO: one day support keeping comments
            }

            let mut split = trimmed.splitn(2, ' ');
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
                "BatchMode" => target.BatchMode = Some(YesNo::from(value)),
                "BindAddress" => target.BindAddress = Some(YesNo::from(value)),
                "BindInterface" => target.BindInterface = Some(YesNo::from(value)),
                "CanonicalDomains" => target.CanonicalDomains = Some(YesNo::from(value)),
                "CanonicalizeFallbackLocal" => {
                    target.CanonicalizeFallbackLocal = Some(YesNo::from(value))
                }
                "CanonicalizeHostname" => target.CanonicalizeHostname = Some(YesNo::from(value)),
                "CanonicalizeMaxDots" => target.CanonicalizeMaxDots = Some(YesNo::from(value)),
                "CanonicalisePermittedCNAMEs" => {
                    target.CanonicalisePermittedCNAMEs = Some(YesNo::from(value))
                }
                "CertificateFile" => target.CertificateFile = parse_config_pathbuf(value),
                "ChallengeResponseAuthentication" => {
                    target.ChallengeResponseAuthentication = Some(YesNo::from(value))
                }
                "CheckHostIP" => target.CheckHostIP = Some(YesNo::from(value)),
                "Ciphers" => target.Ciphers = parse_config_strings(value),
                "ClearAllForwardings" => target.ClearAllForwardings = Some(YesNo::from(value)),
                "Compression" => target.Compression = Some(YesNo::from(value)),
                "ConnectionAttempts" => target.ConnectionAttempts = parse_config_number(value),
                "ConnectTimeout" => target.ConnectTimeout = parse_config_number(value),
                "ControlMaster" => target.ControlMaster = Some(YesNoAskAutoAutoAsk::from(value)),
                "ControlPath" => target.ControlPath = parse_config_pathbuf(value),
                "ControlPersist" => target.ControlPersist = Some(YesNoDuration::from(value)),
                "DynamicForward" => target.DynamicForward = Some(value),
                "EnableSSHKeysing" => target.EnableSSHKeysing = Some(YesNo::from(value)),
                "EscapeChar" => target.EscapeChar = parse_config_char(value),
                "ExitOnForwardFailure" => target.ExitOnForwardFailure = Some(YesNo::from(value)),
                "FingerprintHash" => target.FingerprintHash = Some(value),
                "ForwardAgent" => target.ForwardAgent = Some(YesNo::from(value)),
                "ForwardX11" => target.ForwardX11 = Some(YesNo::from(value)),
                "ForwardX11Timeout" => target.ForwardX11Timeout = Some(Duration::from(value)),
                "ForwardX11Trusted" => target.ForwardX11Trusted = Some(YesNo::from(value)),
                "GatewayPorts" => target.GatewayPorts = Some(YesNo::from(value)),
                "GlobalKnownHostsFile" => target.GlobalKnownHostsFile = parse_config_pathbuf(value),
                "GSSAPIAuthentication" => target.GSSAPIAuthentication = Some(YesNo::from(value)),
                "GSSAPIDelegateCredentials" => {
                    target.GSSAPIDelegateCredentials = Some(YesNo::from(value))
                }
                "HashKnownHosts" => target.HashKnownHosts = Some(YesNo::from(value)),
                "HostbasedAuthentication" => {
                    target.HostbasedAuthentication = Some(YesNo::from(value))
                }
                "HostbasedKeyTypes" => target.HostbasedKeyTypes = Some(value),
                "HostKeyAlgorithms" => target.HostKeyAlgorithms = Some(value),
                "HostKeyAlias" => target.HostKeyAlias = Some(value),
                "Hostname" => target.Hostname = Some(value),
                "IdentitiesOnly" => target.IdentitiesOnly = Some(YesNo::from(value)),
                "IdentityAgent" => target.IdentityAgent = Some(value),
                "IdentityFile" => target.IdentityFile = parse_config_pathbuf(value),
                "IgnoreUnknown" => target.IgnoreUnknown = Some(value),
                "Include" => target.Include = Some(value),
                "IPQoS" => target.IPQoS = Some(value),
                "KbdInteractiveAuthentication" => {
                    target.KbdInteractiveAuthentication = Some(YesNo::from(value))
                }
                "KbdInteractiveDevices" => target.KbdInteractiveDevices = Some(value),
                "KexAlgorithms" => target.KexAlgorithms = parse_config_strings(value),
                "LocalCommand" => target.LocalCommand = Some(value),
                "LocalForward" => target.LocalForward = Some(value),
                "LogLevel" => target.LogLevel = Some(value),
                "MACs" => target.MACs = parse_config_strings(value),
                "NoHostAuthenticationForLocalhost" => {
                    target.NoHostAuthenticationForLocalhost = Some(YesNo::from(value))
                }
                "NumberOfPasswordPrompts" => {
                    target.NumberOfPasswordPrompts = parse_config_number(value)
                }
                "PasswordAuthentication" => {
                    target.PasswordAuthentication = Some(YesNo::from(value))
                }
                "PermitLocalCommand" => target.PermitLocalCommand = Some(YesNo::from(value)),
                "PKCS11Provider" => target.PKCS11Provider = Some(value),
                "Port" => target.Port = parse_config_number(value),
                "PreferredAuthentications" => target.PreferredAuthentications = Some(value),
                "ProxyCommand" => target.ProxyCommand = Some(value),
                "ProxyJump" => target.ProxyJump = Some(value),
                "ProxyUseFdpass" => target.ProxyUseFdpass = Some(YesNo::from(value)),
                "PubkeyAcceptedKeyTypes" => target.PubkeyAcceptedKeyTypes = Some(value),
                "PubkeyAuthentication" => target.PubkeyAuthentication = Some(YesNo::from(value)),
                "RekeyLimit" => target.RekeyLimit = Some(value),
                "RemoteCommand" => target.RemoteCommand = Some(value),
                "RemoteForward" => target.RemoteForward = Some(value),
                "RequestTTY" => target.RequestTTY = Some(value),
                "RevokedHostKeys" => target.RevokedHostKeys = Some(value),
                "SendEnv" => target.SendEnv = Some(value),
                "ServerAliveCountMax" => target.ServerAliveCountMax = parse_config_number(value),
                "ServerAliveInterval" => target.ServerAliveInterval = parse_config_number(value),
                "StreamLocalBindMask" => target.StreamLocalBindMask = Some(value),
                "StreamLocalBindUnlink" => target.StreamLocalBindUnlink = Some(YesNo::from(value)),
                "StrictHostKeyChecking" => {
                    target.StrictHostKeyChecking = Some(YesNoAsk::from(value))
                }
                "SyslogFacility" => target.SyslogFacility = Some(value),
                "TCPKeepAlive" => target.TCPKeepAlive = Some(YesNo::from(value)),
                "Tunnel" => target.Tunnel = Some(value),
                "TunnelDevice" => target.TunnelDevice = Some(value),
                "UpdateHostKeys" => target.UpdateHostKeys = Some(YesNoAsk::from(value)),
                "UsePrivilegedPort" => target.UsePrivilegedPort = Some(YesNo::from(value)),
                "User" => target.User = Some(value),
                "UserKnownHostsFile" => target.UserKnownHostsFile = parse_config_pathbuf(value),
                "VerifyHostKeyDNS" => target.VerifyHostKeyDNS = Some(YesNoAsk::from(value)),
                "VisualHostKey" => target.VisualHostKey = Some(YesNo::from(value)),
                "XAuthLocation" => target.XAuthLocation = parse_config_pathbuf(value),
                _ => {}
            }
        }

        config
    }
}

impl<'a> From<&'a Config> for String {
    fn from(source: &Config) -> String {
        let mut result = String::from("");

        result.push_str(&format_config("AddKeysToAgent", &source.AddKeysToAgent));
        result.push_str(&format_config("AddressFamily", &source.AddressFamily));
        result.push_str(&format_config("BatchMode", &source.BatchMode));
        result.push_str(&format_config("BindAddress", &source.BindAddress));
        result.push_str(&format_config("BindInterface", &source.BindInterface));
        result.push_str(&format_config("CanonicalDomains", &source.CanonicalDomains));
        result.push_str(&format_config(
            "CanonicalizeFallbackLocal",
            &source.CanonicalizeFallbackLocal,
        ));
        result.push_str(&format_config(
            "CanonicalizeHostname",
            &source.CanonicalizeHostname,
        ));
        result.push_str(&format_config(
            "CanonicalizeMaxDots",
            &source.CanonicalizeMaxDots,
        ));
        result.push_str(&format_config(
            "CanonicalisePermittedCNAMEs",
            &source.CanonicalisePermittedCNAMEs,
        ));
        result.push_str(&format_config_pathbuf(
            &"CertificateFile",
            &source.CertificateFile,
        ));
        result.push_str(&format_config(
            "ChallengeResponseAuthentication",
            &source.ChallengeResponseAuthentication,
        ));
        result.push_str(&format_config("CheckHostIP", &source.CheckHostIP));
        result.push_str(&format_config_strings(&"Ciphers", &source.Ciphers));
        result.push_str(&format_config(
            "ClearAllForwardings",
            &source.ClearAllForwardings,
        ));
        result.push_str(&format_config("Compression", &source.Compression));
        result.push_str(&format_config(
            "ConnectionAttempts",
            &source.ConnectionAttempts,
        ));
        result.push_str(&format_config("ConnectTimeout", &source.ConnectTimeout));
        result.push_str(&format_config("ControlMaster", &source.ControlMaster));
        result.push_str(&format_config_pathbuf(&"ControlPath", &source.ControlPath));
        result.push_str(&format_config("ControlPersist", &source.ControlPersist));
        result.push_str(&format_config("DynamicForward", &source.DynamicForward));
        result.push_str(&format_config("EnableSSHKeysing", &source.EnableSSHKeysing));
        result.push_str(&format_config("EscapeChar", &source.EscapeChar));
        result.push_str(&format_config(
            "ExitOnForwardFailure",
            &source.ExitOnForwardFailure,
        ));
        result.push_str(&format_config("FingerprintHash", &source.FingerprintHash));
        result.push_str(&format_config("ForwardAgent", &source.ForwardAgent));
        result.push_str(&format_config("ForwardX11", &source.ForwardX11));
        result.push_str(&format_config(
            "ForwardX11Timeout",
            &source.ForwardX11Timeout,
        ));
        result.push_str(&format_config(
            "ForwardX11Trusted",
            &source.ForwardX11Trusted,
        ));
        result.push_str(&format_config("GatewayPorts", &source.GatewayPorts));
        result.push_str(&format_config_pathbuf(
            &"GlobalKnownHostsFile",
            &source.GlobalKnownHostsFile,
        ));
        result.push_str(&format_config(
            "GSSAPIAuthentication",
            &source.GSSAPIAuthentication,
        ));
        result.push_str(&format_config(
            "GSSAPIDelegateCredentials",
            &source.GSSAPIDelegateCredentials,
        ));
        result.push_str(&format_config("HashKnownHosts", &source.HashKnownHosts));
        result.push_str(&format_config(
            "HostbasedAuthentication",
            &source.HostbasedAuthentication,
        ));
        result.push_str(&format_config(
            "HostbasedKeyTypes",
            &source.HostbasedKeyTypes,
        ));
        result.push_str(&format_config(
            "HostKeyAlgorithms",
            &source.HostKeyAlgorithms,
        ));
        result.push_str(&format_config("HostKeyAlias", &source.HostKeyAlias));
        result.push_str(&format_config("Hostname", &source.Hostname));
        result.push_str(&format_config("IdentitiesOnly", &source.IdentitiesOnly));
        result.push_str(&format_config("IdentityAgent", &source.IdentityAgent));
        result.push_str(&format_config_pathbuf(
            &"IdentityFile",
            &source.IdentityFile,
        ));
        result.push_str(&format_config("IgnoreUnknown", &source.IgnoreUnknown));
        result.push_str(&format_config("Include", &source.Include));
        result.push_str(&format_config("IPQoS", &source.IPQoS));
        result.push_str(&format_config(
            "KbdInteractiveAuthentication",
            &source.KbdInteractiveAuthentication,
        ));
        result.push_str(&format_config(
            "KbdInteractiveDevices",
            &source.KbdInteractiveDevices,
        ));
        result.push_str(&format_config_strings(
            &"KexAlgorithms",
            &source.KexAlgorithms,
        ));
        result.push_str(&format_config("LocalCommand", &source.LocalCommand));
        result.push_str(&format_config("LocalForward", &source.LocalForward));
        result.push_str(&format_config("LogLevel", &source.LogLevel));
        result.push_str(&format_config_strings(&"MACs", &source.MACs));
        result.push_str(&format_config(
            "NoHostAuthenticationForLocalhost",
            &source.NoHostAuthenticationForLocalhost,
        ));
        result.push_str(&format_config(
            "NumberOfPasswordPrompts",
            &source.NumberOfPasswordPrompts,
        ));
        result.push_str(&format_config(
            "PasswordAuthentication",
            &source.PasswordAuthentication,
        ));
        result.push_str(&format_config(
            "PermitLocalCommand",
            &source.PermitLocalCommand,
        ));
        result.push_str(&format_config("PKCS11Provider", &source.PKCS11Provider));
        result.push_str(&format_config("Port", &source.Port));
        result.push_str(&format_config(
            "PreferredAuthentications",
            &source.PreferredAuthentications,
        ));
        result.push_str(&format_config("ProxyCommand", &source.ProxyCommand));
        result.push_str(&format_config("ProxyJump", &source.ProxyJump));
        result.push_str(&format_config("ProxyUseFdpass", &source.ProxyUseFdpass));
        result.push_str(&format_config(
            "PubkeyAcceptedKeyTypes",
            &source.PubkeyAcceptedKeyTypes,
        ));
        result.push_str(&format_config(
            "PubkeyAuthentication",
            &source.PubkeyAuthentication,
        ));
        result.push_str(&format_config("RekeyLimit", &source.RekeyLimit));
        result.push_str(&format_config("RemoteCommand", &source.RemoteCommand));
        result.push_str(&format_config("RemoteForward", &source.RemoteForward));
        result.push_str(&format_config("RequestTTY", &source.RequestTTY));
        result.push_str(&format_config("RevokedHostKeys", &source.RevokedHostKeys));
        result.push_str(&format_config("SendEnv", &source.SendEnv));
        result.push_str(&format_config(
            "ServerAliveCountMax",
            &source.ServerAliveCountMax,
        ));
        result.push_str(&format_config(
            "ServerAliveInterval",
            &source.ServerAliveInterval,
        ));
        result.push_str(&format_config(
            "StreamLocalBindMask",
            &source.StreamLocalBindMask,
        ));
        result.push_str(&format_config(
            "StreamLocalBindUnlink",
            &source.StreamLocalBindUnlink,
        ));
        result.push_str(&format_config(
            "StrictHostKeyChecking",
            &source.StrictHostKeyChecking,
        ));
        result.push_str(&format_config("SyslogFacility", &source.SyslogFacility));
        result.push_str(&format_config("TCPKeepAlive", &source.TCPKeepAlive));
        result.push_str(&format_config("Tunnel", &source.Tunnel));
        result.push_str(&format_config("TunnelDevice", &source.TunnelDevice));
        result.push_str(&format_config("UpdateHostKeys", &source.UpdateHostKeys));
        result.push_str(&format_config(
            "UsePrivilegedPort",
            &source.UsePrivilegedPort,
        ));
        result.push_str(&format_config("User", &source.User));
        result.push_str(&format_config_pathbuf(
            &"UserKnownHostsFile",
            &source.UserKnownHostsFile,
        ));
        result.push_str(&format_config("VerifyHostKeyDNS", &source.VerifyHostKeyDNS));
        result.push_str(&format_config("VisualHostKey", &source.VisualHostKey));
        result.push_str(&format_config_pathbuf(
            &"XAuthLocation",
            &source.XAuthLocation,
        ));

        let host_keys = source.Hosts.keys();
        let mut sorted_host_keys: Vec<&String> = host_keys.collect();
        sorted_host_keys.sort();
        for h in sorted_host_keys {
            let h_config = &source.Hosts[h];
            result.push_str(&format!("\nHost {}\n", h));
            result.push_str(textwrap::indent(String::from(h_config).as_str(), &"  ").as_str());
        }

        for (m, m_config) in &source.Matches {
            result.push_str(&format!("\nMatch {}\n", m));
            result.push_str(textwrap::indent(String::from(m_config).as_str(), &"  ").as_str());
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

fn format_config<D, S>(key: S, value: &Option<D>) -> String
where
    D: Display,
    S: Into<String> + AsRef<str>,
{
    match value {
        Some(v) => format!("{} {}\n", key.as_ref(), v),
        None => String::from(""),
    }
}

fn format_config_pathbuf(key: &str, value: &Option<PathBuf>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v.display()),
        None => String::from(""),
    }
}

fn format_config_strings(key: &str, value: &Option<Vec<String>>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v.join(",")),
        None => String::from(""),
    }
}

pub fn has_ssh() -> bool {
    which::which("ssh").is_ok()
}

#[allow(clippy::needless_pass_by_value)]
fn parse_config_char<S>(text: S) -> Option<char>
where
    S: Into<String> + AsRef<str>,
{
    Some(text.as_ref().chars().next().unwrap_or('~'))
}

#[allow(clippy::needless_pass_by_value)]
fn parse_config_number<S>(text: S) -> Option<i32>
where
    S: Into<String> + AsRef<str>,
{
    match i32::from_str_radix(text.as_ref(), 10) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

#[allow(clippy::needless_pass_by_value)]
fn parse_config_pathbuf<S>(text: S) -> Option<PathBuf>
where
    S: Into<String> + AsRef<str>,
{
    Some(PathBuf::new().join(text.as_ref()))
}

#[allow(clippy::needless_pass_by_value)]
fn parse_config_strings<S>(text: S) -> Option<Vec<String>>
where
    S: Into<String> + AsRef<str>,
{
    Some(
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
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use std::{env::consts::OS, fs, path::Path};

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

        let got = fs::read_to_string(&config_path).unwrap();
        let want = example_config();

        let config = Config::from(got.as_str());

        assert_eq!(config.Hosts, want.Hosts);
        assert_eq!(config.Matches, want.Matches);
        assert_eq!(config, want);
    }

    fn example_config() -> Config {
        let mut cfg = Config::new();
        cfg.AddKeysToAgent = Some(String::from("confirm"));
        cfg.ConnectTimeout = Some(10);
        cfg.VisualHostKey = Some(YesNo::Yes);

        let mut h1 = Config::new();
        h1.Ciphers = Some(vec![
            String::from("aes128-ctr"),
            String::from("aes192-ctr"),
            String::from("aes256-ctr"),
        ]);
        h1.ControlMaster = Some(YesNoAskAutoAutoAsk::Auto);
        h1.ControlPersist = Some(YesNoDuration::Duration(Duration::Time(String::from("30s"))));
        h1.Hostname = Some(String::from("foo.example"));
        cfg.Hosts.insert(String::from("foo"), h1);

        let mut h2 = Config::new();
        h2.Hostname = Some(String::from("bar.example"));
        h2.IdentityFile = Some(PathBuf::new().join("~").join(".ssh").join("id_rsa"));
        cfg.Hosts.insert(String::from("bar"), h2);

        let mut m = Config::new();
        m.EscapeChar = Some('%');
        m.StrictHostKeyChecking = Some(YesNoAsk::Ask);
        cfg.Matches.insert(String::from("exec true"), m);

        cfg
    }

    #[test]
    fn string_from_config() {
        let config_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ssh_config.output.txt");

        let want = if OS == "windows" {
            fs::read_to_string(&config_path)
                .unwrap()
                .replace("IdentityFile ~/.ssh/id_rsa", "IdentityFile ~\\.ssh\\id_rsa")
        } else {
            fs::read_to_string(&config_path).unwrap()
        };
        let config = example_config();

        assert_eq!(String::from(&config), want);
    }
}
