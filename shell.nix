# shell.nix - Compatibility wrapper for older Nix installations
(import (fetchTarball "https://github.com/edolstra/flake-compat/archive/master.tar.gz") {
  src = ./.;
}).shellNix

