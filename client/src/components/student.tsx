import React from "react";
import "../styles/student.css";

interface StudentProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Student(props: StudentProps) {
  const text = "This is my code";
  return (
    <div>
      <div className="header">
        <a href="/">
          <h1>Voltron</h1>
        </a>
      </div>
      <div className="all">
        <h2>Student: {props.name}</h2>
        <h2>Group ID: </h2>
        <form action="/update" method="post" accept-charset="utf-8">
          <textarea>{text}</textarea>
          <br />
          <br />
          <input type="submit" value="Update" />
        </form>
      </div>
    </div>
  );
}
