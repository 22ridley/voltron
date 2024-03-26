import React, { Dispatch, SetStateAction, useEffect, useState } from "react";
import "../styles/instructor.css";
import { firebaseConfig } from "./sign-in.tsx";

interface InstructorProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Instructor(props: InstructorProps) {
  const [newStudentName, setNewStudentName] = useState<string>("");
  const [newStudentGroup, setNewStudentGroup] = useState<string>("");
  const [failMessage, setFailMessage] = useState<string>("");
  const [classID, setClassID] = useState<string>("");
  const [students, setStudents] = useState<any[]>([]);
  const [studentGroups, setStudentGroups] = useState<any[]>([]);

  // Handle submit
  function handleSubmit() {
    setFailMessage("");
    fetch(
      `${firebaseConfig.baseURL}/register_student?stud_group=${newStudentGroup}&stud_name=${newStudentName}&stud_class=${classID}`,
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
        setNewStudentName("");
        setNewStudentGroup("");
      });
    });
  }

  // Fetch students
  const getInstructors = () => {
    fetch(`${firebaseConfig.baseURL}/instructor`, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${props.token}`,
      },
    }).then((response) => {
      response.json().then((response_json) => {
        const class_id: number = response_json.class_id;
        const students = response_json.students;
        const student_groups = response_json.student_groups;
        setClassID(class_id.toString());
        setStudents(students);
        setStudentGroups(student_groups);
        console.log(students);
        console.log(students[0].group_id);
        console.log(student_groups);
      });
    });
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
          <h3>Instructor: {props.name}</h3>
          <hr />
          <div className="register_student">
            <h3>Register a new student:</h3>
            <div className="register_instructor">
              <h4>Student name:</h4>
              <input
                value={newStudentName}
                onChange={(val) => setNewStudentName(val.target.value)}
              ></input>
              <h4>Class ID:</h4>
              <input
                value={newStudentGroup}
                onChange={(val) => setNewStudentGroup(val.target.value)}
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
          <h3>Class ID: {classID}</h3>
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
