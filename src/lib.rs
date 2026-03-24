use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// What kind of thing the agent is telling the duck.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryKind {
    Observation,
    Hypothesis,
    Decision,
    Concern,
    Resolution,
}

impl std::fmt::Display for EntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Observation => write!(f, "Observation"),
            Self::Hypothesis => write!(f, "Hypothesis"),
            Self::Decision => write!(f, "Decision"),
            Self::Concern => write!(f, "Concern"),
            Self::Resolution => write!(f, "Resolution"),
        }
    }
}

/// A single thing the agent said to the duck.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub kind: EntryKind,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

/// A rubber duck debugging session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub topic: String,
    pub started: DateTime<Utc>,
    pub entries: Vec<Entry>,
}

impl Session {
    pub fn new(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            started: Utc::now(),
            entries: Vec::new(),
        }
    }

    /// Tell the duck something. It quacks back.
    pub fn tell(&mut self, kind: EntryKind, message: impl Into<String>) -> &str {
        self.entries.push(Entry {
            kind,
            message: message.into(),
            timestamp: Utc::now(),
        });
        "quack"
    }

    /// Convenience methods for each entry kind.
    pub fn observe(&mut self, message: impl Into<String>) -> &str {
        self.tell(EntryKind::Observation, message)
    }

    pub fn hypothesize(&mut self, message: impl Into<String>) -> &str {
        self.tell(EntryKind::Hypothesis, message)
    }

    pub fn decide(&mut self, message: impl Into<String>) -> &str {
        self.tell(EntryKind::Decision, message)
    }

    pub fn concern(&mut self, message: impl Into<String>) -> &str {
        self.tell(EntryKind::Concern, message)
    }

    pub fn resolve(&mut self, message: impl Into<String>) -> &str {
        self.tell(EntryKind::Resolution, message)
    }

    /// Render the session as markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = format!(
            "## Rubber Duck Session: {}\n*Started: {}*\n",
            self.topic,
            self.started.format("%Y-%m-%d %H:%M UTC")
        );

        for entry in &self.entries {
            out.push_str(&format!(
                "\n### {}\n{}\n\n> *quack*\n",
                entry.kind, entry.message
            ));
        }

        out
    }

    /// Render the session as plain text.
    pub fn to_plaintext(&self) -> String {
        let mut out = format!(
            "=== Rubber Duck Session: {} ===\nStarted: {}\n",
            self.topic,
            self.started.format("%Y-%m-%d %H:%M UTC")
        );

        for entry in &self.entries {
            out.push_str(&format!(
                "\n[{}] {}\n  quack\n",
                entry.kind, entry.message
            ));
        }

        out
    }

    /// Serialize the session as JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duck_quacks() {
        let mut session = Session::new("testing the duck");
        assert_eq!(session.observe("the sky is blue"), "quack");
        assert_eq!(session.hypothesize("because of light scattering"), "quack");
        assert_eq!(session.decide("I believe in Rayleigh scattering"), "quack");
        assert_eq!(session.entries.len(), 3);
    }

    #[test]
    fn markdown_output() {
        let mut session = Session::new("fixing a bug");
        session.observe("it's broken");
        session.resolve("it's fixed");

        let md = session.to_markdown();
        assert!(md.contains("## Rubber Duck Session: fixing a bug"));
        assert!(md.contains("### Observation"));
        assert!(md.contains("it's broken"));
        assert!(md.contains("> *quack*"));
        assert!(md.contains("### Resolution"));
        assert!(md.contains("it's fixed"));
    }

    #[test]
    fn plaintext_output() {
        let mut session = Session::new("debugging");
        session.concern("this looks weird");

        let txt = session.to_plaintext();
        assert!(txt.contains("[Concern] this looks weird"));
        assert!(txt.contains("quack"));
    }

    #[test]
    fn json_roundtrip() {
        let mut session = Session::new("roundtrip test");
        session.observe("testing serialization");

        let json = session.to_json().unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.topic, "roundtrip test");
        assert_eq!(deserialized.entries.len(), 1);
    }
}
