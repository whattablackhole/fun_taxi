import { useNavigate } from "react-router-dom";
import { SigningFormComponent, SigningFormData } from "../components/signing/SigningFormComponent";
import { useAuthService } from "../contexts/AuthServiceContext";

export function AuthView() {
    const {authService} = useAuthService();
    const navigate = useNavigate();

    const handleSubmitForm = async (data: SigningFormData) => {
        if (data.eventType === "login" && data.username && data.password) {
            const result = await authService.login(data.username, data.password);
            if (result) {
                navigate("/");
            }
        } else if (data.email && data.username && data.password) {
            const result = await authService.signup(data.username, data.password, data.email);
            if (result) {
                navigate("/");
            }
        }
    };

    return <SigningFormComponent handleSubmitForm={handleSubmitForm}></SigningFormComponent>;
}