$TARGET_FILE=$args[0]

if (-Not (Test-Path "$TARGET_FILE")) {
  Write-Output "Env file not found"
  exit 1
}

Write-Output "Loading dotenv file"

Get-Content "$TARGET_FILE" | ForEach-Object {
  $NAME, $VALUE = $_.split('=')

  if ([string]::IsNullOrWhiteSpace($NAME) -and $NAME.Contains('#')) {
    continue
  }

  Write-Output "Setting $NAME = $VALUE"
  Set-Item "env:$NAME" "$VALUE"
}
