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
  mesa,
  stdenv,
  makeWrapper,
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
  driverIcdPath = "${mesa}/share/vulkan/icd.d";
  icdArch =
    if stdenv.hostPlatform.system == "x86_64-linux"
    then "x86_64"
    else if stdenv.hostPlatform.system == "aarch64-linux"
    then "aarch64"
    else "x86_64";
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
        "oxiced-0.5.1" = "sha256-pjRHbeuQrbN66AAdpZyhOZ5+xr/XssYgk/DLRR0vCk0=";
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
      makeWrapper
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
      wrapProgram "$out/bin/oxishut" \
        --set VK_ICD_FILENAMES "${driverIcdPath}/radeon_icd.${icdArch}.json:${driverIcdPath}/intel_icd.${icdArch}.json"
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
