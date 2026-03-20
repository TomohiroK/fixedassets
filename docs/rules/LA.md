# Laos (LA) - Fixed Asset Depreciation Rules

## Currency: LAK

## Depreciation Methods
- **Straight-line** (primary)
- **Double-declining balance**
- **Activity-based / Units of production** (since Amended Tax Law 2011)

## Prescribed Straight-Line Rates

| Asset Category | Rate | Implied Life |
|----------------|------|-------------|
| Industrial buildings (life <=20 yr) | 5% | 20 years |
| Industrial buildings (life >20 yr) | 2% | 50 years |
| Permanent commercial/residential buildings | 5% | 20 years |
| Semi-permanent buildings | 10% | 10 years |
| Machinery & vehicles (industrial) | 20% | 5 years |
| Material & office supplies | 20% | 5 years |
| Ships & boats | 10% | 10 years |
| Passenger/cargo planes | Per flight hours | Varies |

## Intangible Asset Rates

| Asset | Rate | Life |
|-------|------|------|
| Company establishment costs | 50% | 2 years |
| Mining exploration & research | 20% | 5 years |
| Software | 20% | 5 years |
| Goodwill | 20% | 5 years |
| Patents | 20% | 5 years |

## Salvage Value
- **0** (in practice). Not explicitly prescribed.

## Calculation

### Straight-Line
```
Annual Expense = Cost x Prescribed Rate
```

### Double-Declining Balance
```
Annual Expense = Book Value x (2 / Useful Life)
```

## Special Rules
- Intangible assets with undetermined useful life: depreciation NOT allowed
- Start-up expenses: amortized over 2 years
- Depreciation commences from acquisition date
- Assets fully depreciated must not be further depreciated
- Losses carried forward 3 years

## Category Restrictions

| Category | SL | DDB | UoP | Notes |
|----------|----|----|-----|-------|
| Land | N/A | N/A | N/A | Non-depreciable |
| Buildings | Yes | No | No | Rate varies by type |
| Machinery | Yes | Yes | Yes | 20% SL |
| Vehicles | Yes | Yes | No | 20% SL |
| Furniture/Office | Yes | Yes | No | 20% SL |
| Software | Yes | No | No | 20% (5 years) |
| Patents | Yes | No | No | 20% (5 years) |
| Goodwill | Yes | No | No | 20% (5 years) |
| Ships | Yes | No | No | 10% (10 years) |

## Implementation Notes
- Relatively simple system with prescribed rates
- DDB available for tangible movable assets
- Intangible assets use SL only at prescribed rates
- Month-pro-rata should apply for partial year acquisition
