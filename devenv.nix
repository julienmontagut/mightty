{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.PROJECT_NAME = "mightty";

  # https://devenv.sh/packages/
  packages = [ 
    pkgs.git 
    pkgs.clang
    pkgs.llvm
    pkgs.pkg-config
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.greet.exec = ''
    Welcome to $PROJECT_NAME!
  '';

  enterShell = ''
    greet
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;
  git-hooks.hooks.rustfmt.enable = true;

  # See full reference at https://devenv.sh/reference/options/
  devcontainer = {
    enable = true;
    settings = {
      updateContentCommand = "direnv allow";
    };
  };
}
