# 在正确的 MSVC 环境里运行 cargo 命令。
# 用法: powershell -File scripts\build.ps1 build
#       powershell -File scripts\build.ps1 run -p od-preview --example hello_window
#
# 为什么需要它：Git Bash 把自己的 link.exe 放到 PATH 前面，会冒充 MSVC 链接器，
# 导致 Rust 链接失败（报 "link: extra operand"）。本脚本先加载 VS 的 vcvarsall.bat，
# 让真正的 MSVC link.exe / Windows SDK 走到 PATH 前面，再执行 cargo。

$ErrorActionPreference = 'Stop'

# 先记住 cargo 的位置——vcvarsall.bat 会重置 PATH，把 ~/.cargo/bin 挤掉。
# 兼容贫瘠的调用上下文：优先用传入的 CARGO_HOME，其次 C:\Users\<whoami>。
$CargoHome = $Env:CARGO_HOME
if (-not $CargoHome) {
    $who = (whoami).Split('\')[-1]
    $CargoHome = "C:\Users\$who\.cargo"
}
$CargoExe = Join-Path $CargoHome 'bin\cargo.exe'

$Vswhere = 'C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe'
$VsRoot  = & $Vswhere -latest -products * -property installationPath
if (-not $VsRoot) { throw 'Visual Studio / Build Tools not found.' }
$Vcvars = Join-Path $VsRoot 'VC\Auxiliary\Build\vcvarsall.bat'
if (-not (Test-Path $Vcvars)) { throw "vcvarsall.bat not found at $Vcvars" }

# vcvarsall.bat 设的是当前 cmd 进程的环境，没法直接传给 PowerShell。
# 用 cmd /c 把环境变量导出成文本，再在 PowerShell 里逐行恢复。
$envDump = & cmd /c "`"$Vcvars`" x64 >nul 2>&1 && set" 2>&1
foreach ($line in $envDump) {
    if ($line -match '^([^=]+)=(.*)$') {
        Set-Item -Path "Env:$($Matches[1])" -Value $Matches[2]
    }
}

Write-Host "[build.ps1] MSVC env loaded. running: cargo $args" -ForegroundColor DarkGray
if (-not (Test-Path $CargoExe)) { throw "cargo not found at $CargoExe" }
# 把 cargo 所在目录前置到 PATH，避免被 vcvarsall 重置后找不到。
$Env:PATH = "$(Split-Path $CargoExe);$Env:PATH"
& $CargoExe @args
$exit = $LASTEXITCODE
exit $exit
