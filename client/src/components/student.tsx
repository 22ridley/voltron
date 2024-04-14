import React, { useEffect, useState } from "react";
import "../styles/student.css";
import { firebaseConfig } from "./firebase.tsx";

interface StudentProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Student(props: StudentProps) {
  const [bufferText, setBufferText] = useState<string>("");
  const [classID, setClassID] = useState<string>("");
  const [groupID, setGroupID] = useState<string>("");

  // Initial fetch of bufferText from backend
  useEffect(() => {
    fetch(`${firebaseConfig.baseURL}/student`, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${props.token}`,
      },
    })
      .then((response) => {
        response
          .json()
          .then((response_json) => {
            const contents: string = response_json.contents;
            const class_id: number = response_json.class_id;
            const group_id: number = response_json.group_id;
            setBufferText(contents);
            setClassID(class_id.toString());
            setGroupID(group_id.toString());
          })
          .catch((error) => console.log(error));
      })
      .catch((error) => console.log(error));
  }, [props.privilege, props.token]);

  // Updating backend based on bufferText
  useEffect(() => {
    if (bufferText != "") {
      fetch(`${firebaseConfig.baseURL}/update?text=${bufferText}`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${props.token}`,
        },
      });
    }
  }, [bufferText]);
  return (
    <div>
      <div className="header">
        <a href="/">
          <h1>Voltron</h1>
        </a>
      </div>
      <div className="all">
        <div className="labels">
          <p className="one-label">
            <b>Student:</b> {props.name}
          </p>
          <p className="one-label">
            <b>Email:</b> {props.email}
          </p>
          <p className="one-label">
            <b>Class ID:</b> {classID}{" "}
          </p>
          <p className="one-label">
            <b>Group ID:</b> {groupID}{" "}
          </p>
        </div>
        <textarea
          value={bufferText}
          onChange={(ev) => {
            setBufferText(ev.target.value);
          }}
        ></textarea>
      </div>
    </div>
  );
}
