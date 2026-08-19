# gmailfiltermanager

A better way to manage your Gmail filters.

_Disclaimer: This is a personal project. The views, code, and opinions
expressed here are my own and do not represent those of my current or past
employers._

## Installing

```bash
cargo install \
  --git https://github.com/glennhartmann/gmailfiltermanager.git
```

Or use [nix](https://nixos.org/).

## Status

While this project _aspires_ to be a manager for your Gmail filters, it is in
extremely early stages. The current implementation just gets OAuth2 access
tokens via the [oauth2-simple
library](https://github.com/glennhartmann/oauth2/tree/2ed5e7ec3daa4991cf7a9f5e3d234b98f382b152/simple).

## Creating OAuth2 Credentials

`gmailfiltermanager` requires that you have a "client info" JSON file with your
client ID and client secret. See [create-client
README](https://github.com/glennhartmann/oauth2/blob/master/doc/create-client/README.md)
for how to set one up. By default, this is expected to be at
`~/.config/gmailfiltermanager/client_info.json`. Make sure to set the
`gmailfiltermanager` directory's permissions to `0700` and the
`client_info.json` file's permissions to `0600`. The file contains important
sensitive data, so treat it accordingly.
