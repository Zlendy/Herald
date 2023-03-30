#!/usr/bin/env python

import requests

from dotenv import load_dotenv
from os import environ as env
from pprint import pprint

load_dotenv() # take environment variables from .env

GOTIFY=f"https://{env.get('BASE_URL')}"

def get_applications() -> list:
    return requests.get(
        url=f"{GOTIFY}/application",
        headers={
            "X-Gotify-Key": env.get('TOKEN'),
        },
        verify=False, ### INSECURE ###
    ).json()
    
def get_messages() -> list:
    return requests.get(
        url=f"{GOTIFY}/message",
        headers={
            "X-Gotify-Key": env.get('TOKEN'),
        },
        verify=False, ### INSECURE ###
    ).json()
    
def get_message(id: int):
    return requests.get(
        url=f"{GOTIFY}/application/{id}/message",
        headers={
            "X-Gotify-Key": env.get('TOKEN'),
        },
        verify=False, ### INSECURE ###
    ).json()
    
if __name__ == "__main__":
    print("-- Application list --")
    apps: list = get_applications()
    pprint(apps)
    
    print("\n-- Message list --")
    messages = get_messages()
    pprint(messages)
    
    print("\n-- Message list from third application --")
    message = get_message(apps[2]["id"])
    pprint(message)