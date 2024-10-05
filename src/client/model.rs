use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
    pub result: Option<WaitUserResult>
}