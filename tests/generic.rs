use builder_macro::Builder;

#[derive(Builder)]
struct Point<A, B>
where
    B: Default,
{
    x: A,
    #[builder(default = Default::default())]
    y: B,
}

#[test]
fn test_generic() {
    let tuple = Point::<i32, i32>::builder().with_x(1).build().unwrap();

    assert_eq!(tuple.x, 1);
    assert_eq!(tuple.y, 0);
}
