import { useRef, useState } from "react";
import "./SigningFormComponent.scss";

 
export interface SigningFormData {
  username?: string;
  password?: string;
  email?: string;
  eventType: "login" | "signup";
}

interface SigningFormComponentProps {
  handleSubmitForm: (data: SigningFormData) => any;
}

export function SigningFormComponent({ handleSubmitForm }: SigningFormComponentProps) {
  const userNameField = useRef<HTMLInputElement>(null);
  const passwordField = useRef<HTMLInputElement>(null);
  const emailField = useRef<HTMLInputElement>(null);
  const [eventType, setEventType] = useState<"login" | "signup">("login");
  return (
    <div className="container">
      {eventType === "login" ? (
        <div className="header">Log In</div>
      ) : (
        <div className="header">Create new account</div>
      )}
      <div className="content">
        {eventType === "login" ? (
          <>
            <div>
              <label>Username or Email</label>
              <input ref={userNameField}></input>
            </div>
            <div>
              <label>Password</label>
              <input ref={passwordField}></input>
            </div>
          </>
        ) : (
          <>
            <div>
              <label>Username</label>
              <input ref={userNameField}></input>
            </div>
            <div>
              <label>Password</label>
              <input ref={passwordField}></input>
            </div>
            <div>
              <label>Email</label>
              <input ref={emailField}></input>
            </div>
          </>
        )}

        <div>
          <button
            onClick={() =>
              handleSubmitForm({
                username: userNameField.current?.value,
                password: passwordField.current?.value,
                email: emailField.current?.value,
                eventType: eventType,
              })
            }
          >
            Continue
          </button>
        </div>
      </div>
      {eventType == "login" ? (
        <a onClick={() => setEventType("signup")}>Don't have account?</a>
      ) : (
        <a onClick={() => setEventType("login")}>Have an account?</a>
      )}
    </div>
  );
}
