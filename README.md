# Aviation Weather

## Makefile
`make help` to list all commands

## Setup

1. Override any environment variables in `.env.local`
2. Build the api and ui images with `make build`
3. Run the application with `make up`

## Data Sources

### Airport Data

Potential Data sources
 - https://adip.faa.gov/agis/public/#/airportSearch/advanced
 - https://www.icao.int/Aviation-API-Data-Service/Pages/default.aspx
 - https://ourairports.com/data/
 - [mborsetti/airportsdata](https://github.com/mborsetti/airportsdata)
 - https://www.iata.org/en/publications/directories/code-search/
 - [openstreet](https://www.openstreetmap.org/#map=13/38.95223/-77.47417)

### Metar Data
Metar data is collected from aviationweather.gov.

#### Decoding METARS
The following resources were used to help decode METARS.
- [Metar Decode Key PDF](https://www.weather.gov/media/wrh/mesowest/metar_decode_key.pdf)
- [Metar Decode (NPS EDU)](https://met.nps.edu/~bcreasey/mr3222/files/helpful/DecodeMETAR-TAF.html)
- [Weather Phenomena](http://www.moratech.com/aviation/metar-class/metar-pg9-ww.html)

### OpenMapTiles
[Generate Vector Tiles](https://openmaptiles.org/docs/generate/generate-openmaptiles/)