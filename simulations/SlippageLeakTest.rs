#[cfg(test)]
mod tests {
    use super::*;
    use kamino_kswap::{SwapEngine, SwapPath, PoolState};

    #[test]
    fn test_slippage_leak_via_phantom_swap_path() {
        let mut engine = SwapEngine::new();
        let mut path = SwapPath::new(vec!["USDC", "SOL"]);

        // Step 1: Declare valid swap path
        engine.set_path(&path);

        // Step 2: Inject phantom route via CPI replay
        let phantom_path = SwapPath::new(vec!["USDC", "ETH", "SOL"]);
        engine.inject_cpi_path(&phantom_path); // Spoofed routing

        // Step 3: Trigger swap
        let result = engine.execute_swap("USDC", "SOL", 1000);

        // Step 4: Assert slippage misfire
        assert!(result.slippage > expected_slippage(), "Slippage leak not detected");

        // Step 5: Assert pool imbalance
        let imbalance = engine.check_pool_imbalance("SOL");
        assert!(imbalance > 0, "Pool imbalance not detected");
    }

    fn expected_slippage() -> f64 {
        0.005 // 0.5%
    }
}

