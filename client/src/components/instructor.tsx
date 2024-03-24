import React, { Dispatch, SetStateAction, useEffect } from "react";

interface InstructorProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Instructor(props: InstructorProps) {
  return (
    <div>
      <div>Instructor: {props.name}</div>
      <div>Token: {props.token}</div>
    </div>
  );
}
