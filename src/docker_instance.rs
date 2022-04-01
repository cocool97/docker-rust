use hyper::body::HttpBody;
use hyper::{Body, Client, Request, Version};
use hyperlocal::{UnixConnector};
use serde::de::DeserializeOwned;

use crate::{constants::DOCKER_API_VERSION, DockerInstanceBuilder, Result};

/// Type containg all methods to dialog with Docker daemon.
pub struct DockerInstance {
    socket_addr: String,
    client: Client<UnixConnector>,
}

impl From<DockerInstanceBuilder> for DockerInstance {
    fn from(builder: DockerInstanceBuilder) -> Self {
        Self::new(&builder.socket_addr)
    }
}

impl DockerInstance {
    pub(crate) fn new(socket_addr: &str) -> Self {
        Self {
            socket_addr: socket_addr.to_string(),
            client: Client::builder().build(UnixConnector),
        }
    }

    pub(crate) async fn query_docker_service<T: DeserializeOwned>(
        &mut self,
        method: &str,
        endpoint: &str,
    ) -> Result<T> {
        let req = Request::builder()
            .header("Host", DOCKER_API_VERSION)
            .method(method)
            .version(Version::HTTP_11)
            .body(Body::empty())
            .unwrap();

        let a = self.client.request(req).await.unwrap();
        let b = a.map_data(|v| Vec::new(&*v)).data().await.unwrap().unwrap().to_vec();
        let body = serde_json::from_slice(&*b)?;

        Ok(body)

        // self.client.write_all(request.as_bytes())?;

        // const BUFFER_SIZE: usize = 1024;
        // let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        // let mut raw_resp: Vec<u8> = Vec::new();
        // loop {
        //     let len = self.client.read(&mut buffer)?;

        //     for byte in buffer {
        //         raw_resp.push(byte);
        //     }

        //     if len < BUFFER_SIZE {
        //         break;
        //     }
        // }

        // TODO: use serde::from_reader !

        // println!("{}", String::from_utf8(raw_resp.to_vec()).unwrap());

        // Ok(serde_json::from_slice(&raw_resp)?)
    }
}
