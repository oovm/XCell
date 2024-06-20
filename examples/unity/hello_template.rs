use dejavu_macros::Template;
use dejavu_types::values::{Context, Value};

#[derive(Template)]
#[template(path = "templates/hello.djv")]
struct HelloTemplate;

fn main() {
    let mut ctx = Context::new();
    ctx.set_var("name".to_string(), Value::String("World".to_string()));

    let items = vec![
        Value::String("Item 1".to_string()),
        Value::String("Item 2".to_string()),
        Value::String("Item 3".to_string()),
    ];
    ctx.set_var("items".to_string(), Value::Array(items));
    ctx.set_var("show_extra".to_string(), Value::Bool(true));

    let template = HelloTemplate;
    let result = template.render(&ctx).unwrap();

    println!("Rendered template:\n{}", result);
}
