use crate::internal::hello_world_use_case_impl::HelloWorldUseCaseImpl;
use domain::usecase::hello_world_use_case::HelloWorldUseCase;
use repository::di::repository_module::RepositoryModule;

pub struct UseCaseModule {
    hello_world_use_case: Box<dyn HelloWorldUseCase>,
}

impl UseCaseModule {
    pub fn new(repository_module: RepositoryModule) -> Self {
        Self {
            hello_world_use_case: Box::new(HelloWorldUseCaseImpl::new(
                repository_module.provide_sample_repository(),
            )),
        }
    }

    pub fn provide_hello_world_use_case(self) -> Box<dyn HelloWorldUseCase> {
        self.hello_world_use_case
    }
}
