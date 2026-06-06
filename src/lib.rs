//! # agent-call-response
//!
//! Call-and-response protocol for agent learning, inspired by the musical tradition
//! where one musician plays a phrase and another answers. This crate models the
//! iterative learning cycle: challenge → response → evaluation → improvement.

use std::collections::HashMap;

/// A challenge posed by one agent to another, analogous to a musical "call."
#[derive(Debug, Clone)]
pub struct Call {
    /// Unique identifier for this call.
    pub id: String,
    /// The agent issuing the call.
    pub caller_id: String,
    /// Category or domain of the challenge (e.g., "reasoning", "creative", "factual").
    pub category: String,
    /// The challenge content — a free-form string describing what's being asked.
    pub content: String,
    /// Difficulty level from 0.0 (trivial) to 1.0 (maximally hard).
    pub difficulty: f64,
    /// Optional metadata for domain-specific hints.
    pub metadata: HashMap<String, String>,
}

impl Call {
    /// Create a new call with the given fields.
    pub fn new(caller_id: impl Into<String>, category: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: format!("call-{}", uuid_counter()),
            caller_id: caller_id.into(),
            category: category.into(),
            content: content.into(),
            difficulty: 0.5,
            metadata: HashMap::new(),
        }
    }

    /// Set difficulty level (clamped to [0.0, 1.0]).
    pub fn with_difficulty(mut self, d: f64) -> Self {
        self.difficulty = d.clamp(0.0, 1.0);
        self
    }

    /// Attach metadata key-value pair.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// A response from an agent to a call.
#[derive(Debug, Clone)]
pub struct Response {
    /// The call this responds to.
    pub call_id: String,
    /// The agent providing the response.
    pub responder_id: String,
    /// The response content.
    pub content: String,
    /// Time taken to respond, in milliseconds.
    pub latency_ms: u64,
}

impl Response {
    /// Create a new response.
    pub fn new(call_id: impl Into<String>, responder_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            call_id: call_id.into(),
            responder_id: responder_id.into(),
            content: content.into(),
            latency_ms: 0,
        }
    }

    /// Set response latency.
    pub fn with_latency(mut self, ms: u64) -> Self {
        self.latency_ms = ms;
        self
    }
}

/// Quality assessment of a response against its call.
#[derive(Debug, Clone, Copy)]
pub struct ResponseQuality {
    /// Relevance: does the response address the call? [0.0, 1.0]
    pub relevance: f64,
    /// Accuracy: is the response correct? [0.0, 1.0]
    pub accuracy: f64,
    /// Creativity: does the response show novel thinking? [0.0, 1.0]
    pub creativity: f64,
    /// Completeness: does it fully address the challenge? [0.0, 1.0]
    pub completeness: f64,
}

impl ResponseQuality {
    /// Create a quality assessment with all four dimensions.
    pub fn new(relevance: f64, accuracy: f64, creativity: f64, completeness: f64) -> Self {
        Self {
            relevance: relevance.clamp(0.0, 1.0),
            accuracy: accuracy.clamp(0.0, 1.0),
            creativity: creativity.clamp(0.0, 1.0),
            completeness: completeness.clamp(0.0, 1.0),
        }
    }

    /// Composite score: weighted average of all dimensions.
    pub fn composite_score(&self) -> f64 {
        // Weighted: accuracy matters most, then relevance, completeness, creativity
        (self.relevance * 0.3 + self.accuracy * 0.35 + self.creativity * 0.15 + self.completeness * 0.2)
    }

    /// Is this a passing response (composite >= 0.6)?
    pub fn is_passing(&self) -> bool {
        self.composite_score() >= 0.6
    }
}

impl Default for ResponseQuality {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5)
    }
}

/// Evaluator function type: given a call and response, produce a quality assessment.
pub type EvaluatorFn = Box<dyn Fn(&Call, &Response) -> ResponseQuality + Send + Sync>;

/// A single call→response→evaluate round.
#[derive(Debug, Clone)]
pub struct LearningRound {
    /// Round number (1-indexed).
    pub round_number: usize,
    /// The call issued.
    pub call: Call,
    /// The response given.
    pub response: Response,
    /// Quality evaluation of the response.
    pub quality: ResponseQuality,
}

impl LearningRound {
    /// Create a completed round.
    pub fn new(round_number: usize, call: Call, response: Response, quality: ResponseQuality) -> Self {
        Self { round_number, call, response, quality }
    }

    /// Did the agent pass this round?
    pub fn passed(&self) -> bool {
        self.quality.is_passing()
    }
}

/// Named pattern for call-and-response practice.
#[derive(Debug, Clone)]
pub struct CallPattern {
    /// Pattern name (e.g., "echo", "invert", "extend").
    pub name: String,
    /// Description of the pattern.
    pub description: String,
    /// Template for generating calls of this pattern.
    pub call_template: String,
    /// Default difficulty for this pattern.
    pub default_difficulty: f64,
    /// Tags for categorization.
    pub tags: Vec<String>,
}

