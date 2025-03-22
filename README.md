# Voltron

## Policies

- Only administrators can create classes and only instructors can enroll students to a class (and assign them to a group)
- A group’s buffer is only accessible to the group’s members and the class’ instructor

## How to set up Firebase

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

## Running

- To run the front end, `cd client`, then `npm install`, then `npm start`.
- To run the back end, `cd server`, then `cargo run`.

## Running tests

```bash
cd server/
cargo test -- --test-threads=1
```

## Running Scrutinizer

Currently, the `build.rs` for running scrutinizer is broken. Instead, scrutinizer must be run manually.

First, install scrutinizer by cloning `https://github.com/brownsys/scrutinizer` to some location `<scrutinizer_dir>` on your machine, and then running
```bash
cd <scrutinizer_dir>
./scripts/scrutinizer-install
```

Then, `cd <voltron_dir>/server`.

If you have already built the code, e.g., to run or test voltron, you will need to clean all intermediate build files, otherwise they conflict with scrutinizer's workflow.
```bash
cargo clean
rm -rf mir_dumps
```

Then, you can run scrutinizer using:
```bash
RUST_BACKTRACE=full RUST_LOG=scrutinizer=trace,scrutils=trace cargo +nightly-2023-08-25 scrutinizer --config-path scrutinizer-config.toml
```

On first run, Scrutinizer will take a long time without displaying much output as it builds the voltron code and its dependencies. Eventually, after all building is completed, scrutinizer will begin analyzing the pure regions, and will output a log message to the terminal for each region it analyzes.
