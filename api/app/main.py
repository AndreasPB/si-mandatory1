from fastapi import FastAPI, Form
from fastapi.exceptions import HTTPException
from fastapi.middleware.cors import CORSMiddleware
from beanie import init_beanie
from .document import client, User, UserLogin, UserRegister, UserBase
from pymongo.errors import DuplicateKeyError
from .utils import generate_token
import jwt


app = FastAPI()

secret = "6horse9"

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


@app.post("/login")
async def sign_in(phone: str = Form(...), password: str = Form(...)):
    """Sign in for users"""
    if user := await User.find_one(User.phone == phone):
        if user.password == password:
            base_user = UserBase(**user.dict())
            return jwt.encode(base_user.dict(), secret, algorithm="HS256")
    raise HTTPException(status_code=403, detail="Phone or password incorrect")


@app.post("/send_message")
def send_message(token: str = Form(...), message: str = Form(...), to_phone: str = Form(...)):
    """Sends a message"""
    try:
        if decoded := jwt.decode(token, secret, algorithms=["HS256"]):
            return decoded
    except jwt.exceptions.DecodeError:
        raise HTTPException(status_code=403, detail="Unautherized")


@app.post("/user", status_code=201)
async def post_user(phone: str = Form(...), name: str = Form(...)):
    """Posts a user"""
    password = generate_token()
    try:
        user = User(phone=phone, password=password, name=name)
        await user.save()
        # TODO: Send SMS
    except DuplicateKeyError as e:
        print(e)
        raise HTTPException(status_code=409, detail="User already exists")
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
