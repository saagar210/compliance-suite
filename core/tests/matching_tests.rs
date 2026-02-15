//! Matching Algorithm Golden Tests (Phase 2.4)
//!
//! Tests the matching algorithm against golden fixtures to ensure
//! deterministic and expected behavior.

use core::answer_bank::AnswerBankEntry;
use core::questionnaire::matching::MatchingEngine;

#[derive(serde::Deserialize)]
struct AnswerFixture {
    id: String,
    question_canonical: String,
    answer_short: String,
    answer_long: String,
}

#[derive(serde::Deserialize)]
struct TestCase {
    question: String,
    expected_top_answer_id: String,
    min_score: f64,
    description: String,
}

fn load_answer_fixtures() -> Vec<AnswerBankEntry> {
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fixtures/matching_baseline/answers.json"
    );
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read answers fixture");

    let fixtures: Vec<AnswerFixture> =
        serde_json::from_str(&content).expect("Failed to parse answers fixture");

    fixtures
        .into_iter()
        .map(|f| AnswerBankEntry {
            entry_id: f.id,
            vault_id: "test_vault".to_string(),
            question_canonical: f.question_canonical,
            answer_short: f.answer_short,
            answer_long: f.answer_long,
            notes: None,
            evidence_links: Vec::new(),
            owner: "test@example.com".to_string(),
            last_reviewed_at: None,
            tags: Vec::new(),
            source: "test".to_string(),
            content_hash: "test_hash".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
        })
        .collect()
}

fn load_test_cases() -> Vec<TestCase> {
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fixtures/matching_baseline/test_cases.json"
    );
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read test cases fixture");

    serde_json::from_str(&content).expect("Failed to parse test cases fixture")
}

#[test]
fn test_matching_against_golden_fixtures() {
    let answers = load_answer_fixtures();
    let test_cases = load_test_cases();
    let engine = MatchingEngine::new(answers);

    for case in test_cases {
        println!("\nTesting: {}", case.description);
        println!("Question: {}", case.question);

        let suggestions = engine
            .get_suggestions(&case.question, 3)
            .expect("Failed to get suggestions");

        assert!(
            !suggestions.is_empty(),
            "Expected at least one suggestion for: {}",
            case.question
        );

        // For baseline algorithm, verify expected answer is in top 3
        let expected_in_results = suggestions
            .iter()
            .any(|s| s.answer_bank_entry_id == case.expected_top_answer_id);

        assert!(
            expected_in_results,
            "Expected {} to be in top 3 matches for '{}', but not found. Got: {:?}",
            case.expected_top_answer_id,
            case.question,
            suggestions
                .iter()
                .map(|s| &s.answer_bank_entry_id)
                .collect::<Vec<_>>()
        );

        let top_suggestion = &suggestions[0];
        println!(
            "Top match: {} (score: {:.2})",
            top_suggestion.answer_bank_entry_id, top_suggestion.score
        );
        println!("Explanation: {}", top_suggestion.confidence_explanation);

        // Verify the expected answer (if it's the top match) has minimum score
        if top_suggestion.answer_bank_entry_id == case.expected_top_answer_id {
            assert!(
                top_suggestion.score >= case.min_score,
                "Expected score >= {}, got {} for '{}'",
                case.min_score,
                top_suggestion.score,
                case.question
            );
        }
    }
}

#[test]
fn test_matching_returns_top_n_limit() {
    let answers = load_answer_fixtures();
    let engine = MatchingEngine::new(answers);

    let suggestions = engine
        .get_suggestions("security access control network", 3)
        .expect("Failed to get suggestions");

    assert!(
        suggestions.len() <= 3,
        "Expected at most 3 suggestions, got {}",
        suggestions.len()
    );
}

#[test]
fn test_matching_suggestions_sorted_by_score() {
    let answers = load_answer_fixtures();
    let engine = MatchingEngine::new(answers);

    let suggestions = engine
        .get_suggestions("access control authentication security", 5)
        .expect("Failed to get suggestions");

    for i in 0..suggestions.len().saturating_sub(1) {
        assert!(
            suggestions[i].score >= suggestions[i + 1].score,
            "Suggestions not sorted: score[{}]={} < score[{}]={}",
            i,
            suggestions[i].score,
            i + 1,
            suggestions[i + 1].score
        );
    }
}

#[test]
fn test_matching_with_no_overlap_returns_empty() {
    let answers = load_answer_fixtures();
    let engine = MatchingEngine::new(answers);

    // Use completely unrelated terms
    let suggestions = engine
        .get_suggestions("quantum physics relativity cosmology", 3)
        .expect("Failed to get suggestions");

    // Should return empty or very low scores (all filtered out)
    assert!(
        suggestions.is_empty() || suggestions[0].score < 0.1,
        "Expected no matches or very low scores for unrelated question"
    );
}

#[test]
fn test_normalized_fields_are_lowercase() {
    let answers = load_answer_fixtures();
    let engine = MatchingEngine::new(answers);

    let suggestions = engine
        .get_suggestions("ACCESS CONTROL", 1)
        .expect("Failed to get suggestions");

    assert!(!suggestions.is_empty());
    let normalized_q = &suggestions[0].normalized_question;
    assert_eq!(
        normalized_q.to_lowercase(),
        *normalized_q,
        "Normalized question should be lowercase"
    );
}
