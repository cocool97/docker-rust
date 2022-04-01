use crate::DockerInstance;

/// Builder type to create [DockerInstance].
pub struct DockerInstanceBuilder {
    pub(crate) socket_addr: String,
}

impl DockerInstanceBuilder {
    /// Instantiates a new builder for [DockerInstance]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a new address for the docker socket.
    pub fn socket_addr(mut self, socket_addr: String) -> Self {
        self.socket_addr = socket_addr;
        self
    }

    /// Builds a [DockerInstance], returning an Err if something bad happened.
    pub fn build(self) -> DockerInstance {
        DockerInstance::from(self)
    }
}

impl Default for DockerInstanceBuilder {
    fn default() -> Self {
        Self {
            socket_addr: String::from("/var/run/docker.sock"),
        }
    }
}
