// Kamino Glyph #007 — Governance Drift via Proposal Replay
// Simulates double execution and parameter override in kgov due to stale CPI replay

#[cfg(test)]
mod tests {
    use super::*;
    use kamino_kgov::{GovernanceModule, Proposal, ExecutionState};

    #[test]
    fn test_governance_drift_via_proposal_replay() {
        // Step 1: Initialize governance module and proposal
        let mut gov = GovernanceModule::new();
        let mut proposal = Proposal::new("Update slippage threshold", "Set to 0.5%");

        // Step 2: Submit and execute proposal
        gov.submit_proposal(&proposal);
        gov.execute_proposal(&proposal);

        // Step 3: Replay stale CPI with delayed state
        let stale_execution = ExecutionState::from_snapshot(&proposal);
        gov.inject_cpi(stale_execution); // Simulate replay

        // Step 4: Assert double execution and parameter override
        let current_value = gov.get_parameter("slippage_threshold");
        assert!(current_value != "0.5%", "Parameter override not detected");

        // Step 5: Assert governance drift
        assert!(gov.has_drifted(), "Governance drift not detected");
    }
}
