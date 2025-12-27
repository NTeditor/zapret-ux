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
    fn module<S: AsRef<str>>(&mut self, extension: S) -> &mut Self;

    /// Insert in chain as rulenum
    ///
    /// # Args
    /// * `chain` - Chain name
    fn insert<S: AsRef<str>>(&mut self, chain: S) -> &mut Self;

    /// Create a new user-defined chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn new_chain<S: AsRef<str>>(&mut self, chain: S) -> &mut Self;

    /// Delete a user-defined chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn delete_chain<S: AsRef<str>>(&mut self, chain: S) -> &mut Self;

    /// Delete all rules in  chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn flush<S: AsRef<str>>(&mut self, chain: S) -> &mut Self;

    /// Delete matching rule from chain
    ///
    /// # Args
    /// * `chain` - Chain name
    fn delete<S: AsRef<str>>(&mut self, chain: S) -> &mut Self;

    /// Table to manipulate
    ///
    /// # Args
    /// * `chain` - Chain name
    fn table<S: AsRef<str>>(&mut self, table: S) -> &mut Self;

    /// Specify protocol type (tcp, udp, icmp, etc.)
    ///
    /// # Args
    /// * `proto` - protocol name
    fn protocol<S: AsRef<str>>(&mut self, proto: S) -> &mut Self;

    /// Target for rule (may load target extension)
    ///
    /// # Args
    /// * `target` - target name
    fn jump<S: AsRef<str>>(&mut self, target: S) -> &mut Self;

    /// Destination port
    ///
    /// # Args
    /// * `port` - port ("80") or port range ("22:80")
    fn dport<S: AsRef<str>>(&mut self, port: S) -> &mut Self;

    /// Match nfmark value with optional mask
    ///
    /// # Args
    /// * `value` - mark value with optional mask (e.g., "0x1", "0x40000000/0x40000000")
    /// * `invert` - when true, matches everything EXCEPT specified mark
    fn mark<S: AsRef<str>>(&mut self, value: S, invert: Option<bool>) -> &mut Self;

    /// Match by connection bytes
    ///
    /// # Args
    /// * `value` - byte/packet range (e.g., "1:6", "1048576:")
    fn connbytes<S: AsRef<str>>(&mut self, value: S, invert: Option<bool>) -> &mut Self;

    /// Set connection bytes direction
    ///
    /// # Args
    /// * `value` - direction: "original", "reply", or "both"
    fn connbytes_dir<S: AsRef<str>>(&mut self, value: S) -> &mut Self;

    /// Set connection bytes counting mode
    ///
    /// # Args
    /// * `value` - connbytes mode name
    fn connbytes_mode<S: AsRef<str>>(&mut self, value: S) -> &mut Self;

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
