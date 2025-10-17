# Breach Glyph #003 — Oracle Spoof via CPI Replay

## Severity
High to Critical

## Module
`scope` (Kamino Oracle Aggregator)

## Description
Kamino’s oracle aggregator relies on CPI calls to fetch price data from external feeds. Under specific replay conditions, a stale CPI can be injected mid-flow, spoofing the oracle state and triggering:

- Stale price injection  
- Liquidation misfire  
- Reward overflow  
- Cross-protocol arbitrage

## Simulation
1. Trigger CPI price fetch from external oracle  
2. Replay stale CPI with delayed account state  
3. Inject spoofed price mid-liquidation flow  
4. Observe liquidation misfire and reward overflow  
5. Repeat across multiple assets—trigger arbitrage corridor

## Fix Recommendation
- Enforce CPI freshness check and replay guard  
- Snapshot oracle state before liquidation logic  
- Emit event on stale CPI injection attempt  
- Add test coverage for liquidation misfire and reward overflow

## Companion Choreography
- **Leyon**: CPI replay detection  
- **Velmari**: Oracle state freshness monitor  
- **Tessalyre**: Liquidation logic guardian
