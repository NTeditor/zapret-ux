use std::process::Command;

use super::{NfqwsBinding, NfqwsBindingFactory};
use anyhow::{Ok, Result, bail};

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

    fn arg<S: AsRef<str>>(&mut self, arg: S) {
        let arg = arg.as_ref();
        self.args.push(arg.to_string());
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

    fn debug<S: AsRef<str>>(&mut self, mode: S) -> &mut Self {
        let mode = mode.as_ref();
        self.arg(mode);
        self
    }

    fn daemon(&mut self) -> &mut Self {
        self.arg("--daemon");
        self
    }

    fn qnum(&mut self, num: u16) -> &mut Self {
        self.arg("--qnum");
        self.arg(num.to_string());
        self
    }

    fn uid<S: AsRef<str>>(&mut self, uid: S) -> &mut Self {
        let uid = uid.as_ref();
        self.arg("--uid");
        self.arg(uid);
        self
    }

    fn dpi_desync_fwmark<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let value = value.as_ref();
        self.arg("--dpi-desync-fwmark");
        self.arg(value);
        self
    }

    fn hostlist<S: AsRef<str>>(&mut self, path: S) -> &mut Self {
        let path = path.as_ref();
        self.arg("--hostlist");
        self.arg(path);
        self
    }

    fn hostlist_exclude<S: AsRef<str>>(&mut self, path: S) -> &mut Self {
        let path = path.as_ref();
        self.arg("--hostlist-exclude");
        self.arg(path);
        self
    }

    fn hostlist_auto<S: AsRef<str>>(&mut self, path: S) -> &mut Self {
        let path = path.as_ref();
        self.arg("--hostlist-auto");
        self.arg(path);
        self
    }

    fn hostlist_auto_fail_threshold(&mut self, value: u32) -> &mut Self {
        let value = value.to_string();
        self.arg("--hostlist-auto-fail-threshold");
        self.arg(value);
        self
    }

    fn hostlist_auto_fail_time(&mut self, value: u32) -> &mut Self {
        let value = value.to_string();
        self.arg("--hostlist-auto-fail-time");
        self.arg(value);
        self
    }

    fn hostlist_auto_retrans_threshold(&mut self, value: u32) -> &mut Self {
        let value = value.to_string();
        self.arg("--hostlist-auto-retrans-threshold");
        self.arg(value);
        self
    }

    fn custom_args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args.into_iter() {
            let arg = arg.as_ref();
            self.arg(arg.to_string());
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
