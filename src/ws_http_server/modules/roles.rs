use super::{super::super::spec::user::Role, ProviderError};

/// Provider represents an arbitrary provider of the roles lib API.
/// The roles API is responsible for managing roles corresponding to certain
/// users.
pub trait Provider {
    /// Determines whether or not a user with the given user ID has the given
    /// role.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn user_has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError>;

    /// Assigns the given role to a user without removing any existing roles
    /// from the aforementioned user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn give_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError>;

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError>;

    /// Removes all of the roles corresponding to the given user, returning
    /// all roles that were removed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be purged
    fn purge_roles(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError>;

    /// Obtains a list of the roles held by a certain user, indicated by the
    /// user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be determined
    fn roles_for_user(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError>;
}
