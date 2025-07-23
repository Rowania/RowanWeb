我要写一个个人网站，初步计划如下：

### 代码层面：

- AI 使用 context7 来获取最新的 API，库和框架
- 使用 Rust 语言编写后端
- 使用 React，Next.js 框架编写前端
- 数据库使用 SQLite，sea-orm-cli 来进行迁移

### 应用层面：

- 我要发布笔记，一些日常记录和笔记分享
- 别人可以发表评论，点赞
- 还可以引用友链

### 后端框架

    backend/
    ├── src/
    │   ├── main.rs                 # 应用入口点，启动 Web 服务器，初始化数据库连接
    │   ├── error.rs                # 统一错误处理，定义应用特有的错误类型
    │   ├── config.rs               # 应用配置管理，加载环境变量等
    │   │
    │   ├── api.rs                  # API 层 (Web 接入层) 的聚合文件
    │   │                           # 内容会 pub use 各个 handler，并可能提供一个总的路由配置函数
    │   │
    │   ├── api/                    # API 层 具体实现文件
    │   │   ├── auth_handler.rs     # 认证相关 HTTP 请求处理函数
    │   │   ├── note_handler.rs     # 笔记相关 HTTP 请求处理函数
    │   │   └── comment_handler.rs  # 评论相关 HTTP 请求处理函数
    │   │
    │   ├── service.rs              # 业务逻辑层 (核心服务层) 的聚合文件
    │   │                           # 内容会 pub use 各个 service_* 文件
    │   │
    │   ├── service/                # 业务逻辑层 具体实现文件
    │   │   ├── auth_service.rs     # 认证业务逻辑
    │   │   ├── note_service.rs     # 笔记业务逻辑
    │   │   └── comment_service.rs  # 评论业务逻辑
    │   │
    │   ├── infra.rs                # 基础设施层 (技术实现层) 的聚合文件
    │   │                           # 内容会 pub use db, repositories 等
    │   │
    │   ├── infra/                  # 基础设施层 具体实现文件
    │   │   ├── db.rs               # 数据库连接池管理与初始化
    │   │   ├── repositories/       # 数据仓库层
    │   │   │   ├── mod.rs          # repository 模块聚合文件
    │   │   │   ├── note_repo.rs    # 笔记数据访问接口实现
    │   │   │   └── comment_repo.rs # 评论数据访问接口实现
    │   │   └── mailer.rs           # 邮件发送服务 (可选)
    │   │
    │   ├── models.rs               # 数据模型定义 (领域模型 / DTOs / SeaORM Entity) 的聚合文件
    │   │                           # 内容会 pub use 各个 model_* 文件
    │   │
    │   ├── models/                 # 数据模型具体定义文件
    │   │   ├── user.rs             # 用户实体 (SeaORM Entity)
    │   │   ├── note.rs             # 笔记实体 (SeaORM Entity)
    │   │   ├── comment.rs          # 评论实体 (SeaORM Entity)
    │   │   └── dtos.rs             # API 请求与响应数据传输对象 (DTOs)
    │   │
    │   └── lib.rs                  # 公共工具函数或共享模块
    │
    ├── migrations/                 # SeaORM 数据库迁移文件
    │   ├── Cargo.toml
    ├── src/
    │   │   ├── lib.rs
    │   │   └── <timestamp>_initial_setup.rs # 具体的迁移脚本
    │   │
    │   └── README.md
    │
    ├── Cargo.toml                  # 后端项目依赖与配置
    ├── Cargo.lock
    └── .env                        # 环境变量 (如 DATABASE_URL, 服务端口等)

前端框架

    frontend/
    ├── public/              # 静态文件，如图片、字体等，可以直接通过 URL 访问
    ├── src/
    │   ├── app/             # Next.js 13+ App Router 结构，基于文件系统的路由
    │   │   ├── layout.tsx   # 根布局文件，定义所有页面的共享 UI
    │   │   ├── page.tsx     # 根页面文件，通常是网站首页
    │   │   │
    │   │   ├── notes/       # 笔记相关的页面和组件，例如 /notes 路由
    │   │   │   ├── [slug]/  # 动态路由，用于单篇笔记详情页 (例如 /notes/my-first-note)
    │   │   │   │   └── page.tsx  # 单篇笔记详情页的组件
    │   │   │   └── page.tsx      # 笔记列表页的组件 (例如 /notes)
    │   │   │
    │   │   ├── comments/    # 评论相关的组件或页面 (如果评论有独立的路由或复杂组件)
    │   │   │   └── (例如：CommentList.tsx, CommentForm.tsx)
    │   │   │
    │   │   ├── links/       # 友链页面和组件 (例如 /links 路由)
    │   │   │   └── page.tsx
    │   │   │
    │   │   ├── api/         # Next.js API 路由 (如果前端需要自己的 API 端点，例如处理表单提交)
    │   │   │   └── route.ts # API 路由文件 (例如 /api/submit-form)
    │   │   │
    │   │   └── globals.css  # 全局样式文件，用于引入 Tailwind CSS 指令和其他自定义全局样式
    │   │
    │   ├── components/      # 可复用 React UI 组件，不与特定路由绑定
    │   │   ├── Header.tsx   # 网站头部组件
    │   │   ├── Footer.tsx   # 网站底部组件
    │   │   ├── NoteCard.tsx # 用于显示笔记摘要的卡片组件
    │   │   └── CommentSection.tsx # 评论区组件，包含评论列表和评论表单
    │   │
    │   ├── lib/             # 前端工具函数、API 客户端等辅助代码
    │   │   └── api.ts       # 调用后端 Rust API 的函数，封装数据请求逻辑
    │   │
    │   └── types/           # TypeScript 类型定义文件，用于增强代码可读性和健壮性
    │
    ├── next.config.js       # Next.js 配置文件，用于自定义构建行为
    ├── package.json         # 前端项目配置文件，管理依赖和脚本
    ├── pnpm-lock.yaml       # pnpm 包管理器的锁定文件 (如果使用 pnpm)
    ├── postcss.config.js    # PostCSS 配置文件，Tailwind CSS 需要 PostCSS
    ├── tailwind.config.ts   # Tailwind CSS 配置文件，用于自定义主题、插件等
    ├── tsconfig.json        # TypeScript 配置文件
    └── README.md            # 前端项目说明文件
