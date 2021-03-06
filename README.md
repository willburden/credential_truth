# credential_truth
*(A.K.A. docker-credential-truth)*

credential_truth is a package that produces a program, docker-credential-truth,
which can act as a [Docker Credential Helper]. It is intended to be
used only with Linux, as it depends on the [pass] command-line
utility for storing secrets.

This program is intended as a replacement for the somewhat broken
[docker-credential-pass] credential helper provided by Docker, which just
doesn't seem to work in some cases.

credential_truth aims to be very unambiguous about where your secrets will be
stored: at `$CREDENTIAL_TRUTH`, or `~/.docker-credential-truth/` if that variable
isn't set.

This is a separate store to your normal `pass` store, and in fact you can perfectly
viably use pass in the normal way outside of your docker authentication needs
and it will never appear to interact or collide with credential_truth.

To help simplify things further, it also provides a utility for initialising
this authentication system on a per-user basis.

## Installation

To install, simply paste the following into the command-line:

```console
wget -q -O - https://raw.githubusercontent.com/willburden/credential_truth/main/install.sh | sudo bash
```

## Usage

To get set up, first **make sure** you're logged in as the user you wish use
Docker with (here I'm logging in as `root`). This step is essential.

```console
user@host:~$ sudo su
root@host:/home/user# 
```

Then, [generate or find an existing GPG key][getting a GPG key] and run the following:

```console
root@host:/home/user# docker-credential-truth init <GPG-Key-ID>
```

This will initialise your password store at
`$CREDENTIAL_TRUTH` if it exists, or `~/.docker-credential-truth/` otherwise.

You're now free to return to normal mode if you had been logged in as the 
superuser:

```console
root@host:/home/user# exit
exit
user@host:~$ 
```

And finally, you'll need to edit the file at `~/.docker/config.json` to inform Docker
that you want it to use `docker-credential-truth`:

```json
{
    "credsStore": "truth"
}
```

Now, when you login to Docker it should save your credentials properly,
allowing you to push and pull to your heart's content.

```console
user@host:~$ sudo docker login
Login with your Docker ID to push and pull images from Docker Hub. If you don't
have a Docker ID, head over to https://hub.docker.com to create one.
Username: username
Password: 
Login Succeeded
user@host:~$ 
```
```console
user@host:~$ sudo docker login
Authenticating with existing credentials...
Login Succeeded
user@host:~$ 
```
```console
user@host:~$ sudo docker logout
Removing login credentials for https://index.docker.io/v1/
user@host:~$
```

Happy days!

[Docker Credential Helper]: https://github.com/docker/docker-credential-helpers
[pass]: https://www.passwordstore.org/
[docker-credential-pass]: https://github.com/docker/docker-credential-helpers/releases/latest
[getting a GPG key]: https://docs.github.com/en/authentication/managing-commit-signature-verification/checking-for-existing-gpg-keys
