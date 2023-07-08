# Flurries :cloud_with_snow:

CLI weather tool for displaying the current conditions for a specific countries zipcode.

## Open Weather API

Flurries leverages Open Weather's API for retrieving weather conditions. An API key is required
from Open Weather and the setup account page can be found [here](https://home.openweathermap.org/users/sign_up).
Once the account is setup, the API key can be added to an `.env` file at the project root.
For a reference on how to setup the environment file please see the example below.

## Environment file example

.env

```dotenv
API_KEY=<api-key>
ZIP_CODE=<zip-code>
COUNTRY_CODE=<country-code>
WEATHER_API_URL=<url>
```
