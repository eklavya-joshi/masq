import React from "react";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import Axios from "axios";

export default function UserList() {

    const params = useParams();

    const [messages, setMessages] = useState([]);

    const fetchMessages = async () => {
        console.log(params)
        const { data } = await Axios.get(
            "http://localhost:8080/messages/find?inbox=" + params.id, {
            withCredentials: true,
        },
        );
        const messages = data.messages;
        setMessages(messages);
    };

    useEffect(() => {
        fetchMessages();
    }, []);

    const displayMessages = () => {
        return (
            messages.map(message => {
                return (
                    <div className="flex flex-col place-items-left ml-3">
                        <div className="text-gray-500 ml-1">
                            {message.created}
                        </div>
                        <div className="w-max bg-blue-800 text-white rounded-full">
                            <div className="text-white px-4 py-1">
                                {message.content}
                            </div>
                        </div>
                    </div>
                )
            })
        )
    }

    return (
        <div className="flex flex-col place-items-left">
            {displayMessages()}
        </div>
    )
}
