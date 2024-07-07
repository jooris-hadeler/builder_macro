use builder_macro::Builder;

#[derive(Builder)]
pub struct User {
    pub name: String,
    pub age: Option<u16>,
}

#[derive(Builder)]
pub struct Complex {
    pub object: Option<Option<u32>>,
    #[builder_use_default]
    pub test: Option<bool>,
}

#[test]
fn test_optional() {
    let u = User::builder()
        .with_name("Alice".to_string())
        .with_age(30u16)
        .build()
        .unwrap();

    assert_eq!(u.name, "Alice");
    assert_eq!(u.age, Some(30));

    let v = User::builder()
        .with_name("Frank".to_string())
        .with_age(15)
        .build()
        .unwrap();

    assert_eq!(v.name, "Frank");
    assert_eq!(v.age, Some(15));

    let c = Complex::builder().with_object(Some(42)).build().unwrap();

    assert_eq!(c.object, Some(Some(42)));

    let d = Complex::builder().with_object(None::<u32>).build().unwrap();

    assert_eq!(d.object, Some(None));

    let e = Complex::builder()
        .with_object(None::<Option<u32>>)
        .build()
        .unwrap();

    assert_eq!(e.object, None);
}
