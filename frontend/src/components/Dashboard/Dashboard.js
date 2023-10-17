import React from "react";
import { useState, useEffect } from "react";
import Axios from "axios";
import UserList from "./UserList";

export default function LandingPage() {

    const [userList, setUserList] = useState([]);
    const [button, setButton] = useState(1);

    const handleRegisterSubmit = async (e) => {
        // e.preventDefault();
        // const response = await fetch("http://localhost:8080/users/find?name=", {
        //     method: "GET",
        //     headers: {
        //         "content-type": "application/json",
        //     },
        //     credentials: "include",
        // });
        // setButton(button + 1);
    }

    return (
        <>
            <div className="grid h-screen place-items-center">
                <button className="def button"
                    onClick={(e) => handleRegisterSubmit(e)}>
                    Register
                </button>
                <UserList/>
            </div>
        </>
    )
}
