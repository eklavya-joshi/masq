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
    const [newMessage, setNewMessage] = useState(undefined);
    const [otherUserOnline, setOtherUserOnline] = useState(true);
    
    const fetchMessages = async () => {
        // const { data } = await Axios.get(
        //     "http://localhost:8080/messages/find?inbox=" + params.id, {
        //     withCredentials: true,
        // },
        // );
        // const messages = data.messages;
        // setMessages(messages);
    };

    const receiveMessage = (e) => {
        const str = e.data;
        console.log(str);
        if (str.includes("joined") || str.includes("left")) return;
        if (str.includes("current_users")) {
            console.log(str);
            const data = JSON.parse(str);
            setOtherUserOnline(Number(data.current_users) > 1);
            return;
        }
        const data = JSON.parse(str.substring(str.indexOf(" ") + 1));
        setNewMessage(data);
    }

    const websocket = useWebsocket(receiveMessage, params.id);

    useEffect(() => {
        fetchMessages();
    }, []);

    useEffect(() => {
        // fetchMessages();
    }, [messageSent]);

    useEffect(() => {
    }, [messages]);

    useEffect(() => {
        if (newMessage === undefined) return;
        setMessages([...messages, newMessage]);
    }, [newMessage]);

    const createOnlineStatus = () => {
        const dotStyle1 = "w-3 h-3 mt-1 ml-1 rounded-full ";
        const dotStyle2 = otherUserOnline ? dotStyle1 + "bg-green-600" : dotStyle1 + "bg-gray-500";
        return (
        <div className="flex flex-row place-items-center">
            Online: {otherUserOnline.valueOf()}<div className={dotStyle2} />
        </div>
        );
    }

    return (
    <div className="grid w-screen h-screen place-items-center">
        <div className="absolute flex flex-col w-max h-3/5 bg-gray-300">
            {createOnlineStatus()}
            <Messages messages={messages}/>
            <SendMessage 
                websocket={websocket} 
                messageSent={messageSent} 
                setMessageSent={setMessageSent}
                otherUserOnline={otherUserOnline}/>
        </div>
    </div>
    )
}