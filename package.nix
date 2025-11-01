{
  pkgs,
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "message-client";
  version = "0.1.0";

  src = ./.;

  buildInputs = with pkgs; [ openssl ];
  nativeBuildInputs = with pkgs; [ pkgconf ];

  cargoLock = {
    lockfile = ./Cargo.lock;
  };
}
