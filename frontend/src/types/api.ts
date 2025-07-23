// API 类型定义
export interface User {
  id: string
  username: string
  email: string
  avatar_url?: string
  bio?: string
  created_at: string
}

export interface Note {
  id: string
  title: string
  content: string
  summary?: string
  author_id: string
  status: string
  tags?: any
  views_count: number
  likes_count: number
  created_at: string
  updated_at: string
}

export interface Comment {
  id: string
  content: string
  note_id: string
  author_id: string
  parent_id?: string
  created_at: string
  updated_at: string
}

// API 请求类型
export interface RegisterRequest {
  username: string
  email: string
  password: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface CreateNoteRequest {
  title: string
  content: string
  summary?: string
  status?: string
  tags?: any
}

export interface UpdateNoteRequest {
  title?: string
  content?: string
  summary?: string
  status?: string
  tags?: any
}

export interface CreateCommentRequest {
  content: string
  note_id: string
  parent_id?: string
}

export interface UpdateCommentRequest {
  content: string
}

// API 响应类型
export interface AuthResponse {
  user: User
  token: string
  message: string
}

export interface ApiResponse<T = any> {
  data?: T
  message?: string
  error?: string
  status?: number
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  per_page: number
}

// 分页查询参数
export interface PaginationQuery {
  page?: number
  per_page?: number
}
