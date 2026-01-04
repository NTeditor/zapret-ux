use std::process::Command;

use super::{NfqwsBinding, NfqwsBindingFactory};
use anyhow::{Result, bail};
use tracing::debug;

macro_rules! add_value_flag {
    ($name:ident, $flag:expr) => {
        fn $name<S: Into<String>>(&mut self, value: S) -> &mut Self {
            let value = value.into();
            debug!(flag = $flag, value = value, "Add flag to nfqws command");
            self.arg($flag);
            self.arg(value);
            self
        }
    };

    ($name:ident, $flag:expr, $type:ident) => {
        fn $name(&mut self, value: $type) -> &mut Self {
            let value = value.to_string();
            debug!(flag = $flag, value = value, "Add flag to nfqws command");
            self.arg($flag);
            self.arg(value);
            self
        }
    };
}

#[derive(Debug)]
pub struct NfqwsCmd {
    path: String,
    args: Vec<String>,
}

impl NfqwsCmd {
    pub(crate) fn new<S: AsRef<str>>(nfqws_path: S) -> Self {
        let nfqws_path = nfqws_path.as_ref();
        Self {
            path: nfqws_path.to_string(),
            args: Vec::new(),
        }
    }

    fn arg<S: Into<String>>(&mut self, arg: S) {
        let arg = arg.into();
        self.args.push(arg);
    }
}

impl NfqwsBinding for NfqwsCmd {
    fn run(self) -> Result<()> {
        let mut cmd = Command::new(&self.path);
        tracing::info!(
            path = self.path,
            args = ?self.args,
            "Running nfqws"
        );

        cmd.args(&self.args);
        let output = cmd.output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr_trim = stderr.trim();
        let stdout_trim = stdout.trim();
        if !output.status.success() {
            tracing::error!(
                stdout = stdout_trim,
                stderr = stderr_trim,
                exitcode = ?output.status.code(),
                "The process exited with a non-zero code",
            );
            bail!("The process exited with a non-zero code");
        }
        tracing::info!(
            stdout = stdout_trim,
            stderr = stderr_trim,
            exitcode = ?output.status.code(),
            "The process completed successfully"
        );
        Ok(())
    }

    fn daemon(&mut self) -> &mut Self {
        self.arg("--daemon");
        self
    }

    add_value_flag!(debug, "--debug");
    add_value_flag!(uid, "--uid");
    add_value_flag!(dpi_desync_fwmark, "--dpi-desync-fwmark");
    add_value_flag!(hostlist, "--hostlist");
    add_value_flag!(hostlist_exclude, "--hostlist-exclude");
    add_value_flag!(hostlist_auto, "--hostlist-auto");
    add_value_flag!(qnum, "--qnum", u16);
    add_value_flag!(
        hostlist_auto_fail_threshold,
        "--hostlist-auto-fail-threshold",
        u32
    );
    add_value_flag!(hostlist_auto_fail_time, "--hostlist-auto-fail-time", u32);
    add_value_flag!(
        hostlist_auto_retrans_threshold,
        "--hostlist-auto-retrans-threshold",
        u32
    );

    fn custom_args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args.into_iter() {
            let arg = arg.into();
            self.arg(arg);
        }
        self
    }
}

pub struct NfqwsCmdFactory;
impl NfqwsBindingFactory for NfqwsCmdFactory {
    type Binding = NfqwsCmd;
    fn create(&self, nfqws_path: &str) -> Self::Binding {
        tracing::debug!(
            nfqws_path = nfqws_path,
            "Creating new NfqwsCmd instance with factory"
        );
        NfqwsCmd::new(nfqws_path)
    }
}

pub(crate) fn pkill(pkill_path: &str, process_name: &str) -> Result<()> {
    tracing::info!(
        pkill_path = pkill_path,
        process_name = process_name,
        "Kill process with pkill",
    );
    let mut cmd = Command::new(pkill_path);
    cmd.arg(process_name);
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout_trim = stdout.trim();
    let stderr_trim = stderr.trim();
    if !output.status.success() {
        tracing::error!(
            stdout = stdout_trim,
            stderr = stderr_trim,
            exitcode = ?output.status.code(),
            "The process exited with a non-zero code",
        );
        bail!("The process exited with a non-zero code");
    }
    tracing::info!(
        stdout = stdout_trim,
        stderr = stderr_trim,
        exitcode = ?output.status.code(),
        "The process completed successfully",
    );
    Ok(())
}

pub(crate) fn pgrep(pgrep_path: &str, process_name: &str) -> Result<bool> {
    tracing::info!(
        pgrep_path = pgrep_path,
        process_name = process_name,
        "Search process with pgrep",
    );
    let mut cmd = Command::new(pgrep_path);
    cmd.arg(process_name);
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout_trim = stdout.trim();
    let stderr_trim = stderr.trim();
    if !output.status.success() {
        if let Some(1) = output.status.code() {
            tracing::info!(
                stdout = stdout_trim,
                stderr = stderr_trim,
                process_name = process_name,
                "Process is not running",
            );
            return Ok(false);
        }
        tracing::error!(
            stdout = stdout_trim,
            stderr = stderr_trim,
            exitcode = ?output.status.code(),
            "The process exited with a non-zero code",
        );
        bail!("The process exited with a non-zero code");
    }
    tracing::info!(
        stdout = stdout_trim,
        stderr = stderr_trim,
        exitcode = ?output.status.code(),
        process_name = process_name,
        "Process is running",
    );
    Ok(true)
}
