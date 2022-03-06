FROM python:3.10-slim as base

WORKDIR /app

RUN pip install poetry --no-cache-dir
COPY pyproject.toml poetry.lock* ./

EXPOSE 2000

RUN echo "Starting backend" \
    && poetry export --dev -f requirements.txt --output requirements.txt --without-hashes \
    && pip install --no-cache-dir -r requirements.txt

COPY . .

CMD uvicorn app.main:app --host 0.0.0.0 --port 2000 --reload
