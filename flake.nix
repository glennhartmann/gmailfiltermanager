{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-compat, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        gmailfiltermanager = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = builtins.path { path = ./.; name = "gmailfiltermanager"; };

          cargoLock = {
            lockFile = ./Cargo.lock;

            outputHashes = {
              "oauth2-0.0.5" = "sha256-tTZ7hO51BSAOi+9nAtuC2wY9i+wUdi+mkBgxwt7x+kg=";
              "oauth2-simple-0.0.5" = "sha256-jS0xfhmBNbuJ/4pkpGECqeXSmFUd4R8c9Wlxn1rbygw=";
            };
          };
        };
        gmailfiltermanager-shell = pkgs.mkShell {
          inputsFrom = [ gmailfiltermanager ];
          packages = with pkgs; [
            clippy
            rustfmt
          ];
        };
      in
      {
        packages = {
          inherit gmailfiltermanager;
          default = gmailfiltermanager;
        };
        devShells = {
          inherit gmailfiltermanager-shell;
          default = gmailfiltermanager-shell;
        };
      }
    );
}
