import React, { useEffect } from "react";
import "./App.css";
import SignIn from "./components/sign-in.tsx";
import Student from "./components/student.tsx";
import Admin from "./components/admin.tsx";
import Instructor from "./components/instructor.tsx";
import { useState } from "react";

function App() {
  const [privilege, setPrivilege] = useState<number>(-1);
  const [token, setToken] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [name, setName] = useState<string>("");

  if (privilege == -1) {
    return (
      <div className="signIn">
        <SignIn
          setToken={setToken}
          setPrivilege={setPrivilege}
          setEmail={setEmail}
          setName={setName}
        />
      </div>
    );
  } else if (privilege == 0) {
    return (
      <div className="student">
        <Student
          token={token}
          privilege={privilege}
          email={email}
          name={name}
        />
      </div>
    );
  } else if (privilege == 1) {
    return (
      <div className="instructor">
        <Instructor
          token={token}
          privilege={privilege}
          email={email}
          name={name}
        />
      </div>
    );
  } else {
    return (
      <div className="admin">
        <Admin token={token} privilege={privilege} email={email} name={name} />
      </div>
    );
  }
}

export default App;
