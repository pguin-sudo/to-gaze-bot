FROM python:3.13-slim-bullseye

WORKDIR /app
COPY pyproject.toml poetry.lock[t] ./

RUN python -m pip install --no-cache-dir poetry==1.8.4
RUN poetry config virtualenvs.create false 
RUN poetry install --without dev --no-interaction --no-ansi  
RUN rm -rf $(poetry config cache-dir)/{cache,artifacts}

RUN apt-get update && \
    apt-get install -y git

WORKDIR /app

COPY . ./

# CMD ["ls"]
CMD ["python", "-m", "to_gaze_bot"]
