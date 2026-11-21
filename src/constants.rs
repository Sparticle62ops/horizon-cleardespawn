//! 1:1 port of io.github.strikerrocker.cleardespawn.Constants.

/// java: public static final String MOD_ID = "cleardespawn";
pub const MOD_ID: &str = "cleardespawn";

/// java: public static final String MOD_NAME = "ClearDespawn";
pub const MOD_NAME: &str = "ClearDespawn";

/// java: Constants.LOGGER = LoggerFactory.getLogger(MOD_NAME), reduced to the
/// debug call the upstream source makes (Services.load's debug line).
pub fn log() -> &'static crate::seams::Logger {
    crate::seams::logger()
}
