use anyhow::Result;
use std::fmt::Debug;

pub trait NfqwsBinding: Debug {
    /// Run nfqws command
    fn run(self) -> Result<()>;

    /// Show nfqws logs
    ///
    /// # Args
    /// * `mode` - Output mode:
    ///     * 0 - off.
    ///     * 1 - stdout/stderr.
    ///     * android - android logcat.
    fn debug<S: AsRef<str>>(&mut self, mode: S) -> &mut Self;

    /// Daemonize
    fn daemon(&mut self) -> &mut Self;

    fn qnum(&mut self, num: u16) -> &mut Self;

    /// Drop root privs
    fn uid<S: AsRef<str>>(&mut self, uid: S) -> &mut Self;

    /// Override fwmark for desync packet. default = 0x40000000 (1073741824)
    ///
    /// # Args
    /// * `value` - fwmark value
    fn dpi_desync_fwmark<S: AsRef<str>>(&mut self, value: S) -> &mut Self;

    /// Install a hosts file to bypass blocking
    ///
    /// # Args
    /// * `path` - Path to hosts.txt
    fn hostlist<S: AsRef<str>>(&mut self, path: S) -> &mut Self;

    /// Set up an exception hosts file to bypass blocking
    ///
    /// # Args
    /// * `path` - Path to hosts-exclude.txt
    fn hostlist_exclude<S: AsRef<str>>(&mut self, path: S) -> &mut Self;

    /// Install a file for automatically added hosts to bypass blocking
    ///
    /// # Args
    /// * `path` - Path to hosts-auto.txt
    fn hostlist_auto<S: AsRef<str>>(&mut self, path: S) -> &mut Self;

    fn hostlist_auto_fail_threshold(&mut self, value: u32) -> &mut Self;

    fn hostlist_auto_fail_time(&mut self, value: u32) -> &mut Self;

    fn hostlist_auto_retrans_threshold(&mut self, value: u32) -> &mut Self;

    /// Append custom args
    ///
    /// # Args
    /// * `args` - Argument iterator
    fn custom_args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
}

pub trait BypassSoftware {
    fn run<I, S>(&self, opt: I) -> Result<()>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;
    fn kill(&self) -> Result<()>;
    fn is_running(&self) -> Result<bool>;
}

pub trait NfqwsBindingFactory {
    type Binding: NfqwsBinding;
    fn create(&self, nfqws_path: &str) -> Self::Binding;
}
