use rust_decimal::Decimal;
use serde::Serialize;
use strum::Display;

#[derive(Debug, Clone, Serialize)]
pub struct PlatzStatus<SN>
where
    SN: Clone + Serialize,
{
    pub status: Status<SN>,
    pub primary_metric: Option<Metric>,
    pub metrics: Option<Vec<Metric>>,
    pub notices: Vec<Notice>,
}

#[derive(Debug, Clone, Serialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum StatusColor {
    #[strum(serialize = "primary")]
    Primary,
    #[strum(serialize = "success")]
    Success,
    #[strum(serialize = "danger")]
    Danger,
    #[strum(serialize = "warning")]
    Warning,
    #[strum(serialize = "secondary")]
    Secondary,
}

#[derive(Debug, Clone, Serialize)]
pub struct Status<SN>
where
    SN: Clone + Serialize,
{
    pub name: SN,
    pub color: StatusColor,
}

#[derive(Debug, Clone, Serialize)]
pub struct Metric {
    pub value: Decimal,
    pub unit: String,
    pub short_description: String,
    pub color: Option<StatusColor>,
}

#[derive(Debug, Clone, Serialize, Display)]
pub enum NoticeLevel {
    Info,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Serialize)]
pub struct Notice {
    pub level: NoticeLevel,
    pub text: String,
}
