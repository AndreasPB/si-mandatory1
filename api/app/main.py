from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from beanie import init_beanie
from .document import client, User, UserCreate

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.on_event("startup")
async def init_db():
    await init_beanie(database=client.db_name, document_models=[User])


@app.post("/user")
async def post_user(user: UserCreate):
    """Posts a user"""
    await User(**user.dict()).save()
    return user


@app.get("/user")
async def get_users():
    """Finds all users"""
    users = await User.find().to_list()
    return list(map(dict, users))


@app.get("/user/{name}")
async def get_user_by_name(name: str):
    """Finds the user by name"""
    return await User.find_one(User.name == name)


@app.get("/")
async def root():
    return {"message": "Hello World"}
