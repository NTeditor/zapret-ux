mod binding;
mod enums;
mod error;
mod traits;

use anyhow::{Context, Result};
use binding::*;
pub use enums::*;
pub use error::*;
pub use traits::*;

const QUEUE_NUM: u16 = 200;
const MARK_VALUE: &str = "0x40000000/0x40000000";
const CONNBYTES_VALUE: &str = "1:6";
const CONNBYTES_DIR_VALUE: &str = "origin";
const CONNBYTES_MODE_VALUE: &str = "packets";
const CHAIN_NAME: &str = "ZAPRET_UX";

#[derive(Debug)]
pub struct Iptables<F = IptablesCmdFactory>
where
    F: IptablesBindingFactory,
{
    factory: F,
    iptables_file: String,
    mark_supported: bool,
    connbytes_supported: bool,
}

impl Iptables<IptablesCmdFactory> {
    pub fn new<S: AsRef<str>>(
        iptables_file: S,
        mark_supported: bool,
        connbytes_supported: bool,
    ) -> Self {
        let iptables_file = iptables_file.as_ref();
        Self {
            factory: IptablesCmdFactory,
            iptables_file: iptables_file.to_string(),
            mark_supported,
            connbytes_supported,
        }
    }
}

impl<F> Iptables<F>
where
    F: IptablesBindingFactory,
{
    fn add_port_rule(&self, port_spec: &PortSpec) -> Result<()> {
        tracing::info!(port_spec = port_spec.to_string(), "Add iptables rule");
        let mut binding = self.factory.create(&self.iptables_file);
        binding
            .table("mangle")
            .insert(CHAIN_NAME)
            .protocol(port_spec.protocol)
            .module(port_spec.protocol)
            .dport(port_spec.port.to_string())
            .jump("NFQUEUE")
            .queue_num(QUEUE_NUM)
            .queue_bypass();

        if self.mark_supported {
            tracing::info!(port_spec = port_spec.to_string(), "Add mark options");
            binding.module("mark").mark(MARK_VALUE, Some(true));
        }

        if self.connbytes_supported {
            tracing::info!(proc_spec = port_spec.to_string(), "Add connbytes options");
            binding
                .module("connbytes")
                .connbytes(CONNBYTES_VALUE, None)
                .connbytes_dir(CONNBYTES_DIR_VALUE)
                .connbytes_mode(CONNBYTES_MODE_VALUE);
        }

        binding
            .run()
            .with_context(|| format!("Failed to add port rule for {}", port_spec))?;
        Ok(())
    }
}

impl<F> FirewallProvider for Iptables<F>
where
    F: IptablesBindingFactory,
{
    fn setup_rules<I>(&self, ports_spec: I) -> Result<()>
    where
        I: IntoIterator<Item = PortSpec>,
    {
        tracing::info!("Setup iptables rules");
        let mut binding = self.factory.create(&self.iptables_file);
        tracing::info!(target_chain = CHAIN_NAME, "Create target chain");
        binding.table("mangle").new_chain(CHAIN_NAME);
        binding
            .run()
            .with_context(|| format!("Failed to create chain {}", CHAIN_NAME))?;

        let mut binding = self.factory.create(&self.iptables_file);
        tracing::info!(
            from_chain = "POSTROUTING",
            target_chain = CHAIN_NAME,
            "Create jump rule to target chain"
        );
        binding
            .table("mangle")
            .insert("POSTROUTING")
            .jump(CHAIN_NAME);
        binding.run().with_context(|| {
            format!("Failed to add jump rule from POSTROUTING to {}", CHAIN_NAME)
        })?;

        for port_spec in ports_spec {
            self.add_port_rule(&port_spec)?;
        }

        Ok(())
    }

    fn clean_rules(&self) -> Result<()> {
        tracing::info!(
            from_chain = "POSTROUTING",
            target_chain = CHAIN_NAME,
            "Remove jump rule to target chain"
        );

        let mut binding = self.factory.create(&self.iptables_file);
        binding
            .table("mangle")
            .delete("POSTROUTING")
            .jump(CHAIN_NAME);

        let result = binding.run();
        match result {
            Err(BindingError::NotFoundByThatName { stderr, stdout }) => {
                tracing::warn!(
                    stderr = stderr,
                    stdout = stdout,
                    "Jump rule not found. Continuing cleanup.."
                );
            }
            Err(e) => return Err(e).context("Failed to remove jump rule"),
            Ok(_) => {}
        }

        tracing::info!(target_chain = CHAIN_NAME, "Flush target chain");
        let mut binding = self.factory.create(&self.iptables_file);
        binding.table("mangle").flush(CHAIN_NAME);

        let result = binding.run();
        match result {
            Err(BindingError::NotFoundByThatName { stderr, stdout }) => {
                tracing::warn!(
                    stderr = stderr,
                    stdout = stdout,
                    "Target chain not found during flush. Continuing cleanup.."
                );
            }
            Err(e) => return Err(e).context("Failed to flush target chain"),
            Ok(_) => {
                tracing::info!(target_chain = CHAIN_NAME, "Remove target chain");
                let mut binding = self.factory.create(&self.iptables_file);
                binding.table("mangle").delete_chain(CHAIN_NAME);

                let result = binding.run();
                match result {
                    Err(BindingError::NotFoundByThatName { stderr, stdout }) => {
                        tracing::warn!(
                            stderr = stderr,
                            stdout = stdout,
                            "Target chain not found during deletion. Cleanup completed."
                        );
                    }
                    Err(e) => return Err(e).with_context(|| "Failed to delete target chain"),
                    Ok(_) => tracing::info!("Target chain successfully removed"),
                }
            }
        }

        Ok(())
    }
}
