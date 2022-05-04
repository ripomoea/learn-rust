use repository::di::repository_module::RepositoryModule;
use usecase::di::use_case_module::UseCaseModule;

pub(crate) struct AppModule {
    use_case_module: UseCaseModule,
}

impl AppModule {
    pub(crate) fn new() -> Self {
        Self {
            use_case_module: UseCaseModule::new(RepositoryModule::new()),
        }
    }

    pub(crate) fn use_case_module(self) -> UseCaseModule {
        self.use_case_module
    }
}
