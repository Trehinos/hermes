#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Permission(String);

pub trait HasPermissions {
    fn has(&self, permission: &Permission) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct AccessControl {
    permissions: Vec<Permission>,
}

impl AccessControl {
    pub fn new(permissions: Vec<Permission>) -> Self {
        Self { permissions }
    }
    pub fn require(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
    pub fn require_all(&self, permissions: &[Permission]) -> bool {
        permissions.iter().all(|p| self.require(p))
    }
    pub fn require_any(&self, permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| self.require(p))
    }
    pub fn require_none(&self, permissions: &[Permission]) -> bool {
        permissions.iter().all(|p| !self.require(p))
    }
    pub fn require_not(&self, permission: &Permission) -> bool {
        !self.require(permission)
    }
    pub fn with(&self, permission: Permission) -> Self {
        Self {
            permissions: self
                .permissions
                .clone()
                .into_iter()
                .chain(vec![permission])
                .collect(),
        }
    }
    pub fn without(&self, permission: Permission) -> Self {
        Self {
            permissions: self
                .permissions
                .clone()
                .into_iter()
                .filter(|p| p != &permission)
                .collect(),
        }
    }

    pub fn is_authorized(&self, actor: &dyn HasPermissions) -> bool {
        self.permissions.iter().all(|p| actor.has(p))
    }
}
