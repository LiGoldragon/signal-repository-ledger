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
pub struct RepositoryCatalogQuery;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryCatalogListing {
    pub repositories: Vec<RepositoryRegistration>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RepositoryLedgerOperationKind {
    RepositoryReceiveHookNotification,
    RepositoryEventQuery,
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
            Match RepositoryEventQuery(RepositoryEventQuery),
            Match RepositoryCatalogQuery(RepositoryCatalogQuery),
        }
        reply RepositoryLedgerReply {
            RepositoryEventRecorded(RepositoryEventRecorded),
            RepositoryEventListing(RepositoryEventListing),
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
            Self::RepositoryEventQuery(_) => RepositoryLedgerOperationKind::RepositoryEventQuery,
            Self::RepositoryCatalogQuery(_) => {
                RepositoryLedgerOperationKind::RepositoryCatalogQuery
            }
        }
    }
}
