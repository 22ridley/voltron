import "./App.css";
import SignIn from "./sign-in.component";

const showSignIn = () => {
  if (window.location.pathname === "/") {
    return <SignIn />;
  }
};

function App() {
  return <div className="App">{showSignIn()}</div>;
}

export default App;
