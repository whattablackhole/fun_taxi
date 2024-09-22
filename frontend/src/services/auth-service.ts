import { TokenData, UserData } from "../models/UserModels";

const authBase = import.meta.env.VITE_AUTH_BASE_URL;

export class AuthService {
  private apiBase: string;
  private refreshToken: string | null = null;
  private accessToken: string | null = null;
  private userData: UserData | null = null;

  constructor(base: string = authBase) {
    this.apiBase = base;

    let refreshToken = localStorage.getItem("refresh_token");
    let accessToken = localStorage.getItem("access_token");
    let userData = localStorage.getItem("user_data");

    // naive approach without validating

    if (refreshToken && accessToken) {
      this.refreshToken = refreshToken;
      this.accessToken = accessToken;
    }

    if (userData) {
      this.userData = JSON.parse(userData);
    }

  }


  public getUserData() {
    return this.userData;
  }

  public async authenticate() {
    if (this.isAccessTokenValid()) {
      return true;
    } else if (this.isRefreshTokenValid()) {
      return await this.refreshAccessToken();
    } else {
      return false;
    }
  }

  public async signup(username: string, password: string, email: string) {
    const response = await fetch(`${this.apiBase}/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        username,
        password,
        email,
      }),
    });

    if (response.ok) {
      const data: UserData = await response.json();
      this.setUser(data);
      return await this.login(username, password);
    }
  }

  public async login(username: string, password: string) {
    const formBody = new URLSearchParams();
    formBody.append("username", username);
    formBody.append("password", password);

    const response = await fetch(`${this.apiBase}/token`, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded"
      },
      body: formBody.toString(),
    })

    if (response.ok) {
      const data: TokenData = await response.json();
      this.setTokens(data);
      return true;
    }
  }

  private async refreshAccessToken() {
    await fetch(`${this.apiBase}/refresh/${this.refreshToken}`)
      .then((response) => {
        if (response.ok) {
          return response.json();
        }
      })
      .then((data: TokenData) => {
        this.setTokens(data);
        return true;
      })
      .catch(() => {
        this.accessToken = null;
        this.refreshToken = null;
        return false;
      });
  }

  private setTokens(data: TokenData) {
        this.accessToken = data.access_token;
        this.refreshToken = data.refresh_token;
        localStorage.setItem("refresh_token", data.refresh_token);
        localStorage.setItem("access_token", data.access_token);
  }

  private setUser(data: UserData) {
    this.userData = data;
    localStorage.setItem("user", JSON.stringify(data));
  }

  private isAccessTokenValid() {
    const payload = this.decodeTokenPayload(this.accessToken);
    return payload?.exp && payload.exp > Date.now() / 1000 && this.accessToken;
  }

  private isRefreshTokenValid() {
    const payload = this.decodeTokenPayload(this.refreshToken);
    return payload?.exp && payload.exp > Date.now() / 1000;
  }

  private decodeTokenPayload(token: string | null) {
    if (!token) return null;

    const base64Url = token.split(".")[1];
    const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");
    const jsonPayload = decodeURIComponent(
      atob(base64)
        .split("")
        .map(function (c) {
          return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
        })
        .join("")
    );

    return JSON.parse(jsonPayload);
  }

}
