use domain::repository::sample::SampleRepository;

pub(crate) struct SampleRepositoryImpl {}

impl SampleRepositoryImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl SampleRepository for SampleRepositoryImpl {
    fn hello_world(&self) {
        println!("Hello, world!");
    }
}
