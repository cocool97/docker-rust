use crate::models::DockerImage;
use crate::DockerInstance;

impl DockerInstance {
    /// Returns a list of images on the server. Note that it uses a different, smaller representation of an image than inspecting a single image.
    pub async fn list_images(
        &mut self,
        all: bool,
        filters: Option<String>,
        digests: bool,
    ) -> Vec<DockerImage> {
        let res = self
            .query_docker_service("GET", "/images/json")
            .await
            .unwrap();
        println!("{:?}", res);

        res
    }
}
