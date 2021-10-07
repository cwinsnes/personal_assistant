use interface::Plugin;

#[no_mangle]
pub fn new_service() -> Box<dyn Plugin> {
    Box::new(EchoPlugin::new())
}

pub struct EchoPlugin {}

impl EchoPlugin {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for EchoPlugin {
    fn name(&self) -> &'static str {
        "Echo"
    }

    fn call(&self, arg: String) {
        println!("{}", arg);
    }
}

impl Drop for EchoPlugin {
    fn drop(&mut self) {
        println!("Get dropped");
    }
}
