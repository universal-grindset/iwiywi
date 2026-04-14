use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawReading {
    pub source: String,
    pub title: String,
    pub text: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedReading {
    pub step: u8,
    pub reason: String,
    pub source: String,
    pub title: String,
    pub text: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classified_reading_round_trips_json() {
        let r = ClassifiedReading {
            step: 3,
            reason: "Surrender".to_string(),
            source: "AA.org".to_string(),
            title: "Daily Reflections".to_string(),
            text: "Made a decision...".to_string(),
            url: "https://www.aa.org/daily-reflections".to_string(),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ClassifiedReading = serde_json::from_str(&json).unwrap();
        assert_eq!(back.step, 3);
        assert_eq!(back.source, "AA.org");
    }
}
