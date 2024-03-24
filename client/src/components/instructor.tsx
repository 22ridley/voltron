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
  const [classID, setClassID] = useState<string>("");
  const [students, setStudents] = useState<any[]>([]);
  const [studentGroups, setStudentGroups] = useState<any[]>([]);
  // Initial fetch of bufferText from backend
  useEffect(() => {
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
            {/* <form action="/register-student" method="post" accept-charset="utf-8">
                <input type="hidden" name="instructor_name" value={{name}} />
                <input type="hidden" name="class_id" value={{class_id}} />
                <label>Student name:
                  <p>
                    <input name="student_name" />
                  </p>
                </label>
                <label>Student group ID:
                  <p>
                    <input name="group_id" />
                  </p>
                </label>
                <input type="submit" value="Submit" />
              </form>
                <h4>
                  Failed to register new student:
                </h4> */}
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
