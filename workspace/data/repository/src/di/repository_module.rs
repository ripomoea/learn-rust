use crate::internal::sample_repository_impl::SampleRepositoryImpl;
use domain::repository::sample::SampleRepository;

pub struct RepositoryModule {
    sample_repository: Box<dyn SampleRepository>,
}

impl RepositoryModule {
    pub fn new() -> Self {
        return Self {
            sample_repository: Box::new(SampleRepositoryImpl::new()),
        };
    }

    pub fn provide_sample_repository(self) -> Box<dyn SampleRepository> {
        self.sample_repository
    }
}
