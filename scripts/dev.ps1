# Rowan Web 开发环境启动脚本

Write-Host "🚀 启动 Rowan Web 开发环境..." -ForegroundColor Green

# 检查是否在正确的目录
if (!(Test-Path "backend\Cargo.toml") -or !(Test-Path "frontend\package.json")) {
    Write-Host "❌ 请在项目根目录运行此脚本！" -ForegroundColor Red
    exit 1
}

# 检查 Rust 是否安装
try {
    $rustVersion = cargo --version
    Write-Host "✅ 检测到 Rust: $rustVersion" -ForegroundColor Green
}
catch {
    Write-Host "❌ 未检测到 Rust，请先安装 Rust:" -ForegroundColor Red
    Write-Host "   https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# 检查 Node.js 是否安装
try {
    $nodeVersion = node --version
    Write-Host "✅ 检测到 Node.js: $nodeVersion" -ForegroundColor Green
}
catch {
    Write-Host "❌ 未检测到 Node.js，请先安装 Node.js:" -ForegroundColor Red
    Write-Host "   https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# 检查 pnpm 是否安装
try {
    $pnpmVersion = pnpm --version
    Write-Host "✅ 检测到 pnpm: v$pnpmVersion" -ForegroundColor Green
}
catch {
    Write-Host "⚠️  未检测到 pnpm，正在安装..." -ForegroundColor Yellow
    npm install -g pnpm
}

Write-Host ""
Write-Host "📦 安装依赖..." -ForegroundColor Blue

# 安装前端依赖
Write-Host "安装前端依赖..." -ForegroundColor Blue
Set-Location frontend
pnpm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 前端依赖安装失败！" -ForegroundColor Red
    exit 1
}
Set-Location ..

# 构建后端（首次运行）
Write-Host "构建后端..." -ForegroundColor Blue
Set-Location backend
cargo check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 后端构建失败！" -ForegroundColor Red
    exit 1
}
Set-Location ..

Write-Host ""
Write-Host "🎉 环境准备完成！" -ForegroundColor Green
Write-Host ""
Write-Host "启动开发服务器..." -ForegroundColor Blue
Write-Host "后端: http://localhost:8000" -ForegroundColor Cyan
Write-Host "前端: http://localhost:3000" -ForegroundColor Cyan
Write-Host ""
Write-Host "按 Ctrl+C 停止服务..." -ForegroundColor Yellow
Write-Host ""

# 同时启动后端和前端
$backend = Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd backend; cargo run" -WindowStyle Normal -PassThru
$frontend = Start-Process -FilePath "powershell" -ArgumentList "-Command", "cd frontend; pnpm dev" -WindowStyle Normal -PassThru

try {
    # 等待用户中断
    Write-Host "开发服务器正在运行，按任意键停止..." -ForegroundColor Green
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}
finally {
    # 清理进程
    Write-Host ""
    Write-Host "正在停止开发服务器..." -ForegroundColor Yellow
    Stop-Process -Id $backend.Id -Force -ErrorAction SilentlyContinue
    Stop-Process -Id $frontend.Id -Force -ErrorAction SilentlyContinue
    Write-Host "开发服务器已停止。" -ForegroundColor Green
}
