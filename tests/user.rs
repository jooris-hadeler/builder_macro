use builder_macro::Builder;

#[derive(Debug, Builder)]
struct User {
    #[builder(skip, default = User::generate_id())]
    id: usize,
    name: String,
    age: u32,
    #[builder(default = false)]
    is_admin: bool,
}

impl User {
    fn generate_id() -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }
}

#[test]
fn test_default() {
    let user = User::builder()
        .with_name("Alice".to_string())
        .with_age(30u32)
        .build()
        .unwrap();

    assert_eq!(user.id, 0);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
    assert_eq!(user.is_admin, false);

    let user = User::builder()
        .with_name("Bob".to_string())
        .with_age(25u32)
        .with_is_admin(true)
        .build()
        .unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Bob");
    assert_eq!(user.age, 25);
    assert_eq!(user.is_admin, true);
}
