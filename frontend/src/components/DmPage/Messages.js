import React from "react";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import Axios from "axios";
import Cookies from "js-cookie";

export default function UserList({messages}) {

    const user = Cookies.get('user');

    const displayMessages = () => {
        return (
            messages.map(message => {
                const align = message.author !== user ? 'flex flex-col place-items-left mx-3' : 'flex flex-col place-items-end mx-3';
                const bubble = message.author !== user ? 'w-max bg-gray-500 rounded-full' : 'w-max bg-blue-800 rounded-full';
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
