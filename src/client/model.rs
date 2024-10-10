use std::default;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

// A General Status Code Response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCodeResponse {
    pub code: i64,
    message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthTokenResponse {
    pub sz_auth_type: String,
    #[serde(default)]
    pub sz_info: Option<String>,
    #[serde(default)]
    pub sz_token: String,
    #[serde(default)]
    pub sz_sta_session_id: Option<String>,
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub result: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitUserResult {
    dw_acc_no: i64,
    sz_logon_name: String,
    sz_alias_name: String,
    sz_card_no: String,
    sz_i_d_card: String,
    sz_true_name: String,
    sz_class_name: String,
    sz_dept_name: String,
    sz_major: Option<String>,
    // TODO
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitUserInResponse {
    #[serde(rename = "SessionId")]
    session_id: String,
    code: i64,
    message: String,
    pub result: Option<WaitUserResult>,
}

/** POST /api/client/PrintJob/Create */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobRequest {
    #[serde(default)]
    pub dw_property: i64, // 0
    pub sz_job_name: String,
    pub dw_copies: i64,
    pub sz_attribe: String,      // "single,collate,NUP1,",
    pub sz_paper_detail: String, // "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]",
    pub sz_color_map: String,    // "0"
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobResponse {
    pub code: i64,
    message: String,
    pub result: CreateJobResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobResult {
    sz_logon_name: String, // JAccount Name
    sz_true_name: String,  // Your Name

    pub dw_job_id: usize,
    dw_create_date: i64, // TODO
    dw_create_time: i64,
    sz_file_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetJobRequest {
    pub dw_job_id: usize,
    pub dw_status: usize,
    #[serde(rename = "OSESSIONID")]
    pub osession_id: String,
}
