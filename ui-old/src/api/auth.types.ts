export interface ResponseAuth {
  token: string;
  user: User;
}

export interface RegisterUser {
  email: string;
  password: string;
  first_name: string;
  last_name: string;
}

export interface User {
  email: string;
  role: string;
  first_name: string;
  last_name: string;
  profile_picture?: string;
}
