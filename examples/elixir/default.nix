let
  nixos = import <nixos> { config = { allowUnfree = true; }; };
  nixos-unstable = import <nixos-unstable> { config = { allowUnfree = true; }; };

  mach-nix = import (builtins.fetchGit {
    url = "https://github.com/DavHau/mach-nix";
  }) {
    python = "python310";
  };

in
  mach-nix.nixpkgs.mkShell {
    buildInputs = [
      nixos-unstable.elixir
    ];

    shellHook = ''
      code
    '';
  }
