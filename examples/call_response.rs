use agent_call_response::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🎵 Call & Response: 5 Rounds of Dialogue 🎵        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let lib = CallPatternLibrary::with_standard_patterns();
    let mut tracker = ImprovementTracker::new("Agent B");

    let patterns = ["echo", "invert", "extend", "harmonize", "resolve"];
    let phrases_a = [
        "Play me a melody in C major",
        "The sky is the color of bronze at dusk",
        "Once upon a time, there was a silence...",
        "I believe we can find common ground here",
        "This tension has gone on long enough",
    ];
    let phrases_b = [
        "A melody in C major — bright and resolved",
        "The bronze softens to silver under moonlight",
        "...and that silence became a symphony of whispers",
        "Indeed — our notes may differ, but the rhythm aligns",
        "Let it resolve. The final chord rings clear.",
    ];

    let mut qualities = [
        ResponseQuality::new(0.7, 0.8, 0.3, 0.7),  // echo: accurate but low creativity
        ResponseQuality::new(0.8, 0.7, 0.6, 0.75),  // invert: more creative
        ResponseQuality::new(0.9, 0.85, 0.7, 0.8),  // extend: building momentum
        ResponseQuality::new(0.85, 0.8, 0.8, 0.85), // harmonize: finding agreement
        ResponseQuality::new(0.95, 0.9, 0.85, 0.95),// resolve: satisfying conclusion
    ];

    for round in 0..5 {
        let pattern = lib.get(patterns[round]).unwrap();
        let call = pattern.create_call("Agent A");

        println!("┌─ Round {} ─────────────────────────────────────────", round + 1);
        println!("│ Pattern: {} ({})", pattern.name, pattern.description);
        println!("│ Difficulty: {:.0}%", pattern.default_difficulty * 100.0);
        println!("│");
        println!("│ 🎤 Agent A calls: \"{}\"", phrases_a[round]);
        println!("│");

        let response = Response::new(&call.id, "Agent B", phrases_b[round]).with_latency(100 + (round as u64 * 50));
        let quality = qualities[round];

        println!("│ 🎶 Agent B responds: \"{}\"", response.content);
        println!("│");
        println!("│ ─── Quality Assessment ───");
        println!("│   Relevance:    {:.0}%  {}", quality.relevance * 100.0, bar(quality.relevance));
        println!("│   Accuracy:     {:.0}%  {}", quality.accuracy * 100.0, bar(quality.accuracy));
        println!("│   Creativity:   {:.0}%  {}", quality.creativity * 100.0, bar(quality.creativity));
        println!("│   Completeness: {:.0}%  {}", quality.completeness * 100.0, bar(quality.completeness));
        println!("│   Composite:    {:.0}%  {}", quality.composite_score() * 100.0, bar(quality.composite_score()));
        println!("│   Latency:      {}ms", response.latency_ms);
        println!("│   Status:       {}", if quality.is_passing() { "✅ PASS" } else { "❌ FAIL" });
        println!("└{}", "─".repeat(55));

        tracker.record(LearningRound::new(round + 1, call, response, quality));
        println!();
    }

    // Summary
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 Session Summary                        ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Total rounds:   {}", tracker.total_rounds());
    println!("║  Average score:  {:.1}%", tracker.average_score() * 100.0);
    println!("║  Pass rate:      {:.0}%", tracker.pass_rate() * 100.0);
    println!("║  Trend:          {}{:.3}",
        if tracker.improvement_trend() > 0.0 { "📈 +" } else { "📉 " },
        tracker.improvement_trend()
    );
    println!("║  Improving?      {}", if tracker.is_improving() { "Yes! 🎉" } else { "No 😞" });
    if let Some(best) = tracker.best_round() {
        println!("║  Best round:     #{} ({:.0}%)", best.round_number, best.quality.composite_score() * 100.0);
    }

    println!("╚══════════════════════════════════════════════════════════════╝");
}

fn bar(value: f64) -> String {
    let filled = (value * 20.0).round() as usize;
    let empty = 20 - filled;
    format!("{}{}",
        "█".repeat(filled),
        "░".repeat(empty)
    )
}
