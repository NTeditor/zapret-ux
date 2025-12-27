use anyhow::Result;
use std::fmt::Debug;

use super::{BindingError, PortSpec};

/// Binding for main iptables
pub trait IptablesBinding: Debug {
    /// Run iptables command
    fn run(self) -> Result<(), BindingError>;

    /// Load extension
    ///
    /// # Args
    /// * `module` - Extension name
    fn module(&mut self, extension: &str) -> &mut Self;

    /// Insert in chain as rulenum
    ///
    /// # Args
    /// * `chain` - Chain name
    fn insert(&mut self, chain: &str) -> &mut Self;

    /// Create a new user-defined chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn new_chain(&mut self, chain: &str) -> &mut Self;

    /// Delete a user-defined chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn delete_chain(&mut self, chain: &str) -> &mut Self;

    /// Delete all rules in  chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn flush(&mut self, chain: &str) -> &mut Self;

    /// Delete matching rule from chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn delete(&mut self, chain: &str) -> &mut Self;

    /// Table to manipulate
    ///
    /// # Args
    /// * `chain` - Chain name
    fn table(&mut self, table: &str) -> &mut Self;

    /// Specify protocol type (tcp, udp, icmp, etc.)
    ///
    /// # Args
    /// * `proto` - protocol name
    fn protocol(&mut self, proto: &str) -> &mut Self;

    /// Target for rule (may load target extension)
    ///
    /// # Args
    /// * `target` - target name
    fn jump(&mut self, target: &str) -> &mut Self;

    /// Destination port
    ///
    /// # Args
    /// * `port` - port ("80") or port range ("22:80")
    fn dport(&mut self, port: &str) -> &mut Self;

    /// Match nfmark value with optional mask
    ///
    /// # Args
    /// * `value` - mark value with optional mask (e.g., "0x1", "0x40000000/0x40000000")
    /// * `invert` - when true, matches everything EXCEPT specified mark
    fn mark(&mut self, value: &str, invert: Option<bool>) -> &mut Self;

    /// Match by connection bytes
    ///
    /// # Args
    /// * `value` - byte/packet range (e.g., "1:6", "1048576:")
    fn connbytes(&mut self, value: &str, invert: Option<bool>) -> &mut Self;

    /// Set connection bytes direction
    ///
    /// # Args
    /// * `value` - direction: "original", "reply", or "both"
    fn connbytes_dir(&mut self, value: &str) -> &mut Self;

    /// Set connection bytes counting mode
    ///
    /// # Args
    /// * `value` - connbytes mode name
    fn connbytes_mode(&mut self, value: &str) -> &mut Self;

    /// Send packet to QUEUE number
    ///
    /// # Args
    /// * `value` - queue number
    fn queue_num(&mut self, value: u16) -> &mut Self;

    /// Bypass Queueing if no queue instance exists
    fn queue_bypass(&mut self) -> &mut Self;
}

pub trait FirewallProvider {
    fn setup_rules<I>(&self, ports_spec: I) -> Result<()>
    where
        I: IntoIterator<Item = PortSpec>;
    fn clean_rules(&self) -> Result<()>;
}

pub trait IptablesBindingFactory {
    type Binding: IptablesBinding;
    fn create(&self, iptables_file: &str) -> Self::Binding;
}
