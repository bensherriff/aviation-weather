import pandas as pd
from datetime import date
import json

# Load the airports.csv file from the web
url = 'https://davidmegginson.github.io/ourairports-data/airports.csv'
df = pd.read_csv(url, index_col=0)

# convert the dataframe to a dictionary
airports = df.to_dict('index')
formated_airports = {}
for airport in airports:

  category = airports[airport]['type']
  if pd.isnull(category) or category == 'nan' or category == 'NAN':
    category = 'UNKNOWN'

  elevation = airports[airport]['elevation_ft']
  if pd.isnull(elevation) or elevation == 'nan' or elevation == 'NAN':
    elevation = 0

  country = airports[airport]['iso_country']
  if pd.isnull(country) or country == 'nan' or country == 'NAN':
    country = 'UNKNOWN'
  
  region = airports[airport]['iso_region']
  if pd.isnull(region) or region == 'nan' or region == 'NAN':
    region = 'UNKNOWN'
  
  municipality = airports[airport]['municipality']
  if pd.isnull(municipality) or municipality == 'nan' or municipality == 'NAN':
    municipality = 'UNKNOWN'

  iata = airports[airport]['iata_code']
  if pd.isnull(iata) or iata == 'nan' or iata == 'NAN':
    iata = ''
  
  local = airports[airport]['local_code']
  if pd.isnull(local) or local == 'nan' or local == 'NAN':
    local = ''

  formated_airports[airport] = {
    'icao': airports[airport]['ident'],
    'category': category,
    'name': airports[airport]['name'],
    'elevation_ft': elevation,
    'iso_country': country,
    'iso_region': region,
    'municipality': municipality,
    'iata_code': iata,
    'local_code': local,
    'latitude': airports[airport]['latitude_deg'],
    'longitude': airports[airport]['longitude_deg']
  }

# convert the dictionary to a list of dictionaries
formated_airports = list(formated_airports.values())

# convert the list of dictionaries to a json file
today = date.today()
date = today.strftime("%Y-%m-%d")
with open(f'airports_{date}.json', 'wb') as file:
  file.write(json.dumps(formated_airports).encode('utf-8'))