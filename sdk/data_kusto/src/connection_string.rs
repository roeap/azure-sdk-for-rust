// Set of properties that can be use in a connection string provided to KustoConnectionStringBuilder.
// For a complete list of properties go to https://docs.microsoft.com/en-us/azure/kusto/api/connection-strings/kusto
pub const DATA_SOURCE_NAME: &str = "Data Source";
const DATA_SOURCE_ALIAS: [&str; 5] = [
    "data source",
    "addr",
    "address",
    "network address",
    "server",
];
pub const FEDERATED_SECURITY_NAME: &str = "AAD Federated Security";
const FEDERATED_SECURITY_ALIAS: [&str; 5] = [
    "aad federated security",
    "federated security",
    "federated",
    "fed",
    "aadfed",
];
pub const USER_ID_NAME: &str = "AAD User ID";
const USER_ID_ALIAS: [&str; 1] = ["aad user id"];
pub const PASSWORD_NAME: &str = "Password";
const PASSWORD_ALIAS: [&str; 2] = ["password", "pwd"];
pub const APPLICATION_CLIENT_ID_NAME: &str = "Application Client Id";
const APPLICATION_CLIENT_ID_ALIAS: [&str; 2] = ["application client id", "appclientid"];
pub const APPLICATION_KEY_NAME: &str = "Application Key";
const APPLICATION_KEY_ALIAS: [&str; 2] = ["application key", "appkey"];
pub const APPLICATION_CERTIFICATE_NAME: &str = "ApplicationCertificate";
pub const APPLICATION_CERTIFICATE_PRIVATE_KEY_NAME: &str = "Application Certificate PrivateKey";
pub const APPLICATION_CERTIFICATE_THUMBPRINT_NAME: &str = "Application Certificate Thumbprint";
const APPLICATION_CERTIFICATE_THUMBPRINT_ALIAS: [&str; 2] =
    ["application certificate thumbprint", "AppCert"];
pub const APPLICATION_CERTIFICATE_X5C_NAME: &str = "Application Certificate x5c";
const APPLICATION_CERTIFICATE_X5C_ALIAS: [&str; 4] = [
    "application certificate x5c",
    "Application Certificate Send Public Certificate",
    "Application Certificate SendX5c",
    "SendX5c",
];
pub const AUTHORITY_ID_NAME: &str = "Authority Id";
const AUTHORITY_ID_ALIAS: [&str; 6] = [
    "authority id",
    "authorityid",
    "authority",
    "tenantid",
    "tenant",
    "tid",
];
pub const APPLICATION_TOKEN_NAME: &str = "ApplicationToken";
pub const USER_TOKEN_NAME: &str = "UserToken";
pub const MSI_AUTH_NAME: &str = "MSI Authentication";
pub const MSI_PARAMS_NAME: &str = "MSI Params";
pub const AZ_CLI_NAME: &str = "AZ CLI";
// pub const PUBLIC_APPLICATION_CERTIFICATE_NAME: &str = "Public Application Certificate";
// pub const INTERACTIVE_LOGIN_NAME: &str = "Interactive Login";
// pub const LOGIN_HINT_NAME: &str = "Login Hint";
// pub const DOMAIN_HINT_NAME: &str = "Domain Hint";

fn normalize_key(key: &str) -> &str {
    if DATA_SOURCE_ALIAS.contains(&key) {
        return DATA_SOURCE_NAME;
    }
    if FEDERATED_SECURITY_ALIAS.contains(&key) {
        return FEDERATED_SECURITY_NAME;
    }
    if USER_ID_ALIAS.contains(&key) {
        return USER_ID_NAME;
    }
    if PASSWORD_ALIAS.contains(&key) {
        return PASSWORD_NAME;
    }
    if APPLICATION_CLIENT_ID_ALIAS.contains(&key) {
        return APPLICATION_CLIENT_ID_NAME;
    }
    if APPLICATION_KEY_ALIAS.contains(&key) {
        return APPLICATION_KEY_NAME;
    }
    if AUTHORITY_ID_ALIAS.contains(&key) {
        return AUTHORITY_ID_NAME;
    }
    if APPLICATION_CERTIFICATE_THUMBPRINT_ALIAS.contains(&key) {
        return APPLICATION_CERTIFICATE_THUMBPRINT_NAME;
    }
    if APPLICATION_CERTIFICATE_X5C_ALIAS.contains(&key) {
        return APPLICATION_CERTIFICATE_X5C_NAME;
    }
    key
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionStringError {
    #[error("Missing value for key '{}'", key)]
    MissingValue { key: String },
    #[error("Unexpected key '{}'", key)]
    UnexpectedKey { key: String },
    #[error("Parsing error: {}", msg)]
    ParsingError { msg: String },
}

/// Build a connection string to connect to a Kusto service instance.
///
/// For more information on Kusto connection strings visit:
/// https://docs.microsoft.com/en-us/azure/data-explorer/kusto/api/connection-strings/kusto
#[derive(Default)]
pub struct ConnectionStringBuilder<'a>(ConnectionString<'a>);

