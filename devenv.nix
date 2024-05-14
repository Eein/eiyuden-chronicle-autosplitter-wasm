{ pkgs, ... }:

{
  packages = [ pkgs.git ];
  languages.rust = {
    enable = true;
    channel = "nightly";
    targets = [ "wasm32-unknown-unknown" ];
  };
}
