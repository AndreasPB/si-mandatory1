from enum import unique
from beanie import Document
from beanie import Indexed
from motor.motor_asyncio import AsyncIOMotorClient
from pydantic import BaseModel


class UserRegister(BaseModel):
    name: str
    phone: str


class UserLogin(BaseModel):
    phone: str
    password: str


class UserBase(BaseModel):
    phone: str
    name: str
    password: str


class Message(BaseModel):
    message: str
    to_phone: str
    jwt: str


# Beanie model
class User(Document):
    phone: Indexed(str, unique=True)
    name: str
    password: str
    email: str

client = AsyncIOMotorClient("mongodb://root:example@mongo:27017")
