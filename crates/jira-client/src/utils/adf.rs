pub fn adf_collect_text(node: &serde_json::Value, out: &mut String) {
    if let Some(obj) = node.as_object() {
        let ty = obj.get("type").and_then(|v| v.as_str()).unwrap_or("");

        match ty {
            "text" => {
                if let Some(t) = obj.get("text").and_then(|v| v.as_str()) {
                    out.push_str(t);
                }
            }
            "hardBreak" => out.push('\n'),
            "mention" => {
                if let Some(n) = obj
                    .get("attrs")
                    .and_then(|a| a.get("text"))
                    .and_then(|v| v.as_str())
                {
                    out.push_str(n);
                } else if let Some(id) = obj
                    .get("attrs")
                    .and_then(|a| a.get("id"))
                    .and_then(|v| v.as_str())
                {
                    out.push('@');
                    out.push_str(id);
                }
            }
            _ => {}
        }

        if let Some(children) = obj.get("content").and_then(|v| v.as_array()) {
            if ty == "listItem" {
                out.push_str("- ");
            }

            for child in children {
                adf_collect_text(child, out);
            }

            if matches!(ty, "paragraph" | "heading" | "listItem" | "tableRow" | "tableCell") {
                out.push('\n');
            }
        }
    }
}

pub fn normalize_whitespace(mut s: String) -> String {
    s = s.replace('\r', "");

    let mut out = String::with_capacity(s.len());
    let mut prev_newlines = 0;
    for ch in s.chars() {
        if ch == '\n' {
            prev_newlines += 1;

            if prev_newlines <= 2 {
                out.push('\n');
            }
        } else {
            prev_newlines = 0;
            out.push(ch);
        }
    }
    out.trim().to_string()
}
