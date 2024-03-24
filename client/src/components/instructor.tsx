import React, { Dispatch, SetStateAction, useEffect, useState } from "react";
import { firebaseConfig } from "./sign-in.tsx";

interface InstructorProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Instructor(props: InstructorProps) {
  const [classID, setClassID] = useState<string>("");
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
        const group_id: number = response_json.group_id;
        const students = response_json.students;
        const student_groups = response_json.student_groups;
        setClassID(class_id.toString());
        console.log(students);
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
      <div className="all">
        <div>
          <div className="side">
            <h2>Instructor: {props.name}</h2>
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
              <h3>Students:</h3>
              <div>{}</div>
              {/* {{#each students}}
                <div class="student">
                  {{this.name}}:
                  {{this.group_id}}
                </div>
              {{/each}} */}
            </div>
          </div>
          <div className="main">
            <h2>Class ID: {classID}</h2>
            <hr />
            <h2>Student Code:</h2>
            <div className="grid">
              {/* {{#each student_groups}}
                <div class="grid_item">
                  <b>
                    Group ID:
                    {{this.group_id}}
                  </b>
                  <p class="code">
                    {{this.code}}
                  </p>
                </div>
              {{/each}} */}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
