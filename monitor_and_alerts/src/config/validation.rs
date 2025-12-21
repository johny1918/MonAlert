use std::collections::HashSet;
use crate::types::Config;
use crate::errors::CustomError;

pub struct ConfigValidator;

impl ConfigValidator {
    /// Validates the entire configuration
    pub fn validate(config: &Config) -> Result<(), CustomError> {
        Self::validate_no_duplicate_service_ids(config)?;
        Self::validate_no_duplicate_notification_ids(config)?;
        Self::validate_alert_references(config)?;
        Self::validate_interval_timeout(config)?;
        Self::validate_sensible_defaults(config)?;
        Ok(())
    }

    /// Ensures no duplicate service IDs
    fn validate_no_duplicate_service_ids(config: &Config) -> Result<(), CustomError> {
        let mut seen_ids = HashSet::new();
        let mut duplicates = Vec::new();

        for service in &config.services {
            if !seen_ids.insert(&service.id) {
                duplicates.push(service.id.as_str());
            }
        }

        if !duplicates.is_empty() {
            return Err(CustomError::Error(format!(
                "Duplicate service IDs found: {}",
                duplicates.join(", ")
            )));
        }

        Ok(())
    }

    /// Ensures no duplicate notification channel IDs
    fn validate_no_duplicate_notification_ids(config: &Config) -> Result<(), CustomError> {
        let mut seen_ids = HashSet::new();
        let mut duplicates = Vec::new();

        for notification in &config.notifications {
            if !seen_ids.insert(&notification.id) {
                duplicates.push(notification.id.as_str());
            }
        }

        if !duplicates.is_empty() {
            return Err(CustomError::Error(format!(
                "Duplicate notification channel IDs found: {}",
                duplicates.join(", ")
            )));
        }

        Ok(())
    }

    /// Validates that all alert rules reference valid service IDs and notification channels
    fn validate_alert_references(config: &Config) -> Result<(), CustomError> {
        // Build a set of valid service IDs
        let valid_service_ids: HashSet<&String> = config.services.iter()
            .map(|s| &s.id)
            .collect();

        // Build a set of valid notification channel IDs
        let valid_notification_ids: HashSet<&String> = config.notifications.iter()
            .map(|n| &n.id)
            .collect();

        // Check each alert rule
        for alert in &config.alerts {
            // Validate service reference
            if !valid_service_ids.contains(&alert.check_id) {
                return Err(CustomError::Error(format!(
                    "Alert rule references invalid service ID: '{}'",
                    alert.check_id
                )));
            }

            // Validate notification channel references
            for channel_id in &alert.notification_channels {
                if !valid_notification_ids.contains(channel_id) {
                    return Err(CustomError::Error(format!(
                        "Alert rule for service '{}' references invalid notification channel: '{}'",
                        alert.check_id,
                        channel_id
                    )));
                }
            }
        }

        Ok(())
    }

    /// Validates that interval > timeout for all services
    fn validate_interval_timeout(config: &Config) -> Result<(), CustomError> {
        for service in &config.services {
            if service.interval <= service.timeout {
                return Err(CustomError::Error(format!(
                    "Service '{}' (id: {}) has interval ({}) <= timeout ({}). Interval must be greater than timeout.",
                    service.name,
                    service.id,
                    service.interval,
                    service.timeout
                )));
            }
        }

        Ok(())
    }

