//! Ordinary Signal contract for repository-ledger.
//!
//! This crate carries peer-callable repository event submissions and read
//! queries. Owner-only configuration lives in `owner-signal-repository-ledger`.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryName(String);

impl RepositoryName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct GitoliteUser(String);

impl GitoliteUser {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryObjectIdentifier(String);

impl RepositoryObjectIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryRefName(String);

impl RepositoryRefName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryTimestamp(String);

impl RepositoryTimestamp {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryCommitMessage(String);

impl RepositoryCommitMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryFilePath(String);

impl RepositoryFilePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryFileStatus(String);

impl RepositoryFileStatus {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryTextSearch(String);

impl RepositoryTextSearch {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct RepositoryEventSequence(u64);

impl RepositoryEventSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RepositoryQueryLimit(u64);

impl RepositoryQueryLimit {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RepositoryLedgerPath(String);

impl RepositoryLedgerPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RepositoryLedgerSocketMode(u32);

impl RepositoryLedgerSocketMode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum RepositoryClass {
    RuntimeComponent,
    OrdinarySignalContract,
    OwnerSignalContract,
    ReportLane,
    Configuration,
    Documentation,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RefUpdate {
    pub old_object_identifier: RepositoryObjectIdentifier,
    pub new_object_identifier: RepositoryObjectIdentifier,
    pub ref_name: RepositoryRefName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryReceiveHookNotification {
    pub repository_name: RepositoryName,
    pub gitolite_user: GitoliteUser,
    pub received_at: RepositoryTimestamp,
    pub daemon_socket_present: bool,
    pub ref_updates: Vec<RefUpdate>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryFileChange {
    pub status: RepositoryFileStatus,
    pub path: RepositoryFilePath,
    pub old_path: Option<RepositoryFilePath>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCommitObservation {
    pub object_identifier: RepositoryObjectIdentifier,
    pub ref_name: RepositoryRefName,
    pub commit_timestamp: RepositoryTimestamp,
    pub message: RepositoryCommitMessage,
    pub changed_files: Vec<RepositoryFileChange>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryPushObservation {
    pub notification: RepositoryReceiveHookNotification,
    pub commits: Vec<RepositoryCommitObservation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRegistration {
    pub repository_name: RepositoryName,
    pub repository_class: RepositoryClass,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryEvent {
    pub sequence: RepositoryEventSequence,
    pub notification: RepositoryReceiveHookNotification,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryEventRecorded {
    pub sequence: RepositoryEventSequence,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryEventQuery {
    pub repository_name: Option<RepositoryName>,
    pub since_sequence: Option<RepositoryEventSequence>,
    pub limit: RepositoryQueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryEventListing {
    pub events: Vec<RepositoryEvent>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRecentRepositoriesQuery {
    pub since_received_at: Option<RepositoryTimestamp>,
    pub limit: RepositoryQueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRecentRepository {
    pub repository_name: RepositoryName,
    pub latest_received_at: RepositoryTimestamp,
    pub latest_sequence: RepositoryEventSequence,
    pub push_count: RepositoryQueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRecentRepositoriesListing {
    pub repositories: Vec<RepositoryRecentRepository>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryChangedFileQuery {
    pub repository_name: Option<RepositoryName>,
    pub since_received_at: Option<RepositoryTimestamp>,
    pub until_received_at: Option<RepositoryTimestamp>,
    pub path_contains: Option<RepositoryTextSearch>,
    pub limit: RepositoryQueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryChangedFile {
    pub repository_name: RepositoryName,
    pub received_at: RepositoryTimestamp,
    pub sequence: RepositoryEventSequence,
    pub commit_object_identifier: RepositoryObjectIdentifier,
    pub ref_name: RepositoryRefName,
    pub status: RepositoryFileStatus,
    pub path: RepositoryFilePath,
    pub old_path: Option<RepositoryFilePath>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryChangedFileListing {
    pub files: Vec<RepositoryChangedFile>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCommitMessageQuery {
    pub repository_name: Option<RepositoryName>,
    pub since_received_at: Option<RepositoryTimestamp>,
    pub until_received_at: Option<RepositoryTimestamp>,
    pub message_contains: Option<RepositoryTextSearch>,
    pub limit: RepositoryQueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCommit {
    pub repository_name: RepositoryName,
    pub received_at: RepositoryTimestamp,
    pub sequence: RepositoryEventSequence,
    pub object_identifier: RepositoryObjectIdentifier,
    pub ref_name: RepositoryRefName,
    pub commit_timestamp: RepositoryTimestamp,
    pub message: RepositoryCommitMessage,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCommitListing {
    pub commits: Vec<RepositoryCommit>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCatalogQuery;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCatalogListing {
    pub repositories: Vec<RepositoryRegistration>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryLedgerDaemonConfiguration {
    pub ordinary_socket_path: RepositoryLedgerPath,
    pub ordinary_socket_mode: RepositoryLedgerSocketMode,
    pub owner_socket_path: RepositoryLedgerPath,
    pub owner_socket_mode: RepositoryLedgerSocketMode,
    pub store_path: RepositoryLedgerPath,
    pub spool_directory: RepositoryLedgerPath,
}

nota_config::impl_rkyv_configuration!(RepositoryLedgerDaemonConfiguration);

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RepositoryLedgerOperationKind {
    RepositoryReceiveHookNotification,
    RepositoryPushObservation,
    RepositoryEventQuery,
    RepositoryRecentRepositoriesQuery,
    RepositoryChangedFileQuery,
    RepositoryCommitMessageQuery,
    RepositoryCatalogQuery,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RepositoryLedgerUnimplementedReason {
    DaemonSocketMissing,
    StoreUnavailable,
    NotInPrototypeScope,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryLedgerRequestUnimplemented {
    pub operation: RepositoryLedgerOperationKind,
    pub reason: RepositoryLedgerUnimplementedReason,
}

signal_channel! {
    channel RepositoryLedger {
        request RepositoryLedgerRequest {
            Assert RepositoryReceiveHookNotification(RepositoryReceiveHookNotification),
            Assert RepositoryPushObservation(RepositoryPushObservation),
            Match RepositoryEventQuery(RepositoryEventQuery),
            Match RepositoryRecentRepositoriesQuery(RepositoryRecentRepositoriesQuery),
            Match RepositoryChangedFileQuery(RepositoryChangedFileQuery),
            Match RepositoryCommitMessageQuery(RepositoryCommitMessageQuery),
            Match RepositoryCatalogQuery(RepositoryCatalogQuery),
        }
        reply RepositoryLedgerReply {
            RepositoryEventRecorded(RepositoryEventRecorded),
            RepositoryEventListing(RepositoryEventListing),
            RepositoryRecentRepositoriesListing(RepositoryRecentRepositoriesListing),
            RepositoryChangedFileListing(RepositoryChangedFileListing),
            RepositoryCommitListing(RepositoryCommitListing),
            RepositoryCatalogListing(RepositoryCatalogListing),
            RepositoryLedgerRequestUnimplemented(RepositoryLedgerRequestUnimplemented),
        }
    }
}

pub type Frame = RepositoryLedgerFrame;
pub type FrameBody = RepositoryLedgerFrameBody;
pub type ChannelRequest = RepositoryLedgerChannelRequest;
pub type ChannelReply = RepositoryLedgerChannelReply;
pub type RequestBuilder = RepositoryLedgerRequestBuilder;

impl RepositoryLedgerRequest {
    pub fn operation_kind(&self) -> RepositoryLedgerOperationKind {
        match self {
            Self::RepositoryReceiveHookNotification(_) => {
                RepositoryLedgerOperationKind::RepositoryReceiveHookNotification
            }
            Self::RepositoryPushObservation(_) => {
                RepositoryLedgerOperationKind::RepositoryPushObservation
            }
            Self::RepositoryEventQuery(_) => RepositoryLedgerOperationKind::RepositoryEventQuery,
            Self::RepositoryRecentRepositoriesQuery(_) => {
                RepositoryLedgerOperationKind::RepositoryRecentRepositoriesQuery
            }
            Self::RepositoryChangedFileQuery(_) => {
                RepositoryLedgerOperationKind::RepositoryChangedFileQuery
            }
            Self::RepositoryCommitMessageQuery(_) => {
                RepositoryLedgerOperationKind::RepositoryCommitMessageQuery
            }
            Self::RepositoryCatalogQuery(_) => {
                RepositoryLedgerOperationKind::RepositoryCatalogQuery
            }
        }
    }
}
