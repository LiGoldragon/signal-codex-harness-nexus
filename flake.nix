{
  description = "Codex Harness reports and subscription vocabulary.";
  inputs.base.url = "github:LiGoldragon/build-base";
  inputs.ethos.url = "github:LiGoldragon/ethos-zero";
  outputs = { self, base, ethos }: base.lib.workspaceOutputs {
    cargoNix = ./Cargo.nix;
    extraChecks = { pkgs, toolchain, ... }: {
      fmt = pkgs.runCommand "signal-codex-harness-nexus-fmt" { nativeBuildInputs = [ toolchain ]; } ''
        rustfmt --edition 2024 --check ${./src}/lib.rs ${./src}/following.rs ${./src}/hook.rs
        touch "$out"
      '';
      generated = pkgs.runCommand "signal-codex-harness-nexus-generated" {
        nativeBuildInputs = [ ethos.packages.${pkgs.stdenv.hostPlatform.system}.ethos-zero ];
      } ''
        ethos-zero "Generate.{ ${./ethos}/signal.ethos $PWD/generated }"
        diff -u ${./src/generated/signal.rs} generated/signal.rs
        touch "$out"
      '';
    };
    memberChecks = { check, ... }: {
      signal-codex-harness-nexus = check "signal-codex-harness-nexus" [ "datom" ];
    };
  };
}
