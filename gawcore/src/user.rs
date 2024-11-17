use crate::config::user_config::UserConfig;

/// A User Stamp is some informations used to identify a user in the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserStamp {
    pub name: String,
    pub email: String,
    
}

/// The proxy user represent the user while connected to a remote workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyUser {
    pub name: String,
    pub email: String,
    pub auth_status: AuthStatus,
}

/// Represents the local user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalUser{
    pub name: String,
    pub email: String,
    pub auth: UserAuth,
    pub config: UserConfig,
}

/// Data structure for the user's authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAuth{
    username: String,
    auth: Option<AuthMethod>,
}
/// Token based authentication information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthToken{
    pub key: String,
}

/// Authentication method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod
{
    Password(String),
    Token(AuthToken),
}

/// Authentication status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthStatus
{
    Authenticated(AuthMethod),
    Unauthenticated,
}
