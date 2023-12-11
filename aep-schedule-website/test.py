import requests

headers = {
    'User-Agent': 'NCSA Mosaic/1.0 (X11;SunOS 4.1.4 sun4m)',
}
url = 'https://cours.polymtl.ca/Horaire/public/horsage.csv'
response = requests.get(url, headers=headers)
print(response.text)