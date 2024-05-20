{ rustPlatform
, rust-bin
, pkg-config
, wrapGAppsHook4
, gtk4
, gtk4-layer-shell
, libadwaita
, lib
, lockFile
, ...
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = ../.;

  buildInputs = [
    pkg-config
    gtk4
    libadwaita
    gtk4-layer-shell
  ];

  cargoLock = {
    inherit lockFile;
  };

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
    # (rust-bin.selectLatestNightlyWith
    # (toolchain: toolchain.default))
    rust-bin.nightly."2024-05-10".default
  ];
  copyLibs = true;

  meta = with lib; {
    description = "A small, simple calculator written in rust/gtk4";
    homepage = "https://github.com/DashieTM/OxiShut";
    changelog = "https://github.com/DashieTM/OxiShut/releases/tag/${version}";
    license = licenses.gpl3;
    maintainers = with maintainers; [ DashieTM ];
    mainProgram = "oxishut";
  };
}
