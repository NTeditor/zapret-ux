use camino::Utf8PathBuf;
use iptables::{Port, PortSpec, Protocol};
use nfqws::FilterMode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub iptables: ConfigIptables,
    pub nfqws: ConfigNfqws,
    pub mark_supported: bool,
    pub autostart_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigIptables {
    pub iptables_path: Utf8PathBuf,
    pub connbytes_supported: bool,
    pub ports: Vec<PortSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigNfqws {
    pub nfqws_path: Utf8PathBuf,
    pub pgrep_path: Utf8PathBuf,
    pub pkill_path: Utf8PathBuf,
    pub filter_mode: FilterMode,
    pub opt: Vec<String>,
}

impl Default for ConfigIptables {
    fn default() -> Self {
        Self {
            iptables_path: "iptables".into(),
            connbytes_supported: false,
            ports: vec![
                PortSpec::new(Port::Single(80), Protocol::Tcp),
                PortSpec::new(Port::Single(443), Protocol::Udp),
                PortSpec::new(Port::Range(50000, 50099), Protocol::Udp),
            ],
        }
    }
}

impl Default for ConfigNfqws {
    fn default() -> Self {
        Self {
            nfqws_path: "nfqws".into(),
            pgrep_path: "pgrep".into(),
            pkill_path: "pkill".into(),
            filter_mode: FilterMode::AutoHostFile,
            opt: Vec::new(),
        }
    }
}
