import React, { Dispatch, SetStateAction, useEffect } from "react";

interface InstructorProps {
  token: string;
  privilege: number;
}

export default function Instructor(props: InstructorProps) {
  useEffect(() => {
    console.log("In instructor: ", props.token);
  }, [props.token]);

  return (
    <div>
      <div>Instructor</div>
      <div>Token: {props.token}</div>
    </div>
  );
}
