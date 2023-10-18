import React from "react";
import { useNavigate } from "react-router-dom";
import Axios from "axios";

export default function UserInfo({name, created}) {

    const navigate = useNavigate();

    const handleUserClick = async (target) => {
        const { data } = await Axios.post(
            "http://localhost:8080/messages/new",
            { target },
            { withCredentials: true },
        );
        console.log(data.dm);
        navigate("/dm/" + data.dm);
    };

    return (
        <div className="inline-flex items-center justify-between h-12 w-[500px] my-2">
            <div className="flex flex-row">
            <div className="flex h-12 w-12 mx-2 my-1
                                rounded-full bg-secondary"/>
            <div className="def w-30 ml-2 text-white text-lg">
                {name}
                <br />
                <span className="def text-gray-300">created: {created}</span>
            </div>
            </div>
            <button className="flex h-full w-32 mr-2 
                                   place-content-center place-items-center
                                   border-transparent rounded-full bg-secondary
                                   text-white font-bold
                                   hover:bg-white transition-all hover:text-primary"
                    onClick={(_) => handleUserClick(name)}>
                CREATE DM
            </button>
        </div>
    )
}