{
  rustPlatform,
  pkg-config,
  libGL,
  libxkbcommon,
  wayland,
  libclang,
  cargo,
  cargo-watch,
  rustc,
  rust-analyzer,
  clippy,
  lib,
  lockFile,
  vulkan-loader,
  wayland-protocols,
  libX11,
  libXrandr,
  libXi,
  libXcursor,
  ...
}: let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  libPath = lib.makeLibraryPath [
    libGL
    libxkbcommon
    wayland
    pkg-config
    libclang
  ];
in
  rustPlatform.buildRustPackage rec {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ../.;

    buildInputs = [
      pkg-config
      libGL
      libxkbcommon
      wayland
      libclang
    ];

    cargoLock = {
      inherit lockFile;
      outputHashes = {
        "cryoglyph-0.1.0" = "sha256-Jc+rhzd5BIT7aYBtIfsBFFKkGChdEYhDHdYGiv4KE+c=";
        "dpi-0.1.1" = "sha256-hlVhlQ8MmIbNFNr6BM4edKdZbe+ixnPpKm819zauFLQ=";
        "iced-0.14.0-dev" = "sha256-ToInrksjWeUj7yKF4I7/GOD883abHX6WrmADCZrOa80=";
        "iced_exdevtools-0.14.0-dev" = "sha256-1ncfSYSeHUl59cGchpbXyAh/IB6Mxse6D3P5CLRh9kE=";
        "oxiced-0.5.1" = "sha256-U8gYs3Xzvso0QdDapOTgR3sPPMDjdPc7jwbI32o3TyE=";
      };
    };

    nativeBuildInputs = [
      pkg-config
      wayland
      cargo
      cargo-watch
      rustc
      rust-analyzer
      clippy
      libGL
      libxkbcommon
      libclang
    ];

    copyLibs = true;
    LD_LIBRARY_PATH = libPath;
    LIBCLANG_PATH = "${libclang.lib}/lib";

    postFixup = let
      libPath = lib.makeLibraryPath [
        libGL
        vulkan-loader
        wayland
        wayland-protocols
        libxkbcommon
        libX11
        libXrandr
        libXi
        libXcursor
      ];
    in ''
      patchelf --set-rpath "${libPath}" "$out/bin/oxishut"
    '';

    postInstall = ''
      install -D --mode=444 $src/assets/shutdown.svg $out/share/pixmaps/oxishut/shutdown.svg
      install -D --mode=444 $src/assets/reboot.svg $out/share/pixmaps/oxishut/reboot.svg
      install -D --mode=444 $src/assets/sleep.svg $out/share/pixmaps/oxishut/sleep.svg
    '';

    meta = with lib; {
      description = "A simple iced logout prompt";
      homepage = "https://github.com/Xetibo/OxiShut";
      changelog = "https://github.com/Xetibo/OxiShut/releases/tag/${version}";
      license = licenses.gpl3;
      maintainers = with maintainers; [DashieTM];
      mainProgram = "oxishut";
    };
  }
