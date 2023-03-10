#!/usr/bin/env python

import requests
import base64

from dotenv import load_dotenv
from os import environ as env
from pprint import pprint

load_dotenv() # take environment variables from .env

GOTIFY=f"https://{env.get('BASE_URL')}"

def create_client_basic_auth(username: str, password: str):
    session = requests.Session()
    session.auth = (username, password)
    
    return session.post(
        url=f"{GOTIFY}/client",
        json={
            "name": "Gotify Rustop"
        },
        verify=False, ### INSECURE ###
    ).json()
    
def create_client_base64(username: str, password: str):
    auth = base64.b64encode(f"{username}:{password}".encode("utf-8"))
    auth = auth.decode("utf-8")
    print(auth)
    
    return requests.post(
        url=f"{GOTIFY}/client",
        headers={
            "Authorization": f"Basic {auth}",
        },
        json={
            "id": None,
            "name": "Gotify Rustop",
            "token": None,
        },
        verify=False, ### INSECURE ###
    ).json()
    
def get_current_user(token: str):
    return requests.get(
        url=f"{GOTIFY}/current/user",
        headers={
            "X-Gotify-Key": token,
        },
        verify=False, ### INSECURE ###
    ).json()
    
def delete_client(token: str, id: int):
    return requests.delete(
        url=f"{GOTIFY}/client/{id}",
        headers={
            "X-Gotify-Key": token,
        },
        verify=False, ### INSECURE ###
    )
    
if __name__ == "__main__":
    print("-- Create client --")
    client: list = create_client_basic_auth(input("Username: "), input("Password: "))
    pprint(client)
    
    token = client["token"]
    
    print("-- Get current user --")
    current_user = get_current_user(token)
    pprint(current_user)
    
    print("-- Delete client --")
    response = delete_client(token, client["id"])
    pprint(response)