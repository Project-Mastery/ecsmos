{
  description = "Nixos config flake";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
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
        udev alsa-lib vulkan-loader
        xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
        # libxkbcommon wayland # To use the wayland feature
      ];
    };
  };
}