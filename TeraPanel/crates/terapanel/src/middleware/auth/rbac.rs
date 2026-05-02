//! Role-Based Access Control (RBAC) implementation

use tera_common::error::{Error, Result};

/// Permission types
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    // Node management
    NodeList,
    NodeView,
    NodeControl,

    // Service management
    ServiceList,
    ServiceView,
    ServiceStart,
    ServiceStop,
    ServiceRestart,
    ServiceDelete,

    // File management
    FileList,
    FileRead,
    FileWrite,
    FileDelete,

    // Terminal
    TerminalList,
    TerminalView,
    TerminalControl,

    // User management
    UserList,
    UserView,
    UserCreate,
    UserUpdate,
    UserDelete,

    // System administration
    SystemConfig,
    SystemView,
}

/// Role definition
#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: Vec<Permission>,
}

/// Check if a role has a specific permission
pub fn has_permission(role: &Role, permission: &Permission) -> bool {
    role.permissions.contains(permission)
}

/// Check if a user has a specific permission based on their roles
pub fn user_has_permission(roles: &[Role], permission: &Permission) -> bool {
    roles.iter().any(|role| has_permission(role, permission))
}

/// Define system roles
pub fn get_system_roles() -> Vec<Role> {
    vec![
        // Super administrator - all permissions
        Role {
            name: "admin".to_string(),
            permissions: vec![
                Permission::NodeList,
                Permission::NodeView,
                Permission::NodeControl,
                Permission::ServiceList,
                Permission::ServiceView,
                Permission::ServiceStart,
                Permission::ServiceStop,
                Permission::ServiceRestart,
                Permission::ServiceDelete,
                Permission::FileList,
                Permission::FileRead,
                Permission::FileWrite,
                Permission::FileDelete,
                Permission::TerminalList,
                Permission::TerminalView,
                Permission::TerminalControl,
                Permission::UserList,
                Permission::UserView,
                Permission::UserCreate,
                Permission::UserUpdate,
                Permission::UserDelete,
                Permission::SystemConfig,
                Permission::SystemView,
            ],
        },
        // Operator - can manage services and view everything
        Role {
            name: "operator".to_string(),
            permissions: vec![
                Permission::NodeList,
                Permission::NodeView,
                Permission::ServiceList,
                Permission::ServiceView,
                Permission::ServiceStart,
                Permission::ServiceStop,
                Permission::ServiceRestart,
                Permission::FileList,
                Permission::FileRead,
                Permission::TerminalList,
                Permission::TerminalView,
            ],
        },
        // Viewer - read-only access
        Role {
            name: "viewer".to_string(),
            permissions: vec![
                Permission::NodeList,
                Permission::NodeView,
                Permission::ServiceList,
                Permission::ServiceView,
                Permission::FileList,
                Permission::FileRead,
                Permission::TerminalList,
                Permission::TerminalView,
            ],
        },
    ]
}

/// Get a role by name
pub fn get_role_by_name(name: &str) -> Option<Role> {
    let roles = get_system_roles();
    roles.into_iter().find(|role| role.name == name)
}

/// Authorization error
#[derive(Debug)]
pub struct AuthzError(pub String);

impl std::fmt::Display for AuthzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authorization error: {}", self.0)
    }
}

impl std::error::Error for AuthzError {}
