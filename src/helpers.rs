use crate::{handlers::Handler, injectables::Injectable, service_container::SERVICE_CONTAINER};

pub fn inject_service<F, Args>(handler: F)
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(SERVICE_CONTAINER.get().unwrap());
    handler.call(args)
}
