use regex::Regex;
use tera::{Context, Tera};

fn main() {
    let re = Regex::new(r"\((?P<prop_name>[^,]+), (?P<func_name>[^,]+), (?P<value_type>[^,]+), (?P<style_type>[^)]+), [a-z]+\)").unwrap();

    let input = include_str!("../../lvgl-sys/vendor/lvgl/src/lv_core/lv_obj_style_dec.h");

    let mut tera = Tera::default();
    tera.add_raw_template("styles.rs", include_str!("../templates/style.rs.j2"))
        .unwrap();

    for line in input.lines() {
        if !line.starts_with("_LV_OBJ_STYLE_SET_GET_DECLARE") {
            continue;
        }

        if let Some(cap) = re.captures(line) {
            let style_type = cap.get(4).unwrap().as_str().to_string();
            if style_type.eq("_ptr") {
                // Just a few, we will take care of this manually.
                continue;
            }

            let value_type = cap.get(3).unwrap().as_str().to_string();

            let mut ctx = Context::new();
            ctx.insert("prop_name", cap.get(1).unwrap().as_str());
            ctx.insert("func_name", cap.get(2).unwrap().as_str());
            ctx.insert("value_type", value_type.as_str());
            ctx.insert("style_type", style_type.as_str());
            println!("{}", tera.render("styles.rs", &ctx).unwrap());
        }
    }
}
