use domain::repository::sample::SampleRepository;
use domain::usecase::hello_world_use_case::HelloWorldUseCase;

pub(crate) struct HelloWorldUseCaseImpl {
    sample_repository: Box<dyn SampleRepository>,
}

impl HelloWorldUseCaseImpl {
    pub(crate) fn new(sample_repository: Box<dyn SampleRepository>) -> Self {
        Self { sample_repository }
    }
}

impl HelloWorldUseCase for HelloWorldUseCaseImpl {
    fn handle(&self) {
        self.sample_repository.hello_world()
    }
}
