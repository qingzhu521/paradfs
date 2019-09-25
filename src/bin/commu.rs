use std::thread;

fn main() {
    let handler = thread::Builder::new()
    .name("named thread".into())
    .spawn(|| {
        let handle = thread::current();
        assert_eq!(handle.name(), Some("named thread"));
    }).unwrap();

    handler.join().unwrap();
}