    /// Validates sensible defaults and constraints
    fn validate_sensible_defaults(config: &Config) -> Result<(), CustomError> {
        // Validate version is not empty
        if config.version.trim().is_empty() {
            return Err(CustomError::Error(
                "Config version cannot be empty".to_string()
            ));
        }

        // Validate services have at least one entry
        if config.services.is_empty() {
            return Err(CustomError::Error(
                "Configuration must contain at least one service to monitor".to_string()
            ));
        }

        // Validate service fields
        for service in &config.services {
            if service.id.trim().is_empty() {
                return Err(CustomError::Error(
                    "Service ID cannot be empty".to_string()
                ));
            }

            if service.name.trim().is_empty() {
                return Err(CustomError::Error(format!(
                    "Service '{}' has an empty name",
                    service.id
                )));
            }

            if service.target.trim().is_empty() {
                return Err(CustomError::Error(format!(
                    "Service '{}' has an empty target",
                    service.id
                )));
            }

            if service.interval == 0 {
                return Err(CustomError::Error(format!(
                    "Service '{}' has interval set to 0, must be greater than 0",
                    service.id
                )));
            }

            if service.timeout == 0 {
                return Err(CustomError::Error(format!(
                    "Service '{}' has timeout set to 0, must be greater than 0",
                    service.id
                )));
            }
        }

        // Validate alert rules
        for alert in &config.alerts {
            if alert.check_id.trim().is_empty() {
                return Err(CustomError::Error(
                    "Alert rule has empty check_id".to_string()
                ));
            }

            if alert.threshold == 0 {
                return Err(CustomError::Error(format!(
                    "Alert rule for service '{}' has threshold set to 0, must be greater than 0",
                    alert.check_id
                )));
            }

            if alert.notification_channels.is_empty() {
                return Err(CustomError::Error(format!(
                    "Alert rule for service '{}' has no notification channels specified",
                    alert.check_id
                )));
            }
        }

        // Validate notification channels
        for notification in &config.notifications {
            if notification.id.trim().is_empty() {
                return Err(CustomError::Error(
                    "Notification channel has empty ID".to_string()
                ));
            }

            if notification.target.trim().is_empty() {
                return Err(CustomError::Error(format!(
                    "Notification channel '{}' has empty target",
                    notification.id
                )));
            }
        }

        // Validate log level
        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&config.global.log_level.to_lowercase().as_str()) {
            return Err(CustomError::Error(format!(
                "Invalid log level '{}'. Must be one of: {}",
                config.global.log_level,
                valid_log_levels.join(", ")
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Config, ServiceCheck, ServiceType, AlertRule, TriggerBehavior,
        AlertSeverity, NotificationChannel, CommunicationChannel, GlobalSettings
    };

    fn create_valid_config() -> Config {
        Config {
            version: "1.0".to_string(),
            services: vec![
                ServiceCheck {
                    id: "web1".to_string(),
                    name: "Website Check".to_string(),
                    check_type: ServiceType::Http,
                    target: "https://example.com".to_string(),
                    interval: 60,
                    timeout: 30,
                    enabled: true,
                }
            ],
            alerts: vec![
                AlertRule {
                    check_id: "web1".to_string(),
                    condition: TriggerBehavior::ConsecutiveFailures,
                    threshold: 3,
                    severity: AlertSeverity::Critical("Service down".to_string()),
                    notification_channels: vec!["slack1".to_string()],
                }
            ],
            notifications: vec![
                NotificationChannel {
                    id: "slack1".to_string(),
                    channel_type: CommunicationChannel::Slack,
                    target: "#alerts".to_string(),
                    enabled: true,
                }
            ],
            global: GlobalSettings {
                log_level: "info".to_string(),
            },
        }
    }

    #[test]
    fn test_valid_config() {
        let config = create_valid_config();
        assert!(ConfigValidator::validate(&config).is_ok());
    }

    #[test]
    fn test_duplicate_service_ids() {
        let mut config = create_valid_config();
        config.services.push(ServiceCheck {
            id: "web1".to_string(),
            name: "Duplicate".to_string(),
            check_type: ServiceType::Http,
            target: "https://example2.com".to_string(),
            interval: 60,
            timeout: 30,
            enabled: true,
        });

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate service IDs"));
    }

    #[test]
    fn test_invalid_alert_reference() {
        let mut config = create_valid_config();
        config.alerts[0].check_id = "nonexistent".to_string();

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid service ID"));
    }

    #[test]
    fn test_interval_less_than_timeout() {
        let mut config = create_valid_config();
        config.services[0].interval = 30;
        config.services[0].timeout = 60;

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Interval must be greater than timeout"));
    }

    #[test]
    fn test_interval_equal_to_timeout() {
        let mut config = create_valid_config();
        config.services[0].interval = 30;
        config.services[0].timeout = 30;

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_notification_channel_reference() {
        let mut config = create_valid_config();
        config.alerts[0].notification_channels = vec!["nonexistent".to_string()];

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid notification channel"));
    }

    #[test]
    fn test_empty_version() {
        let mut config = create_valid_config();
        config.version = "".to_string();

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("version cannot be empty"));
    }

    #[test]
    fn test_invalid_log_level() {
        let mut config = create_valid_config();
        config.global.log_level = "invalid".to_string();

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid log level"));
    }
}
