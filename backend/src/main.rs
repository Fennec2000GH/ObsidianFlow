


#macros
extern crate handlebars;
extern crate serde_json;

use std::error::Error;
use handlebars::Handlebars;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();
    // render without register
    println!(
        "{}",
        reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
    );

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);
    Ok(())
}
