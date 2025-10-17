#[cfg(test)]
mod tests {
    use super::*;
    use kamino_epoch::{EpochManager, SyncEngine};
    use kamino_kvault::Vault;
    use kamino_kfarms::Farm;
    use kamino_klend::LendingProtocol;

    #[test]
    fn test_epoch_override_via_cross_program_sync() {
        let mut epoch = EpochManager::new();
        let mut sync = SyncEngine::new();
        let mut vault = Vault::new("USDC");
        let mut farm = Farm::new("USDC");
        let mut lend = LendingProtocol::new();

        // Step 1: Trigger CPI sync from external module
        sync.inject_cpi_sync("external_module");

        // Step 2: Override epoch boundary
        epoch.override_boundary(); // Desync begins

        // Step 3: Observe desync across modules
        assert!(vault.is_epoch_desynced(&epoch), "Vault desync not detected");
        assert!(farm.is_epoch_desynced(&epoch), "Farm desync not detected");
        assert!(lend.is_epoch_desynced(&epoch), "Lending desync not detected");

        // Step 4: Assert governance override
        assert!(epoch.has_governance_override(), "Governance override not detected");
    }
}
