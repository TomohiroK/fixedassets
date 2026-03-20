# Japan (JP) - Fixed Asset Depreciation Rules

## Currency: JPY

## Depreciation Methods

### Straight-Line (定額法)
```
年間償却費 = 取得原価 x 償却率 (1 / 耐用年数)
```
- Mandatory for: Buildings (post 1998-04), Building Equipment & Structures (post 2016-04), all Intangible Assets

### 200% Declining Balance (定率法)
```
年間償却費 = 期首帳簿価額 x 償却率 (2 / 耐用年数)
```
- Available for: Machinery, Vehicles, Tools/Fixtures, Leased Assets
- **Guarantee Amount Rule**: When DB expense < (取得原価 x 保証率), switch to revised SL
- **Revised SL**: Remaining book value / remaining years

## Salvage Value
- **1 JPY** (備忘価額 / memorandum value)
- Since 2007 tax reform, all assets depreciate down to 1 JPY

## Guarantee Rate Table (保証率)

| Useful Life | Guarantee Rate | Revised Rate |
|-------------|---------------|--------------|
| 2 | 0.00000 | 1.000 |
| 3 | 0.02789 | 0.334 |
| 4 | 0.05274 | 0.334 |
| 5 | 0.06249 | 0.334 |
| 6 | 0.05776 | 0.334 |
| 7 | 0.05496 | 0.334 |
| 8 | 0.05111 | 0.334 |
| 9 | 0.04731 | 0.334 |
| 10 | 0.04448 | 0.334 |
| 15 | 0.03217 | 0.200 |
| 20 | 0.02517 | 0.167 |

## Standard Useful Lives

| Category | Useful Life |
|----------|-------------|
| RC Building (office) | 50 years |
| RC Building (residential) | 47 years |
| Wooden Building | 22-24 years |
| Building Equipment (electrical) | 15 years |
| Structures (metal) | 20-45 years |
| Machinery (general) | 7-15 years |
| Vehicles (passenger >660cc) | 6 years |
| Vehicles (light <=660cc) | 4 years |
| Trucks | 4-5 years |
| Furniture (metal) | 15 years |
| Furniture (other) | 8 years |
| Computers/Electronics | 4 years |
| Software (internal use) | 5 years |
| Software (for sale) | 3 years |
| Goodwill | 5 years |
| Patents | 8 years |
| Trademarks | 10 years |

## Special Rules
- Assets < 100,000 JPY: immediate expense
- Assets < 200,000 JPY: 3-year lump-sum depreciation
- SMEs (capital <=100M JPY): assets < 300,000 JPY can be expensed (max 3M/year)
- Intangible assets: Straight-line only, salvage = 0 JPY (not 1 JPY)

## Category Restrictions

| Category | SL | DB | Notes |
|----------|----|----|-------|
| Land | N/A | N/A | Non-depreciable |
| Building | Yes | No | SL only (post 1998-04) |
| Building Equipment | Yes | No | SL only (post 2016-04) |
| Structures | Yes | No | SL only (post 2016-04) |
| Machinery | Yes | Yes | |
| Tools/Fixtures | Yes | Yes | |
| Vehicles | Yes | Yes | |
| Leased Assets | Yes | Yes | |
| Construction in Progress | N/A | N/A | Non-depreciable |
| Patents | Yes | No | Intangible, salvage=0 |
| Trademarks | Yes | No | Intangible, salvage=0 |
| Leasehold Rights | N/A | N/A | Non-depreciable (generally) |
| Software | Yes | No | Intangible, salvage=0 |
| Facility Rights | Yes | No | Intangible, salvage=0 |
| Goodwill | Yes | No | Intangible, salvage=0 |
