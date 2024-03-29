FROM debian:latest

RUN apt update \
    && apt install -y curl gpg \
    && echo "deb https://download.opensuse.org/repositories/network:/messaging:/zeromq:/git-draft/Debian_10/ ./" >> /etc/apt/sources.list \
    && curl https://download.opensuse.org/repositories/network:/messaging:/zeromq:/git-draft/Debian_10/Release.key | apt-key add \
    && apt update \
    && apt install -y libzmq3-dev \
    && apt remove -y curl gpg \
    && apt autoremove -y \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m sawtooth

COPY --chown=sawtooth:sawtooth target/release/alica-messages-client /usr/bin

USER sawtooth
WORKDIR /home/sawtooth

CMD ["bash"]