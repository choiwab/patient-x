use codec::{Decode, Encode, MaxEncodedLen};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec;
use sp_std::vec::Vec;

/// Type alias for session token (32 bytes)
pub type SessionToken = [u8; 32];

/// Maximum number of roles per user
pub const MAX_ROLES_PER_USER: u32 = 10;

/// Maximum number of permissions per role
pub const MAX_PERMISSIONS_PER_ROLE: u32 = 50;

/// System roles in Patient X platform
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Role {
	/// Patient - can manage their own data and consents
	Patient,
	/// Researcher - can request access to data
	Researcher,
	/// Institution - can verify identities and negative data
	Institution,
	/// Auditor - can verify and audit activities
	Auditor,
	/// Publisher - can publish negative data
	Publisher,
	/// Regulator - can oversee compliance
	Regulator,
	/// Administrator - full system access
	Administrator,
}

/// Granular permissions for fine-grained access control
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Permission {
	// Identity permissions
	RegisterIdentity,
	VerifyIdentity,
	UpdateIdentity,

	// Consent permissions
	GrantConsent,
	RevokeConsent,
	UpdateConsent,
	CheckConsent,

	// Data permissions
	UploadData,
	AccessData,
	DeleteData,
	ExportData,

	// Negative registry permissions
	SubmitNegativeData,
	VerifyNegativeData,
	RejectNegativeData,
	ClaimReward,
	AddCitation,

	// Administrative permissions
	ManageRoles,
	ManagePermissions,
	ViewAuditLog,
	ManageCompliance,

	// Marketplace permissions
	CreateListing,
	PurchaseData,
	SetPrice,
	WithdrawFunds,
}

impl Role {
	/// Get default permissions for a role
	pub fn default_permissions(&self) -> Vec<Permission> {
		match self {
			Role::Patient => vec![
				Permission::RegisterIdentity,
				Permission::UpdateIdentity,
				Permission::GrantConsent,
				Permission::RevokeConsent,
				Permission::UpdateConsent,
				Permission::UploadData,
				Permission::DeleteData,
				Permission::ViewAuditLog,
				Permission::CreateListing,
				Permission::SetPrice,
				Permission::WithdrawFunds,
			],
			Role::Researcher => vec![
				Permission::RegisterIdentity,
				Permission::UpdateIdentity,
				Permission::CheckConsent,
				Permission::AccessData,
				Permission::SubmitNegativeData,
				Permission::ClaimReward,
				Permission::AddCitation,
				Permission::PurchaseData,
			],
			Role::Institution => vec![
				Permission::RegisterIdentity,
				Permission::VerifyIdentity,
				Permission::VerifyNegativeData,
				Permission::RejectNegativeData,
				Permission::ViewAuditLog,
			],
			Role::Auditor => vec![
				Permission::RegisterIdentity,
				Permission::VerifyIdentity,
				Permission::VerifyNegativeData,
				Permission::ViewAuditLog,
				Permission::ManageCompliance,
			],
			Role::Publisher => vec![
				Permission::RegisterIdentity,
				Permission::SubmitNegativeData,
				Permission::AddCitation,
				Permission::CreateListing,
			],
			Role::Regulator => vec![
				Permission::ViewAuditLog,
				Permission::ManageCompliance,
				Permission::VerifyIdentity,
				Permission::VerifyNegativeData,
			],
			Role::Administrator => {
				// Admins have all permissions
				vec![
					Permission::RegisterIdentity,
					Permission::VerifyIdentity,
					Permission::UpdateIdentity,
					Permission::GrantConsent,
					Permission::RevokeConsent,
					Permission::UpdateConsent,
					Permission::CheckConsent,
					Permission::UploadData,
					Permission::AccessData,
					Permission::DeleteData,
					Permission::ExportData,
					Permission::SubmitNegativeData,
					Permission::VerifyNegativeData,
					Permission::RejectNegativeData,
					Permission::ClaimReward,
					Permission::AddCitation,
					Permission::ManageRoles,
					Permission::ManagePermissions,
					Permission::ViewAuditLog,
					Permission::ManageCompliance,
					Permission::CreateListing,
					Permission::PurchaseData,
					Permission::SetPrice,
					Permission::WithdrawFunds,
				]
			},
		}
	}
}

/// Session information for authenticated users
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct SessionInfo<T: crate::Config> {
	/// Account that owns this session
	pub account: T::AccountId,
	/// When the session was created
	pub created_at: BlockNumberFor<T>,
	/// When the session expires
	pub expires_at: BlockNumberFor<T>,
	/// Last activity timestamp
	pub last_activity: BlockNumberFor<T>,
	/// Whether session is still active
	pub is_active: bool,
}

/// Role assignment record
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct RoleAssignment<T: crate::Config> {
	/// The role assigned
	pub role: Role,
	/// Who assigned this role
	pub assigned_by: T::AccountId,
	/// When it was assigned
	pub assigned_at: BlockNumberFor<T>,
	/// Optional expiration
	pub expires_at: Option<BlockNumberFor<T>>,
}

/// Convert u8 to Role for dispatchable simplicity
impl From<u8> for Role {
	fn from(value: u8) -> Self {
		match value {
			0 => Role::Patient,
			1 => Role::Researcher,
			2 => Role::Institution,
			3 => Role::Auditor,
			4 => Role::Publisher,
			5 => Role::Regulator,
			6 => Role::Administrator,
			_ => Role::Patient, // Default to Patient for unknown values
		}
	}
}

/// Convert Role to u8 for storage efficiency
impl From<Role> for u8 {
	fn from(role: Role) -> Self {
		match role {
			Role::Patient => 0,
			Role::Researcher => 1,
			Role::Institution => 2,
			Role::Auditor => 3,
			Role::Publisher => 4,
			Role::Regulator => 5,
			Role::Administrator => 6,
		}
	}
}

/// Convert u8 to Permission
impl From<u8> for Permission {
	fn from(value: u8) -> Self {
		match value {
			0 => Permission::RegisterIdentity,
			1 => Permission::VerifyIdentity,
			2 => Permission::UpdateIdentity,
			3 => Permission::GrantConsent,
			4 => Permission::RevokeConsent,
			5 => Permission::UpdateConsent,
			6 => Permission::CheckConsent,
			7 => Permission::UploadData,
			8 => Permission::AccessData,
			9 => Permission::DeleteData,
			10 => Permission::ExportData,
			11 => Permission::SubmitNegativeData,
			12 => Permission::VerifyNegativeData,
			13 => Permission::RejectNegativeData,
			14 => Permission::ClaimReward,
			15 => Permission::AddCitation,
			16 => Permission::ManageRoles,
			17 => Permission::ManagePermissions,
			18 => Permission::ViewAuditLog,
			19 => Permission::ManageCompliance,
			20 => Permission::CreateListing,
			21 => Permission::PurchaseData,
			22 => Permission::SetPrice,
			23 => Permission::WithdrawFunds,
			_ => Permission::RegisterIdentity, // Default
		}
	}
}
