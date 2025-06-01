{ rustPlatform, src }:
rustPlatform.buildRustPackage {
  pname = "slicer";
  version = "0.1.0";

  inherit src;

  cargoHash = "sha256-rMYQDtqz6zpeGNlfPWNA8rNb5jSXmB0vTOqT2JlwlT0=";
}
