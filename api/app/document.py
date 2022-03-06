from beanie import Document
from beanie import Indexed
from motor.motor_asyncio import AsyncIOMotorClient
from pydantic import BaseModel


class UserBase(BaseModel):
    name: str
    phone: str
    description: str | None = None


class UserCreate(UserBase):
    ...


class User(Document, UserBase):
    ...


client = AsyncIOMotorClient("mongodb://root:example@mongo:27017")
