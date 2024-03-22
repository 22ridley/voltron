import { signInWithGooglePopup } from "./utils/firebase.utils";
// import {
//   getAuth,
//   GoogleAuthProvider,
//   onAuthStateChanged,
//   signInWithPopup,
//   signOut,
// } from "firebase/auth";

const SignIn = () => {
  const logGoogleUser = async () => {
    const response = await signInWithGooglePopup();
    console.log(response);
    const bearerToken = await response._tokenResponse;
    console.log(bearerToken);
  };
  return (
    <div>
      <button onClick={logGoogleUser}>Sign In With Google</button>
    </div>
  );
};
export default SignIn;
