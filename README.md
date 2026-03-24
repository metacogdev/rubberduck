# rubberduck

A rubber duck for your agent. It listens to everything, says "quack", and produces a useful log of the agent's reasoning.

The idea: agents that narrate their thinking produce better artifacts. The duck doesn't judge. It just quacks. But the session log it captures — observations, hypotheses, decisions, concerns, resolutions — is structured enough to drop into a PR description or a debug postmortem.

## As a library

Add it to your `Cargo.toml`:

```toml
[dependencies]
rubberduck = { path = "../rubberduck" }
```

Then talk to the duck:

```rust
use rubberduck::Session;

let mut duck = Session::new("migrate users table to UUIDs");

duck.observe("The users table still uses auto-increment integers");
duck.observe("Three services join on users.id — they all need updating");
duck.hypothesize("We can add a uuid column and backfill without downtime");
duck.concern("The analytics service does raw SQL joins, not ORM — might miss it");
duck.decide("Add uuid column, dual-write for one release, then cut over");
duck.resolve("Deployed. Analytics service updated. Old id column dropped");

// Dump the session for a PR description
println!("{}", duck.to_markdown());
```

Every method returns `"quack"`.

### Output formats

```rust
duck.to_markdown()   // Markdown — good for PRs and docs
duck.to_plaintext()  // Plain text — good for logs
duck.to_json()       // JSON — good for tooling and storage
```

### Entry kinds

| Method | Kind | Use it for |
|---|---|---|
| `observe()` | Observation | What you see — current state, symptoms, facts |
| `hypothesize()` | Hypothesis | What you think might be true |
| `decide()` | Decision | What you chose to do and why |
| `concern()` | Concern | What could go wrong |
| `resolve()` | Resolution | What actually happened |

You can also use `tell(EntryKind, message)` directly if you want to pass the kind dynamically.

### Serialization

`Session`, `Entry`, and `EntryKind` all implement `Serialize` and `Deserialize`. Save a session to disk, load it later, pass it between services — the duck doesn't care.

```rust
// Save
let json = duck.to_json().unwrap();
std::fs::write("duck-session.json", &json).unwrap();

// Load
let json = std::fs::read_to_string("duck-session.json").unwrap();
let restored: Session = serde_json::from_str(&json).unwrap();
```

## As a CLI

```
cargo run
```

The CLI prompts you interactively. Prefix lines with a kind (`o:`, `h:`, `d:`, `c:`, `r:`) or just type and it defaults to an observation. Type `done` to finish.

```
$ cargo run
What are you working on?
> fixing the auth timeout

Tell the duck. (format: kind: message)
Kinds: observation, hypothesis, decision, concern, resolution
Type 'done' to finish.

o: middleware times out after 30s on cold start
  quack
h: new connection pool config reduced max idle from 10 to 2
  quack
d: reverting pool config and adding warmup call
  quack
r: p99 dropped from 31s to 200ms
  quack
done
```

Output goes to stdout in markdown by default. Use `--json` or `--text` for other formats:

```
cargo run -- --json
cargo run -- --text
```

## Integration patterns

### Agent loop

Instrument your agent's decision points. The duck session becomes an audit trail.

```rust
use rubberduck::Session;

fn agent_task(task: &str) -> Session {
    let mut duck = Session::new(task);

    // Gather context
    let files = find_relevant_files(task);
    duck.observe(format!("Found {} relevant files", files.len()));

    // Analyze
    for issue in analyze(&files) {
        duck.observe(format!("Issue: {issue}"));
    }

    // Plan
    duck.hypothesize("Refactoring the handler should fix the race condition");
    duck.concern("This handler is called from 4 places — need to check all of them");
    duck.decide("Wrapping the shared state in a Mutex");

    // Execute
    apply_fix();
    duck.resolve("Fix applied, all 4 call sites verified");

    duck
}

// Attach the log to the PR
let session = agent_task("fix race condition in order handler");
create_pr(title, body: &session.to_markdown());
```

### CI / post-mortem

Pipe the CLI into a file during a debugging session, then attach it to the incident:

```
cargo run -- --json > duck-session.json
```

### Chaining sessions

For multi-step workflows, create one session per phase:

```rust
let mut planning = Session::new("planning: new auth flow");
// ... plan ...

let mut implementation = Session::new("implementing: new auth flow");
// ... build ...

let mut review = Session::new("reviewing: new auth flow");
// ... verify ...

// Combine for the PR
let full_log = format!(
    "{}\n---\n{}\n---\n{}",
    planning.to_markdown(),
    implementation.to_markdown(),
    review.to_markdown(),
);
```
