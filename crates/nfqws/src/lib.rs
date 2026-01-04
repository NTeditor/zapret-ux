mod binding;
mod enums;
mod traits;

use anyhow::{Context, Result};
use binding::*;
pub use enums::*;
pub use traits::*;

const QUEUE_NUM: u16 = 200;
const NFQWS_LOGMODE: &str = "1";
const FWMARK_VALUE: &str = "0x40000000";
const UID_VALUE: &str = "0:0";
const NFQWS_PROCESS_NAME: &str = "nfqws";

#[derive(Debug)]
pub struct Nfqws<F, PG, PK>
where
    F: NfqwsBindingFactory,
    PG: Fn(&str, &str) -> Result<bool>,
    PK: Fn(&str, &str) -> Result<()>,
{
    nfqws_path: String,
    pgrep_path: String,
    pkill_path: String,
    mark_supported: bool,
    filter_mode: FilterMode,
    pgrep: PG,
    pkill: PK,
    factory: F,
}

impl Nfqws<NfqwsCmdFactory, fn(&str, &str) -> Result<bool>, fn(&str, &str) -> Result<()>> {
    pub fn new<S, PGS, PKS>(
        nfqws_path: S,
        pgrep_path: PGS,
        pkill_path: PKS,
        mark_supported: bool,
        filter_mode: FilterMode,
    ) -> Self
    where
        S: AsRef<str>,
        PGS: AsRef<str>,
        PKS: AsRef<str>,
    {
        let nfqws_path = nfqws_path.as_ref();
        let pgrep_path = pgrep_path.as_ref();
        let pkill_path = pkill_path.as_ref();
        let factory = NfqwsCmdFactory;

        let pgrep = binding::pgrep;
        let pkill = binding::pkill;

        Self {
            nfqws_path: nfqws_path.to_string(),
            pgrep_path: pgrep_path.to_string(),
            pkill_path: pkill_path.to_string(),
            mark_supported,
            filter_mode,
            pgrep,
            pkill,
            factory,
        }
    }
}

impl<F, PG, PK> BypassSoftware for Nfqws<F, PG, PK>
where
    F: NfqwsBindingFactory,
    PG: Fn(&str, &str) -> Result<bool>,
    PK: Fn(&str, &str) -> Result<()>,
{
    fn run<I, S>(&self, opt: I) -> Result<()>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut binding = self.factory.create(&self.nfqws_path);
        binding
            .debug(NFQWS_LOGMODE)
            .daemon()
            .qnum(QUEUE_NUM)
            .uid(UID_VALUE);
        if self.mark_supported {
            binding.dpi_desync_fwmark(FWMARK_VALUE);
        }
        self.parse_opt(&mut binding, opt);
        binding.run()?;
        Ok(())
    }

    fn kill(&self) -> Result<()> {
        (self.pkill)(&self.pkill_path, NFQWS_PROCESS_NAME)
            .context("Failed to kill nfqws process")?;
        Ok(())
    }

    fn is_running(&self) -> Result<bool> {
        let is_running = (self.pgrep)(&self.pgrep_path, NFQWS_PROCESS_NAME)
            .context("Failed to search nfqws process")?;
        Ok(is_running)
    }
}

impl<F, PG, PK> Nfqws<F, PG, PK>
where
    F: NfqwsBindingFactory,
    PG: Fn(&str, &str) -> Result<bool>,
    PK: Fn(&str, &str) -> Result<()>,
{
    fn parse_opt<B: NfqwsBinding, S, I>(&self, binging: &mut B, opt: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        #[cfg(target_os = "android")]
        const HOSTLIST_PATH: &str = "/data/adb/zapret-ux/hosts.txt";
        #[cfg(target_os = "android")]
        const HOSTLIST_EXCLUDE_PATH: &str = "/data/adb/zapret-ux/hosts-exclude.txt";
        #[cfg(target_os = "android")]
        const HOSTLIST_AUTO_PATH: &str = "/data/adb/zapret-ux/hosts-auto.txt";

        #[cfg(not(target_os = "android"))]
        const HOSTLIST_PATH: &str = "/opt/zapret-ux/hosts.txt";
        #[cfg(not(target_os = "android"))]
        const HOSTLIST_EXCLUDE_PATH: &str = "/opt/zapret-ux/hosts-exclude.txt";
        #[cfg(not(target_os = "android"))]
        const HOSTLIST_AUTO_PATH: &str = "/opt/zapret-ux/hosts-auto.txt";

        for arg in opt {
            let arg = arg.as_ref();
            if arg == "<FILTER_MODE>" {
                match self.filter_mode {
                    FilterMode::AutoHostFile => {
                        binging
                            .hostlist(HOSTLIST_PATH)
                            .hostlist_exclude(HOSTLIST_EXCLUDE_PATH)
                            .hostlist_auto(HOSTLIST_AUTO_PATH)
                            .hostlist_auto_fail_threshold(3)
                            .hostlist_auto_fail_time(60)
                            .hostlist_auto_retrans_threshold(3);
                    }
                    FilterMode::HostFile => {
                        binging
                            .hostlist(HOSTLIST_PATH)
                            .hostlist_exclude(HOSTLIST_EXCLUDE_PATH);
                    }
                    FilterMode::None => {}
                }
            } else {
                binging.custom_args([arg]);
            }
        }
    }
}
