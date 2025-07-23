# Rowan Web - 个人笔记分享平台

基于 **Rust + Next.js** 构建的现代化个人笔记分享平台。支持 Markdown 编辑、评论互动、点赞收藏等功能。

## 🚀 技术栈

### 后端 (Rust)
- **Web 框架**: Axum 0.7 (最新版本)
- **ORM**: Sea-ORM 1.1 (最新版本) 
- **数据库**: SQLite (开发) / PostgreSQL (生产)
- **认证**: JWT + Argon2 密码加密
- **日志**: tracing + tracing-subscriber
- **配置管理**: config + dotenvy

### 前端 (Next.js)
- **框架**: Next.js 15.1.8 (最新版本)
- **语言**: TypeScript 5.7
- **样式**: Tailwind CSS 3.4
- **状态管理**: SWR + React Hook Form
- **UI 组件**: Headless UI + Heroicons
- **HTTP 客户端**: Axios

## 📁 项目结构

```
RowanWeb/
├── backend/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs            # 应用入口点
│   │   ├── config.rs          # 配置管理
│   │   ├── error.rs           # 错误处理
│   │   ├── api.rs             # API 路由聚合
│   │   ├── service.rs         # 业务逻辑聚合
│   │   ├── models.rs          # 数据模型聚合
│   │   ├── infra.rs           # 基础设施聚合
│   │   ├── api/               # API 处理器
│   │   │   ├── auth_handler.rs
│   │   │   ├── note_handler.rs
│   │   │   └── comment_handler.rs
│   │   ├── service/           # 业务逻辑
│   │   │   ├── auth_service.rs
│   │   │   ├── note_service.rs
│   │   │   └── comment_service.rs
│   │   ├── models/            # 数据模型
│   │   │   ├── user.rs
│   │   │   ├── note.rs
│   │   │   ├── comment.rs
│   │   │   └── dtos.rs
│   │   └── infra/             # 基础设施
│   │       ├── db.rs
│   │       └── repositories/
│   ├── migrations/            # 数据库迁移
│   ├── Cargo.toml
│   └── .env
│
└── frontend/                   # Next.js 前端
    ├── src/
    │   ├── app/               # App Router (Next.js 15)
    │   │   ├── layout.tsx     # 根布局
    │   │   ├── page.tsx       # 首页
    │   │   ├── globals.css    # 全局样式
    │   │   ├── notes/         # 笔记相关页面
    │   │   ├── auth/          # 认证相关页面
    │   │   └── api/           # API 路由 (如需要)
    │   ├── components/        # 可复用组件
    │   ├── lib/               # 工具函数
    │   ├── types/             # TypeScript 类型
    │   └── hooks/             # 自定义 Hooks
    ├── package.json
    ├── next.config.js
    ├── tailwind.config.ts
    └── .env.local
```

## 🛠️ 开发环境设置

### 后端启动

1. 安装 Rust (如果还没有):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 进入后端目录:
```bash
cd backend
```

3. 安装依赖并运行:
```bash
cargo run
```

后端将在 `http://localhost:8000` 启动

### 前端启动

1. 安装 Node.js 18+ 和 pnpm:
```bash
npm install -g pnpm
```

2. 进入前端目录:
```bash
cd frontend
```

3. 安装依赖:
```bash
pnpm install
```

4. 启动开发服务器:
```bash
pnpm dev
```

前端将在 `http://localhost:3000` 启动

## 🔧 配置说明

### 后端配置 (.env)
```env
# 服务器配置
ROWAN_SERVER_HOST=127.0.0.1
ROWAN_SERVER_PORT=8000

# 数据库配置
ROWAN_DATABASE_URL=sqlite://./data.db

# JWT 配置
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
ROWAN_JWT_EXPIRES_IN=86400

# 日志级别
RUST_LOG=debug
```

### 前端配置 (.env.local)
```env
# API 基础URL
NEXT_PUBLIC_API_URL=http://localhost:8000/api

# 应用配置
NEXT_PUBLIC_APP_NAME="Rowan Web"
NEXT_PUBLIC_APP_DESCRIPTION="个人笔记分享平台"
```

## 📚 API 文档

### 认证接口
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `GET /api/auth/me` - 获取当前用户信息
- `POST /api/auth/refresh` - 刷新令牌

### 笔记接口
- `GET /api/notes` - 获取笔记列表
- `POST /api/notes` - 创建笔记
- `GET /api/notes/:id` - 获取单个笔记
- `PUT /api/notes/:id` - 更新笔记
- `DELETE /api/notes/:id` - 删除笔记
- `POST /api/notes/:id/like` - 点赞笔记
- `DELETE /api/notes/:id/unlike` - 取消点赞

### 评论接口
- `POST /api/comments` - 创建评论
- `GET /api/comments/:id` - 获取单个评论
- `PUT /api/comments/:id` - 更新评论
- `DELETE /api/comments/:id` - 删除评论
- `GET /api/comments/note/:note_id` - 获取笔记的评论列表

## 🎯 功能特性

### 已实现
- ✅ 用户注册和登录
- ✅ JWT 认证
- ✅ 笔记的 CRUD 操作
- ✅ 评论系统（支持回复）
- ✅ 点赞功能
- ✅ 分页查询
- ✅ 响应式设计

### 待实现
- [ ] 数据库迁移文件
- [ ] 用户头像上传
- [ ] 笔记标签系统
- [ ] 搜索功能
- [ ] 友链功能
- [ ] 邮件通知
- [ ] 深色模式
- [ ] 国际化 (i18n)

## 🔄 数据库迁移

使用 SeaORM CLI 管理数据库迁移:

```bash
# 安装 SeaORM CLI
cargo install sea-orm-cli

# 创建迁移
sea-orm-cli migrate generate <migration_name>

# 运行迁移
sea-orm-cli migrate up

# 回滚迁移
sea-orm-cli migrate down
```

## 🚀 部署

### 使用 Docker (推荐)

1. 构建并运行:
```bash
docker-compose up -d
```

### 手动部署

1. 后端部署:
```bash
cd backend
cargo build --release
./target/release/rowan-web-backend
```

2. 前端部署:
```bash
cd frontend
pnpm build
pnpm start
```

## 🤝 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## ✨ 致谢

- [Axum](https://github.com/tokio-rs/axum) - 现代化的 Rust Web 框架
- [SeaORM](https://github.com/SeaQL/sea-orm) - 异步 Rust ORM
- [Next.js](https://nextjs.org/) - React 生产级框架
- [Tailwind CSS](https://tailwindcss.com/) - 实用优先的 CSS 框架
