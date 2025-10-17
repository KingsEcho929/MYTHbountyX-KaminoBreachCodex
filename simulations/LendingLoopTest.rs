// Kamino Glyph #002 — Lending Loop via Collateral Reentry
// Simulates recursive deposit and borrow flow in klend

#[cfg(test)]
mod tests {
    use super::*;
    use kamino_lending::{LendingProtocol, UserAccount};

    #[test]
    fn test_recursive_collateral_reentry() {
        // Step 1: Initialize protocol and user
        let mut protocol = LendingProtocol::new();
        let mut user = UserAccount::new("KingsEcho");

        // Step 2: Deposit collateral
        user.deposit_collateral(&mut protocol, "SOL", 1000);

        // Step 3: Trigger borrow flow via CPI
        user.borrow_asset(&mut protocol, "USDC", 500);

        // Step 4: Reenter deposit logic mid-flow (simulate CPI reentry)
        user.deposit_collateral(&mut protocol, "SOL", 1000); // Phantom collateral

        // Step 5: Borrow again against inflated collateral
        user.borrow_asset(&mut protocol, "USDC", 1000);

        // Step 6: Assert leverage loop and liquidation bypass
        assert!(user.total_borrowed() > 1500, "Leverage loop not detected");
        assert!(!protocol.can_liquidate(&user), "Liquidation bypass detected");
    }
}
