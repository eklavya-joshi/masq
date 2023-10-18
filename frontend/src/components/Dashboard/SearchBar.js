import React from "react";
import { useState } from "react";
import { FiSearch } from "react-icons/fi";
import Axios from "axios";
import UserInfo from "./UserInfo";

export default function SearchBar() {

    const [search, setSearch] = useState("");
    const [userList, setUserList] = useState([]);

    const handleSearchButton = async () => {
        if (search.length === 0) return;
        const { data } = await Axios.get(
            "http://localhost:8080/users/find?name=" + search, {
            withCredentials: true,
        },
        );
        const userList = data.users;
        setUserList(userList);
    }

    const userMap = () => {
        return userList.map((user, index) => {
            const separator = index === 0 ? <></> : <div className="flex h-1 w-[450px] bg-black opacity-25 m-0 rounded-xl" />;
            return (
                <>
                    {separator}
                    <UserInfo name={user.name} created={user.name} />
                </>
            )
        })
    }

    return (
        <>
        <div className="inline-flex items-center justify-end">
        <input   className="inline-flex items-center justify-end 
                            h-16 w-96 mt-6 rounded-full
                          bg-white place-items-center
                            indent-4 text-2xl
                            focus:outline focus:outline-offset-0 focus:outline-secondary"
                  onChange={(e) => setSearch(e.target.value)}
                  placeholder="Search for a user">
        </input>
            <button className="absolute flex h-14 w-14 mr-2 mt-6
                            border-transparent rounded-full bg-primary text-white 
                            place-content-center place-items-center
                            hover:bg-secondary transition-all"
                    onClick={(_) => handleSearchButton()}>
            <FiSearch size="24" />
        </button>
        </div>
        <div className="flex h-4 w-[500px] bg-gray-300 m-8 mb-4 rounded-xl" />
            {userMap()}
        </>
    )
}