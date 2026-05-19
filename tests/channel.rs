use signal_core::{RequestPayload, SignalVerb};
use signal_repository_ledger::{
    RepositoryCatalogQuery, RepositoryChangedFileQuery, RepositoryCommitMessageQuery,
    RepositoryEventQuery, RepositoryLedgerDaemonConfiguration, RepositoryLedgerPath,
    RepositoryLedgerRequest, RepositoryLedgerSocketMode, RepositoryName, RepositoryQueryLimit,
    RepositoryRecentRepositoriesQuery, RepositoryTextSearch, RepositoryTimestamp,
};

#[test]
fn request_variants_declare_expected_signal_verbs() {
    let query = RepositoryLedgerRequest::RepositoryEventQuery(RepositoryEventQuery {
        repository_name: Some(RepositoryName::new("repository-ledger")),
        since_sequence: None,
        limit: RepositoryQueryLimit::new(16),
    });
    assert_eq!(query.signal_verb(), SignalVerb::Match);

    let catalog = RepositoryLedgerRequest::RepositoryCatalogQuery(RepositoryCatalogQuery);
    assert_eq!(catalog.signal_verb(), SignalVerb::Match);

    let recent = RepositoryLedgerRequest::RepositoryRecentRepositoriesQuery(
        RepositoryRecentRepositoriesQuery {
            since_received_at: Some(RepositoryTimestamp::new("20260519T000000Z")),
            limit: RepositoryQueryLimit::new(16),
        },
    );
    assert_eq!(recent.signal_verb(), SignalVerb::Match);

    let files = RepositoryLedgerRequest::RepositoryChangedFileQuery(RepositoryChangedFileQuery {
        repository_name: Some(RepositoryName::new("repository-ledger")),
        since_received_at: None,
        until_received_at: None,
        path_contains: Some(RepositoryTextSearch::new("src")),
        limit: RepositoryQueryLimit::new(16),
    });
    assert_eq!(files.signal_verb(), SignalVerb::Match);

    let messages =
        RepositoryLedgerRequest::RepositoryCommitMessageQuery(RepositoryCommitMessageQuery {
            repository_name: None,
            since_received_at: None,
            until_received_at: None,
            message_contains: Some(RepositoryTextSearch::new("query")),
            limit: RepositoryQueryLimit::new(16),
        });
    assert_eq!(messages.signal_verb(), SignalVerb::Match);
}

#[test]
fn daemon_configuration_round_trips_through_nota() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let configuration = RepositoryLedgerDaemonConfiguration {
        ordinary_socket_path: RepositoryLedgerPath::new(
            "/run/repository-ledger/repository-ledger.sock",
        ),
        ordinary_socket_mode: RepositoryLedgerSocketMode::new(0o660),
        owner_socket_path: RepositoryLedgerPath::new(
            "/run/repository-ledger/repository-ledger-owner.sock",
        ),
        owner_socket_mode: RepositoryLedgerSocketMode::new(0o600),
        store_path: RepositoryLedgerPath::new("/var/lib/repository-ledger/repository-ledger.redb"),
        spool_directory: RepositoryLedgerPath::new("/var/lib/repository-ledger/spool"),
    };

    let mut encoder = Encoder::new();
    configuration.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let decoded = RepositoryLedgerDaemonConfiguration::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, configuration);
}
