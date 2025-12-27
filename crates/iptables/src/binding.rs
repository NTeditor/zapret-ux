use super::*;
use anyhow::Result;
use std::process::Command;

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
    fn module<S: AsRef<str>>(&mut self, module: S) -> &mut Self {
        let module = module.as_ref();
        self.arg("--match");
        self.arg(module);
        self
    }

    fn insert<S: AsRef<str>>(&mut self, chain: S) -> &mut Self {
        let chain = chain.as_ref();
        self.arg("--insert");
        self.arg(chain);
        self
    }

    fn new_chain<S: AsRef<str>>(&mut self, chain: S) -> &mut Self {
        let chain = chain.as_ref();
        self.arg("--new");
        self.arg(chain);
        self
    }

    fn delete_chain<S: AsRef<str>>(&mut self, chain: S) -> &mut Self {
        let chain = chain.as_ref();
        self.arg("--delete-chain");
        self.arg(chain);
        self
    }

    fn delete<S: AsRef<str>>(&mut self, chain: S) -> &mut Self {
        let chain = chain.as_ref();
        self.arg("--delete");
        self.arg(chain);
        self
    }

    fn flush<S: AsRef<str>>(&mut self, chain: S) -> &mut Self {
        let chain = chain.as_ref();
        self.arg("--flush");
        self.arg(chain);
        self
    }

    fn table<S: AsRef<str>>(&mut self, table: S) -> &mut Self {
        let table = table.as_ref();
        self.arg("--table");
        self.arg(table);
        self
    }

    fn protocol<S: AsRef<str>>(&mut self, proto: S) -> &mut Self {
        let proto = proto.as_ref();
        self.arg("--protocol");
        self.arg(proto);
        self
    }

    fn jump<S: AsRef<str>>(&mut self, target: S) -> &mut Self {
        let target = target.as_ref();
        self.arg("--jump");
        self.arg(target);
        self
    }

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

    fn dport<S: AsRef<str>>(&mut self, port: S) -> &mut Self {
        let port = port.as_ref();
        self.arg("--dport");
        self.arg(port);
        self
    }

    fn mark<S: AsRef<str>>(&mut self, value: S, invert: Option<bool>) -> &mut Self {
        let value = value.as_ref();
        if let Some(true) = invert {
            self.arg("!");
        }
        self.arg("--mark");
        self.arg(value);
        self
    }

    fn connbytes<S: AsRef<str>>(&mut self, value: S, invert: Option<bool>) -> &mut Self {
        let value = value.as_ref();
        if let Some(true) = invert {
            self.arg("!");
        }
        self.arg("--connbytes");
        self.arg(value);
        self
    }

    fn connbytes_dir<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let value = value.as_ref();
        self.arg("--connbytes-dir");
        self.arg(value);
        self
    }

    fn connbytes_mode<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let value = value.as_ref();
        self.arg("--connbytes-mode");
        self.arg(value);
        self
    }

    fn queue_num(&mut self, value: u16) -> &mut Self {
        let value = value.to_string();
        self.arg("--queue-num");
        self.arg(value);
        self
    }

    fn queue_bypass(&mut self) -> &mut Self {
        self.arg("--queue-bypass");
        self
    }
}

pub struct IptablesCmdFactory;
impl IptablesBindingFactory for IptablesCmdFactory {
    type Binding = IptablesCmd;
    fn create(&self, iptables_file: &str) -> Self::Binding {
        tracing::debug!(
            iptables_file = iptables_file,
            "Creating new IptablesCmd instance with factory"
        );
        IptablesCmd::new(iptables_file)
    }
}
