use crate::models::ClassifiedReading;

// Map step number to a hex color matching TUI palette
fn step_color(step: u8) -> &'static str {
    match step {
        1  => "#ff6b6b",
        2  => "#ffd93d",
        3  => "#6bcbff",
        4  => "#c678dd",
        5  => "#56b6c2",
        6  => "#98c379",
        7  => "#e06c75",
        8  => "#e5c07b",
        9  => "#61afef",
        10 => "#be5af7",
        11 => "#4ec9b0",
        12 => "#b5f0a5",
        _  => "#ffffff",
    }
}

pub fn render(readings: &[ClassifiedReading], _vercel_url: &str) -> String {
    let cards: String = readings.iter().map(|r| {
        let color = step_color(r.step);
        let text_escaped = html_escape(&r.text);
        let source_escaped = html_escape(&r.source);
        format!(r#"
        <div class="card">
          <div class="card-header" style="color:{color}">
            <span class="step">Step {step}</span>
            <span class="source">{source}</span>
          </div>
          <p class="text">{text}</p>
        </div>
        "#,
            color = color,
            step = r.step,
            source = source_escaped,
            text = text_escaped,
        )
    }).collect();

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>iwiywi — Daily AA Readings</title>
<style>
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{ background: #0d1117; color: #e6edf3; font-family: -apple-system, sans-serif; padding: 16px; }}
  h1 {{ color: #58a6ff; font-size: 16px; letter-spacing: 2px; text-transform: uppercase; margin-bottom: 20px; padding-bottom: 10px; border-bottom: 1px solid #21262d; }}
  .card {{ margin-bottom: 20px; }}
  .card-header {{ display: flex; justify-content: space-between; font-size: 11px; font-weight: bold; letter-spacing: 1px; text-transform: uppercase; margin-bottom: 8px; }}
  .text {{ font-size: 15px; line-height: 1.7; color: #c9d1d9; padding-left: 12px; border-left: 3px solid currentColor; }}
  .divider {{ border: none; border-top: 1px solid #21262d; margin: 20px 0; }}
</style>
</head>
<body>
<h1>iwiywi — {date}</h1>
{cards}
</body>
</html>"#,
        date = chrono::Local::now().format("%B %-d, %Y"),
        cards = cards,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_reading(step: u8) -> ClassifiedReading {
        ClassifiedReading {
            step,
            reason: "test".to_string(),
            source: "AA.org".to_string(),
            title: "Daily Reflections".to_string(),
            text: "Made a <decision> & more".to_string(),
            url: "https://www.aa.org/daily-reflections".to_string(),
        }
    }

    #[test]
    fn render_produces_valid_html_structure() {
        let readings = vec![fixture_reading(3), fixture_reading(7)];
        let html = render(&readings, "https://iwiywi.vercel.app");
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Step 3"));
        assert!(html.contains("Step 7"));
    }

    #[test]
    fn render_escapes_html_in_text() {
        let readings = vec![fixture_reading(1)];
        let html = render(&readings, "https://iwiywi.vercel.app");
        assert!(html.contains("&lt;decision&gt;"));
        assert!(html.contains("&amp;"));
        assert!(!html.contains("<decision>"));
    }

    #[test]
    fn render_uses_step_color() {
        let readings = vec![fixture_reading(3)];
        let html = render(&readings, "https://iwiywi.vercel.app");
        assert!(html.contains("#6bcbff")); // Step 3 color
    }
}
