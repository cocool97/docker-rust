mod tests {
    use docker_rust::DockerInstanceBuilder;

    #[test]
    pub fn test_images() {
        let mut instance = DockerInstanceBuilder::new().build().unwrap();
        instance.list_images(false, None, false);
    }
}
