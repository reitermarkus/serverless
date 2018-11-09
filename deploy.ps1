param (
  [alias("n")]
  [switch]$noAuth = $false,
  [alias("h")]
  [switch]$help = $false,
  [alias("r")]
  [string[]]$restart
)

if ($help -eq $false -and $noAuth -eq $false -and !$restart) {
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

if ($noAuth) {
  Invoke-Expression "cargo script deploy.rs -- --no-auth"
}
