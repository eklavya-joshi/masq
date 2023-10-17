import React from "react";
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Axios from "axios";
import Cookies from "js-cookie";

export default function UserList() {

    const navigate = useNavigate();

    const [userList, setUserList] = useState([]);

    const fetchUsers = async () => {
        const { data } = await Axios.get(
            "http://localhost:8080/users/find?name=", {
            withCredentials: true,
        },
        );
        const userList = data.users;
        setUserList(userList);
        console.log(userList);
    };

    const handleUserClick = async (target) => {
        const { data } = await Axios.post(
            "http://localhost:8080/messages/new", 
            { target },
            { withCredentials: true },
        );
        console.log(data.dm);
        navigate("/dm/" + data.dm);
    };

    useEffect(() => {
        fetchUsers();
    }, []);


    return (
        <>
            {userList.map(user => {
                return (
                    <div>
                        <button className="def button" onClick={() => handleUserClick(user.name)}>
                            {user.name}
                        </button>
                    </div>
                )
            })}
        </>
    )
}
