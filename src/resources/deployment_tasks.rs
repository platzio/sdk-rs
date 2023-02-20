use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub enum DeploymentTaskStatus {
    Pending,
    Started,
    Failed,
    Done,
}

pub type JsonDiff = HashMap<String, (Value, Value)>;
#[derive(Default, IntoVec)]
pub struct DeploymentTaskFilters {
    #[kv(optional)]
    pub cluster_id: Option<Uuid>,
    #[kv(optional)]
    pub deployment_id: Option<Uuid>,
    #[kv(optional)]
    pub active_only: Option<bool>,
    #[kv(optional)]
    pub created_from: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct DeploymentTask {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub first_attempted_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub cluster_id: Uuid,
    pub deployment_id: Uuid,
    pub acting_user_id: Option<Uuid>,
    pub acting_deployment_id: Option<Uuid>,
    pub operation: DeploymentTaskOperation,
    pub status: DeploymentTaskStatus,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentTaskOperation {
    Install(DeploymentInstallTask),
    Upgrade(DeploymentUpgradeTask),
    Reinstall(DeploymentReinstallTask),
    Recreate(DeploymentRecreaseTask),
    Uninstall(DeploymentUninstallTask),
    InvokeAction(DeploymentInvokeActionTask),
    RestartK8sResource(DeploymentRestartK8sResourceTask),
}

impl DeploymentTaskOperation {
    pub fn get_type_name(&self) -> String {
        match self {
            Self::Install(_) => "Install".into(),
            Self::Upgrade(_) => "Upgrade".into(),
            Self::Reinstall(_) => "Reinstall".into(),
            Self::Recreate(_) => "Recreate".into(),
            Self::Uninstall(_) => "Uninstall".into(),
            Self::InvokeAction(_) => "Invoke Action".into(),
            Self::RestartK8sResource(_) => "Restart K8s Resource".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInstallTask {
    pub helm_chart_id: Uuid,
    pub config_inputs: serde_json::Value,
    pub values_override: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentUpgradeTask {
    pub helm_chart_id: Uuid,
    pub prev_helm_chart_id: Option<Uuid>,
    pub config_inputs: serde_json::Value,
    pub config_delta: Option<JsonDiff>,
    pub values_override: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentReinstallTask {
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRecreaseTask {
    pub old_cluster_id: Uuid,
    pub old_namespace: String,
    pub new_cluster_id: Uuid,
    pub new_namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentUninstallTask {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInvokeActionTask {
    pub helm_chart_id: Uuid,
    pub action_id: String,
    pub body: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRestartK8sResourceTask {
    pub resource_id: Uuid,
    pub resource_name: String,
}

impl PlatzClient {
    pub async fn deployment_tasks(
        &self,
        filters: DeploymentTaskFilters,
    ) -> Result<Vec<DeploymentTask>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployment-tasks")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }
}
