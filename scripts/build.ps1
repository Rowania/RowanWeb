# Rowan Web 生产环境构建脚本

Write-Host "🏗️  构建 Rowan Web 生产版本..." -ForegroundColor Green

# 检查是否在正确的目录
if (!(Test-Path "backend\Cargo.toml") -or !(Test-Path "frontend\package.json")) {
    Write-Host "❌ 请在项目根目录运行此脚本！" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "📦 构建后端..." -ForegroundColor Blue
Set-Location backend
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 后端构建失败！" -ForegroundColor Red
    exit 1
}
Set-Location ..

Write-Host ""
Write-Host "📦 构建前端..." -ForegroundColor Blue
Set-Location frontend
pnpm install --frozen-lockfile
pnpm build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 前端构建失败！" -ForegroundColor Red
    exit 1
}
Set-Location ..

Write-Host ""
Write-Host "✅ 构建完成！" -ForegroundColor Green
Write-Host ""
Write-Host "📁 构建产物位置:" -ForegroundColor Blue
Write-Host "  后端: backend\target\release\rowan-web-backend.exe" -ForegroundColor Cyan
Write-Host "  前端: frontend\.next\" -ForegroundColor Cyan
Write-Host ""
Write-Host "🚀 运行生产版本:" -ForegroundColor Blue
Write-Host "  后端: .\backend\target\release\rowan-web-backend.exe" -ForegroundColor Cyan
Write-Host "  前端: cd frontend && pnpm start" -ForegroundColor Cyan
