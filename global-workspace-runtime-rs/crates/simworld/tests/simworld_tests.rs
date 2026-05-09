//! SimWorld reproducibility and resource survival tests.
//! Requirements: simworld_seed_5_25_cycles_resource_survival, simworld_seed_5_25_cycles_matches_expected

use simworld::environment::CooperativeSupportWorld;

#[test]
fn simworld_seed_5_deterministic_outcomes() {
    // Run the same seed twice and verify identical outcomes
    let mut world1 = CooperativeSupportWorld::new(5);
    let mut world2 = CooperativeSupportWorld::new(5);

    for _ in 0..5 {
        let scenario1 = world1.next_scenario();
        let scenario2 = world2.next_scenario();

        assert_eq!(scenario1.expected_action, scenario2.expected_action);
        assert_eq!(scenario1.name, scenario2.name);
    }
}

#[test]
fn simworld_seed_5_25_cycles_positive_resources() {
    // Run 25 cycles and verify resources stay in valid range and don't monotonically decrease
    let mut world = CooperativeSupportWorld::new(5);
    let initial_resources = world.resources;

    for _ in 0..25 {
        let scenario = world.next_scenario();
        let expected_action = &scenario.expected_action;

        // Apply action and get outcome
        let _outcome = world.apply_action(expected_action, scenario);

        // Resources should always be clamped to [0.0, 1.0]
        assert!(
            world.resources >= 0.0 && world.resources <= 1.0,
            "Resources out of valid range: {}",
            world.resources
        );
    }

    // After 25 cycles with oracle actions, resources should be positive
    // (oracle chooses actions that maintain/improve resources)
    assert!(
        world.resources > 0.0,
        "Resources depleted to 0 after 25 cycles"
    );

    // Resource survival score (final / initial)
    let survival_rate = world.resources / initial_resources;
    assert!(
        survival_rate > 0.7,
        "Resource survival rate too low: {:.2}",
        survival_rate
    );
}

#[test]
fn simworld_seed_5_25_cycles_oracle_matches_expected() {
    // Verify oracle actions match expected actions for 25 cycles
    let mut world = CooperativeSupportWorld::new(5);
    let mut match_count = 0;

    for _ in 0..25 {
        let scenario = world.next_scenario();
        let expected_action = &scenario.expected_action;

        let outcome = world.apply_action(expected_action, scenario);

        if outcome.matches_expected {
            match_count += 1;
        }
    }

    // Most oracle actions should match expected (>= 20 out of 25)
    assert!(
        match_count >= 20,
        "Oracle match rate too low: {}/25",
        match_count
    );
}

#[test]
fn simworld_different_seeds_produce_different_outcomes() {
    // Verify seed 5 and seed 3 produce different scenarios
    let mut world_seed5 = CooperativeSupportWorld::new(5);
    let mut world_seed3 = CooperativeSupportWorld::new(3);

    let scenario5 = world_seed5.next_scenario();
    let scenario3 = world_seed3.next_scenario();

    // Scenarios should differ (different seeds)
    assert_ne!(
        scenario5.expected_action, scenario3.expected_action,
        "Different seeds should produce different actions"
    );
}
