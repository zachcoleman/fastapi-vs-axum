from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()


@app.get("/hello_world")
async def hello_world_handler():
    return "Hello, World!"


# path parameters
@app.get("/{id}/{user}")
async def path_params_handler(id, user):
    return f"{id} - {user}"


class Payload(BaseModel):
    str1: str
    str2: str
    int1: int
    int2: int
    alphanumeric: str


# json parsing
@app.post("/json")
async def json_parsing_handler(payload: Payload):
    return payload.alphanumeric
