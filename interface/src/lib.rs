pub trait Plugin {
    fn name(&self) -> &'static str;
    fn call(&self, arg: String);
}
