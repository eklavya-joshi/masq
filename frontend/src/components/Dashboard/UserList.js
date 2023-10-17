import React from "react";
import { useState, useEffect } from "react";
import Axios from "axios";
import Cookies from "js-cookie";

export default function UserList() {

    const [userList, setUserList] = useState([]);

    const fetchUsers = async () => {
        const { a } = await Axios.get(
            "http://localhost:8080/hello", {
            withCredentials: true,
            headers: { 'X-Requested-With': 'XMLHttpRequest' }
        },
        );
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
