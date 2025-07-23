export interface AuthContextType {
  user: User | null
  token: string | null
  login: (email: string, password: string) => Promise<void>
  register: (username: string, email: string, password: string) => Promise<void>
  logout: () => void
  loading: boolean
}

export interface User {
  id: string
  username: string
  email: string
  avatar_url?: string
  bio?: string
  created_at: string
}
