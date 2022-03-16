import random
import string
import httpx
from .config import get_settings


def generate_token():
    return "".join(
        random.choice(string.ascii_uppercase + string.ascii_lowercase + string.digits)
        for _ in range(4)
    )


async def fatsms_send_sms(message: str, to_phone: str):
    url = get_settings().fat_sms_url + "/send-sms"
    api_key = get_settings().fatsms_key

    async with httpx.AsyncClient() as client:
        return await client.post(url, data={"message": message, "api_key": api_key, "to_phone": to_phone})
