use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(rename = "StarRailCopilot")]
    pub starrailcopilot_config: StarRailCopilotConfig,
    #[serde(rename = "MAA")]
    pub maa_config: MAAConfig,
    #[serde(rename = "AndroidEmulator")]
    pub android_emulator: AndroidEmulatorConfig,
    #[serde(rename = "PlatformTools")]
    pub platform_tools: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StarRailCopilotConfig {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Task")]
    pub task: Vec<TaskConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MAAConfig {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Task")]
    pub task: Vec<TaskConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AndroidEmulatorConfig {
    #[serde(rename = "Type")]
    pub emulator_type: String,
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskConfig {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}
