{
  description = "Nixos config flake";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in 
  {
    devShells.${system}.default= pkgs.mkShell
    {
      nativeBuildInputs = with pkgs; [
        cargo 
        rustc
        rustfmt
        pre-commit
        rustPackages.clippy
        alsa-lib 
        udev 
        vulkan-loader
        xorg.libX11
        xorg.libXrandr
        xorg.libXcursor
        xorg.libXi 
        pkg-config
        libxkbcommon
        wayland
      ];

      LD_LIBRARY_PATH = with pkgs;  pkgs.lib.makeLibraryPath [
        vulkan-loader 
        libGL
        libxkbcommon
        wayland
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
	    ];

      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      RUST_BACKTRACE = 1;
    };
  };
}