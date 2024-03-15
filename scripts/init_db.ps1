# Enable detailed error messages and stop on errors
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

# Check if a custom user has been set, otherwise default to 'postgres'
$DB_USER = if ($null -ne $env:POSTGRES_USER) { $env:POSTGRES_USER } else { "postgres" }

# Check if a custom password has been set, otherwise default to 'password'
$DB_PASSWORD = if ($null -ne $env:POSTGRES_PASSWORD) { $env:POSTGRES_PASSWORD } else { "password" }

# Check if a custom database name has been set, otherwise default to 'newsletter'
$DB_NAME = if ($null -ne $env:POSTGRES_DB) { $env:POSTGRES_DB } else { "newsletter" }

# Check if a custom port has been set, otherwise default to '5432'
$DB_PORT = if ($null -ne $env:POSTGRES_PORT) { $env:POSTGRES_PORT } else { "5432" }

$DB_HOST = if ($null -ne $env:POSTGRES_HOST) { $env:POSTGRES_HOST } else { "localhost" }

if (-not [System.Environment]::GetEnvironmentVariable('SKIP_DOCKER')) {
# Launch postgres using Docker
docker run `
    --name newsletter_container `
    -e POSTGRES_USER=$DB_USER `
    -e POSTGRES_PASSWORD=$DB_PASSWORD `
    -e POSTGRES_DB=$DB_NAME `
    -p "${DB_PORT}:5432" `
    -d postgres `
    postgres -N 10000

$env:SKIP_DOCKER = "true"
}

Write-Host "PostgreSQL container named newsletter_container is up and running."

# Keep pinging Postgres until it's ready to accept commands
do {
    try {
        $output = psql -h $DB_HOST -U $DB_USER -p $DB_PORT -d "postgres" -c '\q'
        $isReady = "true"
    } catch {
        Write-Host "Postgres is still unavailable - sleeping"
        Start-Sleep -Seconds 1
        $isReady = "false"
    }
} while (-not $isReady)

Write-Host "Postgres is up and running on port $DB_PORT!"

$env:DATABASE_URL = "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run