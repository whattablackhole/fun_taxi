import React, { createContext, useContext, useMemo, useState } from "react";
import { AuthService } from "../services/auth-service";
import { UserData } from "../models/UserModels";

const AuthServiceContext = createContext<{
  authService: AuthService;
  userData: UserData | null;
} | null>(null);

export const AuthServiceProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const [userData, setUserData] = useState<UserData | null>(null);
  const authService = useMemo(() => new AuthService(setUserData), []);

  const value = {
    authService,
    userData,
  };

  return (
    <AuthServiceContext.Provider value={value}>
      {children}
    </AuthServiceContext.Provider>
  );
};

export const useAuthService = () => {
  const context = useContext(AuthServiceContext);
  if (!context) {
    throw new Error(
      "useAuthService must be used within an AuthServiceProvider"
    );
  }
  return context;
};
