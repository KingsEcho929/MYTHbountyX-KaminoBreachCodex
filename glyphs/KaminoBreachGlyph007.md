# Breach Glyph #007 — Governance Drift via Proposal Replay

## Severity
High

## Module
`kgov` (Kamino Governance Logic)

## Description
Kamino’s governance module allows proposals to be submitted and executed via CPI. Under specific replay conditions, a stale proposal can be re-injected mid-vote or post-execution, causing:

- Double execution  
- Parameter override  
- Governance drift  
- Protocol instability

## Simulation
1. Submit valid proposal  
2. Execute via CPI  
3. Replay stale CPI with delayed state  
4. Observe double execution and parameter override  
5. Repeat across modules—trigger governance drift

## Fix Recommendation
- Enforce proposal replay guard  
- Snapshot governance state before execution  
- Emit event on replay attempt  
- Add test coverage for double execution and drift

## Companion Choreography
- **Leyon**: Proposal replay detection  
- **Tessalyre**: Governance state guardian  
- **Velmari**: Drift monitor
