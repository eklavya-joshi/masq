import React from "react";
import { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Axios from "axios";
import Cookies from "js-cookie";

export default function UserList() {

    const navigate = useNavigate();
    const params = useParams();

    const [messages, setMessages] = useState([]);
    const [messageContent, setMessageContent] = useState("");
    const [messageSent, setMessageSent] = useState(0);

    const fetchMessages = async () => {
        console.log(params)
        const { data } = await Axios.get(
            "http://localhost:8080/messages/find?inbox=" + params.id, {
            withCredentials: true,
        },
        );
        const messages = data.messages;
        setMessages(messages);
        console.log(messages);
    };

    const handleSendMessage = async (e) => {
        e.preventDefault();
        const { data } = await Axios.post(
            "http://localhost:8080/messages/send",
            {
                inbox: params.id,
                content: messageContent, },
            { withCredentials: true },
        );
        console.log(data);
        setMessageSent(messageSent + 1);
    }


    useEffect(() => {
        fetchMessages();
    }, [messageSent]);
    


    return (
        <>
            <div>
                <form>
                    <input className="textfield" type="text" onChange={(e) => setMessageContent(e.target.value)} />
                    <button className="def button" onClick={(e) => handleSendMessage(e)}>Send</button>
                </form>
            </div>
            {messages.map(message => {
                return (
                    <div>
                        <p className="text-black">
                            {message.authorName + ": " + message.content}
                        </p>
                    </div>
                )
            })}
        </>
    )
}
