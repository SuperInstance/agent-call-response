# agent-call-response

> Call-and-response protocol for agent learning, inspired by the musical tradition.

In music, call-and-response is a foundational practice: one musician plays a phrase (the *call*) and another answers (the *response*). This isn't just imitation — it's a dialogue. The responder must listen, interpret, and create something that fits. Over repeated rounds, musicians develop fluency, creativity, and style.

**agent-call-response** brings this practice to AI agent training. Instead of static benchmarks, agents learn through iterative call-and-response cycles where:

1. One agent (the *caller*) poses a challenge
2. Another agent (the *responder*) answers
3. The response is evaluated across multiple quality dimensions
4. Improvement is tracked across rounds, categories, and patterns

## Core Concepts

### Call

A `Call` is a challenge posed by an agent. It carries:
- **Category** — the domain (reasoning, creative, factual, social, etc.)
- **Difficulty** — from 0.0 (trivial) to 1.0 (maximally hard)
- **Content** — the actual challenge as a free-form string
- **Metadata** — optional key-value hints for domain-specific context

```rust
use agent_call_response::Call;

let call = Call::new("teacher-agent", "reasoning", "Prove that the square root of 2 is irrational")
    .with_difficulty(0.8)
    .with_metadata("hint", "use contradiction");
```

### Response

A `Response` is an agent's answer to a call. It records who responded, what they said, and how fast.

```rust
use agent_call_response::Response;

let response = Response::new(&call.id, "student-agent", "Assume √2 = p/q...")
    .with_latency(3400); // 3.4 seconds
```

### ResponseQuality

Every response is evaluated on four dimensions:
- **Relevance** (30%) — does it address the call?
- **Accuracy** (35%) — is it correct?
- **Creativity** (15%) — does it show novel thinking?
- **Completeness** (20%) — does it fully address the challenge?

These are combined into a composite score with configurable passing threshold.

```rust
use agent_call_response::ResponseQuality;

let quality = ResponseQuality::new(0.9, 0.95, 0.6, 0.85);
println!("Composite: {:.2}", quality.composite_score()); // 0.855
assert!(quality.is_passing()); // ≥ 0.6
```

### LearningRound

A `LearningRound` captures one complete call→response→evaluate cycle:

```rust
use agent_call_response::LearningRound;

let round = LearningRound::new(1, call, response, quality);
assert!(round.passed());
```

### CallPatternLibrary

Pre-built patterns for structured practice, drawn from musical call-and-response traditions:

| Pattern | Description | Difficulty |
|---------|-------------|-----------|
| **echo** | Repeat the call exactly | 0.2 |
| **invert** | Respond with the opposite perspective | 0.5 |
| **extend** | Add new information to the call | 0.6 |
| **harmonize** | Find agreement/common ground | 0.7 |
| **resolve** | Bring the call to a satisfying conclusion | 0.8 |
| **contrapuntal** | Provide a counterargument | 0.75 |

```rust
use agent_call_response::CallPatternLibrary;

let lib = CallPatternLibrary::with_standard_patterns();
let pattern = lib.get("echo").unwrap();
let call = pattern.create_call("teacher");
```

### ImprovementTracker

Track how an agent improves over time:

```rust
use agent_call_response::ImprovementTracker;

let mut tracker = ImprovementTracker::new("student-agent");
tracker.record(round);

println!("Average score: {:.2}", tracker.average_score());
println!("Pass rate: {:.0}%", tracker.pass_rate() * 100.0);
println!("Improving? {}", tracker.is_improving());
```

The tracker computes:
- **Pass rate** — fraction of rounds meeting the quality threshold
- **Average score** — mean composite across all rounds
- **Improvement trend** — first-half vs second-half average (positive = getting better)
- **Best round** — highest-scoring round
- **Scores by category** — breakdown per challenge domain

## Design Philosophy

This crate is inspired by how musicians actually learn call-and-response:

1. **Start simple** — echo the phrase back exactly
2. **Build complexity** — invert, extend, harmonize
3. **Find your voice** — contrapuntal responses, creative resolution
4. **Track growth** — not just "right/wrong" but multi-dimensional quality

The evaluation dimensions mirror musical judgment: relevance (did you listen?), accuracy (is it in tune?), creativity (did you add something?), completeness (did you finish the phrase?).

## Use Cases

- **Agent training** — structured practice sessions for AI agents
- **Multi-agent evaluation** — benchmark different agents on the same calls
- **Curriculum design** — build learning progressions using pattern libraries
- **Quality assurance** — automated evaluation of agent outputs
- **Research** — studying how agents improve with iterative feedback

## Running Tests

```bash
cargo test
```

All 20 tests cover: call/response creation, quality evaluation, learning rounds, pattern libraries, improvement tracking, and full cycle workflows.

## License

MIT
