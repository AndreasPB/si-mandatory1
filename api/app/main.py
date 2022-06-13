from datetime import datetime, timezone, timedelta
from uuid import uuid4
from fastapi import FastAPI, Form, Header
from fastapi.exceptions import HTTPException
from fastapi.middleware.cors import CORSMiddleware
from beanie import init_beanie
from fastapi.param_functions import Depends
import httpx
from pydantic.main import BaseModel
from .document import client, User, UserBase
from pymongo.errors import DuplicateKeyError
from .utils import fatsms_send_sms, generate_token, send_email
from .config import get_settings
import jwt


app = FastAPI()

secret = get_settings().jwt_secret
mitid_secret = get_settings().mitid_secret


app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


class Message(BaseModel):
    id: str = str(uuid4())
    content: str

async def verify_auth(auth: str = Header(...)):
    try:
        if jwt.decode(auth, secret, algorithms=["HS256"]):
            return auth

    except jwt.exceptions.DecodeError as e:
        print(e)

    raise HTTPException(status_code=403, detail="Unauthorized")



async def verify_mitid(auth: str = Header(...)):
    try:
        if jwt.decode(auth, mitid_secret, algorithms=["HS256"]):
            return auth

    except jwt.exceptions.DecodeError as e:
        print(e)

    except jwt.exceptions.ExpiredSignatureError as e:
        raise HTTPException(status_code=419, detail="Your session has expired")


    raise HTTPException(status_code=403, detail="Unauthorized")

@app.on_event("startup")
async def init_db():
    await init_beanie(database=client.db_name, document_models=[User])


@app.post("/create-message", dependencies=[Depends(verify_auth)], status_code=201)
async def create_message(content: str = Form(...), topic: str = Form(...), auth: str = Header(...)):
    """Creates a message by calling ESB"""
    with httpx.Client() as httpx_client:
        res = httpx_client.post(f"http://go_esb:9999/create-message?topic={topic}", headers={"auth": auth},
                                json=Message(content=content).dict())
        return res.json()


@app.post("/login")
async def sign_in(phone: str = Form(...), password: str = Form(...)):
    """Sign in for users"""
    if user := await User.find_one(User.phone == phone):
        if user.password == password:
            base_user = (UserBase(**user.dict())).dict()
            base_user["exp"] = datetime.now(tz=timezone.utc) + timedelta(hours=1)

            return jwt.encode(base_user,  secret, algorithm="HS256")
    raise HTTPException(status_code=403, detail="Phone or password incorrect")


@app.post("/send_message", dependencies=[Depends(verify_auth)])
async def send_message(token: str = Form(...), message: str = Form(...), to_phone: str = Form(...)):
    """Sends a message"""
    try:
        if jwt.decode(token, secret, algorithms=["HS256"]):
            res = await fatsms_send_sms(message=message, to_phone=to_phone)
            if res.status_code == 200:
                return res.json()
            else:
                raise HTTPException(status_code=res.status_code, detail=res.json()['info'])
    except jwt.exceptions.DecodeError:
        raise HTTPException(status_code=403, detail="Unauthorized")


@app.post("/user", dependencies=[Depends(verify_mitid)], status_code=201)
async def register_user(phone: str = Form(...), name: str = Form(...), email: str = Form(...)):
    """Posts a user"""
    password = generate_token()
    try:
        user = User(phone=phone, password=password, name=name, email=email)
        await user.save()
        send_email(email, password, name)
        # res = await fatsms_send_sms(message=password, to_phone=phone)
        # if res.status_code != 200:
        #     raise HTTPException(status_code=res.status_code, detail=res.json()['info'])
        # return res.json()
    except DuplicateKeyError as e:
        print(e)
        raise HTTPException(status_code=409, detail="User already exists")


@app.get("/user", dependencies=[Depends(verify_mitid)])
async def get_users():
    """Finds all users"""
    users = await User.find().to_list()
    return list(map(dict, users))


@app.get("/user/{name}", dependencies=[Depends(verify_mitid)])
async def get_user_by_name(name: str):
    """Finds the user by name"""
    return await User.find_one(User.name == name)


@app.get("/read-messages/{topic}", dependencies=[Depends(verify_auth)], status_code=200)
async def read_messages(topic: str, auth: str = Header(...)):
    """Reads all messages from a given topic by calling ESB"""
    res = httpx.get(f"http://go_esb:9999/topic/{topic}/skip/0/limit/100/format/JSON", headers={"auth": auth})
    return res.json()


@app.get("/")
async def root():
    return {"message": "Hello World"}
