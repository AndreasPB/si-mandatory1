from enum import Enum
from pydantic import BaseSettings
from functools import lru_cache
from pathlib import Path


class Environment(str, Enum):
    DEVELOPMENT = "dev"
    TESTING = "test"
    PRODUCTION = "prod"


class Settings(BaseSettings):
    """
    These settings can be overwritten by environment variables
    The environement variable name is the upper-cased version of the variable name below
    E.g. FATSMS_KEY == fatsms_key
    """
    app_environment: Environment = Environment.DEVELOPMENT
    fatsms_key: str = "no_key_present"
    jwt_secret: str = "secret"
    fat_sms_url: str = "https://fatsms.com"
    email_password: str = "Weebenmikael"


@lru_cache
def get_settings() -> Settings:
    return Settings()
