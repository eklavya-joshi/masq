import Cookies from "js-cookie";
import React from "react";
import Axios from "axios";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function LoginPage() {

    const navigate = useNavigate();

    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const handleLoginSubmit = async (e) => {
        e.preventDefault();
        const data = {
            username: username,
            password: password
        };
        const response = await fetch("http://localhost:8080/users/login", {
            method: "POST",
            headers: {
                "content-type": "application/json",
            },
            credentials: "include",
            body: JSON.stringify(data),
        });
        console.log(response);
        if (response.ok) {
            response.json().then((a) => Cookies.set("token", a.token));
            response.headers.getSetCookie();
            navigate("/dashboard");
        }
    }

    const handleRegisterSubmit = async (e) => {
        e.preventDefault();
        const { data } = await Axios.post(
            "http://localhost:8080/users/create",
            {
                username: username,
                password: password
            },
            { withCredentials: true },
        );
    }

    return (
        <div className="grid place-items-center bg-violet-400 w-1/6 h-1/3">
            <form className="">
            <label className="def block text-white" >
                Username
            </label>
                <input className="textfield" type="text" onChange={(e) => setUsername(e.target.value)} />
            <label className="def block text-white">
                Password
            </label>
                <input className="def textfield" type="password" onChange={(e) => setPassword(e.target.value)} />
            <div className="flex justify-end">
                <button className="def button"
                    onClick={(e) => handleRegisterSubmit(e)}>
                    Register
                </button>
                <button className="def button" 
                onClick={(e) => handleLoginSubmit(e)}>
                    Login
                </button>
            </div>
            </form>
        </div>
    )

}