impl CallPattern {
    pub fn new(name: impl Into<String>, description: impl Into<String>, call_template: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            call_template: call_template.into(),
            default_difficulty: 0.5,
            tags: Vec::new(),
        }
    }

    pub fn with_difficulty(mut self, d: f64) -> Self {
        self.default_difficulty = d.clamp(0.0, 1.0);
        self
    }

    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Instantiate a call from this pattern for a given caller.
    pub fn create_call(&self, caller_id: &str) -> Call {
        Call::new(caller_id, &self.name, &self.call_template)
            .with_difficulty(self.default_difficulty)
    }
}

/// Library of call-and-response patterns for structured practice.
#[derive(Debug, Clone)]
pub struct CallPatternLibrary {
    patterns: HashMap<String, CallPattern>,
}

impl CallPatternLibrary {
    pub fn new() -> Self {
        Self { patterns: HashMap::new() }
    }

    /// Add a pattern to the library.
    pub fn add(&mut self, pattern: CallPattern) {
        self.patterns.insert(pattern.name.clone(), pattern);
    }

    /// Get a pattern by name.
    pub fn get(&self, name: &str) -> Option<&CallPattern> {
        self.patterns.get(name)
    }

    /// List all pattern names.
    pub fn pattern_names(&self) -> Vec<&str> {
        self.patterns.keys().map(|s| s.as_str()).collect()
    }

    /// Number of patterns in the library.
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Is the library empty?
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Get patterns matching a tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&CallPattern> {
        self.patterns.values().filter(|p| p.tags.iter().any(|t| t == tag)).collect()
    }

    /// Build a library pre-loaded with common musical call-and-response patterns.
    pub fn with_standard_patterns() -> Self {
        let mut lib = Self::new();
        lib.add(CallPattern::new(
            "echo", "Repeat the call exactly", "Repeat this phrase back exactly"
        ).with_tags(vec!["beginner", "memory"]).with_difficulty(0.2));
        lib.add(CallPattern::new(
            "invert", "Invert the call (opposite perspective)", "Give the opposite of this statement"
        ).with_tags(vec!["intermediate", "reasoning"]).with_difficulty(0.5));
        lib.add(CallPattern::new(
            "extend", "Extend the call with new information", "Add something new to this idea"
        ).with_tags(vec!["intermediate", "creative"]).with_difficulty(0.6));
        lib.add(CallPattern::new(
            "harmonize", "Find agreement / common ground with the call", "Find a perspective that harmonizes with this"
        ).with_tags(vec!["advanced", "social"]).with_difficulty(0.7));
        lib.add(CallPattern::new(
            "resolve", "Bring the call to a satisfying conclusion", "Resolve this tension"
        ).with_tags(vec!["advanced", "creative"]).with_difficulty(0.8));
        lib.add(CallPattern::new(
            "contrapuntal", "Respond with a competing melody", "Provide a counterargument"
        ).with_tags(vec!["advanced", "reasoning"]).with_difficulty(0.75));
        lib
    }
}

impl Default for CallPatternLibrary {
    fn default() -> Self {
        Self::with_standard_patterns()
    }
}

/// Tracks improvement of an agent across multiple learning rounds.
#[derive(Debug, Clone)]
pub struct ImprovementTracker {
    /// Agent being tracked.
    pub agent_id: String,
    /// All completed rounds.
    pub rounds: Vec<LearningRound>,
}

impl ImprovementTracker {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self { agent_id: agent_id.into(), rounds: Vec::new() }
    }

    /// Record a completed round.
    pub fn record(&mut self, round: LearningRound) {
        self.rounds.push(round);
    }

    /// Number of rounds recorded.
    pub fn total_rounds(&self) -> usize {
        self.rounds.len()
    }

    /// Overall pass rate (fraction of rounds that passed).
    pub fn pass_rate(&self) -> f64 {
        if self.rounds.is_empty() { return 0.0; }
        self.rounds.iter().filter(|r| r.passed()).count() as f64 / self.rounds.len() as f64
    }

    /// Average composite score across all rounds.
    pub fn average_score(&self) -> f64 {
        if self.rounds.is_empty() { return 0.0; }
        self.rounds.iter().map(|r| r.quality.composite_score()).sum::<f64>() / self.rounds.len() as f64
    }

    /// Trend: compare first-half avg to second-half avg. Positive = improving.
    pub fn improvement_trend(&self) -> f64 {
        if self.rounds.len() < 2 { return 0.0; }
        let mid = self.rounds.len() / 2;
        let first_half: f64 = self.rounds[..mid].iter().map(|r| r.quality.composite_score()).sum::<f64>() / mid as f64;
        let second_half: f64 = self.rounds[mid..].iter().map(|r| r.quality.composite_score()).sum::<f64>() / (self.rounds.len() - mid) as f64;
        second_half - first_half
    }

    /// Best round (highest composite score).
    pub fn best_round(&self) -> Option<&LearningRound> {
        self.rounds.iter().max_by(|a, b| {
            a.quality.composite_score().partial_cmp(&b.quality.composite_score()).unwrap()
        })
    }

    /// Scores by category.
    pub fn scores_by_category(&self) -> HashMap<String, Vec<f64>> {
        let mut map: HashMap<String, Vec<f64>> = HashMap::new();
        for r in &self.rounds {
            map.entry(r.call.category.clone()).or_default().push(r.quality.composite_score());
        }
        map
    }

    /// Is the agent improving? (trend > 0.05)
    pub fn is_improving(&self) -> bool {
        self.improvement_trend() > 0.05
    }
}

