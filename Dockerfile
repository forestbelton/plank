# Backend
FROM ubuntu:16.04

RUN apt-get -y update
RUN apt-get -yq install \
    curl \
    build-essential \
    libsqlite3-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH $PATH:/root/.cargo/bin

WORKDIR /app
COPY . .
RUN cargo build --release

# Frontend
FROM ubuntu:16.04

RUN apt-get -y update
RUN apt-get -yq install \
    curl

RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.2/install.sh | bash
RUN . $HOME/.nvm/nvm.sh \
    && nvm install stable \
    && npm install -g yarn

WORKDIR /app
COPY js/ .

ENV OUTPUT_DIRECTORY /app
RUN . $HOME/.nvm/nvm.sh \
    && nvm use stable \
    && yarn \
    && npm run build

# Application
FROM ubuntu:16.04

RUN apt-get -y update \
    && apt-get -yq install libsqlite3-0

WORKDIR /app
COPY --from=0 /app/target/release/plank .
COPY --from=0 /app/.env .
COPY --from=0 /app/static/ .
COPY --from=1 /app/bundle.js static/bundle.js

EXPOSE 3000
CMD ["/bin/sh", "-c", "./plank"]
