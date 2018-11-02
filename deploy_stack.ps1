#http://jongurgul.com/blog/get-stringhash-get-filehash/
Function Get-StringHash([String] $HashString, $HashName = "SHA256")
{
  $StringBuilder = New-Object System.Text.StringBuilder

  [System.Security.Cryptography.HashAlgorithm]::Create($HashName).ComputeHash([System.Text.Encoding]::UTF8.GetBytes($HashString)) | ForEach-Object {
    [Void]$StringBuilder.Append($_.ToString("x2"))
  }

  $StringBuilder.ToString()
}

if (-Not (Get-Command docker -errorAction SilentlyContinue))
{
  Write-Host "Unable to find docker command, please install Docker (https://www.docker.com/) and retry"
  exit 1
}

if (-Not (Get-Command faas-cli -errorAction SilentlyContinue))
{
  Write-Host "Installing faas-cli"
  choco install faas-cli -y
}

docker swarm init

$env:BASIC_AUTH="false"

Write-Host "Attempting to create credentials for gateway.."
Write-Output "admin" | docker secret create basic-auth-user -
Get-StringHash Get-Random | docker secret create basic-auth-password -

Write-Host "Deploying OpenFaaS core services"

mkdir faas -ErrorAction SilentlyContinue | Out-Null
Copy-Item deploy.yml .\faas\deploy.yml

mkdir .\faas\prometheus -ErrorAction SilentlyContinue | Out-Null

Invoke-RestMethod https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alertmanager.yml | Out-File .\faas\prometheus\alertmanager.yml
Invoke-RestMethod https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alert.rules.yml | Out-File .\faas\prometheus\alert.rules.yml
Invoke-RestMethod https://raw.githubusercontent.com/openfaas/faas/master/prometheus/prometheus.yml | Out-File .\faas\prometheus\prometheus.yml

docker stack deploy func --compose-file .\faas\deploy.yml
