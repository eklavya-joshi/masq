import React from "react";
import SearchBar from "./SearchBar";
import Banner from "./Banner";

export default function DmPage() {
    return (
    <div className="">
        <Banner/>
        <div className="flex flex-col h-96 w-screen bg-secondary"></div>
        <div className="flex flex-col h-auto w-screen bg-primary place-items-center">
            <SearchBar/>
        </div>
    </div>
    )
}