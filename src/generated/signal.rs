#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type Name = String;
#[rustfmt::skip]
pub type GitoliteUser = String;
#[rustfmt::skip]
pub type ObjectIdentifier = String;
#[rustfmt::skip]
pub type RefName = String;
#[rustfmt::skip]
pub type Timestamp = String;
#[rustfmt::skip]
pub type CommitMessage = String;
#[rustfmt::skip]
pub type FilePath = String;
#[rustfmt::skip]
pub type FileStatus = String;
#[rustfmt::skip]
pub type TextSearch = String;
#[rustfmt::skip]
pub type FilesystemPath = String;
#[rustfmt::skip]
pub type EventSequence = i64;
#[rustfmt::skip]
pub type QueryLimit = i64;
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OperationKind {
    Receive,
    Observe,
    Query,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum QueryKind {
    Events,
    RecentRepositories,
    ChangedFiles,
    CommitMessages,
    Catalog,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UnimplementedReason {
    DaemonSocketMissing,
    StoreUnavailable,
    NotInPrototypeScope,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Class {
    RuntimeComponent,
    OrdinarySignalContract,
    MetaSignalContract,
    ReportLane,
    Configuration,
    Documentation,
}
#[rustfmt::skip]
pub type OldObjectIdentifier = ObjectIdentifier;
#[rustfmt::skip]
pub type NewObjectIdentifier = ObjectIdentifier;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefUpdate {
    pub old_object_identifier: OldObjectIdentifier,
    pub new_object_identifier: NewObjectIdentifier,
    pub ref_name: RefName,
}
#[rustfmt::skip]
pub type RefUpdates = std::vec::Vec<RefUpdate>;
#[rustfmt::skip]
pub type RepositoryName = Name;
#[rustfmt::skip]
pub type ReceivedAt = Timestamp;
#[rustfmt::skip]
pub type DaemonSocketPresent = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReceiveHookNotification {
    pub repository_name: RepositoryName,
    pub gitolite_user: GitoliteUser,
    pub received_at: ReceivedAt,
    pub daemon_socket_present: DaemonSocketPresent,
    pub ref_updates: RefUpdates,
}
#[rustfmt::skip]
pub type OldFilePath = std::option::Option<FilePath>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FileChange {
    pub file_status: FileStatus,
    pub file_path: FilePath,
    pub old_file_path: OldFilePath,
}
#[rustfmt::skip]
pub type FileChanges = std::vec::Vec<FileChange>;
#[rustfmt::skip]
pub type CommitTimestamp = Timestamp;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CommitObservation {
    pub object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub commit_timestamp: CommitTimestamp,
    pub commit_message: CommitMessage,
    pub file_changes: FileChanges,
}
#[rustfmt::skip]
pub type CommitObservations = std::vec::Vec<CommitObservation>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PushObservation {
    pub receive_hook_notification: ReceiveHookNotification,
    pub commit_observations: CommitObservations,
}
#[rustfmt::skip]
pub type RepositoryClass = Class;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Registration {
    pub repository_name: RepositoryName,
    pub repository_class: RepositoryClass,
}
#[rustfmt::skip]
pub type Registrations = std::vec::Vec<Registration>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Event {
    pub event_sequence: EventSequence,
    pub receive_hook_notification: ReceiveHookNotification,
}
#[rustfmt::skip]
pub type EventRecords = std::vec::Vec<Event>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EventRecorded {
    pub event_sequence: EventSequence,
}
#[rustfmt::skip]
pub type SelectedRepositoryName = std::option::Option<Name>;
#[rustfmt::skip]
pub type SinceSequence = std::option::Option<EventSequence>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EventsQuery {
    pub selected_repository_name: SelectedRepositoryName,
    pub since_sequence: SinceSequence,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EventListing {
    pub event_records: EventRecords,
}
#[rustfmt::skip]
pub type SinceReceivedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
pub type UntilReceivedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecentRepositoriesQuery {
    pub since_received_at: SinceReceivedAt,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
pub type LatestReceivedAt = Timestamp;
#[rustfmt::skip]
pub type LatestSequence = EventSequence;
#[rustfmt::skip]
pub type PushCount = QueryLimit;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecentRepository {
    pub repository_name: RepositoryName,
    pub latest_received_at: LatestReceivedAt,
    pub latest_sequence: LatestSequence,
    pub push_count: PushCount,
}
#[rustfmt::skip]
pub type RecentRepositoryRecords = std::vec::Vec<RecentRepository>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecentRepositoriesListing {
    pub recent_repository_records: RecentRepositoryRecords,
}
#[rustfmt::skip]
pub type PathContains = std::option::Option<TextSearch>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChangedFilesQuery {
    pub selected_repository_name: SelectedRepositoryName,
    pub since_received_at: SinceReceivedAt,
    pub until_received_at: UntilReceivedAt,
    pub path_contains: PathContains,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
pub type CommitObjectIdentifier = ObjectIdentifier;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChangedFile {
    pub repository_name: RepositoryName,
    pub received_at: ReceivedAt,
    pub event_sequence: EventSequence,
    pub commit_object_identifier: CommitObjectIdentifier,
    pub ref_name: RefName,
    pub file_status: FileStatus,
    pub file_path: FilePath,
    pub old_file_path: OldFilePath,
}
#[rustfmt::skip]
pub type ChangedFileRecords = std::vec::Vec<ChangedFile>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChangedFileListing {
    pub changed_file_records: ChangedFileRecords,
}
#[rustfmt::skip]
pub type MessageContains = std::option::Option<TextSearch>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CommitMessagesQuery {
    pub selected_repository_name: SelectedRepositoryName,
    pub since_received_at: SinceReceivedAt,
    pub until_received_at: UntilReceivedAt,
    pub message_contains: MessageContains,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Commit {
    pub repository_name: RepositoryName,
    pub received_at: ReceivedAt,
    pub event_sequence: EventSequence,
    pub object_identifier: ObjectIdentifier,
    pub ref_name: RefName,
    pub commit_timestamp: CommitTimestamp,
    pub commit_message: CommitMessage,
}
#[rustfmt::skip]
pub type CommitRecords = std::vec::Vec<Commit>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CommitListing {
    pub commit_records: CommitRecords,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CatalogListing {
    pub registrations: Registrations,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LedgerQuery {
    Events(EventsQuery),
    RecentRepositories(RecentRepositoriesQuery),
    ChangedFiles(ChangedFilesQuery),
    CommitMessages(CommitMessagesQuery),
    Catalog,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum QueryResult {
    Events(EventListing),
    RecentRepositories(RecentRepositoriesListing),
    ChangedFiles(ChangedFileListing),
    Commits(CommitListing),
    Catalog(CatalogListing),
}
#[rustfmt::skip]
pub type SelectedQueryKind = std::option::Option<QueryKind>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RequestUnimplemented {
    pub operation_kind: OperationKind,
    pub selected_query_kind: SelectedQueryKind,
    pub unimplemented_reason: UnimplementedReason,
}
#[rustfmt::skip]
pub type OrdinarySocketPath = FilesystemPath;
#[rustfmt::skip]
pub type OrdinarySocketMode = SocketMode;
#[rustfmt::skip]
pub type MetaSocketPath = FilesystemPath;
#[rustfmt::skip]
pub type MetaSocketMode = SocketMode;
#[rustfmt::skip]
pub type StorePath = FilesystemPath;
#[rustfmt::skip]
pub type SpoolDirectory = FilesystemPath;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DaemonConfiguration {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub ordinary_socket_mode: OrdinarySocketMode,
    pub meta_socket_path: MetaSocketPath,
    pub meta_socket_mode: MetaSocketMode,
    pub store_path: StorePath,
    pub spool_directory: SpoolDirectory,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Receive(ReceiveHookNotification),
    Observe(PushObservation),
    Query(LedgerQuery),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    EventRecorded(EventRecorded),
    QueryResult(QueryResult),
    RequestUnimplemented(RequestUnimplemented),
}
