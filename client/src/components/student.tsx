import React from "react";

interface StudentProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Student(props: StudentProps) {
  return <div>Student: {props.name}</div>;
}
