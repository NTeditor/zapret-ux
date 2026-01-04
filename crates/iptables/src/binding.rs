use super::*;
use anyhow::Result;
use std::process::Command;
use tracing::debug;

macro_rules! add_value_flag {
    ($name:ident, $flag:expr) => {
        fn $name(&mut self, value: &str) -> &mut Self {
            debug!(flag = $flag, value = value, "Add flag to iptables command",);
            self.arg($flag);
            self.arg(value);
            self
        }
    };
}

#[derive(Debug)]
pub struct IptablesCmd {
    path: String,
    command: Vec<String>,
}

impl IptablesCmd {
    pub(crate) fn new<S: AsRef<str>>(iptables_path: S) -> Self {
        let iptables_path = iptables_path.as_ref();
        Self {
            path: iptables_path.to_string(),
            command: Vec::new(),
        }
    }

    fn arg<S: AsRef<str>>(&mut self, arg: S) {
        let arg = arg.as_ref();
        self.command.push(arg.to_string());
    }
}

impl IptablesBinding for IptablesCmd {
    fn run(self) -> Result<(), BindingError> {
        let mut cmd = Command::new(&self.path);
        tracing::info!(
            path = self.path,
            args = ?self.command,
            "Running iptables"
        );

        cmd.args(&self.command);
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr_trim = stderr.trim();
            let stdout_trim = stdout.trim();
            if stderr_trim.contains("Directory not empty") {
                return Err(BindingError::DirectoryNotEmpty {
                    stderr: stderr_trim.to_owned(),
                    stdout: stdout_trim.to_owned(),
                });
            }
            if stderr_trim.contains("Chain already exists") {
                return Err(BindingError::ChainAlreadyExists {
                    stderr: stderr_trim.to_owned(),
                    stdout: stdout_trim.to_owned(),
                });
            }
            if stderr_trim.contains("No chain/target/match by that name") {
                return Err(BindingError::NotFoundByThatName {
                    stderr: stderr_trim.to_owned(),
                    stdout: stdout_trim.to_owned(),
                });
            }
            return Err(BindingError::Unknown {
                stderr: stderr_trim.to_owned(),
                stdout: stdout_trim.to_owned(),
            });
        }
        Ok(())
    }

    add_value_flag!(module, "--match");
    add_value_flag!(insert, "--insert");
    add_value_flag!(new_chain, "--new");
    add_value_flag!(delete_chain, "--delete-chain");
    add_value_flag!(delete, "--delete");
    add_value_flag!(flush, "--flush");
    add_value_flag!(table, "--table");
    add_value_flag!(protocol, "--protocol");
    add_value_flag!(jump, "--jump");
    add_value_flag!(dport, "--dport");

    fn mark(&mut self, value: &str, invert: Option<bool>) -> &mut Self {
        debug!(
            flag = "--mark",
            value = value,
            invert = invert,
            "Add flag to iptables command",
        );
        if let Some(true) = invert {
            self.arg("!");
        }
        self.arg("--mark");
        self.arg(value);
        self
    }

    fn connbytes(&mut self, value: &str, invert: Option<bool>) -> &mut Self {
        debug!(
            flag = "--connbytes",
            value = value,
            invert = invert,
            "Add flag to iptables command",
        );
        if let Some(true) = invert {
            self.arg("!");
        }
        self.arg("--connbytes");
        self.arg(value);
        self
    }

    add_value_flag!(connbytes_dir, "--connbytes-dir");
    add_value_flag!(connbytes_mode, "--connbytes-mode");

    fn queue_num(&mut self, value: u16) -> &mut Self {
        debug!(
            flag = "--queue-num",
            value = value,
            "Add flag to iptables command",
        );
        let value = value.to_string();
        self.arg("--queue-num");
        self.arg(value);
        self
    }

    fn queue_bypass(&mut self) -> &mut Self {
        debug!(flag = "--queue-bypass", "Add flag to iptables command");
        self.arg("--queue-bypass");
        self
    }
}

pub struct IptablesCmdFactory;
impl IptablesBindingFactory for IptablesCmdFactory {
    type Binding = IptablesCmd;
    fn create(&self, iptables_file: &str) -> Self::Binding {
        debug!(
            iptables_file = iptables_file,
            "Creating new IptablesCmd instance with factory"
        );
        IptablesCmd::new(iptables_file)
    }
}
