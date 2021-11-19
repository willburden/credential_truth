FROM debian:bookworm

# Dependencies
RUN \
    apt-get update &&\
    apt-get -y install wget &&\
    apt-get -y install gpg &&\
    apt-get -y install pass

# Set-up
WORKDIR /home/
COPY /keys/ keys/
COPY /scripts .
RUN ["./setup.sh"]
ENV INSTALL_SCRIPT='https://raw.githubusercontent.com/willburden/credential_truth/v0.1/install.sh'

# Run-time
CMD ["./run.sh"]
