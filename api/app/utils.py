import random
import string
import httpx
import smtplib, ssl, random
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
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


def send_email(receiver_email: str, token: str, name: str):
    sender_email = "hannepoelse@gmail.com"
    password = get_settings().email_password

    message = MIMEMultipart("alternative")
    message["Subject"] = "Mandatory1 Signup"
    message["From"] = sender_email
    message["To"] = receiver_email

    # Create the plain-text and HTML version of your message
    text = f"""\
    Hello {name},
    Thank you for signing up!
    """

    html = f"""\
    <html>
      <body>
        <p>
          <b>Hello {name},</b>
          <b>thank you for signing up!</b><br>
          <b>Your password is: {token}</b><br>
        </p>
      </body>
    </html>
    """

    # Turn these into plain/html MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    # Create secure connection with server and send email
    context = ssl.create_default_context()
    with smtplib.SMTP_SSL("smtp.gmail.com", 465, context=context) as server:
        try:
            server.login(sender_email, password)
            server.sendmail(sender_email, receiver_email, message.as_string())
        except Exception as ex:
            print(ex)