impl<'a> ConnectionStringBuilder<'a> {
    pub fn new() -> Self {
        Self(ConnectionString::default())
    }

    pub fn new_with_aad_application_key_authentication(
        service_url: &'a str,
        authority_id: &'a str,
        application_client_id: &'a str,
        application_key: &'a str,
    ) -> Self {
        Self(ConnectionString {
            data_source: Some(service_url),
            federated_security: Some(true),
            application_client_id: Some(application_client_id),
            application_key: Some(application_key),
            authority_id: Some(authority_id),
            ..ConnectionString::default()
        })
    }

    pub fn build(&self) -> String {
        let mut kv_pairs = Vec::new();

        if let Some(data_source) = self.0.data_source {
            kv_pairs.push(format!("{}={}", DATA_SOURCE_NAME, data_source));
        }
        if let Some(user_id) = self.0.user_id {
            kv_pairs.push(format!("{}={}", USER_ID_NAME, user_id));
        }
        if let Some(application_client_id) = self.0.application_client_id {
            kv_pairs.push(format!(
                "{}={}",
                APPLICATION_CLIENT_ID_NAME, application_client_id
            ));
        }
        if let Some(application_key) = self.0.application_key {
            kv_pairs.push(format!("{}={}", APPLICATION_KEY_NAME, application_key));
        }
        if let Some(application_token) = self.0.application_token {
            kv_pairs.push(format!("{}={}", APPLICATION_TOKEN_NAME, application_token));
        }
        if let Some(authority_id) = self.0.authority_id {
            kv_pairs.push(format!("{}={}", AUTHORITY_ID_NAME, authority_id));
        }

        kv_pairs.join(";")
    }

    pub fn data_source(&'a mut self, data_source: &'a str) -> &'a mut Self {
        self.0.data_source = Some(data_source);
        self
    }

    pub fn user_id(&'a mut self, user_id: &'a str) -> &'a mut Self {
        self.0.user_id = Some(user_id);
        self
    }

    pub fn application_client_id(&'a mut self, application_client_id: &'a str) -> &'a mut Self {
        self.0.application_client_id = Some(application_client_id);
        self
    }

    pub fn application_token(&'a mut self, application_token: &'a str) -> &'a mut Self {
        self.0.application_token = Some(application_token);
        self
    }

    pub fn application_key(&'a mut self, application_key: &'a str) -> &'a mut Self {
        self.0.application_key = Some(application_key);
        self
    }

    pub fn authority_id(&'a mut self, authority_id: &'a str) -> &'a mut Self {
        self.0.authority_id = Some(authority_id);
        self
    }
}

/// A Kusto service connection string.
///
/// For more information on Kusto connection strings visit:
/// https://docs.microsoft.com/en-us/azure/kusto/api/connection-strings/kusto
#[derive(Debug, Default)]
pub struct ConnectionString<'a> {
    /// The URI specifying the Kusto service endpoint.
    /// For example, https://mycluster.kusto.windows.net or net.tcp://localhost
    pub data_source: Option<&'a str>,
    /// A Boolean value that instructs the client to perform Azure Active Directory login.
    pub federated_security: Option<bool>,
    /// A String value that instructs the client to perform user authentication with the indicated user name.
    pub user_id: Option<&'a str>,
    pub user_token: Option<&'a str>,
    /// ...
    pub password: Option<&'a str>,
    /// A String value that provides the application client ID to use when authenticating.
    pub application_client_id: Option<&'a str>,
    /// A String value that provides the application key to use when authenticating using an application secret flow.
    pub application_key: Option<&'a str>,
    /// A String value that instructs the client to perform application authenticating with the specified bearer token.
    pub application_token: Option<&'a str>,
    /// ...
    pub application_certificate: Option<&'a str>,
    /// A String value that provides the thumbprint of the client
    /// certificate to use when using an application client certificate authenticating flow.
    pub application_certificate_thumbprint: Option<&'a str>,
    /// A String value that provides the name or ID of the tenant in which the application is registered.
    pub authority_id: Option<&'a str>,
    /// Denotes if MSI authorization should be used
    pub msi_auth: Option<bool>,
    pub msi_params: Option<&'a str>,
    pub az_cli: Option<bool>,
}

