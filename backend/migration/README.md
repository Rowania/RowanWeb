# 数据库迁移 (Migration)

## 概述

`migration` 目录包含了博客项目的数据库 schema 变更脚本。我使用 **SeaORM 的迁移工具** 来管理数据库版本。

---

## 数据库设计概览

### notes_metadata

| 字段名       | 数据类型                     | 约束                                | 备注              |
| ------------ | ---------------------------- | ----------------------------------- | ----------------- |
| id           | integer                      | PRIMARY KEY, AUTOINCREMENT          | 唯一 ID           |
| file_id      | uuid_text                    | NOT NULL, UNIQUE                    | 文件唯一标识符    |
| slug         | varchar(255)                 | NOT NULL, UNIQUE                    | URL 友好名称      |
| title        | varchar(255)                 | NOT NULL                            | 笔记/文章标题     |
| summary      | text                         | NULL                                | 摘要              |
| published_at | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP | 发布时间          |
| updated_at   | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP | 更新时间          |
| views        | integer                      | NOT NULL, DEFAULT 0                 | 浏览量            |
| likes_count  | integer                      | NOT NULL, DEFAULT 0                 | 点赞数            |
| tags         | text                         | NULL                                | 标签 (如逗号分隔) |
| category     | text                         | NULL                                | 分类              |

**用途**: 存储博客文章的结构化信息，如标题、摘要、发布时间、阅读量和点赞数。文章的实际内容存储在文件系统中，此表只存储其元数据。

**关键点**:

- `likes_count` 字段直接存储了文章的点赞总数，方便快速读取。

---

### comments (评论表)

| 字段名             | 数据类型                     | 约束                                                           | 备注                     |
| ------------------ | ---------------------------- | -------------------------------------------------------------- | ------------------------ |
| id                 | integer                      | PRIMARY KEY, AUTOINCREMENT                                     | 唯一 ID                  |
| note_metadata_id   | integer                      | NULL, FOREIGN KEY (notes_metadata.id) ON DELETE CASCADE        | 关联笔记元数据 ID        |
| essay_id           | integer                      | NULL, FOREIGN KEY (essays.id) ON DELETE CASCADE                | 关联文章 ID              |
| visitor_profile_id | integer                      | NOT NULL, FOREIGN KEY (visitor_profiles.id) ON DELETE RESTRICT | 关联访客资料 ID          |
| content            | text                         | NOT NULL                                                       | 评论内容                 |
| parent_id          | integer                      | NULL, FOREIGN KEY (comments.id) ON DELETE SET NULL             | 父评论 ID (用于嵌套评论) |
| created_at         | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP                            | 创建时间                 |
| is_approved        | boolean                      | NOT NULL, DEFAULT FALSE                                        | 是否审核通过             |

**用途**: 存储用户在文章下的留言内容。

**关键点**:

- **匿名评论**: 用户无需登录即可评论。
- **昵称绑定**: 通过 `visitor_profile_id` 关联到 `visitor_profiles` 表，每次显示评论时都会从 `visitor_profiles` 获取评论者当前最新的昵称。这意味着如果访客更改了昵称，其所有历史评论的昵称也会随之更新。
- **支持嵌套回复**: 使用 `parent_id` 字段实现嵌套回复。

---

### likes (点赞记录表)

| 字段名           | 数据类型    | 约束                                                        | 备注              |
| ---------------- | ----------- | ----------------------------------------------------------- | ----------------- |
| id               | integer     | PRIMARY KEY, AUTOINCREMENT                                  | 唯一 ID           |
| note_metadata_id | integer     | NOT NULL, FOREIGN KEY (notes_metadata.id) ON DELETE CASCADE | 关联笔记元数据 ID |
| ip_address       | varchar(45) | NOT NULL                                                    | 点赞者 IP 地址    |

**用途**: 记录每篇笔记的点赞行为，最主要用于限制重复点赞。

**关键点**:

- **完全匿名**: 不存储任何用户身份信息（如 `visitor_profile_id`），也不存储点赞时间。
- **防重复机制**: 包含 `note_metadata_id` 和 `ip_address` 字段，并通过这两个字段的联合唯一约束来防止同一个 IP 地址对同一篇文章重复点赞。
- **不统计数量**: 该表不直接提供点赞总数，点赞总数存储在 `notes_metadata` 表的 `likes_count` 字段中。

---

### friends_links (友链表)

| 字段名      | 数据类型                     | 约束                                | 备注          |
| ----------- | ---------------------------- | ----------------------------------- | ------------- |
| id          | integer                      | PRIMARY KEY, AUTOINCREMENT          | 唯一 ID       |
| name        | varchar(15)                  | NOT NULL                            | 链接名称      |
| url         | varchar(255)                 | NOT NULL, UNIQUE                    | 链接 URL      |
| description | varchar(100)                 | NULL                                | 链接描述      |
| logo_url    | varchar(255)                 | NULL                                | Logo 图片 URL |
| sort_order  | integer                      | NOT NULL                            | 排序顺序      |
| created_at  | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP | 创建时间      |
| updated_at  | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP | 更新时间      |

**用途**: 存储你的博客友情链接信息。

**关键点**:

- 字段长度根据实际需求做了优化，例如:
  - `name` (100 字符)
  - `url` (500 字符)
  - `description` (255 字符)
  - `logo_url` (500 字符)
- 所有时间戳都采用 UTC 时区。

---

### essays

| 字段名     | 数据类型                     | 约束                                | 备注     |
| ---------- | ---------------------------- | ----------------------------------- | -------- |
| id         | integer                      | PRIMARY KEY, AUTOINCREMENT          | 唯一 ID  |
| title      | varchar(20)                  | NULL                                | 文章标题 |
| content    | text                         | NOT NULL                            | 文章内容 |
| created_at | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP | 创建时间 |
| updated_at | timestamp_with_timezone_text | NOT NULL, DEFAULT CURRENT_TIMESTAMP | 更新时间 |

**用途**: 存储博客文章的内容。

**关键点**:

- 包含文章标题和内容。
- 提供创建和更新时间戳。
