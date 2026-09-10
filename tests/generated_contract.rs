use meta_signal_orchestrate::{
    ByteViewable, ConfigurationRejection, ConfigurationRejectionReason, Query, Response,
    Restorable, Signal, Signalizable,
};
use signal_orchestrate::{ConfigurationReceipt, OrchestrateNexusConfiguration};

fn configure() -> OrchestrateNexusConfiguration {
    OrchestrateNexusConfiguration {
        ordinary_socket_path: "/tmp/orchestrate.sock".into(),
        meta_socket_path: "/tmp/meta-orchestrate.sock".into(),
    }
}

#[test]
fn query_and_response_round_trip_through_fresh_portable_signals() {
    let query = Query::Configure(configure());
    let query_signal = query.signalize().expect("signalize query");
    let received = Signal::<Query>::from(query_signal.bytes().to_vec());
    assert_eq!(received.restore().expect("restore query"), query);

    let response = Response::ConfigurationRejected(ConfigurationRejection {
        configuration_rejection_reason: ConfigurationRejectionReason::InvalidConfiguration,
    });
    let response_signal = response.signalize().expect("signalize response");
    let received = Signal::<Response>::from(response_signal.bytes().to_vec());
    assert_eq!(received.restore().expect("restore response"), response);
}

#[cfg(feature = "datom")]
#[test]
fn query_and_response_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Query::Configure(configure());
    let query_text = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(query_text);
    let query_decoded = pending
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("restore query datom");
    assert_eq!(query_decoded, query);

    let response = Response::Configured(ConfigurationReceipt {
        orchestrate_nexus_configuration: configure(),
        meta_configure_done: true,
    });
    let response_text = response.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Response>::from(response_text);
    let response_decoded = pending
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("restore response datom");
    assert_eq!(response_decoded, response);
}
