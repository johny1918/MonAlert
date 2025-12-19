
// Defines what services can monitor.
enum ServiceType {
    Http,
    Tcp,
    Icmp,
    Command,
}

// Defines trigger conditions.
enum TriggerBehavior {
    ConsecutiveFailures,
    ResponseTimeThreshold,
}

// Defines severity of the alert.
enum AlertSeverity {
    Info(String),
    Warning(String),
    Critical(String),
}

// Defines communications channels.
enum CommunicationChannel {
    Slack,
    Email,
}

enum GlobalSettings {
    LogLevel
}

// Defines what to monitor.
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
pub struct AlertRule {
    pub check_id: String,
    pub condition: TriggerBehavior,
    pub threshold: u32,
    pub severity: AlertSeverity,
    pub notification_channels: Vec<String> // slack, email, etc
}

// Defines how to send the alerts.
pub struct NotificationChannel {
    pub id: String,
    pub channel_type: CommunicationChannel,
    pub target: String,
    pub enabled: bool,
}

// Defines reading of config file i.e. yaml file
pub struct Config {
    pub version: String,
    pub services: Vec<ServiceCheck>,
    pub alerts: Vec<AlertRule>,
    pub notifications: Vec<NotificationChannel>,
    pub global: GlobalSettings,
}