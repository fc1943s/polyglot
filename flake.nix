{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
  outputs = { self, nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
        inherit system;
        config = { allowUnfree = true; };
    };
    cupy = with pkgs.python311Packages; buildPythonPackage rec {
        pname = "cupy_cuda12x";
        version = "13.1.0";
        format = "wheel";
        src = pkgs.fetchurl {
            url = "https://files.pythonhosted.org/packages/46/ed/c8cb2faaea91376faf3effd061abdc1735afbcf0c558516de5128fa8039f/cupy_cuda12x-13.1.0-cp311-cp311-manylinux2014_x86_64.whl";
            sha256 = "e7a25ef1b44ae6276b5105affc2289edb34f1aa6676babd5bcd80907348c4cfa";
        };
    };
    pythonEnv = pkgs.python311.withPackages (ps: [
        pkgs.python311Packages.mpmath
        pkgs.python311Packages.nbconvert
        pkgs.python311Packages.numpy
        pkgs.python311Packages.fastrlock
        cupy
        pkgs.gcc
    ]);
  in {
    packages.${system} = { default = pythonEnv; };
    devShells.${system} = {
      default = pkgs.mkShell {
        buildInputs = [ pythonEnv pkgs.stdenv.cc.cc.lib ];
      };
    };
  };
}
