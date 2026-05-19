//! Ordinary Signal contract for repository-ledger.
//!
//! This crate carries peer-callable repository event submissions and read
//! queries. Owner-only configuration lives in `owner-signal-repository-ledger`.

use nota_codec::{NotaEnum, NotaRecord, NotaSum, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Name(String);

impl Name {
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
pub struct ObjectIdentifier(String);

impl ObjectIdentifier {
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
pub struct RefName(String);

impl RefName {
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
pub struct Timestamp(String);

impl Timestamp {
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
pub struct CommitMessage(String);

impl CommitMessage {
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
pub struct FilePath(String);

impl FilePath {
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
pub struct FileStatus(String);

impl FileStatus {
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
pub struct TextSearch(String);

impl TextSearch {
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
pub struct EventSequence(u64);

impl EventSequence {
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
pub struct QueryLimit(u64);

impl QueryLimit {
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
pub struct FilesystemPath(String);

impl FilesystemPath {
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
pub struct SocketMode(u32);

impl SocketMode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Class {
    RuntimeComponent,
    OrdinarySignalContract,
    OwnerSignalContract,
    ReportLane,
    Configuration,
    Documentation,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RefUpdate {
    pub old_object_identifier: ObjectIdentifier,
    pub new_object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ReceiveHookNotification {
    pub repository_name: Name,
    pub gitolite_user: GitoliteUser,
    pub received_at: Timestamp,
    pub daemon_socket_present: bool,
    pub ref_updates: Vec<RefUpdate>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    pub status: FileStatus,
    pub path: FilePath,
    pub old_path: Option<FilePath>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CommitObservation {
    pub object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub commit_timestamp: Timestamp,
    pub message: CommitMessage,
    pub changed_files: Vec<FileChange>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PushObservation {
    pub notification: ReceiveHookNotification,
    pub commits: Vec<CommitObservation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub repository_name: Name,
    pub repository_class: Class,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub sequence: EventSequence,
    pub notification: ReceiveHookNotification,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EventRecorded {
    pub sequence: EventSequence,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Events {
    pub repository_name: Option<Name>,
    pub since_sequence: Option<EventSequence>,
    pub limit: QueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EventListing {
    pub events: Vec<Event>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecentRepositories {
    pub since_received_at: Option<Timestamp>,
    pub limit: QueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecentRepository {
    pub repository_name: Name,
    pub latest_received_at: Timestamp,
    pub latest_sequence: EventSequence,
    pub push_count: QueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecentRepositoriesListing {
    pub repositories: Vec<RecentRepository>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ChangedFiles {
    pub repository_name: Option<Name>,
    pub since_received_at: Option<Timestamp>,
    pub until_received_at: Option<Timestamp>,
    pub path_contains: Option<TextSearch>,
    pub limit: QueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ChangedFile {
    pub repository_name: Name,
    pub received_at: Timestamp,
    pub sequence: EventSequence,
    pub commit_object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub status: FileStatus,
    pub path: FilePath,
    pub old_path: Option<FilePath>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ChangedFileListing {
    pub files: Vec<ChangedFile>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CommitMessages {
    pub repository_name: Option<Name>,
    pub since_received_at: Option<Timestamp>,
    pub until_received_at: Option<Timestamp>,
    pub message_contains: Option<TextSearch>,
    pub limit: QueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub repository_name: Name,
    pub received_at: Timestamp,
    pub sequence: EventSequence,
    pub object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub commit_timestamp: Timestamp,
    pub message: CommitMessage,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CommitListing {
    pub commits: Vec<Commit>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Catalog;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CatalogListing {
    pub repositories: Vec<Registration>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Events(Events),
    RecentRepositories(RecentRepositories),
    ChangedFiles(ChangedFiles),
    CommitMessages(CommitMessages),
    Catalog(Catalog),
}

impl Query {
    pub fn kind(&self) -> QueryKind {
        match self {
            Self::Events(_) => QueryKind::Events,
            Self::RecentRepositories(_) => QueryKind::RecentRepositories,
            Self::ChangedFiles(_) => QueryKind::ChangedFiles,
            Self::CommitMessages(_) => QueryKind::CommitMessages,
            Self::Catalog(_) => QueryKind::Catalog,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum QueryResult {
    Events(EventListing),
    RecentRepositories(RecentRepositoriesListing),
    ChangedFiles(ChangedFileListing),
    Commits(CommitListing),
    Catalog(CatalogListing),
}

impl QueryResult {
    pub fn kind(&self) -> QueryKind {
        match self {
            Self::Events(_) => QueryKind::Events,
            Self::RecentRepositories(_) => QueryKind::RecentRepositories,
            Self::ChangedFiles(_) => QueryKind::ChangedFiles,
            Self::Commits(_) => QueryKind::CommitMessages,
            Self::Catalog(_) => QueryKind::Catalog,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DaemonConfiguration {
    pub ordinary_socket_path: FilesystemPath,
    pub ordinary_socket_mode: SocketMode,
    pub owner_socket_path: FilesystemPath,
    pub owner_socket_mode: SocketMode,
    pub store_path: FilesystemPath,
    pub spool_directory: FilesystemPath,
}

nota_config::impl_rkyv_configuration!(DaemonConfiguration);

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OperationKind {
    Receive,
    Observe,
    Query,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum QueryKind {
    Events,
    RecentRepositories,
    ChangedFiles,
    CommitMessages,
    Catalog,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    DaemonSocketMissing,
    StoreUnavailable,
    NotInPrototypeScope,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub query: Option<QueryKind>,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel Ledger {
        operation Receive(ReceiveHookNotification),
        operation Observe(PushObservation),
        operation Query(Query),
    }
    reply Reply {
        EventRecorded(EventRecorded),
        QueryResult(QueryResult),
        RequestUnimplemented(RequestUnimplemented),
    }
}

pub type Frame = LedgerFrame;
pub type FrameBody = LedgerFrameBody;
pub type ChannelRequest = LedgerChannelRequest;
pub type ChannelReply = LedgerChannelReply;
pub type RequestBuilder = LedgerRequestBuilder;
pub type Request = LedgerOperation;

impl LedgerOperation {
    pub fn operation_kind(&self) -> OperationKind {
        match self {
            Self::Receive(_) => OperationKind::Receive,
            Self::Observe(_) => OperationKind::Observe,
            Self::Query(_) => OperationKind::Query,
        }
    }
}
