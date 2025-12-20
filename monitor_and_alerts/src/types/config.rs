use serde::{Deserialize, Serialize};


// Defines what services can monitor.
#[derive(Serialize, Deserialize, Debug)]
pub enum ServiceType {
    Http,
    Tcp,
    Icmp,
    Command,
}

// Defines trigger conditions.
#[derive(Serialize, Deserialize, Debug)]
pub enum TriggerBehavior {
    ConsecutiveFailures,
    ResponseTimeThreshold,
}

// Defines severity of the alert.
#[derive(Serialize, Deserialize, Debug)]
pub enum AlertSeverity {
    Info(String),
    Warning(String),
    Critical(String),
}

// Defines communications channels.
#[derive(Serialize, Deserialize, Debug)]
pub enum CommunicationChannel {
    Slack,
    Email,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalSettings {
    pub log_level: String,
}

// Defines what to monitor.
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceCheck {
    pub id: String,
    pub name: String,
    pub check_type: ServiceType,
    pub target: String,
    pub interval: u32,
    pub timeout: u32,
    pub enabled: bool,
}

// Defines when to alert.
#[derive(Serialize, Deserialize, Debug)]
pub struct AlertRule {
    pub check_id: String,
    pub condition: TriggerBehavior,
    pub threshold: u32,
    pub severity: AlertSeverity,
    pub notification_channels: Vec<String> // slack, email, etc
}

// Defines how to send the alerts.
#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationChannel {
    pub id: String,
    pub channel_type: CommunicationChannel,
    pub target: String,
    pub enabled: bool,
}

// Defines reading of config file i.e. TOML file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub services: Vec<ServiceCheck>,
    pub alerts: Vec<AlertRule>,
    pub notifications: Vec<NotificationChannel>,
    pub global: GlobalSettings,
}