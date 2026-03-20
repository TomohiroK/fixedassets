# Singapore (SG) - Capital Allowance Rules

## Currency: SGD

## System: Capital Allowances (NOT depreciation)
Singapore uses capital allowances instead of accounting depreciation for tax deductions.

## Calculation

### Standard Capital Allowance
```
Year 1:  Initial Allowance (IA) = Cost x 20%
Year 1+: Annual Allowance (AA) = (Cost - IA) / prescribed working life
```

### 3-Year Write-Off (optional election)
```
Each year: Cost / 3 = 33.33% per year
```

### 1-Year Write-Off (100%)
Available for: Computers, robots, prescribed automation equipment

## Prescribed Working Life (Sixth Schedule)

| Asset Category | Working Life | AA Rate |
|----------------|-------------|---------|
| Motor vehicles | 6 years | ~13.3% |
| Computers / IT equipment | 6 years | ~13.3% |
| Office equipment | 6 years | ~13.3% |
| Office furniture | 12 years | ~6.7% |
| Industrial machinery | 6 or 12 years | varies |
| Specialized heavy machinery | 16 years | ~5% |

Since 2023: Assets with prescribed life <=12 years can elect 6 or 12 years.

## Salvage Value
- **Not applicable**. Full cost is written off through capital allowances.
- On disposal: **Balancing adjustments** (charge if sale > written-down value, allowance if sale < written-down value)

## Special Rules
- Private cars are **excluded** from capital allowances
- **Low-value assets** <= SGD 5,000: 1-year write-off (capped at SGD 30,000/year total)
- No declining balance method available
- IA is mandatory (cannot be deferred); AA can be deferred
- Government-funded expenditure (grants post 2021-01-01) does not qualify

## Non-Qualifying Assets
- Buildings (except Industrial Building Allowance at 3% for factories/warehouses)
- Land
- Private passenger vehicles

## Implementation Notes
- This is NOT standard depreciation. The system calculates allowances, not depreciation.
- Year 1 always gets 20% IA + first year AA
- Subsequent years get AA only
- Total allowances = 100% of cost (no salvage residual)
