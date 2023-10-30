import React from "react";
import Messages from "./Messages";
import SendMessage from "./SendMessage";

export default function DmPage() {
    return (
    <div className="grid w-screen h-screen place-items-center">
        <div className="flex flex-col w-max h-max bg-gray-300">
            <Messages/>
            <SendMessage />
        </div>
    </div>
    )
}