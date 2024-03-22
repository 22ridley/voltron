import { initializeApp } from "firebase/app";
import { getAuth, signInWithPopup, GoogleAuthProvider } from "firebase/auth";

const firebaseConfig = {
  apiKey: "AIzaSyBhZq1cflqUOXts2-1dtCDl7j-NBhpE7tw",
  authDomain: "voltron-1ea5c.firebaseapp.com",
  projectId: "voltron-1ea5c",
  storageBucket: "voltron-1ea5c.appspot.com",
  messagingSenderId: "1074514211093",
  appId: "1:1074514211093:web:ce0aa886c2224ccbb9b91c",
};
const app = initializeApp(firebaseConfig);

// Initialize Firebase
const firebaseApp = initializeApp(firebaseConfig);
// Initialize Firebase Auth provider
const provider = new GoogleAuthProvider();

// whenever a user interacts with the provider, we force them to select an account
// provider.setCustomParameters({
//     prompt : "select_account "
// });
export const auth = getAuth();
export const signInWithGooglePopup = () => signInWithPopup(auth, provider);
