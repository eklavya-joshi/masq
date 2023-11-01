import React from "react";
import { useEffect, useState } from "react";
import Messages from "./Messages";
import SendMessage from "./SendMessage";
import useWebsocket from "../../hooks/useWebsocket";
import { useParams } from "react-router-dom";
import Axios from "axios";

export default function DmPage() {

    const params = useParams();

    const [messages, setMessages] = useState([]);
    const [messageSent, setMessageSent] = useState(0);
    
    const fetchMessages = async () => {
        const { data } = await Axios.get(
            "http://localhost:8080/messages/find?inbox=" + params.id, {
            withCredentials: true,
        },
        );
        const messages = data.messages;
        setMessages(messages);
    };

    const websocket = useWebsocket(fetchMessages, params.id);

    useEffect(() => {
        fetchMessages();
    }, []);

    useEffect(() => {
        console.log("poaisdfjaopsidfho")
        fetchMessages();
    }, [messageSent]);

    useEffect(() => {
        
    }, [messages]);

    return (
    <div className="grid w-screen h-screen place-items-center">
        <div className="flex flex-col w-max h-3/5 bg-gray-300">
            <Messages messages={messages} setMessages={setMessages}/>
            <SendMessage websocket={websocket} messageSent={messageSent} setMessageSent={setMessageSent}/>
        </div>
    </div>
    )
}