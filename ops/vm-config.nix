let pkgs = import <nixpkgs> { };
in pkgs.buildEnv {
  name = "vm-packages";
  paths = with pkgs; [ docker docker-compose git htop tree ];
}
