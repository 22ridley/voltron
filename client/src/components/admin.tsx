import React from "react";

interface AdminProps {
  token: string;
  privilege: number;
}

export default function Admin(props: AdminProps) {
  return (
    <div>
      <div>Admin</div>
      <div>Token: {props.token}</div>
    </div>
  );
}
