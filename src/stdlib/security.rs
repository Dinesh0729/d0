#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    UnauthorizedTarget(String),
    MissingPermission(String),
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::UnauthorizedTarget(t) => write!(f, "Access denied for target: {}", t),
            SecurityError::MissingPermission(p) => write!(f, "Missing required capability: {}", p),
        }
    }
}

impl std::error::Error for SecurityError {}

#[derive(Debug, Clone)]
pub struct AuditScope {
    pub target: String,
    pub authorized: bool,
}

pub struct D0Security;

impl D0Security {
    pub fn authorize(target: &str, token: &str) -> Result<AuditScope, SecurityError> {
        if token.is_empty() {
            return Err(SecurityError::MissingPermission("empty authorization token".into()));
        }
        Ok(AuditScope {
            target: target.to_string(),
            authorized: true,
        })
    }

    pub fn verify_scope(scope: &AuditScope) -> Result<(), SecurityError> {
        if !scope.authorized {
            return Err(SecurityError::UnauthorizedTarget(scope.target.clone()));
        }
        Ok(())
    }
}
