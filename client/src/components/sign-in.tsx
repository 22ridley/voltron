import "../styles/sign-in.css";
import { initializeApp } from "firebase/app";
import {
  getAuth,
  GoogleAuthProvider,
  onAuthStateChanged,
  signInWithPopup,
  User,
  signOut,
} from "firebase/auth";
import React, { Dispatch, SetStateAction, useEffect, useState } from "react";

interface SignInProps {
  setToken: Dispatch<SetStateAction<string>>;
  setPrivilege: Dispatch<SetStateAction<number>>;
  setEmail: Dispatch<SetStateAction<string>>;
  setName: Dispatch<SetStateAction<string>>;
}

const firebaseConfig = {
  apiKey: "AIzaSyBhZq1cflqUOXts2-1dtCDl7j-NBhpE7tw",
  authDomain: "voltron-1ea5c.firebaseapp.com",
  projectId: "voltron-1ea5c",
  storageBucket: "voltron-1ea5c.appspot.com",
  messagingSenderId: "1074514211093",
  appId: "1:1074514211093:web:ce0aa886c2224ccbb9b91c",
  baseURL: "http://127.0.0.1:8000",
};

const firebaseApp = initializeApp(firebaseConfig);

export default function SignIn(props: SignInProps) {
  const [jsonResponse, setJsonResponse] = useState<any>(null);

  useEffect(() => {
    if (jsonResponse) {
      const success: boolean = jsonResponse.success;
      const privilege: number = jsonResponse.privilege;
      const email: string = jsonResponse.email;
      const name: string = jsonResponse.name;
      if (success) {
        props.setPrivilege(privilege);
        props.setEmail(email);
        props.setName(name);
      }
    }
  }, [jsonResponse]);

  const handleSignOut = async () => {
    const auth = getAuth(firebaseApp);
    // Sign out of firebase
    await signOut(auth);
    // Set the local state back to it's initial state
    props.setToken("");
  };

  const logGoogleUser = async () => {
    var provider = new GoogleAuthProvider();
    const auth = getAuth();

    onAuthStateChanged(auth, async (user: User | null) => {
      if (user) {
        // User is signed in
        const userToken: string = await user.getIdToken();
        props.setToken(userToken);
        const response = await fetch(`${firebaseConfig.baseURL}/login`, {
          method: "GET",
          headers: {
            Authorization: `Bearer ${userToken}`,
          },
        });
        const json_response = await response.json();
        setJsonResponse(json_response);
      } else {
        // User is signed out
        signInWithPopup(auth, provider)
          .then((result) => {
            // This gives you a Google Access Token. You can use it to access the Google API.
            const credential = GoogleAuthProvider.credentialFromResult(result);
            if (credential) {
              const token = credential.accessToken;
              // The signed-in user info.
              const user = result.user;
            }
          })
          .catch((error) => {
            console.log("Error: ", error);
          });
      }
    });
  };
  return (
    <div>
      <div className="header">
        <a href="/">
          <h1>Voltron</h1>
        </a>
      </div>
      <div className="all">
        <div className="loginbox">
          <h2>Login</h2>
          <button onClick={logGoogleUser}>Sign In With Google</button>
          <br />
          <button onClick={handleSignOut}>Clear Last Sign In</button>
        </div>
      </div>
    </div>
  );
}
