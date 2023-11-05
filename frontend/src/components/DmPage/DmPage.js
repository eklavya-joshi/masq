import React from "react";
import { useEffect, useState } from "react";
import Messages from "./Messages";
import SendMessage from "./SendMessage";
import useWebsocket from "../../hooks/useWebsocket";
import { useParams } from "react-router-dom";
import Axios from "axios";
import { decryptMessage } from "../../aes";

const crypto = require('crypto-browserify');

global.Buffer = global.Buffer || require("buffer").Buffer;

export default function DmPage() {

    const params = useParams();

    const [messages, setMessages] = useState([]);
    const [messageSent, setMessageSent] = useState(0);
    const [newMessage, setNewMessage] = useState(undefined);

    const [otherUserOnline, setOtherUserOnline] = useState(false);

    const k = crypto.createECDH('secp256k1');
    k.generateKeys();

    const [key, setKey] = useState(k);
    const [generateKey, setGenerateKey] = useState(0);
    const [otherUserKey, setOtherUserKey] = useState("");
    const [sharedSecret, setSharedSecret] = useState("");

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
        // console.log(str);
        if (str.includes("joined") || str.includes("left") || str === "undefined") return;
        if (str.includes("pKeys")) {
            const data = JSON.parse(str);
            const other = data.pKeys.a === k.getPublicKey().toString('base64') ? data.pKeys.b : data.pKeys.a;
            if (other === "") return;
            setOtherUserKey(other);
            const secret = key.computeSecret(other, 'base64', 'hex');
            setSharedSecret(secret);
            return;
        }
        if (str.includes("current_users")) {
            const data = JSON.parse(str);
            setOtherUserOnline(Number(data.current_users) > 1);
            return;
        }
        setNewMessage(str);
    }

    const websocket = useWebsocket(receiveMessage, params.id);

    let keyObj = { key: key.getPublicKey().toString('base64') };
    websocket(JSON.stringify(keyObj));

    useEffect(() => {
        fetchMessages();
    }, []);

    useEffect(() => {
        // fetchMessages();
    }, [messageSent]);

    useEffect(() => {
        console.log("my key       : ", key.getPublicKey().toString('base64'));
        console.log("other key    : ", otherUserKey);
        console.log("shared secret: ", sharedSecret);
    }, [messages, sharedSecret]);

    useEffect(() => {
        if (newMessage === undefined) return;
        console.log("secret:", sharedSecret)
        const decrypted = decryptMessage(newMessage.substring(newMessage.indexOf(" ") + 1), sharedSecret);
        console.log(newMessage.substring(newMessage.indexOf(" ") + 1));
        console.log(decrypted);
        try {
            const data = JSON.parse(decrypted);
            console.log(data);
            setMessages([...messages, data]);
        } catch {
            const data = {
                author: "error",
                content: "Couldn't decrypt",
                created: new Date().toLocaleString('en-US', { hour: 'numeric', hour12: true, minute: 'numeric' })
            };
            setMessageSent([...messages, data]);
        }
    }, [newMessage]);

    const createOnlineStatus = () => {
        const dotStyle1 = "w-3 h-3 mt-1 ml-1 rounded-full ";
        const dotStyle2 = otherUserOnline ? dotStyle1 + "bg-green-600" : dotStyle1 + "bg-gray-500";
        return (<div>
        <div className="flex flex-row place-items-center">
            Online: {otherUserOnline.valueOf()}<div className={dotStyle2} />
        </div>
        <div className="break-all">
            {/* Your private key : {key.getPrivateKey().toString('base64')}<br/>
            Your public key  : {key.getPublicKey().toString('base64')} */}
        </div>
        </div>);
    }

    return (
    <div className="grid w-screen h-screen place-items-center">
        <div className="absolute flex flex-col place-items-center place-content-between w-72 h-3/5 bg-gray-300">
            {createOnlineStatus()}
            <div className="max-h-20">
                <Messages messages={messages}/>
                <SendMessage 
                    websocket={websocket} 
                    messageSent={messageSent} 
                    setMessageSent={setMessageSent}
                    otherUserOnline={otherUserOnline}
                    sharedSecret={sharedSecret}/>
            </div>
        </div>
    </div>
    )
}