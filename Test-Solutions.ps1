$ProjectRoot = $PSScriptRoot

$Programs = 
    @{"name" = "day01_sonar_sweep_part1"; "output" = "1532"},
    @{"name" = "day01_sonar_sweep_part2"; "output" = "1571"},
    @{"name" = "day02_dive_part1"; "output" = "2187380"},
    @{"name" = "day02_dive_part2"; "output" = "2086357770"},
    @{"name" = "day03_binary_diagnostic_part1"; "output" = "1071734"},
    @{"name" = "day03_binary_diagnostic_part2"; "output" = "6124992"},
    @{"name" = "day04_giant_squid_part1"; "output" = "22680"},
    @{"name" = "day04_giant_squid_part2"; "output" = "16168"}

for ($i = 0; $i -lt $Programs.Length; $i++) {
    $program = $Programs[$i]
    $name = $program["name"]
    $expectedOutput = $program["output"]

    $currentOperation = "Testing $name"
    $percent = $i / $Programs.Length * 100
    $status = "$($i + 1)/$($Programs.Length) $name"
    Write-Progress -Id 0 -Activity "Test Solutions" -CurrentOperation $currentOperation -PercentComplete $percent -Status $status

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
