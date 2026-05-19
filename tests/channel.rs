use signal_core::{RequestPayload, SignalVerb};
use signal_repository_ledger::{
    RepositoryCatalogQuery, RepositoryEventQuery, RepositoryLedgerRequest, RepositoryName,
    RepositoryQueryLimit,
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
}
