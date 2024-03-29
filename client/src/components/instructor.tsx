import React, { useEffect, useState } from "react";
import "../styles/instructor.css";
import { firebaseConfig } from "./sign-in.tsx";

interface InstructorProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Instructor(props: InstructorProps) {
  const [newName, setNewName] = useState<string>("");
  const [newEmail, setNewEmail] = useState<string>("");
  const [newGroup, setNewGroup] = useState<string>("");
  const [failMessage, setFailMessage] = useState<string>("");
  const [classID, setClassID] = useState<string>("");
  const [students, setStudents] = useState<any[]>([]);
  const [studentGroups, setStudentGroups] = useState<any[]>([]);

  // Handle submit
  function handleSubmit() {
    setFailMessage("");
    fetch(
      `${firebaseConfig.baseURL}/register_student?stud_group=${newGroup}&stud_name=${newName}&stud_class=${classID}&stud_email=${newEmail}`,
      {
        method: "POST",
        headers: {
          Authorization: `Bearer ${props.token}`,
        },
      }
    )
      .then((response) => {
        response
          .json()
          .then((response_json) => {
            const success: boolean = response_json.success;
            const message: string = response_json.message;
            if (!success) {
              setFailMessage(message);
            }
            getInstructors();
            setNewName("");
            setNewGroup("");
            setNewEmail("");
          })
          .catch((error) => console.log(error));
      })
      .catch((error) => console.log(error));
  }

  // Fetch students
  const getInstructors = () => {
    fetch(`${firebaseConfig.baseURL}/instructor`, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${props.token}`,
      },
    })
      .then((response) => {
        response
          .json()
          .then((response_json) => {
            const class_id: number = response_json.class_id;
            const students = response_json.students;
            const student_groups = response_json.student_groups;
            setClassID(class_id.toString());
            setStudents(students);
            setStudentGroups(student_groups);
          })
          .catch((error) => console.log(error));
      })
      .catch((error) => console.log(error));
  };

  // Initial fetch of students from backend
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
      <div>
        <div className="sideI">
          <h3 className="regStud">Register a new student:</h3>
          <hr />
          <div className="register_student">
            <div className="register_instructor">
              <p className="reg">Student name:</p>
              <input
                value={newName}
                onChange={(val) => setNewName(val.target.value)}
              ></input>
              <p className="reg">Email:</p>
              <input
                value={newEmail}
                onChange={(val) => setNewEmail(val.target.value)}
              ></input>
              <p className="reg">Group ID:</p>
              <input
                value={newGroup}
                onChange={(val) => setNewGroup(val.target.value)}
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
          <hr />
          <div className="student_list">
            <h4>Students:</h4>
            <div>{}</div>
            {students.map((student_json) => (
              <div className="student">
                {student_json.name}: {student_json.group_id}
              </div>
            ))}
          </div>
        </div>
        <div className="mainI">
          <div className="labelsI">
            <p className="one-labelI">
              <b>Instructor:</b> {props.name}
            </p>
            <p className="one-labelI">
              <b>Email:</b> {props.email}
            </p>
            <p className="one-labelI">
              <b>Class ID:</b> {classID}
            </p>
          </div>
          <hr />
          <h3>Student Code:</h3>
          <div className="grid">
            {studentGroups.map((group_json) => (
              <div className="grid_item">
                <b>
                  Group ID:&nbsp;
                  {group_json.group_id}
                </b>
                <p className="code">{group_json.code}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
