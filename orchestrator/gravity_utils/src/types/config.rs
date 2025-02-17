//! contains configuration structs that need to be accessed across crates.

/// Global configuration struct for Gravity bridge tools
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct GravityBridgeToolsConfig {
    #[serde(default = "RelayerConfig::default")]
    pub relayer: RelayerConfig,
    #[serde(default = "OrchestratorConfig::default")]
    pub orchestrator: OrchestratorConfig,
}

/// Relayer configuration options
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RelayerConfig {
    #[serde(default = "default_valset_relaying_mode")]
    pub valset_relaying_mode: ValsetRelayingMode,
    #[serde(default = "default_batch_market_enabled")]
    pub batch_market_enabled: bool,
    #[serde(default = "default_logic_call_market_enabled")]
    pub logic_call_market_enabled: bool,
}

/// The various possible modes for relaying validator set updates
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValsetRelayingMode {
    /// Only ever relay profitable valsets, regardless of all other
    /// considerations
    ProfitableOnly,
    /// Relay validator sets when continued operation of the chain
    /// requires it, this will cost some ETH
    Altruistic,
    /// Relay every validator set update, mostly for developer use
    EveryValset,
}

fn default_batch_market_enabled() -> bool {
    true
}

fn default_logic_call_market_enabled() -> bool {
    true
}

fn default_valset_relaying_mode() -> ValsetRelayingMode {
    ValsetRelayingMode::Altruistic
}

impl Default for RelayerConfig {
    fn default() -> Self {
        RelayerConfig {
            valset_relaying_mode: default_valset_relaying_mode(),
            batch_market_enabled: default_batch_market_enabled(),
            logic_call_market_enabled: default_logic_call_market_enabled(),
        }
    }
}

/// Orchestrator configuration options
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct OrchestratorConfig {
    /// If this Orchestrator should run an integrated relayer or not
    #[serde(default = "default_relayer_enabled")]
    pub relayer_enabled: bool,
}

fn default_relayer_enabled() -> bool {
    false
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        OrchestratorConfig {
            relayer_enabled: default_relayer_enabled(),
        }
    }
}
