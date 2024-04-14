Policies:

- Only administrators can create classes and only instructors can enroll students to a class (and assign them to a group)
- A group’s buffer is only accessible to the group’s members and the class’ instructor

How to set up Firebase:

- In server/src, create a file called firebase-credentials.json.
- Follow [these](https://firebase.google.com/docs/web/setup#add-sdk-and-initialize) instructions on initializing your Firebase account.
- Find and place the following information into the firebase-credentials.json:

```
{
  "type": ...,
  "project_id": ...,
  "private_key_id": ...,
  "private_key": ...,
  "client_email": ...,
  "client_id": ...,
  "auth_uri": ...,
  "token_uri": ...,
  "auth_provider_x509_cert_url": ...,
  "client_x509_cert_url": ...
}
```

- In client/src/components, create a file called firebase.tsx.
- In Firebase, go to Project settings -> General -> Your apps, and scroll down to find your firebase configuration. Copy and paste that section into firebase.tsx.

```
// Your web app's Firebase configuration
const firebaseConfig = {
  apiKey: ...,
  authDomain: ...,
  projectId: ...,
  storageBucket: ...,
  messagingSenderId: ...,
  appId: ...
};
```

How to run locally:

- To run the front end, `cd client`, then `npm install`, then `npm start`.
- To run the back end, `cd server`, then `cargo run`.