/// Simple atomic counter for unique IDs.
fn uuid_counter() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

// ─── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_creation() {
        let call = Call::new("agent-1", "reasoning", "What is 2+2?");
        assert_eq!(call.caller_id, "agent-1");
        assert_eq!(call.category, "reasoning");
        assert_eq!(call.content, "What is 2+2?");
        assert!(!call.id.is_empty());
        assert!(call.id.starts_with("call-"));
    }

    #[test]
    fn test_call_difficulty_clamping() {
        let call = Call::new("a", "b", "c").with_difficulty(5.0);
        assert!((call.difficulty - 1.0).abs() < f64::EPSILON);
        let call = Call::new("a", "b", "c").with_difficulty(-3.0);
        assert!((call.difficulty - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_call_metadata() {
        let call = Call::new("a", "b", "c").with_metadata("hint", "think carefully");
        assert_eq!(call.metadata.get("hint").unwrap(), "think carefully");
    }

    #[test]
    fn test_response_creation() {
        let resp = Response::new("call-1", "agent-2", "4").with_latency(150);
        assert_eq!(resp.call_id, "call-1");
        assert_eq!(resp.responder_id, "agent-2");
        assert_eq!(resp.content, "4");
        assert_eq!(resp.latency_ms, 150);
    }

    #[test]
    fn test_response_quality_dimensions() {
        let q = ResponseQuality::new(0.9, 0.8, 0.6, 0.7);
        assert!((q.relevance - 0.9).abs() < f64::EPSILON);
        assert!((q.accuracy - 0.8).abs() < f64::EPSILON);
        assert!((q.creativity - 0.6).abs() < f64::EPSILON);
        assert!((q.completeness - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn test_response_quality_clamping() {
        let q = ResponseQuality::new(2.0, -1.0, 0.5, 1.5);
        assert!((q.relevance - 1.0).abs() < f64::EPSILON);
        assert!((q.accuracy - 0.0).abs() < f64::EPSILON);
        assert!((q.completeness - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_composite_score_weights() {
        let q = ResponseQuality::new(1.0, 1.0, 1.0, 1.0);
        assert!((q.composite_score() - 1.0).abs() < 1e-10);
        let q = ResponseQuality::new(0.0, 0.0, 0.0, 0.0);
        assert!((q.composite_score() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_passing() {
        let q = ResponseQuality::new(0.7, 0.7, 0.5, 0.7);
        assert!(q.is_passing());
        let q = ResponseQuality::new(0.3, 0.3, 0.3, 0.3);
        assert!(!q.is_passing());
    }

    #[test]
    fn test_learning_round() {
        let call = Call::new("a", "math", "1+1");
        let resp = Response::new(&call.id, "b", "2");
        let quality = ResponseQuality::new(1.0, 1.0, 0.3, 1.0);
        let round = LearningRound::new(1, call, resp, quality);
        assert_eq!(round.round_number, 1);
        assert!(round.passed());
    }

    #[test]
    fn test_call_pattern_creation() {
        let p = CallPattern::new("echo", "repeat back", "say it again")
            .with_difficulty(0.3)
            .with_tags(vec!["beginner", "memory"]);
        assert_eq!(p.name, "echo");
        assert_eq!(p.tags.len(), 2);
        let call = p.create_call("caller-1");
        assert_eq!(call.caller_id, "caller-1");
        assert!((call.difficulty - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_pattern_library_standard() {
        let lib = CallPatternLibrary::with_standard_patterns();
        assert!(lib.len() >= 6);
        assert!(lib.get("echo").is_some());
        assert!(lib.get("resolve").is_some());
        assert!(lib.get("nonexistent").is_none());
    }

    #[test]
    fn test_pattern_library_by_tag() {
        let lib = CallPatternLibrary::with_standard_patterns();
        let beginner = lib.by_tag("beginner");
        assert!(!beginner.is_empty());
        let advanced = lib.by_tag("advanced");
        assert!(!advanced.is_empty());
    }

    #[test]
    fn test_pattern_library_names() {
        let lib = CallPatternLibrary::with_standard_patterns();
        let names = lib.pattern_names();
        assert!(names.contains(&"echo"));
        assert!(names.contains(&"contrapuntal"));
    }

    #[test]
    fn test_improvement_tracker_basic() {
        let mut tracker = ImprovementTracker::new("agent-x");
        assert_eq!(tracker.total_rounds(), 0);
        assert!((tracker.average_score() - 0.0).abs() < f64::EPSILON);
        assert!((tracker.pass_rate() - 0.0).abs() < f64::EPSILON);

        let call = Call::new("caller", "test", "challenge");
        let resp = Response::new(&call.id, "agent-x", "answer");
        let quality = ResponseQuality::new(0.8, 0.8, 0.5, 0.8);
        tracker.record(LearningRound::new(1, call, resp, quality));
        assert_eq!(tracker.total_rounds(), 1);
    }

    #[test]
    fn test_improvement_tracker_trend() {
        let mut tracker = ImprovementTracker::new("agent-x");
        // First half: low scores
        for i in 1..=5 {
            let call = Call::new("c", "math", "q");
            let resp = Response::new(&call.id, "agent-x", "a");
            let quality = ResponseQuality::new(0.3, 0.3, 0.3, 0.3);
            tracker.record(LearningRound::new(i, call, resp, quality));
        }
        // Second half: high scores
        for i in 6..=10 {
            let call = Call::new("c", "math", "q");
            let resp = Response::new(&call.id, "agent-x", "a");
            let quality = ResponseQuality::new(0.9, 0.9, 0.9, 0.9);
            tracker.record(LearningRound::new(i, call, resp, quality));
        }
        assert!(tracker.is_improving());
        assert!(tracker.improvement_trend() > 0.5);
    }

    #[test]
    fn test_improvement_tracker_best_round() {
        let mut tracker = ImprovementTracker::new("agent-x");
        let scores = [0.3, 0.5, 0.9, 0.7];
        for (i, &s) in scores.iter().enumerate() {
            let call = Call::new("c", "cat", "q");
            let resp = Response::new(&call.id, "agent-x", "a");
            let quality = ResponseQuality::new(s, s, s, s);
            tracker.record(LearningRound::new(i + 1, call, resp, quality));
        }
        let best = tracker.best_round().unwrap();
        assert_eq!(best.round_number, 3);
    }

    #[test]
    fn test_improvement_tracker_scores_by_category() {
        let mut tracker = ImprovementTracker::new("agent-x");
        for (cat, score) in [("math", 0.8), ("math", 0.9), ("creative", 0.6)] {
            let call = Call::new("c", cat, "q");
            let resp = Response::new(&call.id, "agent-x", "a");
            let quality = ResponseQuality::new(score, score, score, score);
            tracker.record(LearningRound::new(1, call, resp, quality));
        }
        let by_cat = tracker.scores_by_category();
        assert_eq!(by_cat.get("math").unwrap().len(), 2);
        assert_eq!(by_cat.get("creative").unwrap().len(), 1);
    }

    #[test]
    fn test_full_call_response_cycle() {
        let mut lib = CallPatternLibrary::with_standard_patterns();
        let pattern = lib.get("echo").unwrap();
        let call = pattern.create_call("teacher");
        let response = Response::new(&call.id, "student", "repeated phrase").with_latency(200);
        let quality = ResponseQuality::new(1.0, 1.0, 0.2, 1.0);
        let round = LearningRound::new(1, call, response, quality);
        assert!(round.passed());
        assert_eq!(round.response.latency_ms, 200);
    }

    #[test]
    fn test_multiple_rounds_with_improvement() {
        let mut tracker = ImprovementTracker::new("learner");
        let mut lib = CallPatternLibrary::with_standard_patterns();
        
        // Simulate 4 rounds of increasing skill
        let quality_scores = [0.2, 0.4, 0.7, 0.95];
        for (i, &s) in quality_scores.iter().enumerate() {
            let names = lib.pattern_names();
            let pattern_name = names.first().unwrap();
            let call = lib.get(pattern_name).unwrap().create_call("teacher");
            let resp = Response::new(&call.id, "learner", format!("answer-{}", i));
            let q = ResponseQuality::new(s, s, s, s);
            tracker.record(LearningRound::new(i + 1, call, resp, q));
        }
        
        assert_eq!(tracker.total_rounds(), 4);
        assert!(tracker.is_improving());
        assert!(tracker.pass_rate() >= 0.25);
    }

    #[test]
    fn test_default_quality() {
        let q = ResponseQuality::default();
        assert!((q.composite_score() - 0.5).abs() < 1e-10);
    }
}
