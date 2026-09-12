use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_repository_ledger::{
    ChangedFile, ChangedFileListing, ChangedFilesQuery, Class, CommitObservation, Event,
    EventListing, EventRecorded, EventsQuery, FileChange, LedgerQuery, OperationKind,
    PushObservation, Query, QueryKind, QueryResult, ReceiveHookNotification, RefUpdate,
    Registration, RequestUnimplemented, Response, UnimplementedReason,
};

fn notification() -> ReceiveHookNotification {
    ReceiveHookNotification {
        repository_name: String::from("repository-ledger"),
        gitolite_user: String::from("gitolite-admin"),
        received_at: String::from("20260519T120000Z"),
        daemon_socket_present: false,
        ref_updates: vec![RefUpdate {
            old_object_identifier: String::from("0").repeat(40),
            new_object_identifier: String::from("1").repeat(40),
            ref_name: String::from("refs/heads/main"),
        }],
    }
}

fn observation() -> PushObservation {
    PushObservation {
        receive_hook_notification: notification(),
        commit_observations: vec![CommitObservation {
            object_identifier: String::from("a").repeat(40),
            ref_name: String::from("refs/heads/main"),
            commit_timestamp: String::from("20260519T115900Z"),
            commit_message: String::from("record the ledger"),
            file_changes: vec![FileChange {
                file_status: String::from("R"),
                file_path: String::from("src/lib.rs"),
                old_file_path: Some(String::from("src/old.rs")),
            }],
        }],
    }
}

#[test]
fn a_push_observation_restores_from_fresh_peer_bytes() {
    let query = Query::Observe(observation());
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
}

#[test]
fn a_bounded_changed_files_query_restores_from_fresh_peer_bytes() {
    let query = Query::Query(LedgerQuery::ChangedFiles(ChangedFilesQuery {
        selected_repository_name: Some(String::from("repository-ledger")),
        since_received_at: None,
        until_received_at: Some(String::from("20260519T120000Z")),
        path_contains: Some(String::from("src/")),
        query_limit: 25,
    }));
    let outgoing = query.signalize().expect("archive query");
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
}

#[test]
fn an_event_listing_response_restores_from_fresh_peer_bytes() {
    let response = Response::QueryResult(QueryResult::Events(EventListing {
        event_records: vec![Event {
            event_sequence: 1,
            receive_hook_notification: notification(),
        }],
    }));
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}

/// `QueryKind`, `OperationKind`, `Class` and `UnimplementedReason` are tag
/// enums, and `LedgerQuery::Catalog` is a tag variant. Ethos Zero gives a bare
/// variant the declared type of the same name as its payload, so a contract
/// that spelled `Events` or `Catalog` as a declared type would silently make
/// these carry data. Building each by its bare head and carrying it over the
/// wire is the witness that they stay bare.
#[test]
fn the_kind_tags_and_the_catalog_query_cross_the_wire_as_bare_heads() {
    let query = Query::Query(LedgerQuery::Catalog);
    let incoming = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore"), query);

    let response = Response::RequestUnimplemented(RequestUnimplemented {
        operation_kind: OperationKind::Query,
        selected_query_kind: Some(QueryKind::Catalog),
        unimplemented_reason: UnimplementedReason::DaemonSocketMissing,
    });
    let incoming =
        Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore"), response);
}

#[test]
fn a_registration_carries_its_repository_class() {
    let response = Response::QueryResult(QueryResult::Catalog(
        signal_repository_ledger::CatalogListing {
            registrations: vec![Registration {
                repository_name: String::from("repository-ledger"),
                repository_class: Class::RuntimeComponent,
            }],
        },
    ));
    let incoming =
        Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore"), response);
}

#[test]
fn an_event_receipt_and_a_changed_file_listing_restore() {
    let recorded = Response::EventRecorded(EventRecorded { event_sequence: 42 });
    let incoming =
        Signal::<Response>::from(recorded.signalize().expect("archive").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore"), recorded);

    let listing = Response::QueryResult(QueryResult::ChangedFiles(ChangedFileListing {
        changed_file_records: vec![ChangedFile {
            repository_name: String::from("repository-ledger"),
            received_at: String::from("20260519T120000Z"),
            event_sequence: 7,
            commit_object_identifier: String::from("b").repeat(40),
            ref_name: String::from("refs/heads/main"),
            file_status: String::from("M"),
            file_path: String::from("src/lib.rs"),
            old_file_path: None,
        }],
    }));
    let incoming = Signal::<Response>::from(listing.signalize().expect("archive").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore"), listing);
}

#[test]
fn a_receive_submission_and_an_events_query_restore() {
    for query in [
        Query::Receive(notification()),
        Query::Query(LedgerQuery::Events(EventsQuery {
            selected_repository_name: None,
            since_sequence: Some(3),
            query_limit: 10,
        })),
    ] {
        let incoming = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
        assert_eq!(incoming.restore().expect("restore"), query);
    }
}

#[cfg(feature = "datom")]
mod datom_text {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    use signal_repository_ledger::{Query, Response};

    fn budget() -> Budget {
        Budget {
            remaining: 65536,
            reader: ReaderBudget { remaining: 65536 },
            depth: 0,
            maximum_depth: 256,
        }
    }

    /// Every canonical line must actualize as a request or as a reply, and the
    /// text it actualizes from must be the text the codec writes for the value
    /// it yields. A line that no longer parses, or that round-trips to
    /// different text, is a wire change this file did not record.
    #[test]
    fn every_canonical_line_actualizes_and_re_renders_to_itself() {
        let canonical = include_str!("../examples/canonical.datom");
        let lines: Vec<&str> = canonical
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .collect();
        assert_eq!(lines.len(), 14, "canonical file lost or gained a line");

        let mut as_query = 0;
        for line in lines {
            let rendered = match Potential::<Query>::from(line.to_owned()).actualize(&mut budget()) {
                Ok(query) => {
                    as_query += 1;
                    query.datomize(vec![]).protosize().textualize()
                }
                Err(_) => Potential::<Response>::from(line.to_owned())
                    .actualize(&mut budget())
                    .unwrap_or_else(|error| {
                        panic!("canonical line actualizes as neither request nor reply: {line}\n{error:?}")
                    })
                    .datomize(vec![])
                    .protosize()
                    .textualize(),
            };
            assert_eq!(
                rendered, line,
                "canonical line does not re-render to itself"
            );
        }
        assert_eq!(as_query, 7, "the request half of the canonical file moved");
    }

    #[test]
    fn a_quoted_commit_message_survives_the_text_round_trip() {
        let canonical = include_str!("../examples/canonical.datom");
        assert!(
            canonical.contains('\u{ab}') && canonical.contains('\u{bb}'),
            "a commit message with a space must be carried in guillemets"
        );
    }
}
