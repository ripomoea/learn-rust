use crate::di::app_module::AppModule;

mod di;

fn main() {
    let app_module = AppModule::new();
    let hello_world_use_case = app_module.use_case_module().provide_hello_world_use_case();
    hello_world_use_case.handle();
}
