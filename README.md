# Credential_Truth
## (A.K.A. docker-credential-truth)

This is a package that produces a command-line utility which can act as a
[Docker Credential Helper]. It is intended to be used only with Linux,
as it depends on the [pass] command-line utility for storing secrets.

> Note: this package is still a pre-release; anything may change at
> any time. The API should not be considered stable.
> Furthermore, most of the functionality outlined below is
> not-yet-implemented.

This program is intended as a replacement for the somewhat broken
[docker-credential-pass] credential helper provided by Docker;
their program fails to access the right directories when run with `sudo`,
which is often required when working with Docker. (It also *still* doesn't work
even after you fix that...)

*credential_truth* aims to fix this by establishing an unambiguous source
of truth: secrets for each user are stored at
`$CREDENTIAL_TRUTH/<user>/.password-store`, or
`/var/docker-credential-truth/<user>/.password-store` if that variable isn't set.

This is a separate store to your normal `pass` store, and in fact you can perfectly
viably use pass in the normal way outside of your docker authentication needs
and it will never appear to interact or collide with *credential_truth*.

To help simplify things further, it also provides a utility for initialising
this authentication system on a per-user basis.

## Installation

After compilation, the resultant binary should be renamed to
`docker-credential-truth` and added to the path. This is so Docker accepts it
as a credential helper.

Also, remember you can set the `CREDENTIAL_TRUTH` environment variable
to change your secrets location. If you're going to do this, do it
**before** you run `init`, and be aware that this will mean your
secrets are no longer stored under the same structure as everyone else.

## Usage

To get set up, first **make sure** you're logged in as the user you wish to
authenticate (here I'm logging in as `root`). This step is essential.

```console
user@host:~$ sudo su
root@host:/home/user# 
```

Then, [generate or find an existing GPG key][getting a GPG key] and run the following:

```console
root@host:/home/user# docker-credential-truth init <GPG-Key-ID>
```

This will initialise your password store at
`$CREDENTIAL_TRUTH/<user>/.password-store`.

It will also edit the file at `~/.docker/config.json` to inform Docker
that you want it to use `docker-credential-truth`:

```json
{
    "credsStore": "truth"
}
```

You're now free to return to normal mode if you had been logged in as the 
superuser:

```console
root@host:/home/user# exit
exit
user@host:~$ 
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
