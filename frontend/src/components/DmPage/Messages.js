import React from "react";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import Axios from "axios";
import Cookies from "js-cookie";

export default function UserList({messages}) {

    const user = Cookies.get('user_id');

    const displayMessages = () => {
        return (
            messages.map(message => {
                const align = message.author !== user ? 'flex flex-col place-items-left w-64 mx-3' : 'flex flex-col place-items-end w-64 mx-3';
                let bubble = 'w-max bg-blue-800 rounded-full';
                if (message.author === "Error") {
                    bubble = 'w-max bg-red-800 rounded-full';
                } else if (message.author !== user) {
                    bubble = 'w-max bg-gray-500 rounded-full';
                }
                return (
                    <div className={align}>
                        <div className="text-gray-500 ml-1">
                            {message.created}
                        </div>
                        <div className={bubble}>
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
        <div className="flex flex-col place-items-left h-max overflow-auto scroll-smooth">
            {displayMessages()}
        </div>
    )
}
