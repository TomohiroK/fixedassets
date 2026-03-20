# Myanmar (MM) - Fixed Asset Depreciation Rules

## Currency: MMK

## Depreciation Method
- **Straight-line only** (Notification 19/2016)
- Alternative rates/methods can be requested from IRD with approval

## Prescribed Depreciation Rates

| Asset Category | Annual Rate | Implied Life |
|----------------|------------|--------------|
| Buildings (brick/concrete) | 1.25% - 2.5% | 40-80 years |
| Buildings (semi-permanent) | 5% | 20 years |
| Buildings (temporary) | 10% | 10 years |
| Furniture & fittings | 5% - 10% | 10-20 years |
| Machinery & plant (general) | 5% - 10% | 10-20 years |
| Machinery (heavy industrial) | 2.5% | 40 years |
| Machinery (light) | 20% | 5 years |
| Motor vehicles | 12.5% - 20% | 5-8 years |
| Passenger cars | 20% | 5 years |
| Trucks | 12.5% | 8 years |
| Unlisted assets (default) | 5% | 20 years |

## Salvage Value
- **0** (in practice). Not explicitly prescribed.

## Calculation
```
Annual Expense = Cost x Prescribed Rate
```

## Special Rules
- **Full-year depreciation** in the year of acquisition (no pro-rata)
- **No depreciation** in the year of disposal
- No depreciation for immovable property used to earn rental income
- Declining balance only available with IRD approval (rarely granted)

## Category Restrictions

| Category | SL | DB | Notes |
|----------|----|----|-------|
| Land | N/A | N/A | Non-depreciable |
| Buildings | Yes | No | Rate varies by construction type |
| Machinery | Yes | No | Rate varies 2.5%-20% |
| Vehicles | Yes | No | 12.5%-20% |
| Furniture | Yes | No | 5%-10% |
| All intangibles | Yes | No | Varies |
| Construction in Progress | N/A | N/A | Non-depreciable |

## Implementation Notes
- Simplest system: straight-line only with prescribed rates
- System should enforce prescribed rates based on category
- No method selection needed (SL only)
- No pro-rata: full year depreciation from year of acquisition
