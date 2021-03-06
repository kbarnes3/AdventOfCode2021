$ProjectRoot = $PSScriptRoot

$Programs = 
    @{"name" = "day01_sonar_sweep_part1"; "output" = "1532"},
    @{"name" = "day01_sonar_sweep_part2"; "output" = "1571"},
    @{"name" = "day02_dive_part1"; "output" = "2187380"},
    @{"name" = "day02_dive_part2"; "output" = "2086357770"},
    @{"name" = "day03_binary_diagnostic_part1"; "output" = "1071734"},
    @{"name" = "day03_binary_diagnostic_part2"; "output" = "6124992"},
    @{"name" = "day04_giant_squid_part1"; "output" = "22680"},
    @{"name" = "day04_giant_squid_part2"; "output" = "16168"},
    @{"name" = "day05_hydrothermal_venture_part1"; "output" = "5690"},
    @{"name" = "day05_hydrothermal_venture_part2"; "output" = "17741"},
    @{"name" = "day06_lanternfish_part1"; "output" = "383160"},
    @{"name" = "day06_lanternfish_part2"; "output" = "1721148811504"},
    @{"name" = "day07_the_treachery_of_whales_part1"; "output" = "344535"},
    @{"name" = "day07_the_treachery_of_whales_part2"; "output" = "95581659"},
    @{"name" = "day08_seven_segment_search_part1"; "output" = "543"},
    @{"name" = "day08_seven_segment_search_part2"; "output" = "994266"},
    @{"name" = "day09_smoke_basin_part1"; "output" = "423"},
    @{"name" = "day09_smoke_basin_part2"; "output" = "1198704"},
    @{"name" = "day10_syntax_scoring_part1"; "output" = "315693"},
    @{"name" = "day10_syntax_scoring_part2"; "output" = "1870887234"}

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
    Write-Host "?????? $name passed!" -ForegroundColor Green
    Pop-Location
}

Write-Progress -Id 0 -Activity " " -PercentComplete 100
Write-Progress -Id 0 -Activity " " -Completed
