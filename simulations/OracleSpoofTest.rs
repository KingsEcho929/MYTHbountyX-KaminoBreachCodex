// Kamino Glyph #003 — Oracle Spoof via CPI Replay
// Simulates stale CPI injection and liquidation misfire in scope oracle aggregator

#[cfg(test)]
mod tests {
    use super::*;
    use kamino_scope::{OracleAggregator, PriceFeed, LiquidationEngine};

    #[test]
    fn test_oracle_spoof_via_cpi_replay() {
        // Step 1: Initialize oracle and price feed
        let mut oracle = OracleAggregator::new();
        let mut feed = PriceFeed::new("SOL");

        // Step 2: Inject fresh price
        feed.set_price(25.0);
        oracle.update_price(&feed);

        // Step 3: Replay stale CPI with delayed state
        let mut stale_feed = PriceFeed::new("SOL");
        stale_feed.set_price(10.0); // Stale price
        oracle.inject_cpi(&stale_feed); // Simulate replay

        // Step 4: Trigger liquidation logic
        let mut engine = LiquidationEngine::new(&oracle);
        let result = engine.liquidate("KingsEcho", "SOL");

        // Step 5: Assert liquidation misfire
        assert!(result.is_err(), "Liquidation misfire not detected");

        // Step 6: Simulate reward overflow
        let rewards = engine.calculate_rewards("KingsEcho");
        assert!(rewards > expected_rewards(), "Reward overflow not detected");
    }

    fn expected_rewards() -> u64 {
        // Placeholder for expected reward logic
        1000
    }
}
