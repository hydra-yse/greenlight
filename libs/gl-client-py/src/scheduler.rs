use crate::runtime::exec;
use crate::{pb, pb::scheduler::scheduler_client::SchedulerClient};
use crate::{Credentials, Signer};
use anyhow::Result;
use gl_client::bitcoin::Network;
use prost::Message;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tonic::transport::Channel;

type Client = SchedulerClient<Channel>;

#[pyclass]
#[derive(Clone)]
pub struct Scheduler {
    node_id: Vec<u8>,
    pub inner: gl_client::scheduler::Scheduler,
    creds: Credentials,
}

#[pymethods]
impl Scheduler {
    #[new]
    fn new(node_id: Vec<u8>, network: String, creds: Credentials) -> PyResult<Scheduler> {
        let network: Network = network
            .parse()
            .map_err(|_| PyValueError::new_err("Error parsing the network"))?;

        let id = node_id.clone();
        let uri = gl_client::utils::scheduler_uri();

        let ccreds = creds.clone();
        let res = exec(async move {
            gl_client::scheduler::Scheduler::builder(id, network, ccreds.inner)?
                .with_uri(uri)
                .connect()
                .await
        });

        let inner = match res {
            Ok(v) => v,
            Err(_) => return Err(PyValueError::new_err("could not connect to the scheduler")),
        };

        Ok(Scheduler {
            node_id,
            inner,
            creds,
        })
    }

    fn register(&self, signer: &Signer, invite_code: Option<String>) -> PyResult<Vec<u8>> {
        convert(exec(self.inner.register(&signer.inner, invite_code)))
    }

    fn recover(&self, signer: &Signer) -> PyResult<Vec<u8>> {
        convert(exec(async move { self.inner.recover(&signer.inner).await }))
    }

    fn export_node(&self) -> PyResult<Vec<u8>> {
        convert(exec(async move { self.inner.export_node().await }))
    }

    fn get_node_info(&self) -> PyResult<Vec<u8>> {
        let res: Result<pb::scheduler::NodeInfoResponse> = exec(async move {
            let mut client = self.connect().await.unwrap();

            let info = client
                .get_node_info(pb::scheduler::NodeInfoRequest {
                    node_id: self.node_id.clone(),
                    wait: false,
                })
                .await;

            Ok(info?.into_inner())
        });

        let res = match res {
            Ok(v) => v,
            Err(_) => return Err(PyValueError::new_err("error calling get_node_info")),
        };
        let mut buf = Vec::with_capacity(res.encoded_len());
        res.encode(&mut buf).unwrap();
        Ok(buf)
    }

    fn schedule(&self) -> PyResult<Vec<u8>> {
        convert(exec(async move {
            Ok(self
                .connect()
                .await?
                .schedule(pb::scheduler::ScheduleRequest {
                    node_id: self.node_id.clone(),
                })
                .await?
                .into_inner())
        }))
    }

    fn get_invite_codes(&self) -> PyResult<Vec<u8>> {
        convert(exec(async move { self.inner.get_invite_codes().await }))
    }

    fn add_outgoing_webhook(&self, uri: String) -> PyResult<Vec<u8>> {
        let outgoing_webhook_request = pb::scheduler::AddOutgoingWebhookRequest {
            node_id: self.node_id.clone(),
            uri,
        };

        convert(exec(async move {
            self.inner
                .add_outgoing_webhook(outgoing_webhook_request)
                .await
        }))
    }

    fn list_outgoing_webhooks(&self) -> PyResult<Vec<u8>> {
        let list_outgoing_webhooks_request = pb::scheduler::ListOutgoingWebhooksRequest {
            node_id: self.node_id.clone(),
        };

        convert(exec(async move {
            self.inner
                .list_outgoing_webhooks(list_outgoing_webhooks_request)
                .await
        }))
    }

    fn delete_outgoing_webhooks(&self, webhook_ids: Vec<i64>) -> PyResult<Vec<u8>> {
        let delete_outgoing_webhooks_request = pb::scheduler::DeleteOutgoingWebhooksRequest {
            node_id: self.node_id.clone(),
            ids: webhook_ids,
        };

        convert(exec(async move {
            self.inner
                .delete_webhooks(delete_outgoing_webhooks_request)
                .await
        }))
    }

    fn rotate_outgoing_webhook_secret(&self, webhook_id: i64) -> PyResult<Vec<u8>> {
        let rotate_outgoing_webhook_secret_request =
            pb::scheduler::RotateOutgoingWebhookSecretRequest {
                node_id: self.node_id.clone(),
                webhook_id,
            };

        convert(exec(async move {
            self.inner
                .rotate_outgoing_webhook_secret(rotate_outgoing_webhook_secret_request)
                .await
        }))
    }
}

pub fn convert<T: Message>(r: Result<T>) -> PyResult<Vec<u8>> {
    let res = r.map_err(crate::node::error_calling_remote_method)?;
    let mut buf = Vec::with_capacity(res.encoded_len());
    res.encode(&mut buf).unwrap();
    Ok(buf)
}

impl Scheduler {
    async fn connect_with(&self, uri: String, creds: &Credentials) -> Result<Client> {
        let client_tls = creds.inner.tls_config().client_tls_config();
        let channel = Channel::from_shared(uri)?
            .tls_config(client_tls)?
            .connect()
            .await?;
        Ok(SchedulerClient::new(channel))
    }

    async fn connect(&self) -> Result<Client> {
        let uri = gl_client::utils::scheduler_uri();
        self.connect_with(uri, &self.creds).await
    }
}
