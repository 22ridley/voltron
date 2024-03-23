import React from "react";
import "./App.css";
import SignIn from "./components/sign-in.tsx";
import Student from "./components/student.tsx";
import { useState } from "react";

function App() {
  const [token, setToken] = useState<string>("");
  const showSignIn = () => {
    if (window.location.pathname === "/") {
      return <SignIn token={token} setToken={setToken} />;
    }
  };
  const showStudent = () => {
    if (window.location.pathname === "/student") {
      return <Student token={token} setToken={setToken} />;
    }
  };
  return (
    <div>
      <div className="signIn">{showSignIn()}</div>
      <div className="student">{showStudent()}</div>
    </div>
  );
}

export default App;
