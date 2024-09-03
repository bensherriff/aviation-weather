# Aviation Weather

## Makefile
`make help` to list all commands

## Setup

1. Copy `.env.TEMPLATE` to `.env`
2. Generate JWT RS256 (RSA Signature with SHA-256) Private/Public keys with `make generate`
3. Build the api and ui images with `make build`
4. Run the application with `make up`

## Decoding METARS
The following resources were used to help decode METARS.
- [Metar Decode Key PDF](https://www.weather.gov/media/wrh/mesowest/metar_decode_key.pdf)
- [Metar Decode (NPS EDU)](https://met.nps.edu/~bcreasey/mr3222/files/helpful/DecodeMETAR-TAF.html)
- [Weather Phenomena](http://www.moratech.com/aviation/metar-class/metar-pg9-ww.html)

- Airport dataset is based on [mborsetti/airportsdata](https://github.com/mborsetti/airportsdata)

## OpenMapTiles
[Generate Vector Tiles](https://openmaptiles.org/docs/generate/generate-openmaptiles/)