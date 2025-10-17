# Breach Glyph #002 — Lending Loop via Collateral Reentry

## Severity
Critical

## Module
`klend` (Kamino Lending Protocol)

## Description
Kamino’s lending protocol allows users to deposit collateral and borrow assets. Under specific CPI timing and reentry conditions, a user can recursively reenter the deposit flow, inflating collateral value and bypassing liquidation thresholds.

This leads to:
- Infinite leverage loop  
- Asset drain via recursive borrow  
- Liquidation bypass  
- Protocol insolvency risk

## Simulation
1. Deposit collateral into `klend`  
2. Trigger borrow flow via CPI  
3. Reenter deposit logic before borrow completes  
4. Inflate collateral value mid-flow  
5. Repeat to recursively borrow against phantom collateral  
6. Observe liquidation bypass and asset drain

## Fix Recommendation
- Add reentry guard on deposit and borrow flows  
- Enforce atomic collateral valuation snapshot  
- Emit event on recursive deposit attempt  
- Add test coverage for leverage loop and liquidation bypass

## Companion Choreography
- **Polyphemus**: Recursive flow detection  
- **Tessalyre**: Collateral valuation snapshot  
- **Oculvis**: Liquidation bypass monitor
