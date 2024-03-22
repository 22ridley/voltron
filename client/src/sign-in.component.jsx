// import { signInWithGooglePopup } from "./utils/firebase.utils";
// // import {
// //   getAuth,
// //   GoogleAuthProvider,
// //   onAuthStateChanged,
// //   signInWithPopup,
// //   signOut,
// // } from "firebase/auth";

// const SignIn = () => {
//   const logGoogleUser = async () => {
//     const response = await signInWithGooglePopup();
//     console.log(response);
//     const bearerToken = await response._tokenResponse;
//     console.log(bearerToken);
//   };
//   return (
//     <div>
//       <button onClick={logGoogleUser}>Sign In With Google</button>
//     </div>
//   );
// };
// export default SignIn;

// Import the functions you need from the SDKs you need
import { initializeApp } from "https://www.gstatic.com/firebasejs/10.9.0/firebase-app.js";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries
import {
  getAuth,
  GoogleAuthProvider,
  signInWithRedirect,
  onAuthStateChanged,
  signInWithPopup,
} from "https://www.gstatic.com/firebasejs/10.9.0/firebase-auth.js";

// Your web app's Firebase configuration
const firebaseConfig = {
  apiKey: "AIzaSyBhZq1cflqUOXts2-1dtCDl7j-NBhpE7tw",
  authDomain: "voltron-1ea5c.firebaseapp.com",
  projectId: "voltron-1ea5c",
  storageBucket: "voltron-1ea5c.appspot.com",
  messagingSenderId: "1074514211093",
  appId: "1:1074514211093:web:ce0aa886c2224ccbb9b91c",
};

const SignIn = () => {
  const logGoogleUser = async () => {
    // const response = await signInWithPopup();
    // console.log(response);
    // const bearerToken = await response._tokenResponse;
    // console.log(bearerToken);
    // Initialize Firebase
    const app = initializeApp(firebaseConfig);

    var provider = new GoogleAuthProvider();
    const auth = getAuth();

    onAuthStateChanged(auth, (user) => {
      if (user) {
        // User is signed in, see docs for a list of available properties
        // https://firebase.google.com/docs/reference/js/auth.user
        const uid = user.uid;
        console.log(user);
        console.log(user.displayName);
      } else {
        // User is signed out
        signInWithPopup(auth, provider)
          .then((result) => {
            // This gives you a Google Access Token. You can use it to access the Google API.
            const credential = GoogleAuthProvider.credentialFromResult(result);
            const token = credential.accessToken;
            // The signed-in user info.
            const user = result.user;
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
};
export default SignIn;
