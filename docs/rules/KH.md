# Cambodia (KH) - Fixed Asset Depreciation Rules

## Currency: KHR (USD widely used in practice)

## System: Class-Based Depreciation
Assets are classified into 4 classes. The class determines BOTH the rate AND the method.

## Asset Classes

| Class | Method | Rate | Assets |
|-------|--------|------|--------|
| 1(i) | Straight-line | 5% | Concrete buildings, roads, railways, transport ships |
| 1(ii) | Straight-line | 10% | Non-concrete buildings |
| 2 | Declining balance (pooled) | 50% | Computers, electronic information systems, data handling equipment |
| 3 | Declining balance (pooled) | 25% | Automobiles, trucks, office furniture & equipment |
| 4 | Declining balance (pooled) | 20% | All other tangible property |

## Pooled Basis (Classes 2-4)
- All assets in the same class are **grouped into a single pool**
- Additions increase the pool value; disposals reduce it
- No individual asset gain/loss calculation
- Pool balance approaches zero asymptotically (never fully written off)

## Salvage Value
- **Class 1**: Depreciated to 0 (straight-line)
- **Classes 2-4**: No explicit salvage value (pool never reaches 0)

## Calculation

### Class 1 (Straight-Line per individual asset)
```
Annual Expense = Cost x Rate (5% or 10%)
```

### Classes 2-4 (Pooled Declining Balance)
```
Annual Expense = Pool Opening Balance x Rate
Pool Closing = Pool Opening - Expense + Additions - Disposals
```

## Special Rules
- **QIP (Qualified Investment Project)**: 40% first-year special depreciation for qualifying manufacturing/processing assets
  - Clawback if asset held < 4 years
- **Intangible assets**: SL over useful life, or 10% if life cannot be determined
- **Goodwill**: Depreciable for CIT purposes
- Depreciation starts in the year asset is put into service
- No depreciation in year of disposal

## Category to Class Mapping

| Category | Class | Rate | Method |
|----------|-------|------|--------|
| Land | N/A | N/A | Non-depreciable |
| Building (concrete) | 1(i) | 5% | SL |
| Building (non-concrete) | 1(ii) | 10% | SL |
| Building Equipment | 4 | 20% | DB pooled |
| Structures | 1(i) | 5% | SL |
| Machinery | 4 | 20% | DB pooled |
| Tools/Fixtures | 3 | 25% | DB pooled |
| Vehicles | 3 | 25% | DB pooled |
| Computers | 2 | 50% | DB pooled |
| Software | Intangible | 10% | SL |
| Patents | Intangible | 10% | SL |
| Goodwill | Intangible | 10% | SL |

## Implementation Notes
- **Unique pooled system**: Classes 2-4 don't track individual assets for depreciation
- System may need a "pool" concept where multiple assets share a depreciation pool
- For simplicity, can calculate per-asset but note the theoretical difference
- Class 2 at 50% DB is very aggressive (computers lose 50% value per year)
