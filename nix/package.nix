{
  lib,
  rustPlatform,
  src,
}:
rustPlatform.buildRustPackage {
  pname = "slicer";
  version = "0.1.0";

  inherit src;

  cargoHash = "sha256-rMYQDtqz6zpeGNlfPWNA8rNb5jSXmB0vTOqT2JlwlT0=";

  meta = {
    description = "Run desktop applications as systemd services";
    homepage = "https://github.com/ivan770/slicer";
    platforms = lib.platforms.linux;
    license = lib.licenses.mit;
    mainProgram = "slicer";
  };
}
