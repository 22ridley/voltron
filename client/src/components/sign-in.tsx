// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries
import {
  getAuth,
  GoogleAuthProvider,
  onAuthStateChanged,
  signInWithPopup,
  User,
} from "firebase/auth";
import React, { Dispatch, SetStateAction, useEffect, useState } from "react";

interface SignInProps {
  token: string;
  setToken: Dispatch<SetStateAction<string>>;
  privilege: number;
  setPrivilege: Dispatch<SetStateAction<number>>;
}

// Your web app's Firebase configuration
const firebaseConfig = {
  apiKey: "AIzaSyBhZq1cflqUOXts2-1dtCDl7j-NBhpE7tw",
  authDomain: "voltron-1ea5c.firebaseapp.com",
  projectId: "voltron-1ea5c",
  storageBucket: "voltron-1ea5c.appspot.com",
  messagingSenderId: "1074514211093",
  appId: "1:1074514211093:web:ce0aa886c2224ccbb9b91c",
  baseURL: "http://127.0.0.1:8000",
};

export default function SignIn(props: SignInProps) {
  const [jsonResponse, setJsonResponse] = useState<any>(null);

  useEffect(() => {
    if (jsonResponse) {
      console.log("In signin json: ", jsonResponse);
      console.log("In signin token: ", props.token);
      const success: boolean = jsonResponse.success;
      const prvlg: number = jsonResponse.privilege;
      const user_name: string = jsonResponse.name;
      const email: string = jsonResponse.email;
      props.setPrivilege(prvlg);
      if (success == false) {
        //window.location.href = "/";
      } else {
        if (prvlg == 2) {
          //window.location.href = "/admin";
        } else if (prvlg == 1) {
          //window.location.href = "/";
        } else {
          //window.location.href = "/student";
        }
      }
    }
  }, [jsonResponse]);

  const logGoogleUser = async () => {
    const app = initializeApp(firebaseConfig);

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
            // Make a POST request with the `Authorization` header set with our bearer token
            Authorization: `Bearer ${userToken}`,
          },
        });
        const json_response = await response.json();
        console.log(json_response);
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
            // IdP data available using getAdditionalUserInfo(result)
            // ...
          })
          .catch((error) => {
            // Handle Errors here.
            const errorCode = error.code;
            const errorMessage = error.message;
            // The email of the user's account used.
            const email = error.customData.email;
            // The AuthCredential type that was used.
            const credential = GoogleAuthProvider.credentialFromError(error);
            // ...
            console.log("error!");
          });
      }
    });
  };
  return (
    <div>
      <button onClick={logGoogleUser}>Sign In With Google</button>
    </div>
  );
}
function setState<T>(arg0: null): [any, any] {
  throw new Error("Function not implemented.");
}
