export interface TokenData {
  access_token: string;
  refresh_token: string;
}

export type RoleName = "driver" | "user";

export interface UserRole {
  id: string;
  name: RoleName;
}

export interface UserData {
  id: string;
  username: string;
  email: string;
  is_active: string;
  roles: UserRole[];
}

export interface AuthData extends TokenData {
  user: UserData;
}
