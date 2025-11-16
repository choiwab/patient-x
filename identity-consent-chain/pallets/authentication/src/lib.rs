//! # Authentication Pallet
//!
//! This pallet provides role-based access control (RBAC) and session management for Patient X.
//!
//! ## Overview
//!
//! The Authentication pallet enables:
//! - Role assignment and management
//! - Permission-based access control
//! - Session management for authenticated users
//! - Integration with identity-registry for user verification
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `assign_role`: Assign a role to a user
//! - `revoke_role`: Revoke a role from a user
//! - `create_session`: Create a new session for a user
//! - `end_session`: End an active session
//! - `check_permission`: Verify if a user has a specific permission

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;
pub mod weights;

pub use types::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Hash;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity_registry::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Default session duration in blocks
		#[pallet::constant]
		type DefaultSessionDuration: Get<BlockNumberFor<Self>>;

		/// Maximum number of active sessions per user
		#[pallet::constant]
		type MaxSessionsPerUser: Get<u32>;

		/// Weight information for extrinsics
		type WeightInfo: crate::weights::WeightInfo;
	}

	/// Storage for user roles
	/// Maps (AccountId, Role) -> RoleAssignment
	#[pallet::storage]
	#[pallet::getter(fn user_roles)]
	pub type UserRoles<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Role,
		RoleAssignment<T>,
		OptionQuery,
	>;

	/// Storage for custom permissions
	/// Maps (AccountId, Permission) -> bool
	#[pallet::storage]
	#[pallet::getter(fn custom_permissions)]
	pub type CustomPermissions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Permission,
		bool,
		ValueQuery,
	>;

	/// Storage for active sessions
	/// Maps SessionToken -> SessionInfo
	#[pallet::storage]
	#[pallet::getter(fn sessions)]
	pub type Sessions<T: Config> =
		StorageMap<_, Blake2_128Concat, SessionToken, SessionInfo<T>, OptionQuery>;

	/// Index of sessions per user
	/// Maps AccountId -> Vec<SessionToken>
	#[pallet::storage]
	#[pallet::getter(fn user_sessions)]
	pub type UserSessions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<SessionToken, T::MaxSessionsPerUser>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Role was assigned to a user
		RoleAssigned {
			account: T::AccountId,
			role_id: u8,
			assigned_by: T::AccountId,
		},
		/// Role was revoked from a user
		RoleRevoked {
			account: T::AccountId,
			role_id: u8,
			revoked_by: T::AccountId,
		},
		/// Custom permission was granted
		PermissionGranted {
			account: T::AccountId,
			permission_id: u8,
		},
		/// Custom permission was revoked
		PermissionRevoked {
			account: T::AccountId,
			permission_id: u8,
		},
		/// Session was created
		SessionCreated {
			account: T::AccountId,
			session_token: SessionToken,
		},
		/// Session was ended
		SessionEnded {
			account: T::AccountId,
			session_token: SessionToken,
		},
		/// Session expired
		SessionExpired {
			account: T::AccountId,
			session_token: SessionToken,
		},
		/// Permission check was performed
		PermissionChecked {
			account: T::AccountId,
			permission_id: u8,
			granted: bool,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// User has not registered an identity
		IdentityNotRegistered,
		/// Role already assigned
		RoleAlreadyAssigned,
		/// Role not found for user
		RoleNotFound,
		/// Not authorized to assign roles
		NotAuthorized,
		/// Permission denied
		PermissionDenied,
		/// Session not found
		SessionNotFound,
		/// Session expired
		SessionExpired,
		/// Session already active
		SessionAlreadyActive,
		/// Too many active sessions
		TooManySessions,
		/// Invalid session token
		InvalidSessionToken,
		/// Role has expired
		RoleExpired,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Assign a role to a user
		///
		/// # Arguments
		/// * `account` - The account to assign the role to
		/// * `role_id` - The role ID (0-6: Patient, Researcher, Institution, Auditor, Publisher, Regulator, Admin)
		/// * `duration_blocks` - Optional duration in blocks before expiration
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::assign_role())]
		pub fn assign_role(
			origin: OriginFor<T>,
			account: T::AccountId,
			role_id: u8,
			duration_blocks: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure target has registered identity
			ensure!(
				pallet_identity_registry::Pallet::<T>::identities(&account).is_some(),
				Error::<T>::IdentityNotRegistered
			);

			// Check if caller has permission to assign roles
			ensure!(
				Self::has_permission(&who, Permission::ManageRoles),
				Error::<T>::NotAuthorized
			);

			let role: Role = role_id.into();

			// Check if role already assigned
			ensure!(
				!UserRoles::<T>::contains_key(&account, &role),
				Error::<T>::RoleAlreadyAssigned
			);

			let now = frame_system::Pallet::<T>::block_number();
			let expires_at = duration_blocks.map(|duration| now + duration);

			let assignment = RoleAssignment {
				role: role.clone(),
				assigned_by: who.clone(),
				assigned_at: now,
				expires_at,
			};

			UserRoles::<T>::insert(&account, &role, assignment);

			Self::deposit_event(Event::RoleAssigned {
				account,
				role_id,
				assigned_by: who,
			});

			Ok(())
		}

		/// Revoke a role from a user
		///
		/// # Arguments
		/// * `account` - The account to revoke the role from
		/// * `role_id` - The role ID to revoke
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::revoke_role())]
		pub fn revoke_role(
			origin: OriginFor<T>,
			account: T::AccountId,
			role_id: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if caller has permission to manage roles
			ensure!(
				Self::has_permission(&who, Permission::ManageRoles),
				Error::<T>::NotAuthorized
			);

			let role: Role = role_id.into();

			// Ensure role exists
			ensure!(
				UserRoles::<T>::contains_key(&account, &role),
				Error::<T>::RoleNotFound
			);

			UserRoles::<T>::remove(&account, &role);

			Self::deposit_event(Event::RoleRevoked {
				account,
				role_id,
				revoked_by: who,
			});

			Ok(())
		}

		/// Grant a custom permission to a user
		///
		/// # Arguments
		/// * `account` - The account to grant permission to
		/// * `permission_id` - The permission ID (0-23)
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_parts(40_000_000, 0))]
		pub fn grant_permission(
			origin: OriginFor<T>,
			account: T::AccountId,
			permission_id: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if caller has permission to manage permissions
			ensure!(
				Self::has_permission(&who, Permission::ManagePermissions),
				Error::<T>::NotAuthorized
			);

			let permission: Permission = permission_id.into();

			CustomPermissions::<T>::insert(&account, &permission, true);

			Self::deposit_event(Event::PermissionGranted {
				account,
				permission_id,
			});

			Ok(())
		}

		/// Revoke a custom permission from a user
		///
		/// # Arguments
		/// * `account` - The account to revoke permission from
		/// * `permission_id` - The permission ID
		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_parts(40_000_000, 0))]
		pub fn revoke_permission(
			origin: OriginFor<T>,
			account: T::AccountId,
			permission_id: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if caller has permission to manage permissions
			ensure!(
				Self::has_permission(&who, Permission::ManagePermissions),
				Error::<T>::NotAuthorized
			);

			let permission: Permission = permission_id.into();

			CustomPermissions::<T>::remove(&account, &permission);

			Self::deposit_event(Event::PermissionRevoked {
				account,
				permission_id,
			});

			Ok(())
		}

		/// Create a new session for the caller
		///
		/// # Arguments
		/// * `duration_blocks` - Optional custom duration, otherwise uses default
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::create_session())]
		pub fn create_session(
			origin: OriginFor<T>,
			duration_blocks: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure user has registered identity
			ensure!(
				pallet_identity_registry::Pallet::<T>::identities(&who).is_some(),
				Error::<T>::IdentityNotRegistered
			);

			// Generate session token
			let session_token = Self::generate_session_token(&who);

			// Check session doesn't already exist
			ensure!(
				!Sessions::<T>::contains_key(&session_token),
				Error::<T>::SessionAlreadyActive
			);

			let now = frame_system::Pallet::<T>::block_number();
			let duration = duration_blocks.unwrap_or_else(T::DefaultSessionDuration::get);
			let expires_at = now + duration;

			let session_info = SessionInfo {
				account: who.clone(),
				created_at: now,
				expires_at,
				last_activity: now,
				is_active: true,
			};

			Sessions::<T>::insert(&session_token, session_info);

			// Update user sessions index
			UserSessions::<T>::try_mutate(&who, |sessions| {
				sessions.try_push(session_token).map_err(|_| Error::<T>::TooManySessions)
			})?;

			Self::deposit_event(Event::SessionCreated {
				account: who,
				session_token,
			});

			Ok(())
		}

		/// End an active session
		///
		/// # Arguments
		/// * `session_token` - The session token to end
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::end_session())]
		pub fn end_session(
			origin: OriginFor<T>,
			session_token: SessionToken,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get session
			let mut session = Sessions::<T>::get(&session_token)
				.ok_or(Error::<T>::SessionNotFound)?;

			// Ensure caller owns the session
			ensure!(session.account == who, Error::<T>::NotAuthorized);

			// Mark as inactive
			session.is_active = false;
			Sessions::<T>::insert(&session_token, session);

			Self::deposit_event(Event::SessionEnded {
				account: who,
				session_token,
			});

			Ok(())
		}

		/// Check if a user has a specific permission
		///
		/// # Arguments
		/// * `account` - The account to check
		/// * `permission_id` - The permission ID to check
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::check_permission())]
		pub fn check_permission(
			origin: OriginFor<T>,
			account: T::AccountId,
			permission_id: u8,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let permission: Permission = permission_id.into();
			let has_permission = Self::has_permission(&account, permission.clone());

			Self::deposit_event(Event::PermissionChecked {
				account,
				permission_id,
				granted: has_permission,
			});

			if has_permission {
				Ok(())
			} else {
				Err(Error::<T>::PermissionDenied.into())
			}
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Generate a unique session token
		fn generate_session_token(account: &T::AccountId) -> SessionToken {
			let now = frame_system::Pallet::<T>::block_number();
			let mut data = account.encode();
			data.extend_from_slice(&now.encode());
			let nonce = frame_system::Pallet::<T>::account_nonce(account);
			data.extend_from_slice(&nonce.encode());
			T::Hashing::hash(&data).as_ref()[0..32].try_into().unwrap_or([0u8; 32])
		}

		/// Check if a user has a specific permission
		pub fn has_permission(account: &T::AccountId, permission: Permission) -> bool {
			// Check custom permissions first
			if CustomPermissions::<T>::get(account, &permission) {
				return true;
			}

			// Check role-based permissions
			let now = frame_system::Pallet::<T>::block_number();

			// Iterate through all roles
			for role in [
				Role::Patient,
				Role::Researcher,
				Role::Institution,
				Role::Auditor,
				Role::Publisher,
				Role::Regulator,
				Role::Administrator,
			] {
				if let Some(assignment) = UserRoles::<T>::get(account, &role) {
					// Check if role has expired
					if let Some(expires_at) = assignment.expires_at {
						if now > expires_at {
							continue;
						}
					}

					// Check if role has this permission
					if role.default_permissions().contains(&permission) {
						return true;
					}
				}
			}

			false
		}

		/// Check if a session is valid
		pub fn is_session_valid(session_token: &SessionToken) -> bool {
			if let Some(session) = Sessions::<T>::get(session_token) {
				let now = frame_system::Pallet::<T>::block_number();
				session.is_active && now <= session.expires_at
			} else {
				false
			}
		}

		/// Get all roles for a user
		pub fn get_user_roles(account: &T::AccountId) -> Vec<Role> {
			let mut roles = Vec::new();
			let now = frame_system::Pallet::<T>::block_number();

			for role in [
				Role::Patient,
				Role::Researcher,
				Role::Institution,
				Role::Auditor,
				Role::Publisher,
				Role::Regulator,
				Role::Administrator,
			] {
				if let Some(assignment) = UserRoles::<T>::get(account, &role) {
					// Check if not expired
					if let Some(expires_at) = assignment.expires_at {
						if now > expires_at {
							continue;
						}
					}
					roles.push(role);
				}
			}

			roles
		}
	}
}
