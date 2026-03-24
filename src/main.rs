use rubberduck::{Session, EntryKind};
use std::io::{self, BufRead, Write};

fn main() {
    let mut args = std::env::args().skip(1);
    let format = args.next().unwrap_or_default();

    eprintln!("What are you working on?");
    eprint!("> ");
    io::stderr().flush().unwrap();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let topic = lines.next().unwrap_or(Ok("untitled".into())).unwrap();
    let mut session = Session::new(&topic);

    eprintln!("\nTell the duck. (format: kind: message)");
    eprintln!("Kinds: observation, hypothesis, decision, concern, resolution");
    eprintln!("Type 'done' to finish.\n");

    for line in lines {
        let line = line.unwrap();
        let trimmed = line.trim();

        if trimmed.eq_ignore_ascii_case("done") {
            break;
        }

        let (kind, message) = match trimmed.split_once(':') {
            Some((k, m)) => {
                let kind = match k.trim().to_lowercase().as_str() {
                    "observation" | "o" => Some(EntryKind::Observation),
                    "hypothesis" | "h" => Some(EntryKind::Hypothesis),
                    "decision" | "d" => Some(EntryKind::Decision),
                    "concern" | "c" => Some(EntryKind::Concern),
                    "resolution" | "r" => Some(EntryKind::Resolution),
                    _ => None,
                };
                match kind {
                    Some(k) => (k, m.trim()),
                    None => (EntryKind::Observation, trimmed),
                }
            }
            None => (EntryKind::Observation, trimmed),
        };

        let quack = session.tell(kind, message);
        eprintln!("  {quack}");
    }

    match format.as_str() {
        "--json" => println!("{}", session.to_json().unwrap()),
        "--text" => print!("{}", session.to_plaintext()),
        _ => print!("{}", session.to_markdown()),
    }
}
