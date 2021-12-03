$ProjectRoot = $PSScriptRoot

$Programs = 
    @{"name" = "day01_sonar_sweep_part1"; "output" = "1532"},
    @{"name" = "day01_sonar_sweep_part2"; "output" = "1571"}

for ($i = 0; $i -lt $Programs.Length; $i++) {
    $program = $Programs[$i]
    $name = $program["name"]
    $expectedOutput = $program["output"]

    $currentOperation = "Testing $name"
    $percent = $i / $Programs.Length * 100
    $status = "Program $($i + 1) of $($Programs.Length)"
    Write-Progress -Id 0 -Activity " " -CurrentOperation $currentOperation -PercentComplete $percent -Status $status

    Push-Location (Join-Path $ProjectRoot $name)
    Write-Host "`u{1F9F1} Building $name"
    & cargo build --release
    if (-not $?) {
        throw "Error building $name"
    }
    $binary_name = "$($name).exe"
    $binary_path = Join-Path ".\target\release" $binary_name
    Write-Host "`u{1F3C3} Running $name"
    $output = & $binary_path
    if ($output -ne $expectedOutput) {
        throw "Expected $expectedOutput, got $output"
    }
    Write-Host "✔️ $name passed!" -ForegroundColor Green
    Pop-Location
}

Write-Progress -Id 0 -Activity " " -PercentComplete 100
Write-Progress -Id 0 -Activity " " -Completed
