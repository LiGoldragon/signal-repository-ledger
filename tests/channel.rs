use signal_core::{RequestPayload, SignalVerb};
use signal_repository_ledger::{
    CatalogQuery, ChangedFileQuery, CommitMessageQuery, DaemonConfiguration, EventQuery,
    FilesystemPath, Name, QueryLimit, RecentRepositoriesQuery, Request, SocketMode, TextSearch,
    Timestamp,
};

#[test]
fn request_variants_declare_expected_signal_verbs() {
    let query = Request::EventQuery(EventQuery {
        repository_name: Some(Name::new("repository-ledger")),
        since_sequence: None,
        limit: QueryLimit::new(16),
    });
    assert_eq!(query.signal_verb(), SignalVerb::Match);

    let catalog = Request::CatalogQuery(CatalogQuery);
    assert_eq!(catalog.signal_verb(), SignalVerb::Match);

    let recent = Request::RecentRepositoriesQuery(RecentRepositoriesQuery {
        since_received_at: Some(Timestamp::new("20260519T000000Z")),
        limit: QueryLimit::new(16),
    });
    assert_eq!(recent.signal_verb(), SignalVerb::Match);

    let files = Request::ChangedFileQuery(ChangedFileQuery {
        repository_name: Some(Name::new("repository-ledger")),
        since_received_at: None,
        until_received_at: None,
        path_contains: Some(TextSearch::new("src")),
        limit: QueryLimit::new(16),
    });
    assert_eq!(files.signal_verb(), SignalVerb::Match);

    let messages = Request::CommitMessageQuery(CommitMessageQuery {
        repository_name: None,
        since_received_at: None,
        until_received_at: None,
        message_contains: Some(TextSearch::new("query")),
        limit: QueryLimit::new(16),
    });
    assert_eq!(messages.signal_verb(), SignalVerb::Match);
}

#[test]
fn daemon_configuration_round_trips_through_nota() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let configuration = DaemonConfiguration {
        ordinary_socket_path: FilesystemPath::new("/run/repository-ledger/repository-ledger.sock"),
        ordinary_socket_mode: SocketMode::new(0o660),
        owner_socket_path: FilesystemPath::new(
            "/run/repository-ledger/repository-ledger-owner.sock",
        ),
        owner_socket_mode: SocketMode::new(0o600),
        store_path: FilesystemPath::new("/var/lib/repository-ledger/repository-ledger.redb"),
        spool_directory: FilesystemPath::new("/var/lib/repository-ledger/spool"),
    };

    let mut encoder = Encoder::new();
    configuration.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let decoded = DaemonConfiguration::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, configuration);
}
