use signal_frame::RequestPayload;
use signal_repository_ledger::{
    Catalog, ChangedFiles, CommitListing, CommitMessages, DaemonConfiguration, Events,
    FilesystemPath, Name, Operation, OperationKind, Query, QueryKind, QueryLimit, QueryResult,
    RecentRepositories, RecentRepositoriesListing, Reply, ReplyKind, SocketMode, TextSearch,
    Timestamp,
};

#[test]
fn operations_are_contract_local_without_signal_verbs() {
    let query = Operation::Query(Query::Events(Events {
        repository_name: Some(Name::new("repository-ledger")),
        since_sequence: None,
        limit: QueryLimit::new(16),
    }));
    assert_eq!(query.operation_kind(), OperationKind::Query);
    assert_eq!(query.kind(), OperationKind::Query);

    let catalog = Query::Catalog(Catalog);
    assert_eq!(catalog.kind(), QueryKind::Catalog);

    let recent = Query::RecentRepositories(RecentRepositories {
        since_received_at: Some(Timestamp::new("20260519T000000Z")),
        limit: QueryLimit::new(16),
    });
    assert_eq!(recent.kind(), QueryKind::RecentRepositories);

    let files = Query::ChangedFiles(ChangedFiles {
        repository_name: Some(Name::new("repository-ledger")),
        since_received_at: None,
        until_received_at: None,
        path_contains: Some(TextSearch::new("src")),
        limit: QueryLimit::new(16),
    });
    assert_eq!(files.kind(), QueryKind::ChangedFiles);

    let messages = Query::CommitMessages(CommitMessages {
        repository_name: None,
        since_received_at: None,
        until_received_at: None,
        message_contains: Some(TextSearch::new("query")),
        limit: QueryLimit::new(16),
    });
    assert_eq!(messages.kind(), QueryKind::CommitMessages);

    let result = QueryResult::Commits(CommitListing { commits: vec![] });
    assert_eq!(result.kind(), QueryKind::CommitMessages);
}

#[test]
fn query_operation_round_trips_through_nota() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let operation = Operation::Query(Query::RecentRepositories(RecentRepositories {
        since_received_at: Some(Timestamp::new("20260519T000000Z")),
        limit: QueryLimit::new(16),
    }));

    let mut encoder = Encoder::new();
    operation.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();

    assert_eq!(
        text,
        "(Query (RecentRepositories ((Some \"20260519T000000Z\") 16)))"
    );

    let mut decoder = Decoder::new(&text);
    let decoded = Operation::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, operation);
}

#[test]
fn query_result_reply_round_trips_through_nota() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};

    let reply = Reply::QueryResult(QueryResult::RecentRepositories(RecentRepositoriesListing {
        repositories: vec![],
    }));

    assert_eq!(reply.kind(), ReplyKind::QueryResult);

    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();

    assert_eq!(text, "(QueryResult (RecentRepositories ([])))");

    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn query_operation_builds_single_signal_frame_request() {
    let operation = Operation::Query(Query::Catalog(Catalog));
    let request = operation.into_request();

    assert_eq!(request.payloads().len(), 1);
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
