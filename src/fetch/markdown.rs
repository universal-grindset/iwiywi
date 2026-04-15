use crate::models::ClassifiedReading;

pub fn render(readings: &[ClassifiedReading]) -> String {
    let date = chrono::Local::now().format("%A, %B %-d, %Y").to_string();
    let mut out = String::new();
    out.push_str(&format!("# iwiywi\n\n_{date}_\n\n"));
    for r in readings {
        out.push_str(&format!(
            "### Step {step} · {source}\n\n{text}\n\n",
            step = r.step,
            source = r.source.trim(),
            text = r.text.trim(),
        ));
    }
    out.push_str("---\n\n_It works if you work it._\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(step: u8) -> ClassifiedReading {
        ClassifiedReading {
            step,
            reason: "test".to_string(),
            source: "AA.org".to_string(),
            title: "Daily".to_string(),
            text: "Made a decision".to_string(),
            url: "https://www.aa.org".to_string(),
        }
    }

    #[test]
    fn renders_title_and_date() {
        let md = render(&[fixture(1)]);
        assert!(md.starts_with("# iwiywi\n"));
        assert!(md.contains(&chrono::Local::now().format("%B").to_string()));
    }

    #[test]
    fn renders_each_reading_as_h3() {
        let md = render(&[fixture(3), fixture(7)]);
        assert!(md.contains("### Step 3 · AA.org"));
        assert!(md.contains("### Step 7 · AA.org"));
    }

    #[test]
    fn includes_reading_text() {
        let md = render(&[fixture(5)]);
        assert!(md.contains("Made a decision"));
    }

    #[test]
    fn preserves_order() {
        let md = render(&[fixture(1), fixture(12), fixture(6)]);
        let p1 = md.find("Step 1 ").unwrap();
        let p12 = md.find("Step 12 ").unwrap();
        let p6 = md.find("Step 6 ").unwrap();
        assert!(p1 < p12);
        assert!(p12 < p6);
    }

    #[test]
    fn empty_readings_still_renders_heading() {
        let md = render(&[]);
        assert!(md.contains("# iwiywi"));
    }
}
