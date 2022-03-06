from fastapi import FastAPI
from beanie import init_beanie
from .document import client, User, UserCreate

app = FastAPI()

@app.on_event("startup")
async def init_db():
    await init_beanie(database=client.db_name, document_models=[User])


@app.post("/add-user")
async def _(user: UserCreate):
    await User(**user.dict()).save()
    return user


@app.get("/find-user/{name}")
async def _(name: str):
    return await User.find_one(User.name == name)


@app.get("/")
async def root():
    return {"message": "Hello World"}
