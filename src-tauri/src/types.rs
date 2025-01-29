use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Notification {
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub duration: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PageResponse {
    pub updates: Vec<DomUpdate>,
    pub notification: Option<Notification>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DomUpdate {
    pub html: String,
    pub target: String,
    pub action: String,
}

impl DomUpdate {
    pub fn from(node: html_node::Node, target: &str, action: &str) -> Self {
        DomUpdate {
            html: node.pretty().to_string(),
            target: target.to_string(),
            action: action.to_string(),
        }
    }
}

impl PageResponse {
    pub fn new(update: DomUpdate) -> Self {
        Self {
            updates: vec![update],
            notification: None,
        }
    }

    pub fn with_notification(
        update: DomUpdate,
        message: String,
        notification_type: &str,
        duration: Option<u32>,
    ) -> Self {
        Self {
            updates: vec![update],
            notification: Some(Notification {
                message,
                notification_type: notification_type.to_string(),
                duration,
            }),
        }
    }
}
