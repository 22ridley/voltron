import React from "react";

interface AdminProps {
  token: string;
  privilege: number;
  email: string;
  name: string;
}

export default function Admin(props: AdminProps) {
  return (
    <div>
      <div>Admin: {props.name}</div>
      <div>Token: {props.token}</div>
    </div>
  );
}
