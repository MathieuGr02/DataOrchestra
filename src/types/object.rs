use crate::{common::common_trait::Start, docker::docker_struct::Docker, generate::generate_struct::Generate, process::process_struct::Process, store::store_struct::Store};

pub struct Object<T: Start<()>> {
    init_script: Option<String>,
    run_script: Option<String>,
    term_script: Option<String>,
    docker: Option<Docker>,
    child: T
}

pub type StoreType = Object<Store>;
pub type ProcessType = Object<Process>;
pub type GenerateType = Object<Generate>;

