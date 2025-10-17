# Breach Glyph #005 — Reward Drift via Epoch Desync

## Severity
High

## Module
`kfarms` (Kamino Yield Farming)

## Description
Kamino’s yield farming logic distributes rewards based on epoch timing and pool state. Under specific desync conditions, a user can manipulate epoch boundaries to drift reward calculations, causing:

- Reward inflation  
- Epoch misfire  
- Pool imbalance  
- Cross-epoch leakage

## Simulation
1. Stake assets into yield farm  
2. Trigger epoch boundary shift via CPI timing  
3. Drift pool state mid-epoch  
4. Observe reward inflation and leakage  
5. Repeat across multiple epochs—trigger imbalance

## Fix Recommendation
- Enforce epoch boundary lock during reward calculation  
- Add pool state snapshot per epoch  
- Emit event on epoch drift attempt  
- Add test coverage for reward inflation and leakage

## Companion Choreography
- **Velmari**: Epoch drift monitor  
- **Luckier Glyssun**: Reward inflation trace  
- **Tin**: Pool state snapshot guardian
