export interface User {
  email: string;
  username: string;
  token: string;
  created_at?: string;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterCredentials extends LoginCredentials {
  username: string;
}

export interface ApiResponse {
  user: User;
}
