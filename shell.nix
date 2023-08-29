{ pkgs ? import <nixpkgs> {} }:
    pkgs.mkShell {
        nativeBuildInputs = with pkgs.buildPackages; [
            cargo
            gcc
            openssl
            pkg-config
            rustc
        ];
    }
