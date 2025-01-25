use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Notification {
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub duration: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PageResponse {
    pub html: String,
    pub notification: Option<Notification>,
}

impl PageResponse {
    pub fn new(html: String) -> Self {
        Self {
            html,
            notification: None,
        }
    }

    pub fn with_notification(html: String, message: String, notification_type: &str, duration: Option<u32>) -> Self {
        Self {
            html,
            notification: Some(Notification {
                message,
                notification_type: notification_type.to_string(),
                duration,
            }),
        }
    }
}
