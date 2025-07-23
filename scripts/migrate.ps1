# Rowan Web 数据库迁移脚本

param(
    [string]$Action = "up",
    [string]$Name = ""
)

Write-Host "🗄️  Rowan Web 数据库迁移工具" -ForegroundColor Green

# 检查是否在正确的目录
if (!(Test-Path "backend\Cargo.toml")) {
    Write-Host "❌ 请在项目根目录运行此脚本！" -ForegroundColor Red
    exit 1
}

# 检查 sea-orm-cli 是否安装
try {
    $cliVersion = sea-orm-cli --version
    Write-Host "✅ 检测到 SeaORM CLI: $cliVersion" -ForegroundColor Green
}
catch {
    Write-Host "⚠️  未检测到 SeaORM CLI，正在安装..." -ForegroundColor Yellow
    cargo install sea-orm-cli
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ SeaORM CLI 安装失败！" -ForegroundColor Red
        exit 1
    }
}

Set-Location backend

switch ($Action.ToLower()) {
    "generate" {
        if ($Name -eq "") {
            Write-Host "❌ 请提供迁移名称: .\scripts\migrate.ps1 generate -Name 'migration_name'" -ForegroundColor Red
            exit 1
        }
        Write-Host "📝 创建新迁移: $Name" -ForegroundColor Blue
        sea-orm-cli migrate generate $Name
    }
    "up" {
        Write-Host "⬆️  运行迁移..." -ForegroundColor Blue
        sea-orm-cli migrate up
    }
    "down" {
        Write-Host "⬇️  回滚迁移..." -ForegroundColor Blue
        sea-orm-cli migrate down
    }
    "status" {
        Write-Host "📊 迁移状态..." -ForegroundColor Blue
        sea-orm-cli migrate status
    }
    "fresh" {
        Write-Host "🔄 重置数据库..." -ForegroundColor Blue
        sea-orm-cli migrate fresh
    }
    default {
        Write-Host "用法:" -ForegroundColor Blue
        Write-Host "  .\scripts\migrate.ps1 generate -Name 'migration_name'  # 创建新迁移" -ForegroundColor Cyan
        Write-Host "  .\scripts\migrate.ps1 up                              # 运行迁移" -ForegroundColor Cyan
        Write-Host "  .\scripts\migrate.ps1 down                            # 回滚迁移" -ForegroundColor Cyan
        Write-Host "  .\scripts\migrate.ps1 status                          # 查看迁移状态" -ForegroundColor Cyan
        Write-Host "  .\scripts\migrate.ps1 fresh                           # 重置数据库" -ForegroundColor Cyan
    }
}

Set-Location ..
