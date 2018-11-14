param (
  [alias("n")]
  [switch]$noAuth = $false,
  [alias("h")]
  [switch]$help = $false,
  [alias("b")]
  [string]$build,
  [alias("d")]
  [string]$deploy,
  [alias("r")]
  [string[]]$restart
)

if ($help -eq $false -and $noAuth -eq $false -and !$restart -and !$build -and !$deploy) {
  Write-Output "invalid command line argument! `n"
  Invoke-Expression "cargo script deploy.rs -- --help"
  exit
}

if ($help) {
  Invoke-Expression "cargo script deploy.rs -- --help"
  exit
}

if ($restart) {
  Invoke-Expression "cargo script deploy.rs -- --restart $restart"
  exit
}

if ($deploy) {
  Invoke-Expression "cargo script deploy.rs -- func --deploy $deploy"
  exit
}

if ($build) {
  Invoke-Expression "cargo script deploy.rs -- func --build $build"
  exit
}

if ($noAuth) {
  Invoke-Expression "cargo script deploy.rs -- --no-auth"
}
