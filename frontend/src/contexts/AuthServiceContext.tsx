import React, { createContext, useContext } from 'react';
import { AuthService } from '../services/auth-service';



const AuthServiceContext = createContext<AuthService | null>(null);

export const AuthServiceProvider = ({ children }: { children: React.ReactNode }) => {
    const authService = new AuthService();
    return (
        <AuthServiceContext.Provider value={authService}>
            {children}
        </AuthServiceContext.Provider>
    );
};

export const useAuthService = () => {
    const context = useContext(AuthServiceContext);
    if (!context) {
        throw new Error('useAuthService must be used within an AuthServiceProvider');
    }
    return context;
};