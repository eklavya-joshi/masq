import React from "react";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import Axios from "axios";

export default function SendMessage({websocket, messageSent, setMessageSent}) {

    const params = useParams();

    const [messageContent, setMessageContent] = useState("");

    const handleSendMessage = async (e) => {
        e.preventDefault();
        websocket(JSON.stringify({ inbox: params.id, content: messageContent }))
        const { data } = await Axios.post(
            "http://localhost:8080/messages/send",
            {
                inbox: params.id,
                content: messageContent,
            },
            { withCredentials: true },
        );
        console.log(data);
        setMessageSent(messageSent + 1);
        setMessageContent("");
    }

    return (
        <div>
            <form>
                <input className="textfield" type="text" onChange={(e) => setMessageContent(e.target.value)} />
                <button className="def button" onClick={(e) => handleSendMessage(e)}>Send</button>
            </form>
        </div>
    )
}