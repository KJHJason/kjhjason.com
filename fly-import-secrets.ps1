$envFile = Get-Content .env
$secretsCommand = "flyctl secrets set "

foreach ($line in $envFile) {
    $key, $value = $line -split '=', 2
    $secretsCommand += "$key=$value "
}

# Trim any trailing whitespace from the command string
$secretsCommand = $secretsCommand.Trim()

# Execute the combined command
Invoke-Expression $secretsCommand
