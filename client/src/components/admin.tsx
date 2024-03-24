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
  const [instructors, setInstructors] = useState<any[]>([]);
  // Initial fetch of instructors from backend
  useEffect(() => {
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
            {/* <div className="register_instructor">
                    <form
                        action="/register-instructor"
                        method="post"
                        accept-charset="utf-8"
                    >
                        <label>Instructor name:
                        <p>
                            <input name="instructor_name" />
                        </p>
                        </label>
                        <label>Class ID:
                        <p>
                            <input name="class_id" />
                        </p>
                        </label>
                        <input type="submit" value="Submit" />
                    </form>
                    {{#if fail}}
                        <h4>
                        <b>Failed to register new instructor:</b>
                        {{{fail_message}}}
                        </h4>
                    {{/if}}
                    </div>
                </div> */}
          </div>
        </div>
      </div>
    </div>
  );
}
