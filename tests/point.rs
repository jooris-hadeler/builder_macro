use builder_macro::Builder;

#[derive(Builder, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_simple() {
    let p = Point::builder().with_x(1).with_y(2).build().unwrap();

    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
}

#[test]
fn test_missing() {
    let p = Point::builder().with_x(1).build();

    assert!(p.is_err());

    let err = p.as_ref().unwrap_err();
    assert_eq!(err.to_string(), "y is not set.");
}
