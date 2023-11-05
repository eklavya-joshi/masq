import React from "react";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import Axios from "axios";
import Cookies from "js-cookie";
import { encryptMessage } from "../../aes";

export default function SendMessage({websocket, messageSent, setMessageSent, otherUserOnline, sharedSecret}) {

    const params = useParams();

    const [messageContent, setMessageContent] = useState("");

    const handleSendMessage = async (e) => {
        e.preventDefault();
        if (messageContent.length < 1) return;
        const created = new Date().toLocaleString('en-US', { hour: 'numeric', hour12: true, minute: 'numeric' });
        
        const socketObj = { author: Cookies.get("user_id"), content: messageContent, created: created };
        console.log(sharedSecret);
        const messageStr = encryptMessage(JSON.stringify(socketObj), sharedSecret);
        websocket(messageStr)

        // const reqObj = { inbox: params.id, content: messageContent, created: new Date() };
        // const { data } = await Axios.post(
        //     "http://localhost:8080/messages/send",
        //     reqObj,
        //     { withCredentials: true },
        // );

        setMessageSent(messageSent + 1);
        setMessageContent("");
    }

    const handleOtherUserOnline = () => {
        if (otherUserOnline) return ("");

        return ("pointer-events-none bg-gray-200")
    }

    return (
        <div>
            <form>
                <input className={`$ textfield ${handleOtherUserOnline()}`} 
                type="text" placeholder={otherUserOnline ? "" : "Other user is offline"} onChange={(e) => setMessageContent(e.target.value)} />
                <button className="def button" onClick={(e) => handleSendMessage(e)}>Send</button>
            </form>
        </div>
    )
}