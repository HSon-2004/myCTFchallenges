FROM pwn.red/jail
COPY --from=ubuntu:22.04 / /srv

WORKDIR /srv/app

COPY map map
COPY chal run
RUN chmod +x run




