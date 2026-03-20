# Indonesia (ID) - Fixed Asset Depreciation Rules

## Currency: IDR

## Depreciation Methods
- **Straight-line**
- **Declining balance**
- Both available for non-building assets; buildings use straight-line only
- Method must be applied consistently once chosen

## Asset Groups and Rates (PMK-72/2023)

### Non-Building Assets

| Group | Life | SL Rate | DB Rate | Examples |
|-------|------|---------|---------|---------|
| 1 | 4 years | 25% | 50% | Computers, printers, wood furniture, office equipment, motorcycles |
| 2 | 8 years | 12.5% | 25% | Cars, buses, trucks, metal furniture, AC, telecom equipment |
| 3 | 16 years | 6.25% | 12.5% | Heavy mining machinery, textile/chemical machines, vessels |
| 4 | 20 years | 5% | 10% | Locomotives, heavy vessels, heavy construction equipment |

### Building Assets (SL only)

| Type | Life | SL Rate |
|------|------|---------|
| Permanent buildings | 20 years | 5% |
| Non-permanent buildings | 10 years | 10% |

## Salvage Value
- **0**. Under declining balance, remaining book value at end of useful life is expensed in lump sum in final year.

## Special Rules
- Unlisted assets default to **Group 3** (16 years)
- Permanent buildings with actual life > 20 years may use actual life
- General software: expensed immediately; specialized software: amortized
- Depreciation begins in the month of acquisition
- Month-pro-rata for partial year

## Category Mapping to Groups

| Category | Group | Life | SL Rate | DB Rate |
|----------|-------|------|---------|---------|
| Land | N/A | N/A | N/A | N/A |
| Building (permanent) | Bldg | 20 yr | 5% | N/A |
| Building (non-permanent) | Bldg | 10 yr | 10% | N/A |
| Building Equipment | Group 2 | 8 yr | 12.5% | 25% |
| Structures | Bldg | 20 yr | 5% | N/A |
| Machinery | Group 2-3 | 8-16 yr | varies | varies |
| Tools/Fixtures | Group 1 | 4 yr | 25% | 50% |
| Vehicles | Group 2 | 8 yr | 12.5% | 25% |
| Leased Assets | per group | varies | varies | varies |
| Computers | Group 1 | 4 yr | 25% | 50% |
| Software | Group 1 | 4 yr | 25% | 50% |
| Patents | Intangible | per contract | SL only | N/A |
| Goodwill | Intangible | per group | SL only | N/A |

## Implementation Notes
- The 4-group system determines rates, NOT the asset category name
- User should be able to select which group an asset belongs to
- Or system auto-maps based on category with override option
