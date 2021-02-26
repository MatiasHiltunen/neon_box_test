use neon::prelude::*;

fn middle(line: &str, t_len: usize) -> String {
    "|".to_owned()
        + line
        + format!("{:width$}", "", width = t_len - line.len()).as_str()
        + &"|".to_owned()
}

fn top(t_len: usize) -> String {
    let mut hl = String::from("");

    for _ in 0..(t_len) {
        hl.push_str("─")
    }

    format!("{}{}{}", "┌", hl, "┐")
}

fn bottom(t_len: usize) -> String {
    let mut hl = String::from("");

    for _ in 0..(t_len) {
        hl.push_str("─")
    }

    format!("{}{}{}", "└", hl, "┘")
}

fn form_wrap(s: String) -> String {
    let mut r = String::from("");
    let mut max_len = 0;

    let lines = s.lines().into_iter();
    let v = lines.collect::<Vec<&str>>();

    for item in v.iter() {
        if item.len() > max_len {
            max_len = item.len();
        }
    }

    r.push_str((top(max_len) + "\n").as_str());

    for line in v.iter() {
        r.push_str((middle(line, max_len) + "\n").as_str());
    }

    r.push_str((bottom(max_len) + "\n").as_str());

    r
}

pub fn export_box(mut cx: FunctionContext) -> JsResult<JsString> {
    let test = match cx.argument_opt(0) {
        Some(arg) => arg.downcast::<JsString>().or_throw(&mut cx)?.value(),
        None => "empty".to_string(),
    };
    Ok(cx.string(form_wrap(test)))
}

register_module!(mut m, { m.export_function("export_box", export_box) });
