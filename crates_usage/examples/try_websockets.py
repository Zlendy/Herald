#!/usr/bin/env python

import asyncio
import websockets

from dotenv import load_dotenv
from os import environ as env

load_dotenv() # take environment variables from .env

async def connect():
    async with websockets.connect(f"wss://{env.get('BASE_URL')}/stream?token={env.get('TOKEN')}") as websocket:
        while True:
            message = await websocket.recv()
            print(f"{message = }")

asyncio.run(connect())
