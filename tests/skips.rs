use builder_macro::Builder;

#[derive(Builder)]
pub struct User {
    pub name: String,
    #[builder_default = "human"]
    pub species: String,
    pub age: i32,
}

#[test]
fn test_skip() {
    let u = User::builder()
        .with_name("Alice".to_string())
        .with_age(12)
        .build()
        .unwrap();

    assert_eq!(u.name, "Alice");
    assert_eq!(u.age, 12);
    assert_eq!(u.species, "human");

    let v = User::builder()
        .with_name("Bob".to_string())
        .with_age(20)
        .with_species("alien".to_string())
        .build()
        .unwrap();

    assert_eq!(v.name, "Bob");
    assert_eq!(v.age, 20);
    assert_eq!(v.species, "alien");
}

#[test]
#[should_panic]
fn test_skip_panic() {
    let _ = User::builder()
        .with_name("Alice".to_string())
        .with_species("alien".to_string())
        .build()
        .unwrap();
}
