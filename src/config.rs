use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "android")]
const DEFAULT_NFQWS_PATH: &str = "/data/adb/modules/zapret-ux/nfqws";
#[cfg(not(target_os = "android"))]
const DEFAULT_NFQWS_PATH: &str = "/tmp/nfqws";

#[cfg(target_os = "android")]
const DEFAULT_IPTABLES_PATH: &str = "/system/bin/iptables";
#[cfg(not(target_os = "android"))]
const DEFAULT_IPTABLES_PATH: &str = "/usr/bin/iptables";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub nfqws: NfqwsConfig,
    pub iptables: IptablesConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NfqwsConfig {
    pub binary_path: Utf8PathBuf,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IptablesConfig {
    pub binary_path: Utf8PathBuf,
    pub multiport_support: bool,
    pub mark_support: bool,
    pub connbytes_support: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nfqws: NfqwsConfig {
                binary_path: DEFAULT_NFQWS_PATH.into(),
                args: Vec::default(),
            },
            iptables: IptablesConfig {
                binary_path: DEFAULT_IPTABLES_PATH.into(),
                multiport_support: false,
                connbytes_support: false,
                mark_support: true,
            },
        }
    }
}
