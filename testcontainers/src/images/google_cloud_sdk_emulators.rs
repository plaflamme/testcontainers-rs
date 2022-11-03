use crate::{core::WaitFor, Image, ImageArgs};

const NAME: &str = "google/cloud-sdk";
const TAG: &str = "362.0.0-emulators";

const HOST: &str = "0.0.0.0";
pub const BIGTABLE_PORT: u16 = 8086;
pub const DATASTORE_PORT: u16 = 8081;
pub const FIRESTORE_PORT: u16 = 8080;
pub const PUBSUB_PORT: u16 = 8085;
pub const SPANNER_GRPC_PORT: u16 = 9010;
pub const SPANNER_REST_PORT: u16 = 9020;

#[derive(Debug, Clone)]
pub struct CloudSdkArgs {
    pub host: String,
    pub port: u16,
    pub rest_port: Option<u16>,
    pub emulator: Emulator,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Emulator {
    Bigtable,
    Datastore { project: String },
    Firestore,
    PubSub,
    Spanner,
}

impl ImageArgs for CloudSdkArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        let (emulator, project) = match &self.emulator {
            Emulator::Bigtable => ("bigtable", None),
            Emulator::Datastore { project } => ("datastore", Some(project)),
            Emulator::Firestore => ("firestore", None),
            Emulator::PubSub => ("pubsub", None),
            Emulator::Spanner => ("spanner", None),
        };
        let mut args = vec![
            "gcloud".to_owned(),
            "beta".to_owned(),
            "emulators".to_owned(),
            emulator.to_owned(),
            "start".to_owned(),
        ];
        if let Some(project) = project {
            args.push("--project".to_owned());
            args.push(project.to_owned());
        }
        args.push("--host-port".to_owned());
        args.push(format!("{}:{}", self.host, self.port));

        if let Some(rest_port) = self.rest_port {
            args.push("--rest-port".to_owned());
            args.push(rest_port.to_string());
        }

        Box::new(args.into_iter())
    }
}

#[derive(Debug)]
pub struct CloudSdk {
    exposed_ports: Vec<u16>,
    ready_condition: WaitFor,
}

impl Image for CloudSdk {
    type Args = CloudSdkArgs;

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![self.ready_condition.clone()]
    }

    fn expose_ports(&self) -> Vec<u16> {
        self.exposed_ports.clone()
    }
}

impl CloudSdk {
    fn new(
        port: u16,
        rest_port: Option<u16>,
        emulator: Emulator,
        ready_condition: WaitFor,
    ) -> (Self, CloudSdkArgs) {
        let arguments = CloudSdkArgs {
            host: HOST.to_owned(),
            port,
            rest_port,
            emulator,
        };
        let mut exposed_ports = vec![port];
        exposed_ports.extend(rest_port);

        (
            Self {
                exposed_ports,
                ready_condition,
            },
            arguments,
        )
    }

    pub fn bigtable() -> (Self, CloudSdkArgs) {
        Self::new(
            BIGTABLE_PORT,
            None,
            Emulator::Bigtable,
            WaitFor::message_on_stderr("[bigtable] Cloud Bigtable emulator running on"),
        )
    }

    pub fn firestore() -> (Self, CloudSdkArgs) {
        Self::new(
            FIRESTORE_PORT,
            None,
            Emulator::Firestore,
            WaitFor::message_on_stderr("[firestore] Dev App Server is now running"),
        )
    }

    pub fn datastore(project: impl Into<String>) -> (Self, CloudSdkArgs) {
        let project = project.into();
        Self::new(
            DATASTORE_PORT,
            None,
            Emulator::Datastore { project },
            WaitFor::message_on_stderr("[datastore] Dev App Server is now running"),
        )
    }

    pub fn pubsub() -> (Self, CloudSdkArgs) {
        Self::new(
            PUBSUB_PORT,
            None,
            Emulator::PubSub,
            WaitFor::message_on_stderr("[pubsub] INFO: Server started, listening on"),
        )
    }

    pub fn spanner() -> (Self, CloudSdkArgs) {
        Self::new(
            SPANNER_GRPC_PORT,
            Some(SPANNER_REST_PORT),
            Emulator::Spanner,
            WaitFor::message_on_stderr("Cloud Spanner emulator running"),
        )
    }
}