impl<'a> PartialEq for ConnectionString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data_source == other.data_source
            && self.federated_security == other.federated_security
            && self.user_id == other.user_id
            && self.user_token == other.user_token
            && self.password == other.password
            && self.application_client_id == other.application_client_id
            && self.application_key == other.application_key
            && self.application_token == other.application_token
            && self.application_certificate == other.application_certificate
            && self.application_certificate_thumbprint == other.application_certificate_thumbprint
            && self.authority_id == other.authority_id
            && self.msi_auth == other.msi_auth
            && self.msi_params == other.msi_params
            && self.az_cli == other.az_cli
    }
}

impl<'a> ConnectionString<'a> {
    pub fn new(connection_string: &'a str) -> Result<Self, ConnectionStringError> {
        let mut data_source = None;
        let mut federated_security = None;
        let mut user_id = None;
        let mut user_token = None;
        let mut password = None;
        let mut application_client_id = None;
        let mut application_token = None;
        let mut application_key = None;
        let mut application_certificate = None;
        let mut application_certificate_thumbprint = None;
        let mut authority_id = None;
        let mut msi_auth = None;
        let mut msi_params = None;
        let mut az_cli = None;

        let kv_str_pairs = connection_string
            .split(';')
            .filter(|s| !s.chars().all(char::is_whitespace));

        for kv_pair_str in kv_str_pairs {
            let mut kv = kv_pair_str.trim().split('=');
            let k = match kv.next().filter(|k| !k.chars().all(char::is_whitespace)) {
                None => {
                    return Err(ConnectionStringError::ParsingError {
                        msg: "No key found".to_owned(),
                    })
                }
                Some(k) => k,
            };
            let v = match kv.next().filter(|k| !k.chars().all(char::is_whitespace)) {
                None => return Err(ConnectionStringError::MissingValue { key: k.to_owned() }),
                Some(v) => v,
            };

            match normalize_key(k) {
                DATA_SOURCE_NAME => data_source = Some(v),
                USER_ID_NAME => user_id = Some(v),
                USER_TOKEN_NAME => user_token = Some(v),
                PASSWORD_NAME => password = Some(v),
                APPLICATION_CLIENT_ID_NAME => application_client_id = Some(v),
                APPLICATION_KEY_NAME => application_key = Some(v),
                APPLICATION_TOKEN_NAME => application_token = Some(v),
                APPLICATION_CERTIFICATE_NAME => application_certificate = Some(v),
                APPLICATION_CERTIFICATE_THUMBPRINT_NAME => {
                    application_certificate_thumbprint = Some(v)
                }
                AUTHORITY_ID_NAME => authority_id = Some(v),
                MSI_PARAMS_NAME => msi_params = Some(v),
                FEDERATED_SECURITY_NAME => {
                    federated_security = Some(parse_boolean(v, FEDERATED_SECURITY_NAME)?)
                }
                MSI_AUTH_NAME => msi_auth = Some(parse_boolean(v, MSI_AUTH_NAME)?),
                AZ_CLI_NAME => az_cli = Some(parse_boolean(v, AZ_CLI_NAME)?),
                k => return Err(ConnectionStringError::UnexpectedKey { key: k.to_owned() }),
            }
        }

        Ok(Self {
            data_source,
            federated_security,
            user_id,
            user_token,
            password,
            application_client_id,
            application_key,
            application_token,
            application_certificate,
            application_certificate_thumbprint,
            authority_id,
            msi_auth,
            msi_params,
            az_cli,
        })
    }
}

fn parse_boolean(term: &str, name: &str) -> Result<bool, ConnectionStringError> {
    match term.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ConnectionStringError::ParsingError {
            msg: format!(
                "Unexpected value for {}: {}. Please specify either 'true' or 'false'.",
                name, term
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_parses_empty_connection_string() {
        assert_eq!(
            ConnectionString::new("").unwrap(),
            ConnectionString::default()
        );
    }

    #[test]
    fn it_returns_expected_errors() {
        assert!(matches!(
            ConnectionString::new("Data Source="),
            Err(ConnectionStringError::MissingValue { key }) if key == "Data Source"
        ));
        assert!(matches!(
            ConnectionString::new("="),
            Err(ConnectionStringError::ParsingError { msg: _ })
        ));
        assert!(matches!(
            ConnectionString::new("x=123;"),
            Err(ConnectionStringError::UnexpectedKey { key }) if key == "x"
        ));
    }

    #[test]
    fn it_parses_basic_cases() {
        assert!(matches!(
            ConnectionString::new("Data Source=ds"),
            Ok(ConnectionString {
                data_source: Some("ds"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("addr=ds"),
            Ok(ConnectionString {
                data_source: Some("ds"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("Application Client Id=cid;Application Key=key"),
            Ok(ConnectionString {
                application_client_id: Some("cid"),
                application_key: Some("key"),
                ..
            })
        ));
    }
}
