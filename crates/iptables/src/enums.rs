use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Port {
    Single(u16),
    Range(u16, u16),
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PortSpec {
    pub port: Port,
    pub protocol: Protocol,
}

impl PortSpec {
    pub fn new(port: Port, protocol: Protocol) -> Self {
        tracing::debug!(
            port = port.to_string(),
            protocol = protocol.to_str(),
            "Creating new PortSpec instance"
        );
        Self { port, protocol }
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(port) => {
                write!(f, "{}", port)
            }
            Self::Range(start, end) => {
                write!(f, "{}:{}", start, end)
            }
        }
    }
}

impl Protocol {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Display for PortSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.protocol, self.port)
    }
}

impl AsRef<Port> for PortSpec {
    fn as_ref(&self) -> &Port {
        &self.port
    }
}

impl AsRef<Protocol> for PortSpec {
    fn as_ref(&self) -> &Protocol {
        &self.protocol
    }
}
