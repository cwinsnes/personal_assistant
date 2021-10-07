use interface::Plugin;

fn main() {
    let lib = unsafe { libloading::Library::new("target/debug/libecho.so").expect("load lib") };

    let new_service: libloading::Symbol<fn() -> Box<dyn Plugin>> =
        unsafe { lib.get(b"new_service") }.expect("load symbol");

    let service = new_service();
    println!("{}", service.name());
    service.call(String::from("I am testing"));
}
