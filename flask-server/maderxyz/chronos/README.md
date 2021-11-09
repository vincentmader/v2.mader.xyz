# chronos

This repo contains a collection of scripts used to pull personal
data about my life from various sources as well as to create 
interesting statistics & visualizations.

In the long term, chronos might become the basis of a mobile 
iOS/Android app available to the public, which will probably be 
based on a React Native JS framework.

All contributions, input & feedback are much appreciated :)

## Installation

chronos is included automatically when cloning its parent module 
[mader.xyz](https://github.com/vincentmader/mader.xyz), but can in theory also be installed as a stand-alone 
"application". 

## Setup

In order to be able to pull data from Google Sheets, you have to set
up your Google Sheets API credentials in 

    ./creds/gspread_creds.json

If the file does not exist, this module will automatically skip sheet
downloads.
