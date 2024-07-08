use builder_macro::Builder;

#[derive(Debug, Builder)]
struct Circle {
    x: f64,
    y: f64,
    #[builder(default = 1.0)]
    radius: f64,
}

#[test]
fn test_default() {
    let circle = Circle::builder().with_x(0.0).with_y(0.0).build().unwrap();

    assert_eq!(circle.x, 0.0);
    assert_eq!(circle.y, 0.0);
    assert_eq!(circle.radius, 1.0);
}

#[test]
fn test_set_default() {
    let circle = Circle::builder()
        .with_x(0.0)
        .with_y(0.0)
        .with_radius(2.0)
        .build()
        .unwrap();

    assert_eq!(circle.x, 0.0);
    assert_eq!(circle.y, 0.0);
    assert_eq!(circle.radius, 2.0);
}

#[test]
fn test_missing() {
    let error = Circle::builder().with_x(1.0).build().unwrap_err();

    assert_eq!(error, "Field 'y' is required.");
}

#[test]
#[should_panic]
fn test_panic() {
    let _ = Circle::builder().build().unwrap();
}