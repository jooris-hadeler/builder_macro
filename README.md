# Builder Macro
Builder Macro is a proc macro that automatically derives a Builder for any struct.

## Usage
```rust
use builder_macro::Builder;

#[derive(Debug, Builder)]
pub struct Point<X, Y> 
where 
    X: Debug + Default,
    Y: Debug + Default,
{
    x: X,
    // This will automatically set a default value for `y`.
    #[builder(default = Default::default())]
    y: Y,
    // This will be skipped by the builder.
    #[builder(skip, default = false)]
    opt: bool,
}

fn main() {
    let point = Point::<i32, i32>::builder()
        .with_x(12)
        .build() // Build returns an Err(msg) if a field is missing.
        .unwrap();

    println!("{point:?}");
}
```