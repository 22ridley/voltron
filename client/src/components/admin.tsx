import React, { useEffect, useState } from "react";
import "../styles/admin.css";
import { firebaseConfig } from "./sign-in.tsx";

interface AdminProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Admin(props: AdminProps) {
  const [failMessage, setFailMessage] = useState<string>("");
  const [newInstructorName, setNewInstructorName] = useState<string>("");
  const [newClassID, setNewClassID] = useState<string>("");
  const [instructors, setInstructors] = useState<any[]>([]);
  const [count, setCount] = useState<number>(0);

  function handleSubmit() {
    setCount(count + 1);
    setFailMessage("");
    fetch(
      `${firebaseConfig.baseURL}/register_instructor?instr_name=${newInstructorName}&class_id=${newClassID}`,
      {
        method: "POST",
        headers: {
          Authorization: `Bearer ${props.token}`,
        },
      }
    ).then((response) => {
      response.json().then((response_json) => {
        const success: boolean = response_json.success;
        const message: string = response_json.message;
        if (!success) {
          setFailMessage(message);
        }
        getInstructors();
        setNewInstructorName("");
        setNewClassID("");
      });
    });
  }

  // Fetch instructors
  const getInstructors = () => {
    fetch(`${firebaseConfig.baseURL}/admin`, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${props.token}`,
      },
    }).then((response) => {
      response.json().then((response_json) => {
        const instr = response_json.instructors;
        setInstructors(instr);
      });
    });
  };

  // Fetch of instructors from backend at beginning
  useEffect(() => {
    getInstructors();
  }, [props.privilege]);
  return (
    <div>
      <div className="header">
        <a href="/">
          <h1>Voltron</h1>
        </a>
      </div>
      <div className="allI">
        <div>
          <div className="sideI">
            <h2>Admin</h2>
            <hr />
            <div className="instructor_list">
              <h4>Instructors:</h4>
              {instructors.map((instructor_json) => (
                <div className="instructor">
                  {instructor_json.name}: Class {instructor_json.class_id}
                </div>
              ))}
            </div>
          </div>
          <div className="mainI">
            <h2>Register a new instructor: </h2>
            <hr />
            <div className="register_instructor">
              <h4>Instructor name:</h4>
              <input
                value={newInstructorName}
                onChange={(val) => setNewInstructorName(val.target.value)}
              ></input>
              <h4>Class ID:</h4>
              <input
                value={newClassID}
                onChange={(val) => setNewClassID(val.target.value)}
              ></input>
              <br />
              <div className="submit_button_box">
                <button className="submit_button" onClick={handleSubmit}>
                  Submit
                </button>
              </div>
              <p className="error">{failMessage}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
