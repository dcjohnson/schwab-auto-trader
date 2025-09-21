{ pkgs, naersk, src }:
let 
  naersk-lib = pkgs.callPackage naersk { };
in
  naersk-lib.buildPackage src

