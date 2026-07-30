use pretend_core::PretendService;

fn main() {
    let service = PretendService::new().expect("service should initialize");
    let apps = service.list();
    println\!("Loaded {} applications", apps.len());
}
