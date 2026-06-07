//! Ordinary Signal contract for repository-ledger.
//!
//! This crate carries peer-callable repository event submissions and read
//! queries. Meta-signal configuration lives in `meta-signal-repository-ledger`.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    NotaEncode,
    NotaDecode,
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
    NotaEncode,
    NotaDecode,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketMode(u32);

impl SocketMode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

impl NotaEncode for SocketMode {
    fn to_nota(&self) -> String {
        u64::from(self.0).to_nota()
    }
}

impl NotaDecode for SocketMode {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let value = u32::try_from(value).map_err(|_| {
            NotaDecodeError::Parse(format!("SocketMode: value {value} does not fit u32"))
        })?;
        Ok(Self(value))
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum Class {
    RuntimeComponent,
    OrdinarySignalContract,
    MetaSignalContract,
    ReportLane,
    Configuration,
    Documentation,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RefUpdate {
    pub old_object_identifier: ObjectIdentifier,
    pub new_object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReceiveHookNotification {
    pub repository_name: Name,
    pub gitolite_user: GitoliteUser,
    pub received_at: Timestamp,
    pub daemon_socket_present: bool,
    pub ref_updates: Vec<RefUpdate>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct FileChange {
    pub status: FileStatus,
    pub path: FilePath,
    pub old_path: Option<FilePath>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CommitObservation {
    pub object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub commit_timestamp: Timestamp,
    pub message: CommitMessage,
    pub changed_files: Vec<FileChange>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PushObservation {
    pub notification: ReceiveHookNotification,
    pub commits: Vec<CommitObservation>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Registration {
    pub repository_name: Name,
    pub repository_class: Class,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Event {
    pub sequence: EventSequence,
    pub notification: ReceiveHookNotification,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EventRecorded {
    pub sequence: EventSequence,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Events {
    pub repository_name: Option<Name>,
    pub since_sequence: Option<EventSequence>,
    pub limit: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EventListing {
    pub events: Vec<Event>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RecentRepositories {
    pub since_received_at: Option<Timestamp>,
    pub limit: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RecentRepository {
    pub repository_name: Name,
    pub latest_received_at: Timestamp,
    pub latest_sequence: EventSequence,
    pub push_count: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RecentRepositoriesListing {
    pub repositories: Vec<RecentRepository>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ChangedFiles {
    pub repository_name: Option<Name>,
    pub since_received_at: Option<Timestamp>,
    pub until_received_at: Option<Timestamp>,
    pub path_contains: Option<TextSearch>,
    pub limit: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ChangedFileListing {
    pub files: Vec<ChangedFile>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CommitMessages {
    pub repository_name: Option<Name>,
    pub since_received_at: Option<Timestamp>,
    pub until_received_at: Option<Timestamp>,
    pub message_contains: Option<TextSearch>,
    pub limit: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Commit {
    pub repository_name: Name,
    pub received_at: Timestamp,
    pub sequence: EventSequence,
    pub object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub commit_timestamp: Timestamp,
    pub message: CommitMessage,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CommitListing {
    pub commits: Vec<Commit>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Catalog;

impl NotaEncode for Catalog {
    fn to_nota(&self) -> String {
        "()".to_owned()
    }
}

impl NotaDecode for Catalog {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "Catalog", 0)?;
        Ok(Self)
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CatalogListing {
    pub repositories: Vec<Registration>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DaemonConfiguration {
    pub ordinary_socket_path: FilesystemPath,
    pub ordinary_socket_mode: SocketMode,
    pub meta_socket_path: FilesystemPath,
    pub meta_socket_mode: SocketMode,
    pub store_path: FilesystemPath,
    pub spool_directory: FilesystemPath,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum QueryKind {
    Events,
    RecentRepositories,
    ChangedFiles,
    CommitMessages,
    Catalog,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    DaemonSocketMissing,
    StoreUnavailable,
    NotInPrototypeScope,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
