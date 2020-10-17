
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::{Error, Result};
use std::collections::HashMap;
use tokio::stream::{Stream, StreamExt};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

pub type TlsConnr = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
pub type TlsClient = hyper::Client<TlsConnr, hyper::Body>;
pub type Authenticator = yup_oauth2::authenticator::Authenticator<TlsConnr>;

#[derive(Debug, Clone)]
pub enum ApiError {
  InputDataError(String),
  HTTPError(hyper::StatusCode),
}

impl std::error::Error for ApiError {}
impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(self, f)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AboutDriveThemes {
    /// A link to this theme's background image.
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_link: Option<String>,
    /// The color of this theme as an RGB hex string.
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<String>,
    /// The ID of the theme.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AboutStorageQuota {
    /// i64: The usage limit, if applicable. This will not be present if the user has unlimited storage.
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// i64: The total usage across all services.
    #[serde(rename = "usage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    /// i64: The usage by all files in Google Drive.
    #[serde(rename = "usageInDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_in_drive: Option<String>,
    /// i64: The usage by trashed files in Google Drive.
    #[serde(rename = "usageInDriveTrash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_in_drive_trash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AboutTeamDriveThemes {
    /// Deprecated - use driveThemes/backgroundImageLink instead.
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_link: Option<String>,
    /// Deprecated - use driveThemes/colorRgb instead.
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<String>,
    /// Deprecated - use driveThemes/id instead.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct About {
    /// Whether the user has installed the requesting app.
    #[serde(rename = "appInstalled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_installed: Option<bool>,
    /// Whether the user can create shared drives.
    #[serde(rename = "canCreateDrives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_create_drives: Option<bool>,
    /// Deprecated - use canCreateDrives instead.
    #[serde(rename = "canCreateTeamDrives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_create_team_drives: Option<bool>,
    /// A list of themes that are supported for shared drives.
    #[serde(rename = "driveThemes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_themes: Option<Vec<AboutDriveThemes>>,
    /// A map of source MIME type to possible targets for all supported exports.
    #[serde(rename = "exportFormats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_formats: Option<HashMap<String,Vec<String>>>,
    /// The currently supported folder colors as RGB hex strings.
    #[serde(rename = "folderColorPalette")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_color_palette: Option<Vec<String>>,
    /// A map of source MIME type to possible targets for all supported imports.
    #[serde(rename = "importFormats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_formats: Option<HashMap<String,Vec<String>>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#about".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A map of maximum import sizes by MIME type, in bytes.
    #[serde(rename = "maxImportSizes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_import_sizes: Option<HashMap<String,String>>,
    /// i64: The maximum upload size in bytes.
    #[serde(rename = "maxUploadSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_size: Option<String>,
    /// The user's storage quota limits and usage. All fields are measured in bytes.
    #[serde(rename = "storageQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_quota: Option<AboutStorageQuota>,
    /// Deprecated - use driveThemes instead.
    #[serde(rename = "teamDriveThemes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drive_themes: Option<Vec<AboutTeamDriveThemes>>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Change {
    /// The type of the change. Possible values are file and drive.
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive: Option<Drive>,
    /// The ID of the shared drive associated with this change.
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    /// The ID of the file which has changed.
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#change".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access.
    #[serde(rename = "removed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
    #[serde(rename = "teamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drive: Option<TeamDrive>,
    /// Deprecated - use driveId instead.
    #[serde(rename = "teamDriveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drive_id: Option<String>,
    /// DateTime: The time of this change (RFC 3339 date-time).
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
    /// Deprecated - use changeType instead.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangeList {
    /// The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "changes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<Change>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#changeList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The starting page token for future changes. This will be present only if the end of the current changes list has been reached.
    #[serde(rename = "newStartPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_start_page_token: Option<String>,
    /// The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Channel {
    /// The address where notifications are delivered for this channel.
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// i64: Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// A UUID or similar unique string that identifies this channel.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Additional parameters controlling delivery channel behavior. Optional.
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String,String>>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<bool>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// A version-specific identifier for the watched resource.
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The type of delivery mechanism used for this channel.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentQuotedFileContent {
    /// The MIME type of the quoted content.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The quoted content itself. This is interpreted as plain text if set through the API.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Comment {
    /// A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties.
    #[serde(rename = "anchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    /// The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// DateTime: The time at which the comment was created (RFC 3339 date-time).
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
    /// Whether the comment has been deleted. A deleted comment has no content.
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The content of the comment with HTML formatting.
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// The ID of the comment.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#comment".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// DateTime: The last time the comment or any of its replies was modified (RFC 3339 date-time).
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<DateTime<Utc>>,
    /// The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
    #[serde(rename = "quotedFileContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quoted_file_content: Option<CommentQuotedFileContent>,
    /// The full list of replies to the comment in chronological order.
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Reply>>,
    /// Whether the comment has been resolved by one of its replies.
    #[serde(rename = "resolved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentList {
    /// The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#commentList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContentRestriction {
    /// Whether the content of the file is read-only. If a file is read-only, a new revision of the file may not be added, comments may not be added or modified, and the title of the file may not be modified.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Reason for why the content of the file is restricted. This is only mutable on requests that also set readOnly=true.
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "restrictingUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricting_user: Option<User>,
    /// DateTime: The time at which the content restriction was set (formatted RFC 3339 timestamp). Only populated if readOnly is true.
    #[serde(rename = "restrictionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction_time: Option<DateTime<Utc>>,
    /// The type of the content restriction. Currently the only possible value is globalContentRestriction.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DriveBackgroundImageFile {
    /// The ID of an image file in Google Drive to use for the background image.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename = "xCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_coordinate: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename = "yCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_coordinate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DriveCapabilities {
    /// Whether the current user can add children to folders in this shared drive.
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive.
    #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this shared drive.
    #[serde(rename = "canChangeDomainUsersOnlyRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this shared drive.
    #[serde(rename = "canChangeDriveBackground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_drive_background: Option<bool>,
    /// Whether the current user can change the driveMembersOnly restriction of this shared drive.
    #[serde(rename = "canChangeDriveMembersOnlyRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_drive_members_only_restriction: Option<bool>,
    /// Whether the current user can comment on files in this shared drive.
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    /// Whether the current user can copy files in this shared drive.
    #[serde(rename = "canCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_copy: Option<bool>,
    /// Whether the current user can delete children from folders in this shared drive.
    #[serde(rename = "canDeleteChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive.
    #[serde(rename = "canDeleteDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_drive: Option<bool>,
    /// Whether the current user can download files in this shared drive.
    #[serde(rename = "canDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
    /// Whether the current user can edit files in this shared drive
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of folders in this shared drive.
    #[serde(rename = "canListChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can add members to this shared drive or remove them or change their role.
    #[serde(rename = "canManageMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_members: Option<bool>,
    /// Whether the current user can read the revisions resource of files in this shared drive.
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_revisions: Option<bool>,
    /// Whether the current user can rename files or folders in this shared drive.
    #[serde(rename = "canRename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_rename: Option<bool>,
    /// Whether the current user can rename this shared drive.
    #[serde(rename = "canRenameDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_rename_drive: Option<bool>,
    /// Whether the current user can share files or folders in this shared drive.
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// Whether the current user can trash children from folders in this shared drive.
    #[serde(rename = "canTrashChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trash_children: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DriveRestrictions {
    /// Whether administrative privileges on this shared drive are required to modify restrictions.
    #[serde(rename = "adminManagedRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_managed_restrictions: Option<bool>,
    /// Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive.
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_requires_writer_permission: Option<bool>,
    /// Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive.
    #[serde(rename = "domainUsersOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_users_only: Option<bool>,
    /// Whether access to items inside this shared drive is restricted to its members.
    #[serde(rename = "driveMembersOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_members_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Drive {
    /// An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    #[serde(rename = "backgroundImageFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_file: Option<DriveBackgroundImageFile>,
    /// A short-lived link to this shared drive's background image.
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_link: Option<String>,
    /// Capabilities the current user has on this shared drive.
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<DriveCapabilities>,
    /// The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId.
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<String>,
    /// DateTime: The time at which the shared drive was created (RFC 3339 date-time).
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
    /// Whether the shared drive is hidden from default view.
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// The ID of this shared drive which is also the ID of the top level folder of this shared drive.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#drive".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name of this shared drive.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A set of restrictions that apply to this shared drive or items inside this shared drive.
    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<DriveRestrictions>,
    /// The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename = "themeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DriveList {
    /// The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "drives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<Drive>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#driveList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileCapabilities {
    /// Whether the current user can add children to this folder. This is always false when the item is not a folder.
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can add a folder from another drive (different shared drive or My Drive) to this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename = "canAddFolderFromAnotherDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_folder_from_another_drive: Option<bool>,
    /// Whether the current user can add a parent for the item without removing an existing parent in the same request. Not populated for shared drive files.
    #[serde(rename = "canAddMyDriveParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_my_drive_parent: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this file.
    #[serde(rename = "canChangeCopyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_copy_requires_writer_permission: Option<bool>,
    /// Deprecated
    #[serde(rename = "canChangeViewersCanCopyContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_viewers_can_copy_content: Option<bool>,
    /// Whether the current user can comment on this file.
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    /// Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder.
    #[serde(rename = "canCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_copy: Option<bool>,
    /// Whether the current user can delete this file.
    #[serde(rename = "canDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    /// Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename = "canDeleteChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can download this file.
    #[serde(rename = "canDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
    /// Whether the current user can edit this file. Other factors may limit the type of changes a user can make to a file. For example, see canChangeCopyRequiresWriterPermission or canModifyContent.
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of this folder. This is always false when the item is not a folder.
    #[serde(rename = "canListChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can modify the content of this file.
    #[serde(rename = "canModifyContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_modify_content: Option<bool>,
    /// Whether the current user can modify restrictions on content of this file.
    #[serde(rename = "canModifyContentRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_modify_content_restriction: Option<bool>,
    /// Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename = "canMoveChildrenOutOfDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_children_out_of_drive: Option<bool>,
    /// Deprecated - use canMoveChildrenOutOfDrive instead.
    #[serde(rename = "canMoveChildrenOutOfTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_children_out_of_team_drive: Option<bool>,
    /// Whether the current user can move children of this folder within this drive. This is false when the item is not a folder. Note that a request to move the child may still fail depending on the current user's access to the child and to the destination folder.
    #[serde(rename = "canMoveChildrenWithinDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_children_within_drive: Option<bool>,
    /// Deprecated - use canMoveChildrenWithinDrive instead.
    #[serde(rename = "canMoveChildrenWithinTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_children_within_team_drive: Option<bool>,
    /// Deprecated - use canMoveItemOutOfDrive instead.
    #[serde(rename = "canMoveItemIntoTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_item_into_team_drive: Option<bool>,
    /// Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added.
    #[serde(rename = "canMoveItemOutOfDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_item_out_of_drive: Option<bool>,
    /// Deprecated - use canMoveItemOutOfDrive instead.
    #[serde(rename = "canMoveItemOutOfTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_item_out_of_team_drive: Option<bool>,
    /// Whether the current user can move this item within this drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added and the parent that is being removed.
    #[serde(rename = "canMoveItemWithinDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_item_within_drive: Option<bool>,
    /// Deprecated - use canMoveItemWithinDrive instead.
    #[serde(rename = "canMoveItemWithinTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_item_within_team_drive: Option<bool>,
    /// Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead.
    #[serde(rename = "canMoveTeamDriveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_move_team_drive_item: Option<bool>,
    /// Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives.
    #[serde(rename = "canReadDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_drive: Option<bool>,
    /// Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read.
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_revisions: Option<bool>,
    /// Deprecated - use canReadDrive instead.
    #[serde(rename = "canReadTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_team_drive: Option<bool>,
    /// Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead.
    #[serde(rename = "canRemoveChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_remove_children: Option<bool>,
    /// Whether the current user can remove a parent from the item without adding another parent in the same request. Not populated for shared drive files.
    #[serde(rename = "canRemoveMyDriveParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_remove_my_drive_parent: Option<bool>,
    /// Whether the current user can rename this file.
    #[serde(rename = "canRename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_rename: Option<bool>,
    /// Whether the current user can modify the sharing settings for this file.
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// Whether the current user can move this file to trash.
    #[serde(rename = "canTrash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trash: Option<bool>,
    /// Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename = "canTrashChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trash_children: Option<bool>,
    /// Whether the current user can restore this file from trash.
    #[serde(rename = "canUntrash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_untrash: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileContentHintsThumbnail {
    /// The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5).
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The MIME type of the thumbnail.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileContentHints {
    /// Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements.
    #[serde(rename = "indexableText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable_text: Option<String>,
    /// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileContentHintsThumbnail>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileImageMediaMetadataLocation {
    /// The altitude stored in the image.
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    /// The latitude stored in the image.
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// The longitude stored in the image.
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileImageMediaMetadata {
    /// The aperture used to create the photo (f-number).
    #[serde(rename = "aperture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aperture: Option<f32>,
    /// The make of the camera used to create the photo.
    #[serde(rename = "cameraMake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_make: Option<String>,
    /// The model of the camera used to create the photo.
    #[serde(rename = "cameraModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_model: Option<String>,
    /// The color space of the photo.
    #[serde(rename = "colorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    /// The exposure bias of the photo (APEX value).
    #[serde(rename = "exposureBias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_bias: Option<f32>,
    /// The exposure mode used to create the photo.
    #[serde(rename = "exposureMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_mode: Option<String>,
    /// The length of the exposure, in seconds.
    #[serde(rename = "exposureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<f32>,
    /// Whether a flash was used to create the photo.
    #[serde(rename = "flashUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_used: Option<bool>,
    /// The focal length used to create the photo, in millimeters.
    #[serde(rename = "focalLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<f32>,
    /// The height of the image in pixels.
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// The ISO speed used to create the photo.
    #[serde(rename = "isoSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_speed: Option<i32>,
    /// The lens used to create the photo.
    #[serde(rename = "lens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lens: Option<String>,
    /// Geographic location information stored in the image.
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<FileImageMediaMetadataLocation>,
    /// The smallest f-number of the lens at the focal length used to create the photo (APEX value).
    #[serde(rename = "maxApertureValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_aperture_value: Option<f32>,
    /// The metering mode used to create the photo.
    #[serde(rename = "meteringMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_mode: Option<String>,
    /// The number of clockwise 90 degree rotations applied from the image's original orientation.
    #[serde(rename = "rotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i32>,
    /// The type of sensor used to create the photo.
    #[serde(rename = "sensor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor: Option<String>,
    /// The distance to the subject of the photo, in meters.
    #[serde(rename = "subjectDistance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_distance: Option<i32>,
    /// The date and time the photo was taken (EXIF DateTime).
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// The white balance mode used to create the photo.
    #[serde(rename = "whiteBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_balance: Option<String>,
    /// The width of the image in pixels.
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileShortcutDetails {
    /// The ID of the file that this shortcut points to.
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// The MIME type of the file that this shortcut points to. The value of this field is a snapshot of the target's MIME type, captured when the shortcut is created.
    #[serde(rename = "targetMimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_mime_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileVideoMediaMetadata {
    /// i64: The duration of the video in milliseconds.
    #[serde(rename = "durationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<String>,
    /// The height of the video in pixels.
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// The width of the video in pixels.
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct File {
    /// A collection of arbitrary key-value pairs which are private to the requesting app. Entries with null values are cleared in update and copy requests.
    #[serde(rename = "appProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_properties: Option<HashMap<String,String>>,
    /// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<FileCapabilities>,
    /// Additional information about the content of the file. These fields are never populated in responses.
    #[serde(rename = "contentHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hints: Option<FileContentHints>,
    /// Restrictions for accessing the content of the file. Only populated if such a restriction exists.
    #[serde(rename = "contentRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_restrictions: Option<Vec<ContentRestriction>>,
    /// Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_requires_writer_permission: Option<bool>,
    /// DateTime: The time at which the file was created (RFC 3339 date-time).
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
    /// A short description of the file.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the shared drive the file resides in. Only populated for items in shared drives.
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    /// Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder.
    #[serde(rename = "explicitlyTrashed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicitly_trashed: Option<bool>,
    /// Links for exporting Google Docs to specific formats.
    #[serde(rename = "exportLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_links: Option<HashMap<String,String>>,
    /// The final component of fullFileExtension. This is only available for files with binary content in Google Drive.
    #[serde(rename = "fileExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// The color for a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource. If an unsupported color is specified, the closest color in the palette will be used instead.
    #[serde(rename = "folderColorRgb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_color_rgb: Option<String>,
    /// The full file extension extracted from the name field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive. This is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension.
    #[serde(rename = "fullFileExtension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_file_extension: Option<String>,
    /// Whether there are permissions directly on this file. This field is only populated for items in shared drives.
    #[serde(rename = "hasAugmentedPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_augmented_permissions: Option<bool>,
    /// Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
    #[serde(rename = "hasThumbnail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_thumbnail: Option<bool>,
    /// The ID of the file's head revision. This is currently only available for files with binary content in Google Drive.
    #[serde(rename = "headRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_revision_id: Option<String>,
    /// A static, unauthenticated link to the file's icon.
    #[serde(rename = "iconLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_link: Option<String>,
    /// The ID of the file.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional metadata about image media, if available.
    #[serde(rename = "imageMediaMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_media_metadata: Option<FileImageMediaMetadata>,
    /// Whether the file was created or opened by the requesting app.
    #[serde(rename = "isAppAuthorized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_app_authorized: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#file".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "lastModifyingUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modifying_user: Option<User>,
    /// The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive.
    #[serde(rename = "md5Checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5_checksum: Option<String>,
    /// The MIME type of the file. Google Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Whether the file has been modified by this user.
    #[serde(rename = "modifiedByMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_by_me: Option<bool>,
    /// DateTime: The last time the file was modified by the user (RFC 3339 date-time).
    #[serde(rename = "modifiedByMeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_by_me_time: Option<DateTime<Utc>>,
    /// DateTime: The last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user.
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<DateTime<Utc>>,
    /// The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive.
    #[serde(rename = "originalFilename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
    /// Whether the user owns the file. Not populated for items in shared drives.
    #[serde(rename = "ownedByMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by_me: Option<bool>,
    /// The owners of the file. Currently, only certain legacy files may have more than one owner. Not populated for items in shared drives.
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<User>>,
    /// The IDs of the parent folders which contain the file. If not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list.
    #[serde(rename = "parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    /// List of permission IDs for users with access to this file.
    #[serde(rename = "permissionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_ids: Option<Vec<String>>,
    /// The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives.
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
    /// A collection of arbitrary key-value pairs which are visible to all apps. Entries with null values are cleared in update and copy requests.
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String,String>>,
    /// i64: The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled.
    #[serde(rename = "quotaBytesUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_bytes_used: Option<String>,
    /// Whether the file has been shared. Not populated for items in shared drives.
    #[serde(rename = "shared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    /// DateTime: The time at which the file was shared with the user, if applicable (RFC 3339 date-time).
    #[serde(rename = "sharedWithMeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_with_me_time: Option<DateTime<Utc>>,
    #[serde(rename = "sharingUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_user: Option<User>,
    /// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
    #[serde(rename = "shortcutDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_details: Option<FileShortcutDetails>,
    /// i64: The size of the file's content in bytes. This is only applicable to files with binary content in Google Drive.
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'.
    #[serde(rename = "spaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaces: Option<Vec<String>>,
    /// Whether the user has starred the file.
    #[serde(rename = "starred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    /// Deprecated - use driveId instead.
    #[serde(rename = "teamDriveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drive_id: Option<String>,
    /// A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in Files.thumbnailLink must be fetched using a credentialed request.
    #[serde(rename = "thumbnailLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_link: Option<String>,
    /// i64: The thumbnail version for use in thumbnail cache invalidation.
    #[serde(rename = "thumbnailVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_version: Option<String>,
    /// Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.
    #[serde(rename = "trashed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trashed: Option<bool>,
    /// DateTime: The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.
    #[serde(rename = "trashedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trashed_time: Option<DateTime<Utc>>,
    #[serde(rename = "trashingUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trashing_user: Option<User>,
    /// i64: A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user.
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Additional metadata about video media. This may not be available immediately upon upload.
    #[serde(rename = "videoMediaMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_media_metadata: Option<FileVideoMediaMetadata>,
    /// Whether the file has been viewed by this user.
    #[serde(rename = "viewedByMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed_by_me: Option<bool>,
    /// DateTime: The last time the file was viewed by the user (RFC 3339 date-time).
    #[serde(rename = "viewedByMeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed_by_me_time: Option<DateTime<Utc>>,
    /// Deprecated - use copyRequiresWriterPermission instead.
    #[serde(rename = "viewersCanCopyContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewers_can_copy_content: Option<bool>,
    /// A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive.
    #[serde(rename = "webContentLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_content_link: Option<String>,
    /// A link for opening the file in a relevant Google editor or viewer in a browser.
    #[serde(rename = "webViewLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_view_link: Option<String>,
    /// Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives.
    #[serde(rename = "writersCanShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writers_can_share: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileList {
    /// The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    /// Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the "allDrives" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as "user" or "drive".
    #[serde(rename = "incompleteSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_search: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#fileList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneratedIds {
    /// The IDs generated for the requesting user in the specified space.
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#generatedIds".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The type of file that can be created with these IDs.
    #[serde(rename = "space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionPermissionDetails {
    /// Whether this permission is inherited. This field is always populated. This is an output-only field.
    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /// The ID of the item from which this permission is inherited. This is an output-only field.
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    /// The permission type for this user. While new values may be added in future, the following are currently possible:   - file  - member
    #[serde(rename = "permissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<String>,
    /// The primary role for this user. While new values may be added in the future, the following are currently possible:   - organizer  - fileOrganizer  - writer  - commenter  - reader
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionTeamDrivePermissionDetails {
    /// Deprecated - use permissionDetails/inherited instead.
    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /// Deprecated - use permissionDetails/inheritedFrom instead.
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    /// Deprecated - use permissionDetails/role instead.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Deprecated - use permissionDetails/permissionType instead.
    #[serde(rename = "teamDrivePermissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drive_permission_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Permission {
    /// Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone.
    #[serde(rename = "allowFileDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_file_discovery: Option<bool>,
    /// Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The "pretty" name of the value of the permission. The following is a list of examples for each type of permission:   - user - User's full name, as defined for their Google account, such as "Joe Smith."  - group - Name of the Google Group, such as "The Company Administrators."  - domain - String domain name, such as "thecompany.com."  - anyone - No displayName is present.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The domain to which this permission refers.
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The email address of the user or group to which this permission refers.
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// DateTime: The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:   - They can only be set on user and group permissions  - The time must be in the future  - The time cannot be more than a year in the future
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<DateTime<Utc>>,
    /// The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId. IDs should be treated as opaque values.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#permission".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items.
    #[serde(rename = "permissionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_details: Option<Vec<PermissionPermissionDetails>>,
    /// A link to the user's profile photo, if available.
    #[serde(rename = "photoLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_link: Option<String>,
    /// The role granted by this permission. While new values may be supported in the future, the following are currently allowed:   - owner  - organizer  - fileOrganizer  - writer  - commenter  - reader
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Deprecated - use permissionDetails instead.
    #[serde(rename = "teamDrivePermissionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drive_permission_details: Option<Vec<PermissionTeamDrivePermissionDetails>>,
    /// The type of the grantee. Valid values are:   - user  - group  - domain  - anyone  When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    /// Indicates the view for this permission. Only populated for permissions that belong to a view. published is the only supported value.
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionList {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#permissionList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Reply {
    /// The action the reply performed to the parent comment. Valid values are:   - resolve  - reopen
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    /// The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// DateTime: The time at which the reply was created (RFC 3339 date-time).
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
    /// Whether the reply has been deleted. A deleted reply has no content.
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The content of the reply with HTML formatting.
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// The ID of the reply.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#reply".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// DateTime: The last time the reply was modified (RFC 3339 date-time).
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplyList {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#replyList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Reply>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Revision {
    /// Links for exporting Google Docs to specific formats.
    #[serde(rename = "exportLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_links: Option<HashMap<String,String>>,
    /// The ID of the revision.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive.
    #[serde(rename = "keepForever")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_forever: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#revision".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "lastModifyingUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modifying_user: Option<User>,
    /// The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive.
    #[serde(rename = "md5Checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5_checksum: Option<String>,
    /// The MIME type of the revision.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// DateTime: The last time the revision was modified (RFC 3339 date-time).
    #[serde(rename = "modifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<DateTime<Utc>>,
    /// The original filename used to create this revision. This is only applicable to files with binary content in Drive.
    #[serde(rename = "originalFilename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
    /// Whether subsequent revisions will be automatically republished. This is only applicable to Google Docs.
    #[serde(rename = "publishAuto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_auto: Option<bool>,
    /// Whether this revision is published. This is only applicable to Google Docs.
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// A link to the published revision. This is only populated for Google Sites files.
    #[serde(rename = "publishedLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_link: Option<String>,
    /// Whether this revision is published outside the domain. This is only applicable to Google Docs.
    #[serde(rename = "publishedOutsideDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_outside_domain: Option<bool>,
    /// i64: The size of the revision's content in bytes. This is only applicable to files with binary content in Drive.
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevisionList {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#revisionList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "revisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<Revision>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StartPageToken {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#startPageToken".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The starting page token for listing changes.
    #[serde(rename = "startPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamDriveBackgroundImageFile {
    /// The ID of an image file in Drive to use for the background image.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename = "xCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_coordinate: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename = "yCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_coordinate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamDriveCapabilities {
    /// Whether the current user can add children to folders in this Team Drive.
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive.
    #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this Team Drive.
    #[serde(rename = "canChangeDomainUsersOnlyRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this Team Drive.
    #[serde(rename = "canChangeTeamDriveBackground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_team_drive_background: Option<bool>,
    /// Whether the current user can change the teamMembersOnly restriction of this Team Drive.
    #[serde(rename = "canChangeTeamMembersOnlyRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_team_members_only_restriction: Option<bool>,
    /// Whether the current user can comment on files in this Team Drive.
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    /// Whether the current user can copy files in this Team Drive.
    #[serde(rename = "canCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_copy: Option<bool>,
    /// Whether the current user can delete children from folders in this Team Drive.
    #[serde(rename = "canDeleteChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive.
    #[serde(rename = "canDeleteTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_team_drive: Option<bool>,
    /// Whether the current user can download files in this Team Drive.
    #[serde(rename = "canDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
    /// Whether the current user can edit files in this Team Drive
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// Whether the current user can list the children of folders in this Team Drive.
    #[serde(rename = "canListChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can add members to this Team Drive or remove them or change their role.
    #[serde(rename = "canManageMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_members: Option<bool>,
    /// Whether the current user can read the revisions resource of files in this Team Drive.
    #[serde(rename = "canReadRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_revisions: Option<bool>,
    /// Deprecated - use canDeleteChildren or canTrashChildren instead.
    #[serde(rename = "canRemoveChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_remove_children: Option<bool>,
    /// Whether the current user can rename files or folders in this Team Drive.
    #[serde(rename = "canRename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_rename: Option<bool>,
    /// Whether the current user can rename this Team Drive.
    #[serde(rename = "canRenameTeamDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_rename_team_drive: Option<bool>,
    /// Whether the current user can share files or folders in this Team Drive.
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// Whether the current user can trash children from folders in this Team Drive.
    #[serde(rename = "canTrashChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trash_children: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamDriveRestrictions {
    /// Whether administrative privileges on this Team Drive are required to modify restrictions.
    #[serde(rename = "adminManagedRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_managed_restrictions: Option<bool>,
    /// Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive.
    #[serde(rename = "copyRequiresWriterPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_requires_writer_permission: Option<bool>,
    /// Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive.
    #[serde(rename = "domainUsersOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_users_only: Option<bool>,
    /// Whether access to items inside this Team Drive is restricted to members of this Team Drive.
    #[serde(rename = "teamMembersOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_members_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamDrive {
    /// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    #[serde(rename = "backgroundImageFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_file: Option<TeamDriveBackgroundImageFile>,
    /// A short-lived link to this Team Drive's background image.
    #[serde(rename = "backgroundImageLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_link: Option<String>,
    /// Capabilities the current user has on this Team Drive.
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<TeamDriveCapabilities>,
    /// The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId.
    #[serde(rename = "colorRgb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<String>,
    /// DateTime: The time at which the Team Drive was created (RFC 3339 date-time).
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
    /// The ID of this Team Drive which is also the ID of the top level folder of this Team Drive.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#teamDrive".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name of this Team Drive.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<TeamDriveRestrictions>,
    /// The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename = "themeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamDriveList {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#teamDriveList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "teamDrives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_drives: Option<Vec<TeamDrive>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    /// A plain text displayable name for this user.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#user".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Whether this user is the requesting user.
    #[serde(rename = "me")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub me: Option<bool>,
    /// The user's ID as visible in Permission resources.
    #[serde(rename = "permissionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_id: Option<String>,
    /// A link to the user's profile photo, if available.
    #[serde(rename = "photoLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AboutGetParams {
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangesGetStartPageTokenParams {
    /// The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive is returned.
    #[serde(rename = "driveId")]
    pub drive_id: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Deprecated use driveId instead.
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangesListParams {
    /// The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
    #[serde(rename = "driveId")]
    pub drive_id: Option<String>,
    /// Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
    #[serde(rename = "includeCorpusRemovals")]
    pub include_corpus_removals: Option<bool>,
    /// Whether both My Drive and shared drive items should be included in results.
    #[serde(rename = "includeItemsFromAllDrives")]
    pub include_items_from_all_drives: Option<bool>,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
    #[serde(rename = "includeRemoved")]
    pub include_removed: Option<bool>,
    /// Deprecated use includeItemsFromAllDrives instead.
    #[serde(rename = "includeTeamDriveItems")]
    pub include_team_drive_items: Option<bool>,
    /// The maximum number of changes to return per page.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    #[serde(rename = "pageToken")]
    pub page_token: String,
    /// Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
    #[serde(rename = "restrictToMyDrive")]
    pub restrict_to_my_drive: Option<bool>,
    /// A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    #[serde(rename = "spaces")]
    pub spaces: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Deprecated use driveId instead.
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangesWatchParams {
    /// The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
    #[serde(rename = "driveId")]
    pub drive_id: Option<String>,
    /// Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
    #[serde(rename = "includeCorpusRemovals")]
    pub include_corpus_removals: Option<bool>,
    /// Whether both My Drive and shared drive items should be included in results.
    #[serde(rename = "includeItemsFromAllDrives")]
    pub include_items_from_all_drives: Option<bool>,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
    #[serde(rename = "includeRemoved")]
    pub include_removed: Option<bool>,
    /// Deprecated use includeItemsFromAllDrives instead.
    #[serde(rename = "includeTeamDriveItems")]
    pub include_team_drive_items: Option<bool>,
    /// The maximum number of changes to return per page.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    #[serde(rename = "pageToken")]
    pub page_token: String,
    /// Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
    #[serde(rename = "restrictToMyDrive")]
    pub restrict_to_my_drive: Option<bool>,
    /// A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    #[serde(rename = "spaces")]
    pub spaces: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Deprecated use driveId instead.
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChannelsStopParams {
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentsCreateParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentsDeleteParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentsGetParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether to return deleted comments. Deleted comments will not include their original content.
    #[serde(rename = "includeDeleted")]
    pub include_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentsListParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether to include deleted comments. Deleted comments will not include their original content.
    #[serde(rename = "includeDeleted")]
    pub include_deleted: Option<bool>,
    /// The maximum number of comments to return per page.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time).
    #[serde(rename = "startModifiedTime")]
    pub start_modified_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommentsUpdateParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesCreateParams {
    /// An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesDeleteParams {
    /// The ID of the shared drive.
    #[serde(rename = "driveId")]
    pub drive_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesGetParams {
    /// The ID of the shared drive.
    #[serde(rename = "driveId")]
    pub drive_id: String,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesHideParams {
    /// The ID of the shared drive.
    #[serde(rename = "driveId")]
    pub drive_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesListParams {
    /// Maximum number of shared drives to return.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Page token for shared drives.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// Query string for searching shared drives.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesUnhideParams {
    /// The ID of the shared drive.
    #[serde(rename = "driveId")]
    pub drive_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DrivesUpdateParams {
    /// The ID of the shared drive.
    #[serde(rename = "driveId")]
    pub drive_id: String,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesCopyParams {
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter only takes effect if the item is not in a shared drive. Requests that specify more than one parent fail.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
    #[serde(rename = "ignoreDefaultVisibility")]
    pub ignore_default_visibility: Option<bool>,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
    #[serde(rename = "keepRevisionForever")]
    pub keep_revision_forever: Option<bool>,
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    #[serde(rename = "ocrLanguage")]
    pub ocr_language: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesCreateParams {
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter only takes effect if the item is not in a shared drive. Requests that specify more than one parent fail.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
    /// Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
    #[serde(rename = "ignoreDefaultVisibility")]
    pub ignore_default_visibility: Option<bool>,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
    #[serde(rename = "keepRevisionForever")]
    pub keep_revision_forever: Option<bool>,
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    #[serde(rename = "ocrLanguage")]
    pub ocr_language: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Whether to use the uploaded content as indexable text.
    #[serde(rename = "useContentAsIndexableText")]
    pub use_content_as_indexable_text: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesDeleteParams {
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter will only take effect if the item is not in a shared drive. If an item's last parent is deleted but the item itself is not, the item will be placed under its owner's root.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesEmptyTrashParams {
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter will only take effect if the item is not in a shared drive. If an item's last parent is deleted but the item itself is not, the item will be placed under its owner's root.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesExportParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The MIME type of the format requested for this export.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesGenerateIdsParams {
    /// The number of IDs to return.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'.
    #[serde(rename = "space")]
    pub space: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesGetParams {
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    #[serde(rename = "acknowledgeAbuse")]
    pub acknowledge_abuse: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesListParams {
    /// Groupings of files to which the query applies. Supported groupings are: 'user' (files created by, opened by, or shared directly with the user), 'drive' (files in the specified shared drive as indicated by the 'driveId'), 'domain' (files shared to the user's domain), and 'allDrives' (A combination of 'user' and 'drive' for all drives where the user is a member). When able, use 'user' or 'drive', instead of 'allDrives', for efficiency.
    #[serde(rename = "corpora")]
    pub corpora: Option<String>,
    /// The source of files to list. Deprecated: use 'corpora' instead.
    #[serde(rename = "corpus")]
    pub corpus: Option<String>,
    /// ID of the shared drive to search.
    #[serde(rename = "driveId")]
    pub drive_id: Option<String>,
    /// Whether both My Drive and shared drive items should be included in results.
    #[serde(rename = "includeItemsFromAllDrives")]
    pub include_items_from_all_drives: Option<bool>,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Deprecated use includeItemsFromAllDrives instead.
    #[serde(rename = "includeTeamDriveItems")]
    pub include_team_drive_items: Option<bool>,
    /// A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored.
    #[serde(rename = "orderBy")]
    pub order_by: Option<String>,
    /// The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// A query for filtering the file results. See the "Search for Files" guide for supported syntax.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// A comma-separated list of spaces to query within the corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    #[serde(rename = "spaces")]
    pub spaces: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Deprecated use driveId instead.
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesUpdateParams {
    /// A comma-separated list of parent IDs to add.
    #[serde(rename = "addParents")]
    pub add_parents: Option<String>,
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter only takes effect if the item is not in a shared drive. If the item's owner makes a request to add a single parent, the item is removed from all current folders and placed in the requested folder. Other requests that increase the number of parents fail, except when the canAddMyDriveParent file capability is true and a single parent is being added.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
    #[serde(rename = "keepRevisionForever")]
    pub keep_revision_forever: Option<bool>,
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    #[serde(rename = "ocrLanguage")]
    pub ocr_language: Option<String>,
    /// A comma-separated list of parent IDs to remove.
    #[serde(rename = "removeParents")]
    pub remove_parents: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Whether to use the uploaded content as indexable text.
    #[serde(rename = "useContentAsIndexableText")]
    pub use_content_as_indexable_text: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesWatchParams {
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    #[serde(rename = "acknowledgeAbuse")]
    pub acknowledge_abuse: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionsCreateParams {
    /// A plain text custom message to include in the notification email.
    #[serde(rename = "emailMessage")]
    pub email_message: Option<String>,
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter only takes effect if the item is not in a shared drive. See moveToNewOwnersRoot for details.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
    /// The ID of the file or shared drive.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// This parameter only takes effect if the item is not in a shared drive and the request is attempting to transfer the ownership of the item. When set to true, the item is moved to the new owner's My Drive root folder and all prior parents removed. If set to false, when enforceSingleParent=true, parents are not changed. If set to false, when enforceSingleParent=false, existing parents are not changed; however, the file will be added to the new owner's My Drive root folder, unless it is already in the new owner's My Drive.
    #[serde(rename = "moveToNewOwnersRoot")]
    pub move_to_new_owners_root: Option<bool>,
    /// Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers.
    #[serde(rename = "sendNotificationEmail")]
    pub send_notification_email: Option<bool>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect.
    #[serde(rename = "transferOwnership")]
    pub transfer_ownership: Option<bool>,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionsDeleteParams {
    /// The ID of the file or shared drive.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the permission.
    #[serde(rename = "permissionId")]
    pub permission_id: String,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionsGetParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the permission.
    #[serde(rename = "permissionId")]
    pub permission_id: String,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionsListParams {
    /// The ID of the file or shared drive.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Specifies which additional view's permissions to include in the response. Only 'published' is supported.
    #[serde(rename = "includePermissionsForView")]
    pub include_permissions_for_view: Option<String>,
    /// The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionsUpdateParams {
    /// The ID of the file or shared drive.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the permission.
    #[serde(rename = "permissionId")]
    pub permission_id: String,
    /// Whether to remove the expiration date.
    #[serde(rename = "removeExpiration")]
    pub remove_expiration: Option<bool>,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
    /// Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect.
    #[serde(rename = "transferOwnership")]
    pub transfer_ownership: Option<bool>,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepliesCreateParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepliesDeleteParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the reply.
    #[serde(rename = "replyId")]
    pub reply_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepliesGetParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether to return deleted replies. Deleted replies will not include their original content.
    #[serde(rename = "includeDeleted")]
    pub include_deleted: Option<bool>,
    /// The ID of the reply.
    #[serde(rename = "replyId")]
    pub reply_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepliesListParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether to include deleted replies. Deleted replies will not include their original content.
    #[serde(rename = "includeDeleted")]
    pub include_deleted: Option<bool>,
    /// The maximum number of replies to return per page.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepliesUpdateParams {
    /// The ID of the comment.
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the reply.
    #[serde(rename = "replyId")]
    pub reply_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevisionsDeleteParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the revision.
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevisionsGetParams {
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    #[serde(rename = "acknowledgeAbuse")]
    pub acknowledge_abuse: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the revision.
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevisionsListParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The maximum number of revisions to return per page.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevisionsUpdateParams {
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The ID of the revision.
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamdrivesCreateParams {
    /// An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamdrivesDeleteParams {
    /// The ID of the Team Drive
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamdrivesGetParams {
    /// The ID of the Team Drive
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: String,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamdrivesListParams {
    /// Maximum number of Team Drives to return.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Page token for Team Drives.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// Query string for searching Team Drives.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TeamdrivesUpdateParams {
    /// The ID of the Team Drive
    #[serde(rename = "teamDriveId")]
    pub team_drive_id: String,
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
    #[serde(rename = "useDomainAdminAccess")]
    pub use_domain_admin_access: Option<bool>,
}

pub struct AboutService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl AboutService {
  /// Create a new AboutService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> AboutService {
    AboutService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Gets information about the user, the user's Drive, and system capabilities.
pub async fn get(
    &mut self, params: &AboutGetParams) -> Result<About> {

    let rel_path = format!("about", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct ChangesService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl ChangesService {
  /// Create a new ChangesService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> ChangesService {
    ChangesService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Gets the starting pageToken for listing future changes.
pub async fn get_start_page_token(
    &mut self, params: &ChangesGetStartPageTokenParams) -> Result<StartPageToken> {

    let rel_path = format!("changes/startPageToken", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.drive_id {
        url_params.push_str(&format!("&driveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.team_drive_id {
        url_params.push_str(&format!("&teamDriveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists the changes for a user or shared drive.
pub async fn list(
    &mut self, params: &ChangesListParams) -> Result<ChangeList> {

    let rel_path = format!("changes", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.drive_id {
        url_params.push_str(&format!("&driveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_corpus_removals {
        url_params.push_str(&format!("&includeCorpusRemovals={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_items_from_all_drives {
        url_params.push_str(&format!("&includeItemsFromAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_removed {
        url_params.push_str(&format!("&includeRemoved={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_team_drive_items {
        url_params.push_str(&format!("&includeTeamDriveItems={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.restrict_to_my_drive {
        url_params.push_str(&format!("&restrictToMyDrive={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.spaces {
        url_params.push_str(&format!("&spaces={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.team_drive_id {
        url_params.push_str(&format!("&teamDriveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    url_params.push_str(&format!("&pageToken={}",
        percent_encode(format!("{}", params.page_token).as_bytes(), NON_ALPHANUMERIC).to_string()));

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Subscribes to changes for a user.
pub async fn watch(
    &mut self, params: &ChangesWatchParams, req: &Channel) -> Result<Channel> {

    let rel_path = format!("changes/watch", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.drive_id {
        url_params.push_str(&format!("&driveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_corpus_removals {
        url_params.push_str(&format!("&includeCorpusRemovals={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_items_from_all_drives {
        url_params.push_str(&format!("&includeItemsFromAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_removed {
        url_params.push_str(&format!("&includeRemoved={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_team_drive_items {
        url_params.push_str(&format!("&includeTeamDriveItems={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.restrict_to_my_drive {
        url_params.push_str(&format!("&restrictToMyDrive={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.spaces {
        url_params.push_str(&format!("&spaces={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.team_drive_id {
        url_params.push_str(&format!("&teamDriveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    url_params.push_str(&format!("&pageToken={}",
        percent_encode(format!("{}", params.page_token).as_bytes(), NON_ALPHANUMERIC).to_string()));

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct ChannelsService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl ChannelsService {
  /// Create a new ChannelsService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> ChannelsService {
    ChannelsService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Stop watching resources through this channel
pub async fn stop(
    &mut self, params: &ChannelsStopParams, req: &Channel) -> Result<()> {

    let rel_path = format!("channels/stop", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct CommentsService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl CommentsService {
  /// Create a new CommentsService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> CommentsService {
    CommentsService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Creates a new comment on a file.
pub async fn create(
    &mut self, params: &CommentsCreateParams, req: &Comment) -> Result<Comment> {

    let rel_path = format!("files/{fileId}/comments", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deletes a comment.
pub async fn delete(
    &mut self, params: &CommentsDeleteParams) -> Result<()> {

    let rel_path = format!("files/{fileId}/comments/{commentId}", fileId=params.file_id,commentId=params.comment_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Gets a comment by ID.
pub async fn get(
    &mut self, params: &CommentsGetParams) -> Result<Comment> {

    let rel_path = format!("files/{fileId}/comments/{commentId}", fileId=params.file_id,commentId=params.comment_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.include_deleted {
        url_params.push_str(&format!("&includeDeleted={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists a file's comments.
pub async fn list(
    &mut self, params: &CommentsListParams) -> Result<CommentList> {

    let rel_path = format!("files/{fileId}/comments", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.include_deleted {
        url_params.push_str(&format!("&includeDeleted={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.start_modified_time {
        url_params.push_str(&format!("&startModifiedTime={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates a comment with patch semantics.
pub async fn update(
    &mut self, params: &CommentsUpdateParams, req: &Comment) -> Result<Comment> {

    let rel_path = format!("files/{fileId}/comments/{commentId}", fileId=params.file_id,commentId=params.comment_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct DrivesService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl DrivesService {
  /// Create a new DrivesService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> DrivesService {
    DrivesService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Creates a new shared drive.
pub async fn create(
    &mut self, params: &DrivesCreateParams, req: &Drive) -> Result<Drive> {

    let rel_path = format!("drives", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    url_params.push_str(&format!("&requestId={}",
        percent_encode(format!("{}", params.request_id).as_bytes(), NON_ALPHANUMERIC).to_string()));

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
pub async fn delete(
    &mut self, params: &DrivesDeleteParams) -> Result<()> {

    let rel_path = format!("drives/{driveId}", driveId=params.drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Gets a shared drive's metadata by ID.
pub async fn get(
    &mut self, params: &DrivesGetParams) -> Result<Drive> {

    let rel_path = format!("drives/{driveId}", driveId=params.drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Hides a shared drive from the default view.
pub async fn hide(
    &mut self, params: &DrivesHideParams) -> Result<Drive> {

    let rel_path = format!("drives/{driveId}/hide", driveId=params.drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists the user's shared drives.
pub async fn list(
    &mut self, params: &DrivesListParams) -> Result<DriveList> {

    let rel_path = format!("drives", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.q {
        url_params.push_str(&format!("&q={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Restores a shared drive to the default view.
pub async fn unhide(
    &mut self, params: &DrivesUnhideParams) -> Result<Drive> {

    let rel_path = format!("drives/{driveId}/unhide", driveId=params.drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates the metadate for a shared drive.
pub async fn update(
    &mut self, params: &DrivesUpdateParams, req: &Drive) -> Result<Drive> {

    let rel_path = format!("drives/{driveId}", driveId=params.drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct FilesService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl FilesService {
  /// Create a new FilesService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> FilesService {
    FilesService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Creates a copy of a file and applies any requested updates with patch semantics. Folders cannot be copied.
pub async fn copy(
    &mut self, params: &FilesCopyParams, req: &File) -> Result<File> {

    let rel_path = format!("files/{fileId}/copy", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ignore_default_visibility {
        url_params.push_str(&format!("&ignoreDefaultVisibility={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.keep_revision_forever {
        url_params.push_str(&format!("&keepRevisionForever={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ocr_language {
        url_params.push_str(&format!("&ocrLanguage={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Creates a new file.
pub async fn create(
    &mut self, params: &FilesCreateParams, req: &File) -> Result<File> {

    let rel_path = format!("files", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ignore_default_visibility {
        url_params.push_str(&format!("&ignoreDefaultVisibility={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.keep_revision_forever {
        url_params.push_str(&format!("&keepRevisionForever={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ocr_language {
        url_params.push_str(&format!("&ocrLanguage={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_content_as_indexable_text {
        url_params.push_str(&format!("&useContentAsIndexableText={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Creates a new file.
pub async fn create_upload(
    &mut self, params: &FilesCreateParams, data: hyper::body::Bytes) -> Result<File> {
    let rel_path = "upload/drive/v3/files";
    let path = "https://www.googleapis.com/".to_string() + &rel_path;
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?uploadType=media&oauth_token={token}&fields=*", token=tok.as_str());

    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ignore_default_visibility {
        url_params.push_str(&format!("&ignoreDefaultVisibility={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.keep_revision_forever {
        url_params.push_str(&format!("&keepRevisionForever={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ocr_language {
        url_params.push_str(&format!("&ocrLanguage={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_content_as_indexable_text {
        url_params.push_str(&format!("&useContentAsIndexableText={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Length", data.len());
    let body = hyper::Body::from(data);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
pub async fn delete(
    &mut self, params: &FilesDeleteParams) -> Result<()> {

    let rel_path = format!("files/{fileId}", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Permanently deletes all of the user's trashed files.
pub async fn empty_trash(
    &mut self, params: &FilesEmptyTrashParams) -> Result<()> {

    let rel_path = format!("files/trash", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB.
pub async fn export(
    &mut self, params: &FilesExportParams,  dst: &mut std::io::Write) -> Result<()> {

    let rel_path = format!("files/trash", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    url_params.push_str(&format!("&mimeType={}",
        percent_encode(format!("{}", params.mime_type).as_bytes(), NON_ALPHANUMERIC).to_string()));

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = resp.into_body();
    let write_result = resp_body.map(move |chunk| { dst.write(chunk?.as_ref()); Ok(()) }).collect::<Vec<Result<()>>>().await;
    if let Some(e) = write_result.into_iter().find(|r| r.is_err()) {
        return e;
    }
    Ok(())
  }

  
/// Generates a set of file IDs which can be provided in create or copy requests.
pub async fn generate_ids(
    &mut self, params: &FilesGenerateIdsParams) -> Result<GeneratedIds> {

    let rel_path = format!("files/generateIds", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.count {
        url_params.push_str(&format!("&count={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.space {
        url_params.push_str(&format!("&space={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Gets a file's metadata or content by ID.
pub async fn get(
    &mut self, params: &FilesGetParams) -> Result<File> {

    let rel_path = format!("files/{fileId}", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.acknowledge_abuse {
        url_params.push_str(&format!("&acknowledgeAbuse={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists or searches files.
pub async fn list(
    &mut self, params: &FilesListParams) -> Result<FileList> {

    let rel_path = format!("files", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.corpora {
        url_params.push_str(&format!("&corpora={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.corpus {
        url_params.push_str(&format!("&corpus={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.drive_id {
        url_params.push_str(&format!("&driveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_items_from_all_drives {
        url_params.push_str(&format!("&includeItemsFromAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_team_drive_items {
        url_params.push_str(&format!("&includeTeamDriveItems={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.order_by {
        url_params.push_str(&format!("&orderBy={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.q {
        url_params.push_str(&format!("&q={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.spaces {
        url_params.push_str(&format!("&spaces={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.team_drive_id {
        url_params.push_str(&format!("&teamDriveId={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates a file's metadata and/or content. This method supports patch semantics.
pub async fn update(
    &mut self, params: &FilesUpdateParams, req: &File) -> Result<File> {

    let rel_path = format!("files/{fileId}", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.scripts".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.add_parents {
        url_params.push_str(&format!("&addParents={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.keep_revision_forever {
        url_params.push_str(&format!("&keepRevisionForever={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ocr_language {
        url_params.push_str(&format!("&ocrLanguage={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.remove_parents {
        url_params.push_str(&format!("&removeParents={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_content_as_indexable_text {
        url_params.push_str(&format!("&useContentAsIndexableText={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates a file's metadata and/or content. This method supports patch semantics.
pub async fn update_upload(
    &mut self, params: &FilesUpdateParams, data: hyper::body::Bytes) -> Result<File> {
    let rel_path = "upload/drive/v3/files/{fileId}";
    let path = "https://www.googleapis.com/".to_string() + &rel_path;
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?uploadType=media&oauth_token={token}&fields=*", token=tok.as_str());

    if let Some(ref val) = &params.add_parents {
        url_params.push_str(&format!("&addParents={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.keep_revision_forever {
        url_params.push_str(&format!("&keepRevisionForever={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.ocr_language {
        url_params.push_str(&format!("&ocrLanguage={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.remove_parents {
        url_params.push_str(&format!("&removeParents={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_content_as_indexable_text {
        url_params.push_str(&format!("&useContentAsIndexableText={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Length", data.len());
    let body = hyper::Body::from(data);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Subscribes to changes to a file
pub async fn watch(
    &mut self, params: &FilesWatchParams, req: &Channel) -> Result<Channel> {

    let rel_path = format!("files/{fileId}/watch", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.acknowledge_abuse {
        url_params.push_str(&format!("&acknowledgeAbuse={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct PermissionsService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl PermissionsService {
  /// Create a new PermissionsService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> PermissionsService {
    PermissionsService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Creates a permission for a file or shared drive.
pub async fn create(
    &mut self, params: &PermissionsCreateParams, req: &Permission) -> Result<Permission> {

    let rel_path = format!("files/{fileId}/permissions", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.email_message {
        url_params.push_str(&format!("&emailMessage={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.enforce_single_parent {
        url_params.push_str(&format!("&enforceSingleParent={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.move_to_new_owners_root {
        url_params.push_str(&format!("&moveToNewOwnersRoot={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.send_notification_email {
        url_params.push_str(&format!("&sendNotificationEmail={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.transfer_ownership {
        url_params.push_str(&format!("&transferOwnership={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deletes a permission.
pub async fn delete(
    &mut self, params: &PermissionsDeleteParams) -> Result<()> {

    let rel_path = format!("files/{fileId}/permissions/{permissionId}", fileId=params.file_id,permissionId=params.permission_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Gets a permission by ID.
pub async fn get(
    &mut self, params: &PermissionsGetParams) -> Result<Permission> {

    let rel_path = format!("files/{fileId}/permissions/{permissionId}", fileId=params.file_id,permissionId=params.permission_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists a file's or shared drive's permissions.
pub async fn list(
    &mut self, params: &PermissionsListParams) -> Result<PermissionList> {

    let rel_path = format!("files/{fileId}/permissions", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.include_permissions_for_view {
        url_params.push_str(&format!("&includePermissionsForView={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates a permission with patch semantics.
pub async fn update(
    &mut self, params: &PermissionsUpdateParams, req: &Permission) -> Result<Permission> {

    let rel_path = format!("files/{fileId}/permissions/{permissionId}", fileId=params.file_id,permissionId=params.permission_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.remove_expiration {
        url_params.push_str(&format!("&removeExpiration={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_all_drives {
        url_params.push_str(&format!("&supportsAllDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.supports_team_drives {
        url_params.push_str(&format!("&supportsTeamDrives={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.transfer_ownership {
        url_params.push_str(&format!("&transferOwnership={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct RepliesService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl RepliesService {
  /// Create a new RepliesService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> RepliesService {
    RepliesService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Creates a new reply to a comment.
pub async fn create(
    &mut self, params: &RepliesCreateParams, req: &Reply) -> Result<Reply> {

    let rel_path = format!("files/{fileId}/comments/{commentId}/replies", fileId=params.file_id,commentId=params.comment_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deletes a reply.
pub async fn delete(
    &mut self, params: &RepliesDeleteParams) -> Result<()> {

    let rel_path = format!("files/{fileId}/comments/{commentId}/replies/{replyId}", fileId=params.file_id,commentId=params.comment_id,replyId=params.reply_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Gets a reply by ID.
pub async fn get(
    &mut self, params: &RepliesGetParams) -> Result<Reply> {

    let rel_path = format!("files/{fileId}/comments/{commentId}/replies/{replyId}", fileId=params.file_id,commentId=params.comment_id,replyId=params.reply_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.include_deleted {
        url_params.push_str(&format!("&includeDeleted={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists a comment's replies.
pub async fn list(
    &mut self, params: &RepliesListParams) -> Result<ReplyList> {

    let rel_path = format!("files/{fileId}/comments/{commentId}/replies", fileId=params.file_id,commentId=params.comment_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.include_deleted {
        url_params.push_str(&format!("&includeDeleted={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates a reply with patch semantics.
pub async fn update(
    &mut self, params: &RepliesUpdateParams, req: &Reply) -> Result<Reply> {

    let rel_path = format!("files/{fileId}/comments/{commentId}/replies/{replyId}", fileId=params.file_id,commentId=params.comment_id,replyId=params.reply_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct RevisionsService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl RevisionsService {
  /// Create a new RevisionsService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> RevisionsService {
    RevisionsService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
pub async fn delete(
    &mut self, params: &RevisionsDeleteParams) -> Result<()> {

    let rel_path = format!("files/{fileId}/revisions/{revisionId}", fileId=params.file_id,revisionId=params.revision_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Gets a revision's metadata or content by ID.
pub async fn get(
    &mut self, params: &RevisionsGetParams) -> Result<Revision> {

    let rel_path = format!("files/{fileId}/revisions/{revisionId}", fileId=params.file_id,revisionId=params.revision_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.acknowledge_abuse {
        url_params.push_str(&format!("&acknowledgeAbuse={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Lists a file's revisions.
pub async fn list(
    &mut self, params: &RevisionsListParams) -> Result<RevisionList> {

    let rel_path = format!("files/{fileId}/revisions", fileId=params.file_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        "https://www.googleapis.com/auth/drive.metadata".to_string(),
        "https://www.googleapis.com/auth/drive.metadata.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.photos.readonly".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Updates a revision with patch semantics.
pub async fn update(
    &mut self, params: &RevisionsUpdateParams, req: &Revision) -> Result<Revision> {

    let rel_path = format!("files/{fileId}/revisions/{revisionId}", fileId=params.file_id,revisionId=params.revision_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.appdata".to_string(),
        "https://www.googleapis.com/auth/drive.file".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}

pub struct TeamdrivesService {
  client: TlsClient,
  authenticator: Authenticator,
  scopes: Vec<String>,
}

impl TeamdrivesService {
  /// Create a new TeamdrivesService object.
  pub fn new(client: TlsClient, auth: Authenticator) -> TeamdrivesService {
    TeamdrivesService { client: client, authenticator: auth, scopes: vec![] }
  }

  /// Explicitly select which scopes should be requested for authorization. Otherwise,
  /// a possibly too large scope will be requested.
  pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
    self.scopes = scopes.as_ref().into_iter().map(|s| s.as_ref().to_string()).collect();
  }

  
/// Deprecated use drives.create instead.
pub async fn create(
    &mut self, params: &TeamdrivesCreateParams, req: &TeamDrive) -> Result<TeamDrive> {

    let rel_path = format!("teamdrives", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    url_params.push_str(&format!("&requestId={}",
        percent_encode(format!("{}", params.request_id).as_bytes(), NON_ALPHANUMERIC).to_string()));

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("POST")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deprecated use drives.delete instead.
pub async fn delete(
    &mut self, params: &TeamdrivesDeleteParams) -> Result<()> {

    let rel_path = format!("teamdrives/{teamDriveId}", teamDriveId=params.team_drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("DELETE")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deprecated use drives.get instead.
pub async fn get(
    &mut self, params: &TeamdrivesGetParams) -> Result<TeamDrive> {

    let rel_path = format!("teamdrives/{teamDriveId}", teamDriveId=params.team_drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deprecated use drives.list instead.
pub async fn list(
    &mut self, params: &TeamdrivesListParams) -> Result<TeamDriveList> {

    let rel_path = format!("teamdrives", );
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        "https://www.googleapis.com/auth/drive.readonly".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.page_size {
        url_params.push_str(&format!("&pageSize={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.page_token {
        url_params.push_str(&format!("&pageToken={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.q {
        url_params.push_str(&format!("&q={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("GET")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }

  
/// Deprecated use drives.update instead
pub async fn update(
    &mut self, params: &TeamdrivesUpdateParams, req: &TeamDrive) -> Result<TeamDrive> {

    let rel_path = format!("teamdrives/{teamDriveId}", teamDriveId=params.team_drive_id);
    let path = "https://www.googleapis.com/drive/v3/".to_string() + &rel_path;
    let mut scopes = &self.scopes;
    if scopes.is_empty() {
        scopes = &vec!["https://www.googleapis.com/auth/drive".to_string(),
        ];
    }
    let tok = self.authenticator.token(&self.scopes).await?;
    let mut url_params = format!("?oauth_token={token}&fields=*", token=tok.as_str());
    if let Some(ref val) = &params.use_domain_admin_access {
        url_params.push_str(&format!("&useDomainAdminAccess={}",
            percent_encode(format!("{}", val).as_bytes(), NON_ALPHANUMERIC).to_string()));
    }

    let full_uri = path + &url_params;
    let reqb = hyper::Request::builder()
        .uri(full_uri)
        .method("PATCH")
        .header("Content-Type", "application/json");

    let body = hyper::Body::from("");
    let mut body_str = serde_json::to_string(req)?;
    if body_str == "null" {
        body_str.clear();
    }
    let body = hyper::Body::from(body_str);
    let request = reqb.body(body)?;
    let resp = self.client.request(request).await?;
    if !resp.status().is_success() {
        return Err(anyhow::Error::new(ApiError::HTTPError(resp.status())));
    }
    let resp_body = hyper::body::to_bytes(resp.into_body()).await?;
    let bodystr = String::from_utf8(resp_body.to_vec())?;
    let decoded = serde_json::from_str(&bodystr)?;
    Ok(decoded)
  }


}
